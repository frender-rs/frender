use frender_common::{
    convert::{FromMut, IdentityAs, IntoMut},
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
    + for<'a> RenderTextFrom<TempStr<&'a str>, Text = Self::TextFromTempStr>
    + RenderTextFrom<StringElement>
    + for<'a> RenderTextFrom<&'a StringElement, Text = Self::TextFromRefStringElement>
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

pub trait RenderIntoTextKnown: Sized {
    type RenderTextFromSelf<R: ?Sized + RenderTextFromKnown>: ?Sized
        + RenderTextFrom<Self, Text: UiHandle<R>>
        + FromMut<R>
        + IntoMut<R>;
}

impl_many!(
    impl<__> RenderIntoTextKnown
        for each_of![
            &'static str,
            TempStr<&str>,
            StringElement,
            &StringElement,
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
        type RenderTextFromSelf<R: ?Sized + RenderTextFromKnown> = R;
    }
);

pub trait Render: RenderWithContext {
    fn log(&mut self, v: &str);

    type CursorPlaceholder: 'static
        + UiHandle<Self>
        + crate::behaviors::NodeRenderSelf<Self>
        + crate::behaviors::NodeWithRenderContextAfterSelf<Self>
        + crate::behaviors::Node<Self>;
}
