use std::{any::Any, pin::Pin};

use frender_core::{RenderState, UpdateRenderState};
use hooks::{Hook, HookPollNextUpdate, LazyPinnedHook};
use lazy_pinned::LazyPinned;

use crate::ContextAndState;

pub struct FnHookElementWithOwnedProps<Data, S, Props, HDom, HSsr> {
    pub hook_data: Data,
    pub props: Props,
    pub with_dom: HDom,
    pub with_ssr: HSsr,
    __phantom_state: std::marker::PhantomData<S>,
}

pin_project_lite::pin_project! {
    pub struct FnHookElementState<Data: HookPollNextUpdate, State> {
        #[pin]
        hook_data: Data,
        #[pin]
        render_state: LazyPinned<State>,
    }
}

impl<Data: HookPollNextUpdate, S: RenderState> RenderState for FnHookElementState<Data, S> {
    fn unmount(self: Pin<&mut Self>) {
        self.project().render_state.as_pin_mut().map(S::unmount);
    }

    fn poll_reactive(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<bool> {
        let this = self.project();
        let a = this.hook_data.poll_next_update(cx);
        let b = this
            .render_state
            .as_pin_mut()
            .map_or(std::task::Poll::Ready(true), |s| s.poll_reactive(cx));

        match (a, b) {
            (std::task::Poll::Ready(false), std::task::Poll::Ready(false)) => {
                std::task::Poll::Ready(false)
            }
            (std::task::Poll::Ready(true), _) | (_, std::task::Poll::Ready(true)) => {
                std::task::Poll::Ready(true)
            }
            _ => std::task::Poll::Pending,
        }
    }
}

impl<Data: HookPollNextUpdate, Ctx, S: RenderState, HDom, HSsr, Props> UpdateRenderState<Ctx>
    for FnHookElementWithOwnedProps<Data, S, Props, HDom, HSsr>
where
    HDom: for<'c> FnOnce(Pin<&mut Data>, ContextAndState<'c, Ctx, S>, Props),
{
    type State = FnHookElementState<Data, S>;

    fn initialize_render_state(self, ctx: &mut Ctx) -> Self::State {
        FnHookElementState {
            hook_data: self.hook_data,
            render_state: Default::default(),
        }
    }

    fn update_render_state(self, ctx: &mut Ctx, state: std::pin::Pin<&mut Self::State>) {
        let use_hook = self.with_dom;
        let state = state.project();

        use_hook(
            state.hook_data,
            ContextAndState::new(ctx, state.render_state),
            self.props,
        );
    }
}

pub fn fn_dom_hook_element_with_owned_props<
    Data: HookPollNextUpdate,
    S: RenderState,
    HDom: for<'c> FnOnce(Pin<&mut Data>, ContextAndState<'c, frender_dom::Dom, S>, Props),
    Props,
>(
    hook_data: Data,
    use_hook: HDom,
    props: Props,
) -> FnHookElementWithOwnedProps<Data, S, Props, HDom, ()> {
    FnHookElementWithOwnedProps {
        hook_data,
        with_dom: use_hook,
        with_ssr: (),
        props,
        __phantom_state: std::marker::PhantomData,
    }
}
