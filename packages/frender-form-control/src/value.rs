use frender_attr_value::ssr::SsrAttrValue;
pub use provide::{
    BorrowToProvideFormControlValue, MaybeProvideFormControlValue, NeverProvideFormControlValue,
    ProvideFormControlValue,
};

use std::borrow::{Borrow, Cow};

use frender_dom::{
    render_state::non_reactive::NonReactiveRenderState, RenderStateWithParentElementsHandle,
};

use super::{element::FormControlElement, textarea::SsrTextAreaValue};

mod provide;

pub trait FormControlValueKind {
    /// The value, reference, or `Cow` passed on change.
    type FormControlValue<'a>
    where
        Self: 'a;
}

impl FormControlValueKind for str {
    type FormControlValue<'a> = Cow<'a, str>;
}

impl<T: Copy> FormControlValueKind for T {
    type FormControlValue<'a> = T where T: 'a;
}

pub trait FromFormControlValue<VK: ?Sized + FormControlValueKind> {
    fn from_form_control_value(v: VK::FormControlValue<'_>) -> Self;
}

pub trait HandleFormControlValue<V: ?Sized + FormControlValueKind> {
    fn handle_form_control_value(&mut self, v: V::FormControlValue<'_>);
}

impl<V: ?Sized + FormControlValueKind, F: for<'v> FnMut(V::FormControlValue<'v>)>
    HandleFormControlValue<V> for F
{
    fn handle_form_control_value(&mut self, v: <V as FormControlValueKind>::FormControlValue<'_>) {
        self(v)
    }
}

impl FromFormControlValue<str> for String {
    fn from_form_control_value(v: Cow<'_, str>) -> Self {
        v.into_owned()
    }
}

impl FromFormControlValue<str> for Cow<'_, str> {
    fn from_form_control_value(v: Cow<'_, str>) -> Self {
        v.into_owned().into()
    }
}

frender_common::impl_many!(
    impl<__> FromFormControlValue<str> for each_of![std::rc::Rc<str>, std::sync::Arc<str>] {
        fn from_form_control_value(v: Cow<'_, str>) -> Self {
            v.into()
        }
    }
);

impl<VK: Copy> FromFormControlValue<VK> for VK {
    fn from_form_control_value(v: VK) -> Self {
        v
    }
}

pub trait FormControlValue<V: ?Sized + FormControlValueKind> {
    type State<E: FormControlElement<V, R> + ?Sized, R: ?Sized>: Default
        + RenderStateWithParentElementsHandle<E, R>
        + Unpin;

    fn update_with_state<E: FormControlElement<V, R> + ?Sized, R: ?Sized>(
        this: Self,
        state: &mut Self::State<E, R>,
        element: &mut E,
        renderer: &mut R,
    );
}

/// Uncontrolled form control value (no default value).
impl<V: ?Sized + FormControlValueKind> FormControlValue<V> for frender_dom::Empty {
    type State<E: FormControlElement<V, R> + ?Sized, R: ?Sized> = ();

    fn update_with_state<E: FormControlElement<V, R> + ?Sized, R: ?Sized>(
        Self: Self,
        (): &mut Self::State<E, R>,
        _: &mut E,
        _: &mut R,
    ) {
    }
}

/// This wrapper proxies [`SsrAttrValue`] and [`SsrTextAreaValue`].
#[derive(Debug)]
pub struct UncontrolledWithDefaultValue<V>(pub V);

impl<V: SsrTextAreaValue> SsrTextAreaValue for UncontrolledWithDefaultValue<V> {
    type IntoSsrTextAreaValue = V::IntoSsrTextAreaValue;

    fn into_ssr_text_area_value(self) -> Self::IntoSsrTextAreaValue {
        self.0.into_ssr_text_area_value()
    }
}

impl<V: SsrAttrValue<AT>, AT: ?Sized> SsrAttrValue<AT> for UncontrolledWithDefaultValue<V> {
    type HtmlAttributeValue = V::HtmlAttributeValue;

    fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
        V::maybe_into_html_attribute_value(this.0)
    }
}

impl<V: PartialEq + Borrow<VK>, VK: FormControlValueKind + ?Sized> FormControlValue<VK>
    for UncontrolledWithDefaultValue<V>
{
    type State<E: FormControlElement<VK, R> + ?Sized, R: ?Sized> =
        NonReactiveRenderState<Option<V>>;

    fn update_with_state<E: FormControlElement<VK, R> + ?Sized, R: ?Sized>(
        Self(this): Self,
        state: &mut Self::State<E, R>,
        element: &mut E,
        renderer: &mut R,
    ) {
        let state = &mut state.0;

        if let Some(state) = state {
            if *state == this {
                return;
            }
        }

        let value = this.borrow();
        element.set_default_value(renderer, value);
        *state = Some(this);
    }
}

impl<V: Borrow<VK>, VK: ?Sized + FormControlValueKind> MaybeProvideFormControlValue<VK>
    for UncontrolledWithDefaultValue<V>
{
    type ProvideFormControlValue = BorrowToProvideFormControlValue<V>;

    fn maybe_into_provide_form_control_value(this: Self) -> Option<Self::ProvideFormControlValue> {
        Some(BorrowToProvideFormControlValue(this.0))
    }
}

macro_rules! impl_uncontrolled_with_default_value {
    ($VK:ty) => {
        type State<E: FormControlElement<$VK, R> + ?Sized, R: ?Sized> =
            <UncontrolledWithDefaultValue<Self> as FormControlValue<$VK>>::State<E, R>;

        fn update_with_state<E: FormControlElement<$VK, R> + ?Sized, R: ?Sized>(
            this: Self,
            state: &mut Self::State<E, R>,
            element: &mut E,
            renderer: &mut R,
        ) {
            UncontrolledWithDefaultValue::update_with_state(
                UncontrolledWithDefaultValue(this),
                state,
                element,
                renderer,
            )
        }
    };
}

frender_common::impl_many!(
    impl<__> FormControlValue<Self> for each_of![bool, f64] {
        impl_uncontrolled_with_default_value! {Self}
    }
);

frender_common::impl_many!(
    impl<__> FormControlValue<str>
        for each_of![
            //
            &str,
            String,
            Cow<'_, str>,
            std::rc::Rc<str>,
            std::sync::Arc<str>,
        ]
    {
        impl_uncontrolled_with_default_value! {str}
    }
);

#[cfg(feature = "either")]
impl<V: ?Sized + FormControlValueKind, A: FormControlValue<V>, B: FormControlValue<V>>
    FormControlValue<V> for either::Either<A, B>
{
    type State<E: FormControlElement<V, R> + ?Sized, R: ?Sized> =
        crate::render_state::either::EitherRenderState<A::State<E, R>, B::State<E, R>>;

    fn update_with_state<E: FormControlElement<V, R> + ?Sized, R: ?Sized>(
        this: Self,
        state: &mut Self::State<E, R>,
        element: &mut E,
        renderer: &mut R,
    ) {
        use either::Either::{Left, Right};
        let state = state.inner_mut();

        match this {
            Left(this) => {
                let state = match state {
                    Left(state) => state,
                    Right(old_state) => {
                        std::pin::Pin::new(old_state).unmount_with_peh(element, renderer);
                        *state = Left(Default::default());
                        match state {
                            Left(state) => state,
                            Right(_) => unreachable!(),
                        }
                    }
                };

                A::update_with_state(this, state, element, renderer)
            }
            Right(this) => {
                let state = match state {
                    Right(state) => state,
                    Left(old_state) => {
                        std::pin::Pin::new(old_state).unmount_with_peh(element, renderer);
                        *state = Right(Default::default());
                        match state {
                            Right(state) => state,
                            Left(_) => unreachable!(),
                        }
                    }
                };

                B::update_with_state(this, state, element, renderer)
            }
        }
    }
}
