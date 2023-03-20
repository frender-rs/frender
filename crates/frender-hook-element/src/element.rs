use std::{marker::PhantomData, pin::Pin};

use frender_core::{RenderState, UpdateRenderState};
use hooks_core::{HookPollNextUpdate, HookUnmount};

use crate::{ContextAndState, ContextAndStateData, HookContext, Rendered};

mod prelude_names {
    pub(super) use std::pin::Pin;

    pub(super) use frender_core::{RenderState, UpdateRenderState};
    pub(super) use hooks_core::{HookPollNextUpdate, HookUnmount};

    #[cfg(feature = "csr")]
    pub(super) use frender_dom::Dom;

    #[cfg(feature = "ssr")]
    pub(super) use frender_ssr::AnySsrContext;

    pub(super) use super::{fn_wrapper, prelude_names, FnHookElement};
    pub(super) use crate::{ContextAndState, Rendered};
}

pub mod new_fn_hook_element {
    use super::prelude_names::*;

    #[inline]
    #[cfg(feature = "csr")]
    pub fn csr<HookData: HookPollNextUpdate + HookUnmount + Default, U, E: UpdateRenderState<Dom>>(
        use_hook: U,
    ) -> FnHookElement<HookData, fn_wrapper::FnMutOutputElement<U>, ()>
    where
        U: FnMut(Pin<&mut HookData>) -> E,
    {
        FnHookElement {
            use_hook: fn_wrapper::FnMutOutputElement(use_hook),
            _phantom: std::marker::PhantomData,
        }
    }

    #[inline]
    #[cfg(feature = "ssr")]
    pub fn ssr<
        HookData: HookPollNextUpdate + HookUnmount + Default,
        U,
        E: for<'a> UpdateRenderState<AnySsrContext<'a>>,
    >(
        use_hook: U,
    ) -> FnHookElement<HookData, fn_wrapper::FnMutOutputElement<U>, ()>
    where
        U: FnMut(Pin<&mut HookData>) -> E,
    {
        FnHookElement {
            use_hook: fn_wrapper::FnMutOutputElement(use_hook),
            _phantom: std::marker::PhantomData,
        }
    }
}
#[cfg(all(feature = "csr", feature = "ssr"))]
pub fn new_fn_hook_element() {}

pub struct FnHookElement<HookData: HookPollNextUpdate + HookUnmount + Default, U, S: RenderState> {
    use_hook: U,
    _phantom: PhantomData<(HookData, S)>,
}

impl<HookData: HookPollNextUpdate + HookUnmount + Default, U, S: RenderState>
    FnHookElement<HookData, fn_wrapper::FnMut<U>, S>
{
    #[allow(non_snake_case)]
    pub fn FnMut<Ctx>(use_hook: U) -> Self
    where
        U: FnMut(Pin<&mut HookData>, ContextAndState<'_, Ctx, S>) -> Rendered<S>,
    {
        Self {
            use_hook: fn_wrapper::FnMut(use_hook),
            _phantom: PhantomData,
        }
    }
}

impl<HookData: HookPollNextUpdate + HookUnmount + Default, U>
    FnHookElement<HookData, fn_wrapper::FnMutOutputElement<U>, ()>
{
    #[allow(non_snake_case)]
    pub fn FnMutOutputElement<Ctx, E: UpdateRenderState<Ctx>>(use_hook: U) -> Self
    where
        U: FnMut(Pin<&mut HookData>) -> E,
    {
        Self {
            use_hook: fn_wrapper::FnMutOutputElement(use_hook),
            _phantom: PhantomData,
        }
    }
}

pin_project_lite::pin_project!(
    pub struct FnMutHookElementState<HookData, U, Ctx, S>
    where
        HookData: HookPollNextUpdate,
        HookData: HookUnmount,
    {
        #[pin]
        hook_data: HookData,
        #[pin]
        ctx_and_state: ContextAndStateData<Ctx, S>,
        use_hook: U,
        initialized: bool,
    }
);

impl<HookData: HookPollNextUpdate + HookUnmount, U, Ctx, S: RenderState>
    FnMutHookElementState<HookData, U, Ctx, S>
{
    pub fn impl_unmount(self: Pin<&mut Self>) {
        let this = self.project();
        this.hook_data.unmount();
        this.ctx_and_state.unmount();
    }
}

pub mod fn_wrapper {
    pub struct FnMut<U>(pub U);
    pub struct FnMutOutputElement<U>(pub U);
}

impl<HookData: HookPollNextUpdate + HookUnmount, U, Ctx: HookContext, S: RenderState> RenderState
    for FnMutHookElementState<HookData, fn_wrapper::FnMut<U>, Ctx, S>
where
    U: FnMut(Pin<&mut HookData>, ContextAndState<'_, Ctx, S>) -> Rendered<S>,
{
    #[inline]
    fn unmount(self: Pin<&mut Self>) {
        self.impl_unmount()
    }

    fn poll_reactive(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<()> {
        let mut this = self.project();

        let res = if *this.initialized {
            this.hook_data.as_mut().poll_next_update(cx)
        } else {
            std::task::Poll::Ready(true)
        };
        let r = this.ctx_and_state.as_mut().poll_reactive(cx);

        match (res, r) {
            (std::task::Poll::Ready(false), std::task::Poll::Ready(())) => {
                std::task::Poll::Ready(())
            }
            (std::task::Poll::Ready(true), _) => {
                this.ctx_and_state.with_context_and_state(|cs| {
                    let _: Rendered<S> = (this.use_hook.0)(this.hook_data, cs);
                });
                *this.initialized = true;
                cx.waker().wake_by_ref();
                std::task::Poll::Pending
            }
            _ => std::task::Poll::Pending,
        }
    }
}

impl<
        HookData: HookPollNextUpdate + HookUnmount,
        U,
        Ctx: HookContext,
        E: UpdateRenderState<Ctx>,
    > RenderState
    for FnMutHookElementState<HookData, fn_wrapper::FnMutOutputElement<U>, Ctx, E::State>
where
    U: FnMut(Pin<&mut HookData>) -> E,
{
    #[inline]
    fn unmount(self: Pin<&mut Self>) {
        self.impl_unmount()
    }

    fn poll_reactive(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<()> {
        let mut this = self.project();

        let res = if *this.initialized {
            this.hook_data.as_mut().poll_next_update(cx)
        } else {
            std::task::Poll::Ready(true)
        };
        let r = this.ctx_and_state.as_mut().poll_reactive(cx);

        match (res, r) {
            (std::task::Poll::Ready(false), std::task::Poll::Ready(())) => {
                std::task::Poll::Ready(())
            }
            (std::task::Poll::Ready(true), _) => {
                this.ctx_and_state.with_context_and_state(|cs| {
                    let element = (this.use_hook.0)(this.hook_data);
                    let _: Rendered<E::State> = cs.render(element);
                });
                *this.initialized = true;
                cx.waker().wake_by_ref();
                std::task::Poll::Pending
            }
            _ => std::task::Poll::Pending,
        }
    }
}

impl<HookData, U, S, Ctx> UpdateRenderState<Ctx>
    for FnHookElement<HookData, fn_wrapper::FnMut<U>, S>
where
    HookData: HookPollNextUpdate + HookUnmount + Default,
    Ctx: HookContext,
    S: RenderState,
    U: FnMut(Pin<&mut HookData>, ContextAndState<'_, Ctx, S>) -> Rendered<S>,
{
    type State = FnMutHookElementState<HookData, fn_wrapper::FnMut<U>, Ctx, S>;

    fn initialize_render_state(self, ctx: &mut Ctx) -> Self::State {
        FnMutHookElementState {
            hook_data: Default::default(),
            ctx_and_state: ContextAndStateData::new(HookContext::take_context(ctx)),
            use_hook: self.use_hook,
            initialized: false,
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
        *state.initialized = true;
        *state.use_hook = use_hook;
    }
}

impl<HookData, U, E: UpdateRenderState<Ctx>, Ctx> UpdateRenderState<Ctx>
    for FnHookElement<HookData, fn_wrapper::FnMutOutputElement<U>, ()>
where
    HookData: HookPollNextUpdate + HookUnmount + Default,
    Ctx: HookContext,
    U: FnMut(Pin<&mut HookData>) -> E,
{
    type State = FnMutHookElementState<HookData, fn_wrapper::FnMutOutputElement<U>, Ctx, E::State>;

    fn initialize_render_state(self, ctx: &mut Ctx) -> Self::State {
        FnMutHookElementState {
            hook_data: Default::default(),
            ctx_and_state: ContextAndStateData::new(HookContext::take_context(ctx)),
            use_hook: self.use_hook,
            initialized: false,
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

        *state.initialized = true;

        let _: Rendered<E::State> = ctx_and_state.render(element);

        *state.use_hook = use_hook;
    }
}
