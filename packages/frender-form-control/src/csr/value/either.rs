use std::{marker::PhantomData, pin::Pin};

use frender_dom::csr::StateUnmount;

use crate::{csr::element::FormControlElement, values::EitherFormControlValue};

use super::{FormControlValue, FormControlValueKind, FormControlValueStateKind};

enum Never {}
pub struct KindOfEitherFormControlValue<KA, KB>(Never, PhantomData<(KA, KB)>);

pub enum EitherFormControlValueState<A, B> {
    A(A),
    B(B),
}

/// Prefers `A`.
impl<A: Default, B> Default for EitherFormControlValueState<A, B> {
    fn default() -> Self {
        Self::A(A::default())
    }
}

impl<A: Unpin + StateUnmount, B: Unpin + StateUnmount> StateUnmount
    for EitherFormControlValueState<A, B>
{
    fn state_unmount(self: std::pin::Pin<&mut Self>) {
        match self.get_mut() {
            Self::A(this) => Pin::new(this).state_unmount(),
            Self::B(this) => Pin::new(this).state_unmount(),
        }
    }
}

impl<
        KA: FormControlValueStateKind<VK>,
        KB: FormControlValueStateKind<VK>,
        VK: ?Sized + FormControlValueKind,
    > FormControlValueStateKind<VK> for KindOfEitherFormControlValue<KA, KB>
{
    type UnpinnedState<E: FormControlElement<VK, R> + ?Sized, R: ?Sized> =
        EitherFormControlValueState<
            //
            KA::UnpinnedState<E, R>,
            KB::UnpinnedState<E, R>,
        >;

    fn unpinned_poll_render_form_control_value_state<
        E: FormControlElement<VK, R> + ?Sized,
        R: ?Sized,
    >(
        renderer: &mut R,
        element: &mut E,
        state: &mut Self::UnpinnedState<E, R>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        match state {
            EitherFormControlValueState::A(state) => {
                KA::unpinned_poll_render_form_control_value_state(renderer, element, state, cx)
            }
            EitherFormControlValueState::B(state) => {
                KB::unpinned_poll_render_form_control_value_state(renderer, element, state, cx)
            }
        }
    }

    fn render_remove<E: FormControlElement<VK, R> + ?Sized, R: ?Sized>(
        renderer: &mut R,
        element: &mut E,
        state: &Self::UnpinnedState<E, R>,
    ) {
        match state {
            EitherFormControlValueState::A(state) => KA::render_remove(renderer, element, state),
            EitherFormControlValueState::B(state) => KB::render_remove(renderer, element, state),
        }
    }

    fn unpinned_unmount<E: FormControlElement<VK, R> + ?Sized, R: ?Sized>(
        renderer: &mut R,
        element: &mut E,
        state: &mut Self::UnpinnedState<E, R>,
    ) {
        match state {
            EitherFormControlValueState::A(state) => KA::unpinned_unmount(renderer, element, state),
            EitherFormControlValueState::B(state) => KB::unpinned_unmount(renderer, element, state),
        }
    }
}

impl<A: FormControlValue<VK>, B: FormControlValue<VK>, VK: ?Sized + FormControlValueKind>
    FormControlValue<VK> for EitherFormControlValue<A, B>
{
    type StateKind = KindOfEitherFormControlValue<A::StateKind, B::StateKind>;

    fn render_init<E: FormControlElement<VK, R> + ?Sized, R: ?Sized>(
        this: Self,
        renderer: &mut R,
        element: &mut E,
    ) -> <Self::StateKind as super::FormControlValueStateKind<VK>>::UnpinnedState<E, R> {
        match this {
            Self::A(this) => {
                let s = A::render_init(this, renderer, element);
                EitherFormControlValueState::A(s)
            }
            Self::B(this) => {
                let s = B::render_init(this, renderer, element);
                EitherFormControlValueState::B(s)
            }
        }
    }

    fn render_update<E: FormControlElement<VK, R> + ?Sized, R: ?Sized>(
        this: Self,
        renderer: &mut R,
        element: &mut E,
        state: &mut <Self::StateKind as super::FormControlValueStateKind<VK>>::UnpinnedState<E, R>,
    ) {
        match (this, state) {
            (Self::A(this), EitherFormControlValueState::A(state)) => {
                A::render_update(this, renderer, element, state)
            }
            (Self::B(this), EitherFormControlValueState::B(state)) => {
                B::render_update(this, renderer, element, state)
            }
            (this, state) => {
                Self::StateKind::unpinned_unmount(renderer, element, state);
                *state = Self::render_init(this, renderer, element);
            }
        }
    }
}
