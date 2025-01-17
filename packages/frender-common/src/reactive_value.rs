pub use self::render_value::{
    RenderValueMut, RenderValueOnce, RenderValueWithFn, RenderValueWithFnMutAndData,
    RenderValueWithFnOnceAndData,
};

use std::{pin::Pin, task::Poll};

use crate::csr::StateUnmount;

mod render_value;

mod option;
pub mod str;

pub trait ReactiveValueKind: 'static {
    type Value<'a>;
}

impl ReactiveValueKind for str {
    type Value<'a> = &'a str;
}

pub trait ReactiveValueState: StateUnmount {
    type ReactiveValueKind: ?Sized + ReactiveValueKind;

    fn reactive_value_state_poll_render(
        self: Pin<&mut Self>,
        renderer: &mut impl RenderValueMut<Self::ReactiveValueKind, RenderOutput = ()>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()>;
}

impl<T: ReactiveValueState> ReactiveValueState for Option<T> {
    type ReactiveValueKind = T::ReactiveValueKind;

    fn reactive_value_state_poll_render(
        self: Pin<&mut Self>,
        renderer: &mut impl RenderValueMut<Self::ReactiveValueKind, RenderOutput = ()>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        if let Some(this) = self.as_pin_mut() {
            this.reactive_value_state_poll_render(renderer, cx)
        } else {
            Poll::Ready(())
        }
    }
}

mod sealed {
    pub trait AsOptionMut<T: ?Sized> {}
}

/// This trait is sealed to ensure correct implementation:
/// - `as_option_mut()` returning `None` indicates [`<Self as StateUnmount>::state_unmount()`](StateUnmount::state_unmount) is noop.
/// - `as_option_mut()` returning `None` indicates the renderer has been called with `renderer.*_remove()`
///
/// (Search the code for *Correct AsOptionMut Implementation* to find the code relying on this behavior.)
pub trait AsOptionMut<T: ?Sized>: sealed::AsOptionMut<T> {
    // Not using the receiver `&mut self` so that calling this method must be qualified.
    fn as_option_mut(this: &mut Self) -> Option<&mut T>;

    fn as_option_pin_mut(this: Pin<&mut Self>) -> Option<Pin<&mut T>>;
}

impl<T: ?Sized> sealed::AsOptionMut<T> for T {}
impl<T: ?Sized> AsOptionMut<T> for T {
    fn as_option_mut(this: &mut Self) -> Option<&mut T> {
        Some(this)
    }

    fn as_option_pin_mut(this: Pin<&mut Self>) -> Option<Pin<&mut T>> {
        Some(this)
    }
}

impl<T> sealed::AsOptionMut<T> for Option<T> {}
impl<T> AsOptionMut<T> for Option<T> {
    fn as_option_mut(this: &mut Self) -> Option<&mut T> {
        this.as_mut()
    }

    fn as_option_pin_mut(this: Pin<&mut Self>) -> Option<Pin<&mut T>> {
        this.as_pin_mut()
    }
}

/// The value is optional. That's why [`RenderValueOnce`] and [`RenderValueMut`] have a `remove*` method.
pub trait ReactiveValue<VK: ?Sized + ReactiveValueKind> {
    type PinnedState;

    /// Requires [`Default`] so that it can be initialized and pinned safely.
    type PinnedStateDefault: Default
        + From<Self::PinnedState>
        + AsOptionMut<Self::PinnedState>
        + ReactiveValueState<ReactiveValueKind = VK>;

    /// Requires [`Unpin`] so that [`CsrReactiveStrStateUnmount`] can be reused without defining another trait taking `&mut self`.
    type UnpinnedState: Unpin + ReactiveValueState<ReactiveValueKind = VK>;

    /// Requires `Unpin + ReactiveValueState` so that implementations for `Option` like types doesn't need to
    /// introduce a wrapper type to implement `type StateUnpinned`.
    /// `Unpin + ReactiveValueState` is usually already implemented because `Option<_>` derives them.
    type UnpinnedStateDefault: Default
        + From<Self::UnpinnedState>
        + AsOptionMut<Self::UnpinnedState>
        + Unpin
        + ReactiveValueState<ReactiveValueKind = VK>;

    /// `state` is `Default::default()` at a pinned place.
    fn reactive_value_render_init_pinned<Out>(
        self,
        renderer: impl RenderValueOnce<VK, RenderOutput = Out>,
        state: Pin<&mut Self::PinnedStateDefault>,
    ) -> Out;

    /// `old_state` has been [unmounted](ReactiveStrStateUnmount::reactive_value_state_unmount) but not necessarily set to `Default::default()`.
    fn reactive_value_render_init_with_old_state_pinned<Out>(
        self,
        renderer: impl RenderValueOnce<VK, RenderOutput = Out>,
        old_state: Pin<&mut Self::PinnedStateDefault>,
    ) -> Out;

    /// `renderer` doesn't need to be updated if unnecessary.
    fn reactive_value_render_update_pinned<Out>(
        self,
        renderer: impl RenderValueOnce<VK, RenderOutput = Out>,
        state: Pin<&mut Self::PinnedStateDefault>,
    ) -> Option<Out>;

    fn reactive_value_render_init_unpinned<Out>(
        self,
        renderer: impl RenderValueOnce<VK, RenderOutput = Out>,
    ) -> (Self::UnpinnedState, Out);

    fn reactive_value_render_init_with_old_state_unpinned<Out>(
        self,
        renderer: impl RenderValueOnce<VK, RenderOutput = Out>,
        old_state: &mut Self::UnpinnedState,
    ) -> Out;

    /// `renderer` doesn't need to be updated if unnecessary.
    fn reactive_value_render_update_unpinned<Out>(
        self,
        renderer: impl RenderValueOnce<VK, RenderOutput = Out>,
        state: &mut Self::UnpinnedState,
    ) -> Option<Out>;
}

pub trait ReactiveValueExt<VK: ?Sized + ReactiveValueKind>: ReactiveValue<VK> + Sized {
    fn reactive_value_render_init_unpinned_default<Out>(
        self,
        renderer: impl RenderValueOnce<VK, RenderOutput = Out>,
    ) -> (Self::UnpinnedStateDefault, Out) {
        let (state, out) = self.reactive_value_render_init_unpinned(renderer);
        (state.into(), out)
    }

    fn reactive_value_render_init_with_old_state_unpinned_default<Out>(
        self,
        renderer: impl RenderValueOnce<VK, RenderOutput = Out>,
        old_state: &mut Self::UnpinnedStateDefault,
    ) -> Out {
        if let Some(old_state) = AsOptionMut::<Self::UnpinnedState>::as_option_mut(old_state) {
            self.reactive_value_render_init_with_old_state_unpinned(renderer, old_state)
        } else {
            // I pray for the compiler to optimize out this branch when StateUnpinnedDefault::as_option_mut() always returns Some(_)
            let out;
            (*old_state, out) = self.reactive_value_render_init_unpinned_default(renderer);
            out
        }
    }

    /// Note that this method requires `state` to be Some or will panic.
    ///
    /// `renderer` doesn't need to be updated if unnecessary.
    fn reactive_value_render_update_unpinned_default<Out>(
        self,
        renderer: impl RenderValueOnce<VK, RenderOutput = Out>,
        state: &mut Self::UnpinnedStateDefault,
    ) -> Option<Out> {
        if let Some(state) = AsOptionMut::<Self::UnpinnedState>::as_option_mut(state) {
            self.reactive_value_render_update_unpinned(renderer, state)
        } else {
            ::core::panic!("state should have been initialized before render_update")
        }
    }
}
impl<T: ReactiveValue<VK>, VK: ?Sized + ReactiveValueKind> ReactiveValueExt<VK> for T {}

macro_rules! impl_reactive_value_pinned_with_unpinned {
    (
        type ReactiveValueKind = $ReactiveValueKind:ty;
    ) => {
        type PinnedState = Self::UnpinnedState;
        type PinnedStateDefault = Self::UnpinnedStateDefault;

        fn reactive_value_render_init_pinned<Out>(
            self,
            renderer: impl $crate::reactive_value::RenderValueOnce<
                $ReactiveValueKind,
                RenderOutput = Out,
            >,
            default_state: ::core::pin::Pin<&mut Self::PinnedStateDefault>,
        ) -> Out {
            let default_state = default_state.get_mut();

            ::core::debug_assert!(
                <Self::PinnedStateDefault as $crate::reactive_value::AsOptionMut<
                    Self::PinnedState,
                >>::as_option_mut(default_state)
                .is_none(),
                "state must be set to default before render_init"
            );

            let out;
            (*default_state, out) = <Self as $crate::reactive_value::ReactiveValueExt<
                $ReactiveValueKind,
            >>::reactive_value_render_init_unpinned_default(
                self, renderer
            );

            out
        }

        fn reactive_value_render_init_with_old_state_pinned<Out>(
            self,
            renderer: impl $crate::reactive_value::RenderValueOnce<
                $ReactiveValueKind,
                RenderOutput = Out,
            >,
            old_state: ::core::pin::Pin<&mut Self::PinnedStateDefault>,
        ) -> Out {
            #[rustfmt::skip]
            let out = <Self as $crate::reactive_value::ReactiveValueExt<
                $ReactiveValueKind,
            >>::reactive_value_render_init_with_old_state_unpinned_default(
                self, renderer, old_state.get_mut()
            );
            out
        }

        fn reactive_value_render_update_pinned<Out>(
            self,
            renderer: impl $crate::reactive_value::RenderValueOnce<str, RenderOutput = Out>,
            state: ::core::pin::Pin<&mut Self::PinnedStateDefault>,
        ) -> ::core::option::Option<Out> {
            #[rustfmt::skip]
            let out = <Self as $crate::reactive_value::ReactiveValueExt<
                $ReactiveValueKind,
            >>::reactive_value_render_update_unpinned_default(
                self, renderer, state.get_mut()
            );
            out
        }
    };
}

use impl_reactive_value_pinned_with_unpinned;
