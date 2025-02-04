use std::{marker::PhantomData, task::Poll};

use frender_macro_rules::impl_many;

use crate::{csr::StateUnmount, reactive_value::ProvideValueOfKind};

use super::{ReactiveValue, ReactiveValueKind, ReactiveValueRenderInitPinned, ReactiveValueState};

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

pub struct RenderInit<T>(PhantomData<T>);

impl<T: KnownSimpleNonReactive> ReactiveValueRenderInitPinned<T> for RenderInit<T> {
    type State = State<T>;

    fn render_init_pinned<Out>(
        self,
        renderer: impl FnOnce(T) -> Out,
        state: std::pin::Pin<&mut Self::State>,
    ) -> Out {
        renderer(state.0)
    }
}

impl<T: KnownSimpleNonReactive> ReactiveValue<T> for T {
    super::impl_reactive_value_unpinned_with_pinned!(
        type ReactiveValueKind = T;
    );

    type PinnedState = State<T>;
    type PinnedRenderInit = RenderInit<T>;

    fn pinned_render_init(self) -> (Self::PinnedState, Self::PinnedRenderInit) {
        (State(self), RenderInit(PhantomData))
    }

    fn pinned_render_init_by_reusing<Out>(
        self,
        renderer: impl super::ReusableRendererOfKind<T, Output = Out>,
        reused_state: std::pin::Pin<&mut Self::PinnedState>,
    ) -> Out {
        let State(cache) = reused_state.get_mut();
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

    fn pinned_render_update<Out>(
        self,
        renderer: impl FnOnce(T) -> Out,
        state: std::pin::Pin<&mut Self::PinnedState>,
    ) -> Option<Out> {
        let State(cache) = state.get_mut();
        if self == *cache {
            None
        } else {
            *cache = self;
            Some(renderer(self))
        }
    }
}
