use std::{marker::PhantomData, pin::Pin, task::Poll};

use frender_dom::csr::StateUnmount;

use crate::{csr::element::FormControlElement, FormControlValueKind};

use super::{FormControlValue, FormControlValueStateKind};

enum Never {}
pub struct Kind<K>(Never, PhantomData<K>);

impl<K: FormControlValueStateKind<VK>, VK: FormControlValueKind + ?Sized>
    FormControlValueStateKind<VK> for Kind<K>
{
    type UnpinnedState<E: FormControlElement<VK, R> + ?Sized, R: ?Sized> =
        Option<K::UnpinnedState<E, R>>;

    fn unpinned_poll_render_form_control_value_state<
        E: FormControlElement<VK, R> + ?Sized,
        R: ?Sized,
    >(
        renderer: &mut R,
        element: &mut E,
        state: &mut Self::UnpinnedState<E, R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        if let Some(state) = state {
            K::unpinned_poll_render_form_control_value_state(renderer, element, state, cx)
        } else {
            Poll::Ready(())
        }
    }

    fn render_remove<E: FormControlElement<VK, R> + ?Sized, R: ?Sized>(
        renderer: &mut R,
        element: &mut E,
        state: &Self::UnpinnedState<E, R>,
    ) {
        if let Some(state) = state {
            K::render_remove(renderer, element, state);
        }
    }

    fn unpinned_unmount<E: FormControlElement<VK, R> + ?Sized, R: ?Sized>(
        renderer: &mut R,
        element: &mut E,
        state: &mut Self::UnpinnedState<E, R>,
    ) {
        if let Some(state) = state {
            K::unpinned_unmount(renderer, element, state);
        }
    }
}

impl<T: FormControlValue<VK>, VK: FormControlValueKind + ?Sized> FormControlValue<VK>
    for Option<T>
{
    type StateKind = Kind<T::StateKind>;

    fn render_init<E: FormControlElement<VK, R> + ?Sized, R: ?Sized>(
        this: Self,
        renderer: &mut R,
        element: &mut E,
    ) -> <Self::StateKind as FormControlValueStateKind<VK>>::UnpinnedState<E, R> {
        match this {
            Some(this) => Some(T::render_init(this, renderer, element)),
            None => None,
        }
    }

    fn render_update<E: FormControlElement<VK, R> + ?Sized, R: ?Sized>(
        this: Self,
        renderer: &mut R,
        element: &mut E,
        state: &mut <Self::StateKind as FormControlValueStateKind<VK>>::UnpinnedState<E, R>,
    ) {
        match (this, &mut *state) {
            (None, None) => {}
            (Some(this), Some(state)) => T::render_update(this, renderer, element, state),
            (Some(this), state @ None) => *state = Some(T::render_init(this, renderer, element)),
            (None, Some(state_some)) => {
                T::StateKind::unpinned_unmount(renderer, element, state_some);
                *state = None;
            }
        }
    }
}
