use std::pin::Pin;

use crate::csr::StateUnmount as _;

use super::{AsOptionMut, ReactiveValue, ReactiveValueExt as _, ReactiveValueKind};

impl<T: ReactiveValue<VK>, VK: ?Sized + ReactiveValueKind> ReactiveValue<VK> for Option<T> {
    type PinnedState = T::PinnedState;
    type PinnedStateDefault = T::PinnedStateDefault;
    type UnpinnedState = T::UnpinnedStateDefault;
    type UnpinnedStateDefault = T::UnpinnedStateDefault;

    fn reactive_value_render_init_pinned<Out>(
        self,
        renderer: impl super::RenderValueOnce<VK, RenderOutput = Out>,
        state: std::pin::Pin<&mut Self::PinnedStateDefault>,
    ) -> Out {
        if let Some(this) = self {
            this.reactive_value_render_init_pinned(renderer, state)
        } else {
            // state is already default
            renderer.render_value_once_remove()
        }
    }

    fn reactive_value_render_init_with_old_state_pinned<Out>(
        self,
        renderer: impl super::RenderValueOnce<VK, RenderOutput = Out>,
        mut old_state: std::pin::Pin<&mut Self::PinnedStateDefault>,
    ) -> Out {
        if let Some(this) = self {
            this.reactive_value_render_init_with_old_state_pinned(renderer, old_state)
        } else {
            // old_state is already state_unmounted
            // old_state.as_mut().state_unmount();
            old_state.set(Default::default());
            renderer.render_value_once_remove()
        }
    }

    fn reactive_value_render_update_pinned<Out>(
        self,
        renderer: impl super::RenderValueOnce<VK, RenderOutput = Out>,
        mut state: std::pin::Pin<&mut Self::PinnedStateDefault>,
    ) -> Option<Out> {
        if let Some(this) = self {
            this.reactive_value_render_update_pinned(renderer, state)
        } else {
            if AsOptionMut::<T::PinnedState>::as_option_pin_mut(state.as_mut()).is_none() {
                // We don't need to state_unmount and set the old state to default,
                // or to call renderer.*remove().
                // This relies on *Correct AsOptionMut Implementation*.
                return None;
            }

            state.as_mut().state_unmount();
            state.set(Default::default());
            Some(renderer.render_value_once_remove())
        }
    }

    fn reactive_value_render_init_unpinned<Out>(
        self,
        renderer: impl super::RenderValueOnce<VK, RenderOutput = Out>,
    ) -> (Self::UnpinnedState, Out) {
        if let Some(this) = self {
            this.reactive_value_render_init_unpinned_default(renderer)
        } else {
            (Default::default(), renderer.render_value_once_remove())
        }
    }

    fn reactive_value_render_init_with_old_state_unpinned<Out>(
        self,
        renderer: impl super::RenderValueOnce<VK, RenderOutput = Out>,
        old_state: &mut Self::UnpinnedState,
    ) -> Out {
        if let Some(this) = self {
            this.reactive_value_render_init_with_old_state_unpinned_default(renderer, old_state)
        } else {
            // old_state has already been state_unmounted
            *old_state = Default::default();
            renderer.render_value_once_remove()
        }
    }

    fn reactive_value_render_update_unpinned<Out>(
        self,
        renderer: impl super::RenderValueOnce<VK, RenderOutput = Out>,
        state: &mut Self::UnpinnedState,
    ) -> Option<Out> {
        if let Some(this) = self {
            // Not using this.reactive_value_render_update_unpinned_default(renderer, state) because it requires state to be Some
            if let Some(state) = AsOptionMut::<T::UnpinnedState>::as_option_mut(state) {
                this.reactive_value_render_update_unpinned(renderer, state)
            } else {
                // I pray for the compiler to optimize out this branch when StateUnpinnedDefault::as_option_mut() always returns Some(_)

                // We don't need to state_unmount the old state.
                // This relies on *Correct AsOptionMut Implementation*.

                let out;
                (*state, out) = this.reactive_value_render_init_unpinned_default(renderer);
                Some(out)
            }
        } else {
            if let Some(old_state) = AsOptionMut::<T::UnpinnedState>::as_option_mut(state) {
                Pin::new(old_state).state_unmount();
                *state = Default::default();
                Some(renderer.render_value_once_remove())
            } else {
                // We don't need to state_unmount and set the old state to default,
                // or to call renderer.*remove().
                // This relies on *Correct AsOptionMut Implementation*.
                None
            }
        }
    }
}
