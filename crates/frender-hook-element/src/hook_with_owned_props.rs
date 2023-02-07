use std::{any::Any, pin::Pin};

use frender_core::{RenderState, UpdateRenderState};
use hooks::{Hook, HookPollNextUpdate, LazyPinnedHook};

use crate::ContextAndState;

pub struct FnHookElementWithOwnedProps<Data, S, Props, HDom, HSsr> {
    pub data: Data,
    pub props: Props,
    pub with_dom: HDom,
    pub with_ssr: HSsr,
    __phantom_state: std::marker::PhantomData<S>,
}

pin_project_lite::pin_project! {
    pub struct FnHookElementState<Data: HookPollNextUpdate, State> {
        #[pin]
        data: LazyPinnedHook<Data>,
        #[pin]
        render_state: State,
    }
}

impl<Data: HookPollNextUpdate, S: RenderState> RenderState for FnHookElementState<Data, S> {
    fn new_uninitialized() -> Self
    where
        Self: Sized,
    {
        Self {
            data: Default::default(),
            render_state: S::new_uninitialized(),
        }
    }

    fn unmount(self: Pin<&mut Self>) {
        self.project().render_state.unmount()
    }

    fn poll_reactive(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<bool> {
        let this = self.project();
        let a = this.data.poll_next_update(cx);
        let b = this.render_state.poll_reactive(cx);

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

    fn update_render_state(self, ctx: &mut Ctx, state: std::pin::Pin<&mut Self::State>) {
        let use_hook = self.with_dom;
        let state = state.project();

        let data = state.data.use_hook((move || self.data,));

        use_hook(
            data,
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
    data: Data,
    use_hook: HDom,
    props: Props,
) -> FnHookElementWithOwnedProps<Data, S, Props, HDom, ()> {
    FnHookElementWithOwnedProps {
        data,
        with_dom: use_hook,
        with_ssr: (),
        props,
        __phantom_state: std::marker::PhantomData,
    }
}
