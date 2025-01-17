use std::{pin::Pin, task::Poll};

use crate::{
    csr::StateUnmount,
    strings::{known::KnownCsrStr, CsrStr},
    IntoStaticStrCache, ToAsRefStr,
};

use super::{ReactiveValue, ReactiveValueState};

#[derive(Debug, Default)]
pub struct CsrStrReactiveState<C>(pub C);

impl<C> Unpin for CsrStrReactiveState<C> {}

impl<C> StateUnmount for CsrStrReactiveState<C> {
    fn state_unmount(self: Pin<&mut Self>) {}
}

impl<C> ReactiveValueState for CsrStrReactiveState<C> {
    type ReactiveValueKind = str;

    /// The string has been rendered by [`ReactiveValue<str>`]
    /// so there are no more updates.
    fn reactive_value_state_poll_render(
        self: Pin<&mut Self>,
        _: &mut impl super::RenderValueMut<Self::ReactiveValueKind, RenderOutput = ()>,
        _: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        Poll::Ready(())
    }
}

/// [`CsrStr`] are non reactive
impl<T: KnownCsrStr> ReactiveValue<str> for T {
    super::impl_reactive_value_pinned_with_unpinned!(
        type ReactiveValueKind = str;
    );

    #[cfg(todo)]
    type StateUnpinned = CsrStrReactiveState<T::StaticStrCache>;

    #[cfg(todo)]
    fn _reactive_value_render_init_unpinned<R>(
        self,
        update: impl FnOnce(&str) -> R,
    ) -> (Self::StateUnpinned, R) {
        let cache = self.into_into_static_str_cache().into_static_str_cache();
        let res = update(cache.to_as_ref_str().as_ref());
        (CsrStrReactiveState(cache), res)
    }

    #[cfg(todo)]
    fn _reactive_value_render_init_with_old_state_unpinned<R>(
        self,
        update: impl FnOnce(&str) -> R,
        CsrStrReactiveState(cache): &mut Self::StateUnpinned,
    ) -> R {
        self.into_into_static_str_cache()
            .update_into_static_str_cache(cache);
        let res = update(cache.to_as_ref_str().as_ref());
        res
    }

    #[cfg(todo)]
    fn _reactive_value_render_update_unpinned<R>(
        self,
        update: impl FnOnce(&str) -> R,
        CsrStrReactiveState(cache): &mut Self::StateUnpinned,
    ) -> Option<R> {
        crate::strings::csr::update_into_cache(self, update, cache)
    }

    type UnpinnedState = CsrStrReactiveState<T::StaticStrCache>;

    type UnpinnedStateDefault = Option<Self::UnpinnedState>;

    fn reactive_value_render_init_unpinned<Out>(
        self,
        renderer: impl super::RenderValueOnce<str, RenderOutput = Out>,
    ) -> (Self::UnpinnedState, Out) {
        todo!()
    }

    fn reactive_value_render_init_with_old_state_unpinned<Out>(
        self,
        renderer: impl super::RenderValueOnce<str, RenderOutput = Out>,
        old_state: &mut Self::UnpinnedState,
    ) -> Out {
        todo!()
    }

    fn reactive_value_render_update_unpinned<Out>(
        self,
        renderer: impl super::RenderValueOnce<str, RenderOutput = Out>,
        state: &mut Self::UnpinnedState,
    ) -> Option<Out> {
        todo!()
    }
}
