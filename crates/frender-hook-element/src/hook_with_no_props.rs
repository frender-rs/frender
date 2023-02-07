use std::{any::Any, pin::Pin};

use hooks::{Hook, HookPollNextUpdate, LazyPinnedHook};

use frender_core::{RenderState, UpdateRenderState};

use crate::ContextAndState;

pin_project_lite::pin_project! {
    pub struct FnHookComponentWithNoProps<Data, S, U> {
        #[pin]
        data: Data,
        use_hook: U,
        __phantom_state: std::marker::PhantomData<S> ,
    }
}

pub fn fn_hook_component_with_no_props<
    Data: HookPollNextUpdate,
    S: RenderState + 'static,
    U: for<'c> FnMut(
        Pin<&mut Data>,
        ContextAndState<'c, frender_dom::Dom, dyn Any>,
    ) -> ContextAndState<'c, frender_dom::Dom, S>,
>(
    data: Data,
    use_hook: U,
) -> FnHookComponentWithNoProps<Data, S, U> {
    FnHookComponentWithNoProps {
        data,
        use_hook,
        __phantom_state: std::marker::PhantomData,
    }
}

impl<Data: Clone, S, U: Clone> Clone for FnHookComponentWithNoProps<Data, S, U> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            use_hook: self.use_hook.clone(),
            __phantom_state: self.__phantom_state.clone(),
        }
    }
}

impl<Data: Copy, S, U: Copy> Copy for FnHookComponentWithNoProps<Data, S, U> {}

impl<
        Data: HookPollNextUpdate,
        Ctx: crate::HookContext,
        S: RenderState + 'static,
        U: for<'c> FnMut(
            Pin<&mut Data>,
            ContextAndState<'c, Ctx, dyn Any>,
        ) -> ContextAndState<'c, Ctx, S>,
    > UpdateRenderState<Ctx> for FnHookComponentWithNoProps<Data, S, U>
{
    type State = FnHookStateWithNoProps<Data, Ctx, S, U>;

    fn update_render_state(self, ctx: &mut Ctx, state: Pin<&mut Self::State>) {
        let state = state.project();
        let data = state.data.use_hook((move || self.data,));
        let mut use_hook = self.use_hook;
        Ctx::with_context(ctx, |ctx| {
            use_hook(data, ContextAndState::new(ctx, state.render_state));
        });
        *state.ctx_and_use_hook = Some((Ctx::take_context(ctx), use_hook));
    }
}

pin_project_lite::pin_project! {
    // #[project = FnHookStateWithNoPropsProj]
    pub struct FnHookStateWithNoProps<Data: HookPollNextUpdate, Ctx, S, U> {
        #[pin]
        pub data: LazyPinnedHook<Data>,
        #[pin]
        pub render_state: S,
        pub ctx_and_use_hook: Option<(Ctx,U)>,
    }
}

impl<Data: HookPollNextUpdate, Ctx, S, U> RenderState for FnHookStateWithNoProps<Data, Ctx, S, U>
where
    Ctx: crate::HookContext,
    S: RenderState + 'static,
    U: for<'c> FnMut(
        Pin<&mut Data>,
        ContextAndState<'c, Ctx, dyn Any>,
    ) -> ContextAndState<'c, Ctx, S>,
{
    fn new_uninitialized() -> Self
    where
        Self: Sized,
    {
        Self {
            data: Default::default(),
            render_state: S::new_uninitialized(),
            ctx_and_use_hook: None,
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

        let res = this.data.as_mut().poll_next_update(cx);
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
                if let (Some(data), Some((context, use_hook))) =
                    (this.data.pin_project_hook(), this.ctx_and_use_hook.as_mut())
                {
                    Ctx::with_context(context, |context| {
                        use_hook(data, ContextAndState::new(context, this.render_state));
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
