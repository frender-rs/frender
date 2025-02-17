use std::{borrow::Cow, rc::Rc, sync::Arc};

use frender_common::{
    value_kind::{
        KindOfOwned, KindOfRef, KindOfStaticRefOrTempOwned, StaticRefOrTempOwned, ValueKind,
    },
    TempStr,
};
use frender_csr_core::render::RenderWithContext;

use crate::{string_element::StringElement, ui_handle::UiHandle};

pub trait RenderTextFrom<V>: RenderWithContext {
    type Text: UiHandle<Self> + 'static;

    fn render_text_from(render_context: &mut Self::RenderContext<'_>, v: V) -> Self::Text;

    fn update_text_from(&mut self, text: &mut Self::Text, v: V);
}

macro_rules! text_ty {
    ($From:ty,) => {
        <R as RenderTextFrom<$From>>::Text
    };
    ($From:ty,$Text:ident) => {
        R::$Text
    };
}

macro_rules! impl_known_kinds {
    (
        $KnownValueKind:ident
        text($Text:ty)
        kinds($($Kind:ty,)+)
    ) => {
        $(
            impl $KnownValueKind for $Kind {
                type Text<R: ?Sized + RenderTextFromKnown> = $Text;
            }
        )+
    };
}

macro_rules! expand_first {
    ({$($t:tt)*}$($rest:tt)*) => {
        $($t)*
    };
}

macro_rules! define {
    (
        $(#$attr:tt)*
        $vis:vis trait $Trait:ident : $Bound:ident {
            $(
                type From $(<$lt:lifetime>)? = $From:ty;
                $(type Text = $Text:ident;)?
                $(type Kind = $Kind:ty;)+
            )*
        }

        impl $(<__>)? $KnownValueKind:ident for Kind
        {}

        impl $(<__>)? $RenderIntoText:ident for Value
        {}
    ) => {
        $(#$attr)*
        $vis trait $Trait:
        $(
            $(for <$lt>)? $Bound<$From, $(Text = Self::$Text)?> +
        )*
        {
            $($(type $Text: UiHandle<Self> + 'static;)?)*
        }

        impl<T: ?Sized, $($($Text: UiHandle<T> + 'static,)?)*> $Trait for T
        where T:
        $(
            $(for <$lt>)? $Bound<$From, $(Text = $Text)?> +
        )*
        {
            $($(type $Text = $Text;)?)*
        }

        $(
            impl_known_kinds! {
                $KnownValueKind
                text(text_ty![$From, $($Text)?])
                kinds($($Kind,)+)
            }

            impl $(<$lt>)? $RenderIntoText for $From {
                type TextKind = expand_first![$({$Kind})+];

                fn render_text_from_self<Renderer: ?Sized + RenderTextFromKnown>(
                    self,
                    render_context: &mut Renderer::RenderContext<'_>,
                ) -> <Self::TextKind as TextKind>::Text<Renderer> {
                    Renderer::render_text_from(render_context, self)
                }

                fn update_text_from_self<Renderer: ?Sized + RenderTextFromKnown>(
                    self,
                    renderer: &mut Renderer,
                    text: &mut <Self::TextKind as TextKind>::Text<Renderer>,
                ) {
                    renderer.update_text_from(text, self)
                }
            }
        )*
    };
}

define!(
    /// Trait alias for [`RenderTextFrom`] with all known primitive types.
    pub trait RenderTextFromKnown: RenderTextFrom {
        type From = &'static str;
        type Kind = KindOfOwned<&'static str>;
        // type Kind = KindOfOwned<&'static str>;

        type From<'a> = TempStr<&'a str>;
        type Text = TextFromTempStr;
        type Kind = str;

        type From = StringElement;
        type Kind = KindOfOwned<StringElement>;

        type From<'a> = &'a StringElement;
        type Text = TextFromRefStringElement;
        type Kind = KindOfRef<StringElement>;

        type From = String;
        type Kind = KindOfOwned<String>;

        type From = Cow<'static, str>;
        type Kind = KindOfOwned<Cow<'static, str>>;

        type From<'a> = StaticRefOrTempOwned<'a, str>;
        type Text = TextFromStaticRefOrTempOwned;
        type Kind = KindOfStaticRefOrTempOwned<str>;

        type From = Rc<str>;
        type Kind = KindOfOwned<Rc<str>>;

        type From<'a> = &'a Rc<str>;
        type Text = TextFromRefRcStr;
        type Kind = KindOfRef<Rc<str>>;

        type From = Arc<str>;
        type Kind = KindOfOwned<Arc<str>>;

        type From<'a> = &'a Arc<str>;
        type Text = TextFromRefArcStr;
        type Kind = KindOfRef<Arc<str>>;

        type From = i8;
        type Kind = KindOfOwned<i8>;

        type From = u8;
        type Kind = KindOfOwned<u8>;

        type From = i16;
        type Kind = KindOfOwned<i16>;

        type From = u16;
        type Kind = KindOfOwned<u16>;

        type From = i32;
        type Kind = KindOfOwned<i32>;

        type From = u32;
        type Kind = KindOfOwned<u32>;

        type From = i64;
        type Kind = KindOfOwned<i64>;

        type From = u64;
        type Kind = KindOfOwned<u64>;

        type From = i128;
        type Kind = KindOfOwned<i128>;

        type From = u128;
        type Kind = KindOfOwned<u128>;

        type From = isize;
        type Kind = KindOfOwned<isize>;

        type From = usize;
        type Kind = KindOfOwned<usize>;

        type From = f32;
        type Kind = KindOfOwned<f32>;

        type From = f64;
        type Kind = KindOfOwned<f64>;

        type From = char;
        type Kind = KindOfOwned<char>;
    }

    impl<__> TextKind for Kind {}

    impl<__> KnownValueForText for Value {}
);

pub trait TextKind {
    type Text<R: ?Sized + RenderTextFromKnown>: 'static + UiHandle<R>;
}

pub trait KnownValueForText: Sized {
    type TextKind: ?Sized + TextKind;

    fn render_text_from_self<Renderer: ?Sized + RenderTextFromKnown>(
        self,
        render_context: &mut Renderer::RenderContext<'_>,
    ) -> <Self::TextKind as TextKind>::Text<Renderer>;

    fn update_text_from_self<Renderer: ?Sized + RenderTextFromKnown>(
        self,
        renderer: &mut Renderer,
        text: &mut <Self::TextKind as TextKind>::Text<Renderer>,
    );
}

pub trait KnownValueKindForText:
    for<'a> ValueKind<Value<'a>: KnownValueForText<TextKind = Self::ValueTextKind>>
{
    type ValueTextKind: TextKind;
}

impl<
        VK: ?Sized + for<'a> ValueKind<Value<'a>: KnownValueForText<TextKind = ValueTextKind>>,
        ValueTextKind: TextKind,
    > KnownValueKindForText for VK
{
    type ValueTextKind = ValueTextKind;
}
