use std::{marker::PhantomData, pin::Pin};

use hooks::{Hook, HookPollNextUpdate, LazyPinnedHook};

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
    pub fn new(hook_data: Data, props: Props, use_hook_dom: UDom, use_hook_ssr: USsr) -> Self {
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
        pub hook_data: LazyPinnedHook<Data>,
        #[pin]
        pub render_state: S,
        pub ctx_and_use_hook_and_props: Option<(Ctx, U, Props)>,
    }
}

impl<Data: HookPollNextUpdate, S: RenderState, U, Ctx: HookContext, Props> RenderState
    for FnHookElementStateWithRefProps<Data, Props, S, U, Ctx>
where
    U: FnMut(Pin<&mut Data>, ContextAndState<'_, Ctx, S>, &Props),
{
    fn new_uninitialized() -> Self
    where
        Self: Sized,
    {
        Self {
            hook_data: Default::default(),
            render_state: S::new_uninitialized(),
            ctx_and_use_hook_and_props: None,
        }
    }

    fn unmount(self: Pin<&mut Self>) {
        self.project().render_state.unmount()
    }

    fn poll_reactive(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<bool> {
        let mut this = self.project();

        let res = this.hook_data.as_mut().poll_next_update(cx);
        let r = this.render_state.as_mut().poll_reactive(cx);

        match (res, r) {
            (std::task::Poll::Ready(false), std::task::Poll::Ready(false)) => {
                std::task::Poll::Ready(false)
            }
            (
                std::task::Poll::Ready(false) | std::task::Poll::Pending,
                std::task::Poll::Ready(false) | std::task::Poll::Pending,
            ) => std::task::Poll::Pending,
            _ => {
                if let (Some(data), Some((context, use_hook, props))) = (
                    this.hook_data.pin_project_hook(),
                    this.ctx_and_use_hook_and_props.as_mut(),
                ) {
                    Ctx::with_context(context, |context| {
                        use_hook(
                            data,
                            ContextAndState::new(context, this.render_state),
                            props,
                        );
                    });
                    cx.waker().wake_by_ref();
                    std::task::Poll::Pending
                } else {
                    std::task::Poll::Ready(true)
                }
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

    fn update_render_state(self, ctx: &mut frender_dom::Dom, state: Pin<&mut Self::State>) {
        let state = state.project();
        let hook_data = state.hook_data.use_hook((move || self.hook_data,));
        let mut use_hook = self.use_hook_dom;
        use_hook(
            hook_data,
            ContextAndState::new(ctx, state.render_state),
            &self.props,
        );
        *state.ctx_and_use_hook_and_props =
            Some((frender_dom::Dom::take_context(ctx), use_hook, self.props));
    }
}
