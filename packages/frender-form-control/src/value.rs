use frender_attr_value::ssr::SsrAttrValue;
pub use provide::{
    BorrowToProvideFormControlValue, MaybeProvideFormControlValue, NeverProvideFormControlValue,
    ProvideFormControlValue,
};

use std::{
    borrow::{Borrow, Cow},
    marker::PhantomData,
    task::Poll,
};

use frender_dom::StateUnmount;

use super::{element::FormControlElement, textarea::SsrTextAreaValue};

mod provide;

mod either;

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

pub trait FormControlValueStateKind<VK: ?Sized + FormControlValueKind> {
    type UnpinnedState<E: FormControlElement<VK, R> + ?Sized, R: ?Sized>: StateUnmount + Unpin;

    fn unpinned_poll_render_form_control_value_state<
        E: FormControlElement<VK, R> + ?Sized,
        R: ?Sized,
    >(
        renderer: &mut R,
        element: &mut E,
        state: &mut Self::UnpinnedState<E, R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()>;
}

pub trait FormControlValue<VK: ?Sized + FormControlValueKind> {
    type StateKind: FormControlValueStateKind<VK>;

    fn render_init<E: FormControlElement<VK, R> + ?Sized, R: ?Sized>(
        this: Self,
        renderer: &mut R,
        element: &mut E,
    ) -> <Self::StateKind as FormControlValueStateKind<VK>>::UnpinnedState<E, R>;

    fn render_update<E: FormControlElement<VK, R> + ?Sized, R: ?Sized>(
        this: Self,
        renderer: &mut R,
        element: &mut E,
        state: &mut <Self::StateKind as FormControlValueStateKind<VK>>::UnpinnedState<E, R>,
    );
}

pub enum KindOfEmpty {}

impl<VK: ?Sized + FormControlValueKind> FormControlValueStateKind<VK> for KindOfEmpty {
    type UnpinnedState<E: FormControlElement<VK, R> + ?Sized, R: ?Sized> = ();

    fn unpinned_poll_render_form_control_value_state<
        E: FormControlElement<VK, R> + ?Sized,
        R: ?Sized,
    >(
        _: &mut R,
        _: &mut E,
        (): &mut Self::UnpinnedState<E, R>,
        _: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        Poll::Ready(())
    }
}

/// Uncontrolled form control value (no default value).
impl<V: ?Sized + FormControlValueKind> FormControlValue<V> for frender_dom::Empty {
    type StateKind = KindOfEmpty;

    fn render_init<E: FormControlElement<V, R> + ?Sized, R: ?Sized>(
        Self: Self,
        _: &mut R,
        _: &mut E,
    ) -> () {
        ()
    }

    fn render_update<E: FormControlElement<V, R> + ?Sized, R: ?Sized>(
        Self: Self,
        _: &mut R,
        _: &mut E,
        (): &mut (),
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

enum Never {}
pub struct KindOfUncontrolledWithDefaultValue<V>(Never, PhantomData<V>);

pub struct StateOfUncontrolledWithDefaultValue<T>(pub T);

impl<T> Unpin for StateOfUncontrolledWithDefaultValue<T> {}
impl<T> StateUnmount for StateOfUncontrolledWithDefaultValue<T> {
    fn state_unmount(self: std::pin::Pin<&mut Self>) {}
}

impl<V: PartialEq + Borrow<VK>, VK: FormControlValueKind + ?Sized> FormControlValueStateKind<VK>
    for KindOfUncontrolledWithDefaultValue<V>
{
    type UnpinnedState<E: FormControlElement<VK, R> + ?Sized, R: ?Sized> =
        StateOfUncontrolledWithDefaultValue<V>;

    fn unpinned_poll_render_form_control_value_state<
        E: FormControlElement<VK, R> + ?Sized,
        R: ?Sized,
    >(
        _: &mut R,
        _: &mut E,
        _: &mut Self::UnpinnedState<E, R>,
        _: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        Poll::Ready(())
    }
}

impl<V: PartialEq + Borrow<VK>, VK: FormControlValueKind + ?Sized> FormControlValue<VK>
    for UncontrolledWithDefaultValue<V>
{
    type StateKind = KindOfUncontrolledWithDefaultValue<V>;

    fn render_init<E: FormControlElement<VK, R> + ?Sized, R: ?Sized>(
        Self(this): Self,
        renderer: &mut R,
        element: &mut E,
    ) -> <Self::StateKind as FormControlValueStateKind<VK>>::UnpinnedState<E, R> {
        let value = this.borrow();
        element.set_default_value(renderer, value);
        StateOfUncontrolledWithDefaultValue(this)
    }

    fn render_update<E: FormControlElement<VK, R> + ?Sized, R: ?Sized>(
        this: Self,
        renderer: &mut R,
        element: &mut E,
        state: &mut <Self::StateKind as FormControlValueStateKind<VK>>::UnpinnedState<E, R>,
    ) {
        if state.0 == this.0 {
            return;
        }

        *state = Self::render_init(this, renderer, element);
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
        type StateKind = <UncontrolledWithDefaultValue<Self> as FormControlValue<$VK>>::StateKind;

        fn render_init<E: FormControlElement<$VK, R> + ?Sized, R: ?Sized>(
            this: Self,
            renderer: &mut R,
            element: &mut E,
        ) -> <Self::StateKind as FormControlValueStateKind<$VK>>::UnpinnedState<E, R> {
            UncontrolledWithDefaultValue::render_init(
                UncontrolledWithDefaultValue(this),
                renderer,
                element,
            )
        }

        fn render_update<E: FormControlElement<$VK, R> + ?Sized, R: ?Sized>(
            this: Self,
            renderer: &mut R,
            element: &mut E,
            state: &mut <Self::StateKind as FormControlValueStateKind<$VK>>::UnpinnedState<E, R>,
        ) {
            UncontrolledWithDefaultValue::render_update(
                UncontrolledWithDefaultValue(this),
                renderer,
                element,
                state,
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
