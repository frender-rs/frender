pub use self::into_element::ReactiveValueIntoElement;
pub use self::with_kind::{ReactiveValueWithKind, UncachedNonReactiveValueWithKind};

use std::{pin::Pin, task::Poll};

use frender_common::csr::{self, StateUnmount};

use crate::value_kind::ValueKind;

mod value;

pub mod value_kind;

// mod strings; // TODO:

mod array;
mod with_kind;

pub mod temp_str;

mod into_element;

pub mod non_reactive;

pub trait ReactiveValueState: StateUnmount {
    type ReactiveValueKind: ?Sized + ValueKind;

    fn poll_render(
        self: Pin<&mut Self>,
        renderer: impl FnMut(<Self::ReactiveValueKind as ValueKind>::Value<'_>),
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()>;
}

impl<T: ReactiveValueState> ReactiveValueState for Option<T> {
    type ReactiveValueKind = T::ReactiveValueKind;

    fn poll_render(
        self: Pin<&mut Self>,
        renderer: impl FnMut(<Self::ReactiveValueKind as ValueKind>::Value<'_>),
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

pub trait ProvideValueOfKind<VK: ?Sized + ValueKind> {
    fn provide_value_of_kind<Out>(self, f: impl FnOnce(VK::Value<'_>) -> Out) -> Out;
}

impl<F: FnOnce() -> V, VK: ?Sized + for<'a> ValueKind<Value<'a> = V>, V> ProvideValueOfKind<VK>
    for F
{
    fn provide_value_of_kind<Out>(
        self,
        f: impl FnOnce(<VK as ValueKind>::Value<'_>) -> Out,
    ) -> Out {
        f(self())
    }
}

pub trait ReusableRendererOfKind<VK: ?Sized + ValueKind> {
    type Output;
    fn render(self, value: VK::Value<'_>) -> Self::Output;
    /// The provide_value might be ignored or called by renderer to check whether reusing is correct.
    fn reuse(self, provide_value: impl ProvideValueOfKind<VK>) -> Self::Output;
}

/// `for<R: FnOnce(VK::Value<'_>) -> Out, Out> RenderInitPinned<R, S, Output = R::RenderOutput>`
pub trait ReactiveValueRenderInitPinned<VK: ?Sized + ValueKind, S: ?Sized>: Sized {
    type RenderInitPinned<R: FnOnce(VK::Value<'_>) -> Out, Out>: RenderInitPinned<R, S, Output = Out>
        + From<Self>;
}

/// Notable reactive values
///
/// - Cached non-reactive values
///   - Ref - `&T: ReactiveValue<KindOfRef<T>>` where `T: ?Sized + PartialEq + ToOwned`, `type Cache = T::Owned`
///   - [`impl CsrStr`](crate::strings::CsrStr): `ReactiveValue<str>`
///   - `Cow<'static, T>`: `ReactiveValue<KindOfStaticRefOrTempOwned<T>>`
pub trait ReactiveValue<VK: ?Sized + ValueKind> {
    type PinnedState: ReactiveValueState<ReactiveValueKind = VK>;
    type PinnedRenderInit: ReactiveValueRenderInitPinned<VK, Self::PinnedState>;

    /// Requires [`Unpin`] so that [`ReactiveValueState`] can be reused without defining another trait taking `&mut self`.
    type UnpinnedState: Unpin + ReactiveValueState<ReactiveValueKind = VK>;

    fn pinned_render_init(self) -> (Self::PinnedState, Self::PinnedRenderInit);

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

pub trait ReactiveValueExt<VK: ?Sized + ValueKind>: ReactiveValue<VK> + Sized {}
impl<T: ReactiveValue<VK>, VK: ?Sized + ValueKind> ReactiveValueExt<VK> for T {}

#[macro_export]
macro_rules! impl_reactive_value_unpinned_init_with_pinned {
    (
        type ReactiveValueKind = $ReactiveValueKind:ty;
    ) => {
        fn unpinned_render_init<Out>(
            self,
            renderer: impl ::core::ops::FnOnce(
                <$ReactiveValueKind as $crate::value_kind::ValueKind>::Value<'_>,
            ) -> Out,
        ) -> (Self::UnpinnedState, Out) {
            $crate::unpinned_render_init_with_pinned::<Self, $ReactiveValueKind, Out, _>(
                self, renderer,
            )
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
            renderer: impl $crate::ReusableRendererOfKind<$ReactiveValueKind, Output = Out>,
            reused_state: ::core::pin::Pin<&mut Self::PinnedState>,
        ) -> Out {
            #[rustfmt::skip]
                    return <Self as $crate::ReactiveValue::<
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
                <$ReactiveValueKind as $crate::value_kind::ValueKind>::Value<'_>,
            ) -> Out,
            state: ::core::pin::Pin<&mut Self::UnpinnedState>,
        ) -> Option<Out> {
            #[rustfmt::skip]
                    return <Self as $crate::ReactiveValue::<
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
    VK: ?Sized + ValueKind,
    Out,
    R: FnOnce(VK::Value<'_>) -> Out,
>(
    this: V,
    renderer: R,
) -> (V::PinnedState, Out)
where
    V::PinnedState: Unpin,
{
    let (mut state, render_init) = V::pinned_render_init(this);

    let out = <<V::PinnedRenderInit as ReactiveValueRenderInitPinned<VK, _>>::RenderInitPinned<
        R,
        Out,
    >>::from(render_init)
    .render_init_pinned(renderer, ::core::pin::Pin::new(&mut state));
    (state, out)
}

#[macro_export]
macro_rules! proxy_reactive_value {
    (
        for <ValueKind = $VK:ty>
        |$($self_:ident)+ $(,)?|
        -> $ReactiveValue:ty
        $into:block
    ) => {
        type PinnedState =
            <$ReactiveValue as $crate::reactive_value::ReactiveValue<$VK>>::PinnedState;

        type PinnedRenderInit =
            <$ReactiveValue as $crate::reactive_value::ReactiveValue<$VK>>::PinnedRenderInit;

        type UnpinnedState =
            <$ReactiveValue as $crate::reactive_value::ReactiveValue<$VK>>::UnpinnedState;

        $crate::proxy_reactive_value_methods! {
            for<ValueKind = $VK> |$($self_)+| -> $ReactiveValue $into
        }
    };
}

#[macro_export]
macro_rules! proxy_reactive_value_methods {
    (
        for <ValueKind = $VK:ty>
        |$($self_:ident)+ $(,)?|
        $into:expr
    ) => {
        $crate::proxy_reactive_value_methods! {
            for<ValueKind = $VK> |$($self_)+| -> _ { $into }
        }
    };
    (
        for <ValueKind = $VK:ty>
        |$($self_:ident)+ $(,)?| -> $ReactiveValue:ty
        $into:block
    ) => {
        fn pinned_render_init($($self_)+) -> (Self::PinnedState, Self::PinnedRenderInit) {
            <$ReactiveValue as $crate::reactive_value::ReactiveValue<$VK>>::pinned_render_init(
                $into,
            )
        }

        fn pinned_render_init_by_reusing<Out>(
            $($self_)+,
            renderer: impl $crate::reactive_value::ReusableRendererOfKind<$VK, Output = Out>,
            reused_state: ::core::pin::Pin<&mut Self::PinnedState>,
        ) -> Out {
            <$ReactiveValue as $crate::reactive_value::ReactiveValue<
                $VK,
            >>::pinned_render_init_by_reusing(
                $into, renderer, reused_state,
            )
        }

        fn pinned_render_update<Out>(
            $($self_)+,
            renderer: impl ::core::ops::FnOnce(<$VK as $crate::value_kind::ValueKind>::Value<'_>) -> Out,
            state: ::core::pin::Pin<&mut Self::PinnedState>,
        ) -> Option<Out> {
            <$ReactiveValue as $crate::reactive_value::ReactiveValue<$VK>>::pinned_render_update(
                $into, renderer, state,
            )
        }

        fn unpinned_render_init<Out>(
            $($self_)+,
            renderer: impl ::core::ops::FnOnce(<$VK as $crate::value_kind::ValueKind>::Value<'_>) -> Out,
        ) -> (Self::UnpinnedState, Out) {
            <$ReactiveValue as $crate::reactive_value::ReactiveValue<$VK>>::unpinned_render_init(
                $into, renderer,
            )
        }

        fn unpinned_render_init_by_reusing<Out>(
            $($self_)+,
            renderer: impl $crate::reactive_value::ReusableRendererOfKind<$VK, Output = Out>,
            reused_state: &mut Self::UnpinnedState,
        ) -> Out {
            <$ReactiveValue as $crate::reactive_value::ReactiveValue<
                $VK,
            >>::unpinned_render_init_by_reusing(
                $into, renderer, reused_state,
            )
        }

        fn unpinned_render_update<Out>(
            $($self_)+,
            renderer: impl ::core::ops::FnOnce(<$VK as $crate::value_kind::ValueKind>::Value<'_>) -> Out,
            state: &mut Self::UnpinnedState,
        ) -> Option<Out> {
            <$ReactiveValue as $crate::reactive_value::ReactiveValue<$VK>>::unpinned_render_update(
                $into, renderer, state
            )
        }
    };
}
