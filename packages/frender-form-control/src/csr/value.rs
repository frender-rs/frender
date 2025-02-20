use std::{marker::PhantomData, pin::Pin, task::Poll};

use frender_common::{
    reactive_value::{ReactiveValue, ReactiveValueState},
    value_kind::{KindOfOwned, ValueKind},
};
use frender_dom::csr::StateUnmount;

use crate::{
    value::{KindOfChecked, KindOfValue, KindOfValueAsNumber},
    values::{UncontrolledEmptyDefaultValue, UncontrolledWithDefaultValue},
    FormControlValueKind,
};

use super::element::FormControlElement;

mod either;
mod option;

#[cfg(todo)]
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

#[cfg(todo)]
impl FromFormControlValue<str> for String {
    fn from_form_control_value(v: Cow<'_, str>) -> Self {
        v.into_owned()
    }
}

#[cfg(todo)]
impl FromFormControlValue<str> for Cow<'_, str> {
    fn from_form_control_value(v: Cow<'_, str>) -> Self {
        v.into_owned().into()
    }
}

#[cfg(todo)]
frender_common::impl_many!(
    impl<__> FromFormControlValue<str> for each_of![std::rc::Rc<str>, std::sync::Arc<str>] {
        fn from_form_control_value(v: Cow<'_, str>) -> Self {
            v.into()
        }
    }
);

#[cfg(todo)]
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

    fn render_remove<E: FormControlElement<VK, R> + ?Sized, R: ?Sized>(
        renderer: &mut R,
        element: &mut E,
        state: &Self::UnpinnedState<E, R>,
    );

    fn unpinned_unmount<E: FormControlElement<VK, R> + ?Sized, R: ?Sized>(
        renderer: &mut R,
        element: &mut E,
        state: &mut Self::UnpinnedState<E, R>,
    ) {
        Self::render_remove(renderer, element, state);
        Pin::new(state).state_unmount();
    }
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

    fn render_remove<E: FormControlElement<VK, R> + ?Sized, R: ?Sized>(
        _: &mut R,
        _: &mut E,
        (): &Self::UnpinnedState<E, R>,
    ) {
    }
}

/// Uncontrolled form control value (no default value).
impl<V: ?Sized + FormControlValueKind> FormControlValue<V> for UncontrolledEmptyDefaultValue {
    type StateKind = KindOfEmpty;

    fn render_init<E: FormControlElement<V, R> + ?Sized, R: ?Sized>(
        Self: Self,
        renderer: &mut R,
        element: &mut E,
    ) -> () {
        element.remove_default_value(renderer)
    }

    fn render_update<E: FormControlElement<V, R> + ?Sized, R: ?Sized>(
        Self: Self,
        _: &mut R,
        _: &mut E,
        (): &mut (),
    ) {
    }
}

enum Never {}
pub struct KindOfUncontrolledWithDefaultValue<S>(Never, PhantomData<S>);

pub struct StateOfUncontrolledWithDefaultValue<T>(pub T);

impl<T> Unpin for StateOfUncontrolledWithDefaultValue<T> {}
impl<T> StateUnmount for StateOfUncontrolledWithDefaultValue<T> {
    fn state_unmount(self: std::pin::Pin<&mut Self>) {}
}

trait FormControlValueKindWithValueKind<VK: ?Sized + ValueKind>: FormControlValueKind {
    fn provide_value<Out>(value: VK::Value<'_>, f: impl FnOnce(Self::Value<'_>) -> Out) -> Out;
}

impl FormControlValueKindWithValueKind<str> for KindOfValue {
    fn provide_value<Out>(
        value: <str as ValueKind>::Value<'_>,
        f: impl FnOnce(Self::Value<'_>) -> Out,
    ) -> Out {
        f(value.0)
    }
}
impl FormControlValueKindWithValueKind<KindOfOwned<bool>> for KindOfChecked {
    fn provide_value<Out>(value: bool, f: impl FnOnce(Self::Value<'_>) -> Out) -> Out {
        f(value)
    }
}
impl FormControlValueKindWithValueKind<KindOfOwned<f64>> for KindOfValueAsNumber {
    fn provide_value<Out>(value: f64, f: impl FnOnce(Self::Value<'_>) -> Out) -> Out {
        f(value)
    }
}

impl<
        S: Unpin + ReactiveValueState,
        VK: FormControlValueKind + ?Sized + FormControlValueKindWithValueKind<S::ReactiveValueKind>,
    > FormControlValueStateKind<VK> for KindOfUncontrolledWithDefaultValue<S>
{
    type UnpinnedState<E: FormControlElement<VK, R> + ?Sized, R: ?Sized> =
        StateOfUncontrolledWithDefaultValue<S>;

    fn unpinned_poll_render_form_control_value_state<
        E: FormControlElement<VK, R> + ?Sized,
        R: ?Sized,
    >(
        renderer: &mut R,
        element: &mut E,
        state: &mut Self::UnpinnedState<E, R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        Pin::new(&mut state.0).poll_render(
            |value| {
                VK::provide_value(
                    //
                    value,
                    |value| element.set_default_value(renderer, value),
                )
            },
            cx,
        )
    }

    fn render_remove<E: FormControlElement<VK, R> + ?Sized, R: ?Sized>(
        renderer: &mut R,
        element: &mut E,
        _: &Self::UnpinnedState<E, R>,
    ) {
        element.remove_default_value(renderer)
    }
}

macro_rules! impl_for_kinds {
    (
        type For = (
            ($FK:ident, $VK:ident),
            $(($FK_ty:ty, $VK_ty:ty)),* $(,)?
        );

        const _: () = $imp:tt;
    ) => {$(
        const _: () = {
            type $FK = $FK_ty;
            type $VK = $VK_ty;

            const _: () = $imp;
        };
    )*};
}

impl_for_kinds!(
    type For = (
        //
        (FK, VK),
        (KindOfValue, str),
        (KindOfChecked, KindOfOwned<bool>),
        (KindOfValueAsNumber, KindOfOwned<f64>),
    );

    const _: () = {
        impl<V: ReactiveValue<VK>> FormControlValue<FK> for UncontrolledWithDefaultValue<V> {
            type StateKind = KindOfUncontrolledWithDefaultValue<V::UnpinnedState>;

            fn render_init<E: FormControlElement<FK, R> + ?Sized, R: ?Sized>(
                Self(this): Self,
                renderer: &mut R,
                element: &mut E,
            ) -> <Self::StateKind as FormControlValueStateKind<FK>>::UnpinnedState<E, R>
            {
                let (state, ()) =
                    this.unpinned_render_init(renderer_set_default_value::<FK, VK, E, R>(
                        renderer, element,
                    ));
                StateOfUncontrolledWithDefaultValue(state)
            }

            fn render_update<E: FormControlElement<FK, R> + ?Sized, R: ?Sized>(
                Self(this): Self,
                renderer: &mut R,
                element: &mut E,
                state: &mut <Self::StateKind as FormControlValueStateKind<FK>>::UnpinnedState<E, R>,
            ) {
                _ = this.unpinned_render_update(
                    renderer_set_default_value::<FK, VK, E, R>(renderer, element),
                    &mut state.0,
                )
            }
        }
    };
);

fn renderer_set_default_value<
    //
    'a,
    FK: ?Sized + FormControlValueKindWithValueKind<VK>,
    VK: ?Sized + ValueKind,
    E: FormControlElement<FK, R> + ?Sized,
    R: ?Sized,
>(
    renderer: &'a mut R,
    element: &'a mut E,
) -> impl 'a + FnOnce(VK::Value<'_>) {
    |value| {
        <FK as FormControlValueKindWithValueKind<VK>>::provide_value(value, |value| {
            element.set_default_value(renderer, value)
        })
    }
}
