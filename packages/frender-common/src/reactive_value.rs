use std::{pin::Pin, task::Poll};

use crate::csr::StateUnmount;

mod array;
pub mod csr_str;
pub mod simple_non_reactive;

pub trait ReactiveValueKind: 'static {
    type Value<'a>;
}

pub trait ReactiveValueState: StateUnmount {
    type ReactiveValueKind: ?Sized + ReactiveValueKind;

    fn poll_render(
        self: Pin<&mut Self>,
        renderer: impl FnMut(<Self::ReactiveValueKind as ReactiveValueKind>::Value<'_>),
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()>;
}

impl<T: ReactiveValueState> ReactiveValueState for Option<T> {
    type ReactiveValueKind = T::ReactiveValueKind;

    fn poll_render(
        self: Pin<&mut Self>,
        renderer: impl FnMut(<Self::ReactiveValueKind as ReactiveValueKind>::Value<'_>),
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        if let Some(this) = self.as_pin_mut() {
            this.poll_render(renderer, cx)
        } else {
            Poll::Ready(())
        }
    }
}

pub trait RenderInitPinned<R, S: ?Sized> {
    type Output;

    fn render_init_pinned(self, renderer: R, state: Pin<&mut S>) -> Self::Output;
}

pub trait ProvideValueOfKind<VK: ?Sized + ReactiveValueKind> {
    fn provide_value_of_kind<Out>(self, f: impl FnOnce(VK::Value<'_>) -> Out) -> Out;
}

pub trait ReusableRendererOfKind<VK: ?Sized + ReactiveValueKind> {
    type Output;
    fn render(self, value: VK::Value<'_>) -> Self::Output;
    /// The provide_value might be ignored or called by renderer to check whether reusing is correct.
    fn reuse(self, provide_value: impl ProvideValueOfKind<VK>) -> Self::Output;
}

pub trait ReactiveValue<VK: ?Sized + ReactiveValueKind> {
    type PinnedState: ReactiveValueState<ReactiveValueKind = VK>;
    type PinnedRenderInit<R: FnOnce(VK::Value<'_>) -> Out, Out>: RenderInitPinned<
        R,
        Self::PinnedState,
        Output = Out,
    >;

    /// Requires [`Unpin`] so that [`ReactiveValueState`] can be reused without defining another trait taking `&mut self`.
    type UnpinnedState: Unpin + ReactiveValueState<ReactiveValueKind = VK>;

    /// `state` is `Default::default()` at a pinned place.
    fn pinned_render_init<R: FnOnce(VK::Value<'_>) -> Out, Out>(
        self,
    ) -> (Self::PinnedState, Self::PinnedRenderInit<R, Out>);

    /// `old_state` has been [unmounted](ReactiveStrStateUnmount::reactive_value_state_unmount) but not necessarily set to `Default::default()`.
    fn pinned_render_init_by_reusing<Out>(
        self,
        renderer: impl ReusableRendererOfKind<VK, Output = Out>,
        reused_state: Pin<&mut Self::PinnedState>,
    ) -> Out;

    /// `renderer` doesn't need to be updated if unnecessary.
    fn pinned_render_update<Out>(
        self,
        renderer: impl FnOnce(VK::Value<'_>) -> Out,
        state: Pin<&mut Self::PinnedState>,
    ) -> Option<Out>;

    fn unpinned_render_init<Out>(
        self,
        renderer: impl FnOnce(VK::Value<'_>) -> Out,
    ) -> (Self::UnpinnedState, Out);

    fn unpinned_render_init_by_reusing<Out>(
        self,
        renderer: impl ReusableRendererOfKind<VK, Output = Out>,
        reused_state: &mut Self::UnpinnedState,
    ) -> Out;

    /// `renderer` doesn't need to be updated if unnecessary.
    fn unpinned_render_update<Out>(
        self,
        renderer: impl FnOnce(VK::Value<'_>) -> Out,
        state: &mut Self::UnpinnedState,
    ) -> Option<Out>;
}

pub trait ReactiveValueExt<VK: ?Sized + ReactiveValueKind>: ReactiveValue<VK> + Sized {}
impl<T: ReactiveValue<VK>, VK: ?Sized + ReactiveValueKind> ReactiveValueExt<VK> for T {}

#[macro_export]
macro_rules! impl_reactive_value_unpinned_init_with_pinned {
    (
        type ReactiveValueKind = $ReactiveValueKind:ty;
    ) => {
        fn unpinned_render_init<Out>(
            self,
            renderer: impl ::core::ops::FnOnce(
                <$ReactiveValueKind as $crate::reactive_value::ReactiveValueKind>::Value<'_>,
            ) -> Out,
        ) -> (Self::UnpinnedState, Out) {
            $crate::reactive_value::unpinned_render_init_with_pinned(self, renderer)
        }
    };
}

#[macro_export]
macro_rules! impl_reactive_value_unpinned_with_pinned {
    (
        type ReactiveValueKind = $ReactiveValueKind:ty;
    ) => {
        type UnpinnedState = Self::PinnedState;

        $crate::impl_reactive_value_unpinned_init_with_pinned!(
            type ReactiveValueKind = $ReactiveValueKind;
        );

        fn unpinned_render_init_by_reusing<Out>(
            self,
            renderer: impl $crate::reactive_value::ReusableRendererOfKind<
                $ReactiveValueKind,
                Output = Out,
            >,
            reused_state: &mut Self::UnpinnedState,
        ) -> Out {
            #[rustfmt::skip]
            return <Self as $crate::reactive_value::ReactiveValue::<
                $ReactiveValueKind,
            >>::pinned_render_init_by_reusing(
                self,
                renderer,
                ::core::pin::Pin::new(reused_state),
            );
        }

        fn unpinned_render_update<Out>(
            self,
            renderer: impl ::core::ops::FnOnce(
                <$ReactiveValueKind as $crate::reactive_value::ReactiveValueKind>::Value<'_>,
            ) -> Out,
            state: &mut Self::UnpinnedState,
        ) -> Option<Out> {
            #[rustfmt::skip]
            return <Self as $crate::reactive_value::ReactiveValue::<
                $ReactiveValueKind,
            >>::pinned_render_update(self, renderer, ::core::pin::Pin::new(state));
        }
    };
}

#[macro_export]
macro_rules! impl_reactive_value_pinned_reuse_and_update_with_unpinned {
    (
        type ReactiveValueKind = $ReactiveValueKind:ty;
    ) => {
        fn pinned_render_init_by_reusing<Out>(
            self,
            renderer: impl $crate::reactive_value::ReusableRendererOfKind<
                $ReactiveValueKind,
                Output = Out,
            >,
            reused_state: ::core::pin::Pin<&mut Self::PinnedState>,
        ) -> Out {
            #[rustfmt::skip]
            return <Self as $crate::reactive_value::ReactiveValue::<
                $ReactiveValueKind,
            >>::unpinned_render_init_by_reusing(
                self,
                renderer,
                ::core::pin::Pin::get_mut(reused_state),
            );
        }

        fn pinned_render_update<Out>(
            self,
            renderer: impl ::core::ops::FnOnce(
                <$ReactiveValueKind as $crate::reactive_value::ReactiveValueKind>::Value<'_>,
            ) -> Out,
            state: ::core::pin::Pin<&mut Self::UnpinnedState>,
        ) -> Option<Out> {
            #[rustfmt::skip]
            return <Self as $crate::reactive_value::ReactiveValue::<
                $ReactiveValueKind,
            >>::unpinned_render_update(
                self,
                renderer,
                ::core::pin::Pin::get_mut(state),
            );
        }
    };
}

#[macro_export]
macro_rules! impl_reactive_value_with_mixed_unpinned {
    (
        type ReactiveValueKind = $ReactiveValueKind:ty;
    ) => {
        $crate::impl_reactive_value_unpinned_init_with_pinned!(
            type ReactiveValueKind = $ReactiveValueKind;
        );
        $crate::impl_reactive_value_pinned_reuse_and_update_with_unpinned!(
            type ReactiveValueKind = $ReactiveValueKind;
        );
    };
}

pub fn unpinned_render_init_with_pinned<
    V: ReactiveValue<VK>,
    VK: ?Sized + ReactiveValueKind,
    Out,
    R: FnOnce(VK::Value<'_>) -> Out,
>(
    this: V,
    renderer: R,
) -> (V::PinnedState, Out)
where
    V::PinnedState: Unpin,
{
    let (mut state, render_init) = V::pinned_render_init::<R, Out>(this);

    let out = render_init.render_init_pinned(renderer, ::core::pin::Pin::new(&mut state));
    (state, out)
}
