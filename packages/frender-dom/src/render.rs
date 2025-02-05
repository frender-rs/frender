use frender_common::{
    convert::{FromMut, IntoMut},
    impl_many, TempStr,
};
pub use frender_csr::render::{RenderContext, RenderWithContext};

use crate::{string_element::StringElement, ui_handle::UiHandle};

pub trait RenderContextRenderTextFrom<V>: RenderContext {
    fn render_text_from(&mut self, v: V) -> <Self::Renderer as RenderTextFrom<V>>::Text
    where
        Self::Renderer: RenderTextFrom<V>; // TODO: move the where bound to trait
}

pub trait RenderTextFrom<V>:
    for<'a> RenderWithContext<RenderContext<'a>: RenderContextRenderTextFrom<V>>
{
    type Text: UiHandle<Self> + 'static;

    fn update_text_from(&mut self, text: &mut Self::Text, v: V);
}

/// Trait alias for [`RenderTextFrom`] with all known primitive types.
pub trait RenderTextFromKnown:
    RenderTextFrom<&'static str>
    + RenderTextFrom<StringElement>
    + RenderTextFrom<i8>
    + RenderTextFrom<u8>
    + RenderTextFrom<i16>
    + RenderTextFrom<u16>
    + RenderTextFrom<i32>
    + RenderTextFrom<u32>
    + RenderTextFrom<i64>
    + RenderTextFrom<u64>
    + RenderTextFrom<i128>
    + RenderTextFrom<u128>
    + RenderTextFrom<isize>
    + RenderTextFrom<usize>
    + RenderTextFrom<f32>
    + RenderTextFrom<f64>
    + RenderTextFrom<char>
    //
    + for<'a>RenderTextFrom<TempStr<&'a str>,Text = Self::TextFromTempStr>
    + for<'a>RenderTextFrom<&'a StringElement,Text = Self::TextFromRefStringElement>
{
    type TextFromTempStr: UiHandle<Self> + 'static;
    type TextFromRefStringElement: UiHandle<Self> + 'static;
}

impl<
        R: ?Sized,
        TextFromTempStr: UiHandle<Self> + 'static,
        TextFromRefStringElement: UiHandle<Self> + 'static,
    > RenderTextFromKnown for R
where
    R: RenderTextFrom<&'static str>
        + for<'a> RenderTextFrom<TempStr<&'a str>, Text = TextFromTempStr>
        + RenderTextFrom<StringElement>
        + for<'a> RenderTextFrom<&'a StringElement, Text = TextFromRefStringElement>
        + RenderTextFrom<i8>
        + RenderTextFrom<u8>
        + RenderTextFrom<i16>
        + RenderTextFrom<u16>
        + RenderTextFrom<i32>
        + RenderTextFrom<u32>
        + RenderTextFrom<i64>
        + RenderTextFrom<u64>
        + RenderTextFrom<i128>
        + RenderTextFrom<u128>
        + RenderTextFrom<isize>
        + RenderTextFrom<usize>
        + RenderTextFrom<f32>
        + RenderTextFrom<f64>
        + RenderTextFrom<char>,
{
    type TextFromTempStr = TextFromTempStr;
    type TextFromRefStringElement = TextFromRefStringElement;
}

pub trait RenderIntoTextKnownKind {
    type RenderIntoText<R: ?Sized + RenderTextFromKnown>: UiHandle<R> + 'static;
}

pub trait RenderIntoTextKnown: Sized {
    type RenderIntoTextKnownKind: RenderIntoTextKnownKind;
    type StaticRenderIntoTextKnown: 'static
        + RenderIntoTextKnown<RenderIntoTextKnownKind = Self::RenderIntoTextKnownKind>;
    type RenderTextFromSelf<R: ?Sized + RenderTextFromKnown>: ?Sized
        + for<'a> RenderWithContext<
            RenderContext<'a>: FromMut<R::RenderContext<'a>> + IntoMut<R::RenderContext<'a>>,
        > + RenderTextFrom<
            Self,
            Text = <Self::RenderIntoTextKnownKind as RenderIntoTextKnownKind>::RenderIntoText<R>,
        > + RenderTextFrom<
            Self::StaticRenderIntoTextKnown,
            Text = <Self::RenderIntoTextKnownKind as RenderIntoTextKnownKind>::RenderIntoText<R>,
        > + FromMut<R>
        + IntoMut<R>;
}

pub trait SimpleStaticRenderIntoTextKnown: 'static + Sized {
    type SimpleStaticRenderIntoText<R: ?Sized + RenderTextFromKnown>: UiHandle<R> + 'static;
    type SimpleStaticRenderTextFromSelf<R: ?Sized + RenderTextFromKnown>: ?Sized
        + for<'a> RenderWithContext<
            RenderContext<'a>: FromMut<R::RenderContext<'a>> + IntoMut<R::RenderContext<'a>>,
        > + RenderTextFrom<Self, Text = Self::SimpleStaticRenderIntoText<R>>
        + FromMut<R>
        + IntoMut<R>;
}

impl_many!(
    impl<__> SimpleStaticRenderIntoTextKnown
        for each_of![
            &'static str,
            StringElement,
            i8,
            u8,
            i16,
            u16,
            i32,
            u32,
            i64,
            u64,
            i128,
            u128,
            isize,
            usize,
            f32,
            f64,
            char,
        ]
    {
        type SimpleStaticRenderIntoText<R: ?Sized + RenderTextFromKnown> =
            <R as RenderTextFrom<Self>>::Text;
        type SimpleStaticRenderTextFromSelf<R: ?Sized + RenderTextFromKnown> = R;
    }
);

/// Self as Kind
impl<T: SimpleStaticRenderIntoTextKnown> RenderIntoTextKnownKind for T {
    type RenderIntoText<R: ?Sized + RenderTextFromKnown> = T::SimpleStaticRenderIntoText<R>;
}

impl<T: SimpleStaticRenderIntoTextKnown> RenderIntoTextKnown for T {
    type RenderIntoTextKnownKind = T;
    type StaticRenderIntoTextKnown = T;
    type RenderTextFromSelf<R: ?Sized + RenderTextFromKnown> = T::SimpleStaticRenderTextFromSelf<R>;
}

impl RenderIntoTextKnownKind for TempStr<&'static str> {
    type RenderIntoText<R: ?Sized + RenderTextFromKnown> = R::TextFromTempStr;
}

impl RenderIntoTextKnown for TempStr<&str> {
    type RenderIntoTextKnownKind = TempStr<&'static str>;
    type StaticRenderIntoTextKnown = TempStr<&'static str>;
    type RenderTextFromSelf<R: ?Sized + RenderTextFromKnown> = R;
}

impl RenderIntoTextKnownKind for &'static StringElement {
    type RenderIntoText<R: ?Sized + RenderTextFromKnown> = R::TextFromRefStringElement;
}

impl RenderIntoTextKnown for &StringElement {
    type RenderIntoTextKnownKind = &'static StringElement;
    type StaticRenderIntoTextKnown = &'static StringElement;
    type RenderTextFromSelf<R: ?Sized + RenderTextFromKnown> = R;
}

pub trait Render: RenderWithContext {
    fn log(&mut self, v: &str);

    type CursorPlaceholder: 'static
        + UiHandle<Self>
        + crate::behaviors::NodeRenderSelf<Self>
        + crate::behaviors::NodeWithRenderContextAfterSelf<Self>
        + crate::behaviors::Node<Self>;
}
