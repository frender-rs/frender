use std::{borrow::Cow, rc::Rc, sync::Arc};

use frender_csr_core::render::RenderWithContext;
use frender_reactive_value::{
    static_or_temp_ref::StaticOrTempRef,
    temp_ref::TempRef,
    value_kind::{KindOfOwned, KindOfStaticOrTempRef, KindOfTempRef, ValueKind},
};

use crate::{csr::ui_handle::UiHandle, string_element::StringElement};

pub trait RenderTextFrom<V>: RenderWithContext {
    type Text: UiHandle<Self> + 'static;

    fn render_text_from(render_context: &mut Self::RenderContext<'_>, v: V) -> Self::Text;

    fn update_text_from(&mut self, text: &mut Self::Text, v: V);
}

macro_rules! impl_KnownValueForText {
    (KnownOwnedValueForText $Trait:ident $($For:ty),* $(,)?) => {$(
        impl $Trait for $For {
            type TextFromOwnedValue<R: ?Sized + RenderTextFromKnown> = <R as RenderTextFrom<$For>>::Text;
            fn render_text_from_owned_self<Renderer: ?Sized + RenderTextFromKnown>(
                self,
                render_context: &mut Renderer::RenderContext<'_>,
            ) -> Self::TextFromOwnedValue<Renderer> {
                Renderer::render_text_from(render_context, self)
            }

            fn update_text_from_owned_self<Renderer: ?Sized + RenderTextFromKnown>(
                self,
                renderer: &mut Renderer,
                text: &mut Self::TextFromOwnedValue<Renderer>,
            ) {
                renderer.update_text_from(text, self)
            }
        }
    )*};
    (KnownTempRefForText $Trait:ident $($For:ty),* $(,)?) => {$(
        impl $Trait for $For {
            type TextFromTempRef<R: ?Sized + RenderTextFromKnown> = <R as RenderTextFromTempRef<$For>>::TextFromTempRef;

            fn render_text_from_temp_ref_self<Renderer: ?Sized + RenderTextFromKnown>(
                &self,
                render_context: &mut Renderer::RenderContext<'_>,
            ) -> Self::TextFromTempRef<Renderer> {
                Renderer::render_text_from(render_context, TempRef(self))
            }

            fn update_text_from_temp_ref_self<Renderer: ?Sized + RenderTextFromKnown>(
                &self,
                renderer: &mut Renderer,
                text: &mut Self::TextFromTempRef<Renderer>,
            ) {
                renderer.update_text_from(text, TempRef(self))
            }
        }
    )*};
    (KnownStaticOrTempRefForText $Trait:ident $($For:ty),* $(,)?) => {$(
        impl $Trait for $For {
            type TextFromStaticOrTempRef<R: ?Sized + RenderTextFromKnown> = <R as RenderTextFromStaticOrTempRef<$For>>::TextFromStaticOrTempRef;

            fn render_text_from_static_ref_self<Renderer: ?Sized + RenderTextFromKnown>(
                &'static self,
                render_context: &mut Renderer::RenderContext<'_>,
            ) -> Self::TextFromStaticOrTempRef<Renderer> {
                Renderer::render_text_from(render_context, StaticOrTempRef::Static(self))
            }

            fn update_text_from_static_ref_self<Renderer: ?Sized + RenderTextFromKnown>(
                &'static self,
                renderer: &mut Renderer,
                text: &mut Self::TextFromStaticOrTempRef<Renderer>,
            ) {
                renderer.update_text_from(text, StaticOrTempRef::Static(self))
            }

            fn render_text_from_temp_ref_self<Renderer: ?Sized + RenderTextFromKnown>(
                &self,
                render_context: &mut Renderer::RenderContext<'_>,
            ) -> Self::TextFromStaticOrTempRef<Renderer> {
                Renderer::render_text_from(render_context, StaticOrTempRef::Temp(self))
            }

            fn update_text_from_temp_ref_self<Renderer: ?Sized + RenderTextFromKnown>(
                &self,
                renderer: &mut Renderer,
                text: &mut Self::TextFromStaticOrTempRef<Renderer>,
            ) {
                renderer.update_text_from(text, StaticOrTempRef::Temp(self))
            }
        }
    )*};
}

macro_rules! define {
    (
        $(#$attr:tt)*
        $vis:vis trait $Trait:ident
        {
            $(
                type $KnownValueForTextTrait:ident : $Bound:ident <
                    $($From:ty),* $(,)?
                >;
            )*
        }
    ) => {
        $(#$attr)*
        $vis trait $Trait:
        $($(
            $Bound<$From> +
        )*)*
        {}

        impl<T: ?Sized> $Trait for T
        where T:
        $($(
            $Bound<$From> +
        )*)*
        {}

        $(
            impl_KnownValueForText!{
                $KnownValueForTextTrait
                $KnownValueForTextTrait
                $($From,)*
            }
        )*
    };
}

pub trait RenderTextFromTempRef<T: ?Sized>:
    for<'a> RenderTextFrom<TempRef<'a, T>, Text = Self::TextFromTempRef>
{
    type TextFromTempRef: UiHandle<Self> + 'static;
}

impl<
        R: ?Sized + for<'a> RenderTextFrom<TempRef<'a, T>, Text = Text>,
        T: ?Sized,
        Text: UiHandle<Self> + 'static,
    > RenderTextFromTempRef<T> for R
{
    type TextFromTempRef = Text;
}

pub trait RenderTextFromStaticOrTempRef<T: ?Sized + 'static>:
    for<'a> RenderTextFrom<StaticOrTempRef<'a, T>, Text = Self::TextFromStaticOrTempRef>
{
    type TextFromStaticOrTempRef: UiHandle<Self> + 'static;
}

impl<
        R: ?Sized + for<'a> RenderTextFrom<StaticOrTempRef<'a, T>, Text = Text>,
        T: ?Sized + 'static,
        Text: UiHandle<Self> + 'static,
    > RenderTextFromStaticOrTempRef<T> for R
{
    type TextFromStaticOrTempRef = Text;
}

define!(
    /// Trait alias for [`RenderTextFrom`] with all known primitive types.
    pub trait RenderTextFromKnown {
        type KnownOwnedValueForText: RenderTextFrom<
            &'static str,
            StringElement,
            String,
            Cow<'static, str>,
            Rc<str>,
            Arc<str>,
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
        >;

        type KnownTempRefForText: RenderTextFromTempRef<
            //
            str,
            StringElement,
            Rc<str>,
            Arc<str>,
        >;

        type KnownStaticOrTempRefForText: RenderTextFromStaticOrTempRef<
            //
            str,
        >;
    }
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

// region: owned
pub trait KnownOwnedValueForText: 'static {
    type TextFromOwnedValue<R: ?Sized + RenderTextFromKnown>: 'static + UiHandle<R>;
    fn render_text_from_owned_self<Renderer: ?Sized + RenderTextFromKnown>(
        self,
        render_context: &mut Renderer::RenderContext<'_>,
    ) -> Self::TextFromOwnedValue<Renderer>;

    fn update_text_from_owned_self<Renderer: ?Sized + RenderTextFromKnown>(
        self,
        renderer: &mut Renderer,
        text: &mut Self::TextFromOwnedValue<Renderer>,
    );
}

impl<T: KnownOwnedValueForText> KnownValueForText for T {
    type TextKind = KindOfOwned<T>;

    fn render_text_from_self<Renderer: ?Sized + RenderTextFromKnown>(
        self,
        render_context: &mut Renderer::RenderContext<'_>,
    ) -> <Self::TextKind as TextKind>::Text<Renderer> {
        self.render_text_from_owned_self(render_context)
    }

    fn update_text_from_self<Renderer: ?Sized + RenderTextFromKnown>(
        self,
        renderer: &mut Renderer,
        text: &mut <Self::TextKind as TextKind>::Text<Renderer>,
    ) {
        self.update_text_from_owned_self(renderer, text)
    }
}

impl<T: KnownOwnedValueForText> TextKind for KindOfOwned<T> {
    type Text<R: ?Sized + RenderTextFromKnown> = T::TextFromOwnedValue<R>;
}
// endregion
// region: TempRef
pub trait KnownTempRefForText: 'static {
    type TextFromTempRef<R: ?Sized + RenderTextFromKnown>: 'static + UiHandle<R>;

    fn render_text_from_temp_ref_self<Renderer: ?Sized + RenderTextFromKnown>(
        &self,
        render_context: &mut Renderer::RenderContext<'_>,
    ) -> Self::TextFromTempRef<Renderer>;

    fn update_text_from_temp_ref_self<Renderer: ?Sized + RenderTextFromKnown>(
        &self,
        renderer: &mut Renderer,
        text: &mut Self::TextFromTempRef<Renderer>,
    );
}

impl<T: ?Sized + KnownTempRefForText> KnownValueForText for TempRef<'_, T> {
    type TextKind = KindOfTempRef<T>;

    fn render_text_from_self<Renderer: ?Sized + RenderTextFromKnown>(
        self,
        render_context: &mut Renderer::RenderContext<'_>,
    ) -> <Self::TextKind as TextKind>::Text<Renderer> {
        self.0.render_text_from_temp_ref_self(render_context)
    }

    fn update_text_from_self<Renderer: ?Sized + RenderTextFromKnown>(
        self,
        renderer: &mut Renderer,
        text: &mut <Self::TextKind as TextKind>::Text<Renderer>,
    ) {
        self.0.update_text_from_temp_ref_self(renderer, text)
    }
}

impl<T: ?Sized + KnownTempRefForText> TextKind for KindOfTempRef<T> {
    type Text<R: ?Sized + RenderTextFromKnown> = T::TextFromTempRef<R>;
}
// endregion
// region: StaticOrTempRef
pub trait KnownStaticOrTempRefForText: 'static {
    type TextFromStaticOrTempRef<R: ?Sized + RenderTextFromKnown>: 'static + UiHandle<R>;

    fn render_text_from_static_ref_self<Renderer: ?Sized + RenderTextFromKnown>(
        &'static self,
        render_context: &mut Renderer::RenderContext<'_>,
    ) -> Self::TextFromStaticOrTempRef<Renderer>;

    fn update_text_from_static_ref_self<Renderer: ?Sized + RenderTextFromKnown>(
        &'static self,
        renderer: &mut Renderer,
        text: &mut Self::TextFromStaticOrTempRef<Renderer>,
    );

    fn render_text_from_temp_ref_self<Renderer: ?Sized + RenderTextFromKnown>(
        &self,
        render_context: &mut Renderer::RenderContext<'_>,
    ) -> Self::TextFromStaticOrTempRef<Renderer>;

    fn update_text_from_temp_ref_self<Renderer: ?Sized + RenderTextFromKnown>(
        &self,
        renderer: &mut Renderer,
        text: &mut Self::TextFromStaticOrTempRef<Renderer>,
    );
}

impl<T: ?Sized + KnownStaticOrTempRefForText> KnownValueForText for StaticOrTempRef<'_, T> {
    type TextKind = KindOfStaticOrTempRef<T>;

    fn render_text_from_self<Renderer: ?Sized + RenderTextFromKnown>(
        self,
        render_context: &mut Renderer::RenderContext<'_>,
    ) -> <Self::TextKind as TextKind>::Text<Renderer> {
        match self {
            StaticOrTempRef::Static(this) => this.render_text_from_static_ref_self(render_context),
            StaticOrTempRef::Temp(this) => this.render_text_from_temp_ref_self(render_context),
        }
    }

    fn update_text_from_self<Renderer: ?Sized + RenderTextFromKnown>(
        self,
        renderer: &mut Renderer,
        text: &mut <Self::TextKind as TextKind>::Text<Renderer>,
    ) {
        match self {
            StaticOrTempRef::Static(this) => this.update_text_from_static_ref_self(renderer, text),
            StaticOrTempRef::Temp(this) => this.update_text_from_temp_ref_self(renderer, text),
        }
    }
}

impl<T: ?Sized + KnownStaticOrTempRefForText> TextKind for KindOfStaticOrTempRef<T> {
    type Text<R: ?Sized + RenderTextFromKnown> = T::TextFromStaticOrTempRef<R>;
}
// endregion

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
