use std::{marker::PhantomData, pin::Pin};

use frender_core::{RenderState, UpdateRenderState};
use hooks_core::HookPollNextUpdate;

use crate::{ContextAndState, ContextAndStateData, HookContext, Rendered};

pub struct FnHookElement<HookData: HookPollNextUpdate, U, S: RenderState> {
    hook_data: HookData,
    use_hook: U,
    _phantom_state: PhantomData<S>,
}

impl<HookData: HookPollNextUpdate, U, S: RenderState>
    FnHookElement<HookData, fn_wrapper::FnMut<U>, S>
{
    #[allow(non_snake_case)]
    pub fn FnMut<Ctx>(hook_data: HookData, use_hook: U) -> Self
    where
        U: FnMut(Pin<&mut HookData>, ContextAndState<'_, Ctx, S>) -> Rendered<S>,
    {
        Self {
            hook_data,
            use_hook: fn_wrapper::FnMut(use_hook),
            _phantom_state: PhantomData,
        }
    }
}

impl<HookData: HookPollNextUpdate, U>
    FnHookElement<HookData, fn_wrapper::FnMutOutputElement<U>, ()>
{
    #[allow(non_snake_case)]
    pub fn FnMutOutputElement<Ctx, E: UpdateRenderState<Ctx>>(
        hook_data: HookData,
        use_hook: U,
    ) -> Self
    where
        U: FnMut(Pin<&mut HookData>) -> E,
    {
        Self {
            hook_data,
            use_hook: fn_wrapper::FnMutOutputElement(use_hook),
            _phantom_state: PhantomData,
        }
    }
}

impl<HookData: HookPollNextUpdate, U>
    FnHookElement<HookData, fn_wrapper::FnOnceOutputElement<U>, ()>
{
    #[allow(non_snake_case)]
    pub fn FnOnceOutputElement<Ctx, E: UpdateRenderState<Ctx>>(
        hook_data: HookData,
        use_hook: U,
    ) -> Self
    where
        HookData: Unpin,
        U: FnOnce(Pin<&mut HookData>) -> E,
    {
        Self {
            hook_data,
            use_hook: fn_wrapper::FnOnceOutputElement(use_hook),
            _phantom_state: PhantomData,
        }
    }
}

pin_project_lite::pin_project! {
    pub struct FnMutHookElementState<HookData, U, Ctx, S> {
        #[pin]
        hook_data: HookData,
        #[pin]
        ctx_and_state: ContextAndStateData<Ctx, S>,
        use_hook: U,
    }
}

pub mod fn_wrapper {
    pub struct FnMut<U>(pub U);
    pub struct FnMutOutputElement<U>(pub U);

    pub struct FnOnce<U>(pub U);
    pub struct FnOnceOutputElement<U>(pub U);
}

impl<HookData: HookPollNextUpdate, U, Ctx: HookContext, S: RenderState> RenderState
    for FnMutHookElementState<HookData, fn_wrapper::FnMut<U>, Ctx, S>
where
    U: FnMut(Pin<&mut HookData>, ContextAndState<'_, Ctx, S>) -> Rendered<S>,
{
    fn unmount(self: Pin<&mut Self>) {
        self.project().ctx_and_state.unmount()
    }

    fn poll_reactive(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<bool> {
        let mut this = self.project();

        let res = this.hook_data.as_mut().poll_next_update(cx);
        let r = this.ctx_and_state.as_mut().poll_reactive(cx);

        match (res, r) {
            (std::task::Poll::Ready(false), std::task::Poll::Ready(false)) => {
                std::task::Poll::Ready(false)
            }
            (
                std::task::Poll::Ready(false) | std::task::Poll::Pending,
                std::task::Poll::Ready(false) | std::task::Poll::Pending,
            ) => std::task::Poll::Pending,
            _ => {
                this.ctx_and_state.with_context_and_state(|cs| {
                    let _: Rendered<S> = (this.use_hook.0)(this.hook_data, cs);
                });
                cx.waker().wake_by_ref();
                std::task::Poll::Pending
            }
        }
    }
}

impl<HookData: HookPollNextUpdate, U, Ctx: HookContext, E: UpdateRenderState<Ctx>> RenderState
    for FnMutHookElementState<HookData, fn_wrapper::FnMutOutputElement<U>, Ctx, E::State>
where
    U: FnMut(Pin<&mut HookData>) -> E,
{
    fn unmount(self: Pin<&mut Self>) {
        self.project().ctx_and_state.unmount()
    }

    fn poll_reactive(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<bool> {
        let mut this = self.project();

        let res = this.hook_data.as_mut().poll_next_update(cx);
        let r = this.ctx_and_state.as_mut().poll_reactive(cx);

        match (res, r) {
            (std::task::Poll::Ready(false), std::task::Poll::Ready(false)) => {
                std::task::Poll::Ready(false)
            }
            (
                std::task::Poll::Ready(false) | std::task::Poll::Pending,
                std::task::Poll::Ready(false) | std::task::Poll::Pending,
            ) => std::task::Poll::Pending,
            _ => {
                this.ctx_and_state.with_context_and_state(|cs| {
                    let element = (this.use_hook.0)(this.hook_data);
                    let _: Rendered<E::State> = cs.render(element);
                });
                cx.waker().wake_by_ref();
                std::task::Poll::Pending
            }
        }
    }
}

impl<HookData, U, S, Ctx> UpdateRenderState<Ctx>
    for FnHookElement<HookData, fn_wrapper::FnMut<U>, S>
where
    HookData: HookPollNextUpdate,
    Ctx: HookContext,
    S: RenderState,
    U: FnMut(Pin<&mut HookData>, ContextAndState<'_, Ctx, S>) -> Rendered<S>,
{
    type State = FnMutHookElementState<HookData, fn_wrapper::FnMut<U>, Ctx, S>;

    fn initialize_render_state(self, ctx: &mut Ctx) -> Self::State {
        FnMutHookElementState {
            hook_data: self.hook_data,
            ctx_and_state: ContextAndStateData::new(HookContext::take_context(ctx)),
            use_hook: self.use_hook,
        }
    }

    fn update_render_state(self, ctx: &mut Ctx, state: std::pin::Pin<&mut Self::State>) {
        let state = state.project();

        let ctx_and_state = state
            .ctx_and_state
            .update_context(Ctx::take_context(ctx))
            .as_ctx_and_state();
        let mut use_hook = self.use_hook;
        let _: Rendered<S> = (use_hook.0)(state.hook_data, ctx_and_state);
        *state.use_hook = use_hook;
    }
}

impl<HookData, U, E: UpdateRenderState<Ctx>, Ctx> UpdateRenderState<Ctx>
    for FnHookElement<HookData, fn_wrapper::FnMutOutputElement<U>, ()>
where
    HookData: HookPollNextUpdate,
    Ctx: HookContext,
    U: FnMut(Pin<&mut HookData>) -> E,
{
    type State = FnMutHookElementState<HookData, fn_wrapper::FnMutOutputElement<U>, Ctx, E::State>;

    fn initialize_render_state(self, ctx: &mut Ctx) -> Self::State {
        FnMutHookElementState {
            hook_data: self.hook_data,
            ctx_and_state: ContextAndStateData::new(HookContext::take_context(ctx)),
            use_hook: self.use_hook,
        }
    }

    fn update_render_state(self, ctx: &mut Ctx, state: std::pin::Pin<&mut Self::State>) {
        let state = state.project();

        let ctx_and_state = state
            .ctx_and_state
            .update_context(Ctx::take_context(ctx))
            .as_ctx_and_state();
        let mut use_hook = self.use_hook;

        let element: E = (use_hook.0)(state.hook_data);

        let _: Rendered<E::State> = ctx_and_state.render(element);

        *state.use_hook = use_hook;
    }
}

mod fn_once {
    use std::pin::Pin;

    use frender_core::RenderState;
    use hooks_core::HookPollNextUpdate;

    pin_project_lite::pin_project! {
        pub struct HookElementState<HookData, S> {
            #[pin]
            pub hook_data: HookData,
            #[pin]
            pub state: S,
        }
    }

    impl<HookData, S> HookElementState<HookData, S> {
        pub fn pin_project(self: Pin<&mut Self>) -> (Pin<&mut HookData>, Pin<&mut S>) {
            let this = self.project();
            (this.hook_data, this.state)
        }
    }

    impl<HookData: HookPollNextUpdate, S: RenderState> RenderState for HookElementState<HookData, S> {
        fn unmount(self: std::pin::Pin<&mut Self>) {
            S::unmount(self.project().state)
        }

        fn poll_reactive(
            self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<bool> {
            let this = self.project();
            let a = this.hook_data.poll_next_update(cx);
            let b = this.state.poll_reactive(cx);

            match (a, b) {
                (std::task::Poll::Ready(true), _) | (_, std::task::Poll::Ready(true)) => {
                    std::task::Poll::Ready(true)
                }
                (std::task::Poll::Ready(false), std::task::Poll::Ready(false)) => {
                    std::task::Poll::Ready(false)
                }
                _ => std::task::Poll::Pending,
            }
        }
    }
}

impl<HookData, U, E: UpdateRenderState<Ctx>, Ctx> UpdateRenderState<Ctx>
    for FnHookElement<HookData, fn_wrapper::FnOnceOutputElement<U>, ()>
where
    HookData: HookPollNextUpdate + Unpin,
    Ctx: HookContext,
    U: FnOnce(Pin<&mut HookData>) -> E,
{
    type State = fn_once::HookElementState<HookData, E::State>;

    fn initialize_render_state(mut self, ctx: &mut Ctx) -> Self::State {
        fn_once::HookElementState {
            state: (self.use_hook.0)(Pin::new(&mut self.hook_data)).initialize_render_state(ctx),
            hook_data: self.hook_data,
        }
    }

    fn update_render_state(self, ctx: &mut Ctx, state: std::pin::Pin<&mut Self::State>) {
        let (hook_data, state) = state.pin_project();

        (self.use_hook.0)(hook_data).update_render_state(ctx, state);
    }
}
