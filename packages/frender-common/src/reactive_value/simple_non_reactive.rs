use std::task::Poll;

use frender_macro_rules::impl_many;

use crate::{csr::StateUnmount, reactive_value::ProvideValueOfKind};

use super::{
    ReactiveValue, ReactiveValueKind, ReactiveValueRenderInitPinned, ReactiveValueState,
    RenderInitPinned,
};

trait KnownSimpleNonReactive: 'static + Copy + PartialEq {}

impl_many!(
    impl<__> KnownSimpleNonReactive
        for each_of![
            &'static str,
            i8,
            u8,
            i16,
            u16,
            i32,
            u32,
            i64,
            u64,
            i128,
            u128,
            isize,
            usize,
            f32,
            f64,
            char
        ]
    {
    }
);

impl<T: KnownSimpleNonReactive> ReactiveValueKind for T {
    type Value<'a> = T;
}

pub struct State<T>(T);

impl<T> Unpin for State<T> {}
impl<T> StateUnmount for State<T> {
    fn state_unmount(self: std::pin::Pin<&mut Self>) {}
}
impl<T: KnownSimpleNonReactive> ReactiveValueState for State<T> {
    type ReactiveValueKind = T;

    fn poll_render(
        self: std::pin::Pin<&mut Self>,
        _: impl FnMut(<Self::ReactiveValueKind as ReactiveValueKind>::Value<'_>),
        _: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        Poll::Ready(())
    }
}

pub struct RenderInit;

impl<T: KnownSimpleNonReactive, R: FnOnce(T) -> Out, Out> RenderInitPinned<R, State<T>>
    for RenderInit
{
    type Output = Out;

    fn render_init_pinned(self, renderer: R, state: std::pin::Pin<&mut State<T>>) -> Self::Output {
        renderer(state.0)
    }
}

impl<T: KnownSimpleNonReactive> ReactiveValueRenderInitPinned<T, State<T>> for RenderInit {
    type RenderInitPinned<R: FnOnce(<T as ReactiveValueKind>::Value<'_>) -> Out, Out> = Self;
}

impl<T: KnownSimpleNonReactive> ReactiveValue<T> for T {
    type PinnedState = State<T>;
    type PinnedRenderInit = RenderInit;
    type UnpinnedState = State<T>;

    crate::impl_reactive_value_with_mixed_unpinned!(
        type ReactiveValueKind = T;
    );

    fn pinned_render_init(self) -> (Self::PinnedState, Self::PinnedRenderInit) {
        (State(self), RenderInit)
    }

    fn unpinned_render_init_by_reusing<Out>(
        self,
        renderer: impl super::ReusableRendererOfKind<T, Output = Out>,
        State(cache): &mut Self::PinnedState,
    ) -> Out {
        if self == *cache {
            struct Provide<T>(T);

            impl<T: KnownSimpleNonReactive> ProvideValueOfKind<T> for Provide<T> {
                fn provide_value_of_kind<Out>(self, f: impl FnOnce(T) -> Out) -> Out {
                    f(self.0)
                }
            }

            renderer.reuse(Provide(self))
        } else {
            *cache = self;
            renderer.render(self)
        }
    }

    fn unpinned_render_update<Out>(
        self,
        renderer: impl FnOnce(T) -> Out,
        State(cache): &mut Self::PinnedState,
    ) -> Option<Out> {
        if self == *cache {
            None
        } else {
            *cache = self;
            Some(renderer(self))
        }
    }
}
