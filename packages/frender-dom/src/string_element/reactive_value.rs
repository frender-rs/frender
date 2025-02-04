use std::task::Poll;

use frender_common::reactive_value::{
    ProvideValueOfKind, ReactiveValue, ReactiveValueKind, ReactiveValueState, RenderInitPinned,
};
use frender_csr::StateUnmount;

use super::StringElement;

impl ReactiveValueKind for StringElement {
    type Value<'a> = &'a StringElement;
}

pub struct State(StringElement);

impl Unpin for State {}
impl StateUnmount for State {
    fn state_unmount(self: std::pin::Pin<&mut Self>) {}
}
impl ReactiveValueState for State {
    type ReactiveValueKind = StringElement;

    fn poll_render(
        self: std::pin::Pin<&mut Self>,
        _: impl FnMut(<Self::ReactiveValueKind as ReactiveValueKind>::Value<'_>),
        _: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        Poll::Ready(())
    }
}

pub struct RenderInit;

impl<R: FnOnce(&StringElement) -> Out, Out> RenderInitPinned<R, State> for RenderInit {
    type Output = Out;

    fn render_init_pinned(self, renderer: R, state: std::pin::Pin<&mut State>) -> Self::Output {
        renderer(&state.0)
    }
}

impl ReactiveValue<StringElement> for StringElement {
    type UnpinnedState = State;
    type PinnedState = State;
    type PinnedRenderInit<R: FnOnce(<StringElement as ReactiveValueKind>::Value<'_>) -> Out, Out> =
        RenderInit;

    frender_common::impl_reactive_value_with_mixed_unpinned!(
        type ReactiveValueKind = StringElement;
    );

    fn pinned_render_init<
        R: FnOnce(<StringElement as ReactiveValueKind>::Value<'_>) -> Out,
        Out,
    >(
        self,
    ) -> (Self::PinnedState, Self::PinnedRenderInit<R, Out>) {
        (State(self), RenderInit)
    }

    fn unpinned_render_init_by_reusing<Out>(
        self,
        renderer: impl frender_common::reactive_value::ReusableRendererOfKind<
            StringElement,
            Output = Out,
        >,
        State(cache): &mut Self::PinnedState,
    ) -> Out {
        if self == *cache {
            struct Provide<'a>(&'a StringElement);

            impl ProvideValueOfKind<StringElement> for Provide<'_> {
                fn provide_value_of_kind<Out>(self, f: impl FnOnce(&StringElement) -> Out) -> Out {
                    f(self.0)
                }
            }
            renderer.reuse(Provide(cache))
        } else {
            *cache = self;
            renderer.render(cache)
        }
    }

    fn unpinned_render_update<Out>(
        self,
        renderer: impl FnOnce(&StringElement) -> Out,
        State(cache): &mut Self::PinnedState,
    ) -> Option<Out> {
        if self == *cache {
            None
        } else {
            *cache = self;
            Some(renderer(cache))
        }
    }
}
