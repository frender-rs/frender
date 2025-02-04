use std::task::Poll;

use frender_common::reactive_value::{
    ProvideValueOfKind, ReactiveValue, ReactiveValueKind, ReactiveValueRenderInitPinned,
    ReactiveValueState,
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

impl ReactiveValueRenderInitPinned<StringElement> for RenderInit {
    type State = State;

    fn render_init_pinned<Out>(
        self,
        renderer: impl FnOnce(&StringElement) -> Out,
        state: std::pin::Pin<&mut Self::State>,
    ) -> Out {
        renderer(&state.0)
    }
}

impl ReactiveValue<StringElement> for StringElement {
    frender_common::impl_reactive_value_unpinned_with_pinned!(
        type ReactiveValueKind = StringElement;
    );

    type PinnedState = State;
    type PinnedRenderInit = RenderInit;

    fn pinned_render_init(self) -> (Self::PinnedState, Self::PinnedRenderInit) {
        (State(self), RenderInit)
    }

    fn pinned_render_init_by_reusing<Out>(
        self,
        renderer: impl frender_common::reactive_value::ReusableRendererOfKind<
            StringElement,
            Output = Out,
        >,
        reused_state: std::pin::Pin<&mut Self::PinnedState>,
    ) -> Out {
        let State(cache) = reused_state.get_mut();

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

    fn pinned_render_update<Out>(
        self,
        renderer: impl FnOnce(<StringElement as ReactiveValueKind>::Value<'_>) -> Out,
        state: std::pin::Pin<&mut Self::PinnedState>,
    ) -> Option<Out> {
        let State(cache) = state.get_mut();

        if self == *cache {
            None
        } else {
            *cache = self;
            Some(renderer(cache))
        }
    }
}
