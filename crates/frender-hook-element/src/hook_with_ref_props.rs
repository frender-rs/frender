use std::{marker::PhantomData, pin::Pin};

use hooks::{Hook, HookPollNextUpdate};
use lazy_pinned::LazyPinned;

use crate::{ContextAndState, HookContext};

use frender_core::RenderState;

pub struct FnHookElementWithRefProps<Data, Props, UDom, USsr, S> {
    pub hook_data: Data,
    pub props: Props,
    pub use_hook_dom: UDom,
    pub use_hook_ssr: USsr,
    _phantom_render_state: PhantomData<S>,
}

impl<Data, Props, UDom, USsr, S> FnHookElementWithRefProps<Data, Props, UDom, USsr, S> {
    #[cfg(feature = "dom")]
    pub fn new_dom(hook_data: Data, props: Props, use_hook_dom: UDom, use_hook_ssr: USsr) -> Self
    where
        UDom: FnMut(Pin<&mut Data>, ContextAndState<'_, frender_dom::Dom, S>, &Props),
    {
        Self {
            hook_data,
            props,
            use_hook_dom,
            use_hook_ssr,
            _phantom_render_state: PhantomData,
        }
    }
}

pin_project_lite::pin_project! {
    pub struct FnHookElementStateWithRefProps<Data: HookPollNextUpdate, Props, S, U, Ctx> {
        #[pin]
        pub hook_data: Data,
        #[pin]
        pub render_state: LazyPinned<S>,
        pub ctx: Ctx,
        pub use_hook: U,
        pub props: Props,
    }
}

impl<Data: HookPollNextUpdate, S: RenderState, U, Ctx: HookContext, Props> RenderState
    for FnHookElementStateWithRefProps<Data, Props, S, U, Ctx>
where
    U: FnMut(Pin<&mut Data>, ContextAndState<'_, Ctx, S>, &Props),
{
    fn unmount(self: Pin<&mut Self>) {
        self.project().render_state.as_pin_mut().map(S::unmount);
    }

    fn poll_reactive(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<bool> {
        let mut this = self.project();

        let res = this.hook_data.as_mut().poll_next_update(cx);
        let r = this
            .render_state
            .as_mut()
            .as_pin_mut()
            .map_or(std::task::Poll::Ready(true), |s| S::poll_reactive(s, cx));

        match (res, r) {
            (std::task::Poll::Ready(false), std::task::Poll::Ready(false)) => {
                std::task::Poll::Ready(false)
            }
            (
                std::task::Poll::Ready(false) | std::task::Poll::Pending,
                std::task::Poll::Ready(false) | std::task::Poll::Pending,
            ) => std::task::Poll::Pending,
            _ => {
                let context = this.ctx;
                let use_hook = this.use_hook;
                let data = this.hook_data;
                let props = this.props;

                Ctx::with_context(context, |context| {
                    use_hook(
                        data,
                        ContextAndState::new(context, this.render_state),
                        props,
                    );
                });
                cx.waker().wake_by_ref();
                std::task::Poll::Pending
            }
        }
    }
}

#[cfg(feature = "dom")]
impl<Data, S: RenderState, UDom, USsr, Props> frender_core::UpdateRenderState<frender_dom::Dom>
    for FnHookElementWithRefProps<Data, Props, UDom, USsr, S>
where
    Data: HookPollNextUpdate,
    UDom: FnMut(Pin<&mut Data>, ContextAndState<'_, frender_dom::Dom, S>, &Props),
{
    type State = FnHookElementStateWithRefProps<Data, Props, S, UDom, frender_dom::Dom>;

    fn initialize_render_state(self, ctx: &mut frender_dom::Dom) -> Self::State {
        FnHookElementStateWithRefProps {
            hook_data: self.hook_data,
            render_state: Default::default(),
            ctx: frender_dom::Dom::take_context(ctx),
            use_hook: self.use_hook_dom,
            props: self.props,
        }
    }

    fn update_render_state(self, ctx: &mut frender_dom::Dom, state: Pin<&mut Self::State>) {
        let state = state.project();
        let mut use_hook = self.use_hook_dom;
        use_hook(
            state.hook_data,
            ContextAndState::new(ctx, state.render_state),
            &self.props,
        );
        *state.ctx = frender_dom::Dom::take_context(ctx);
        *state.use_hook = use_hook;
        *state.props = self.props;
    }
}
