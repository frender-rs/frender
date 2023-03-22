use std::{marker::PhantomData, pin::Pin};

use frender_core::{RenderState, UpdateRenderState};
use hooks_core::{HookPollNextUpdate, HookUnmount};
use lazy_pinned::LazyPinned;

use crate::{
    lazy_pinned_state_poll_reactive, lazy_pinned_state_unmount, ContextAndState, HookContext,
    Rendered,
};

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
    ) -> FnHookElement<HookData, fn_wrapper::FnMutOutputElement<U>>
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
    ) -> FnHookElement<HookData, fn_wrapper::FnMutOutputElement<U>>
    where
        U: FnMut(Pin<&mut HookData>) -> E,
    {
        FnHookElement {
            use_hook: fn_wrapper::FnMutOutputElement(use_hook),
            _phantom: std::marker::PhantomData,
        }
    }

    #[inline]
    #[cfg(all(feature = "csr", feature = "ssr"))]
    pub fn ssr_and_csr<
        HookData: HookPollNextUpdate + HookUnmount + Default,
        U,
        E: UpdateRenderState<Dom> + for<'a> UpdateRenderState<AnySsrContext<'a>>,
    >(
        use_hook: U,
    ) -> FnHookElement<HookData, fn_wrapper::FnMutOutputElement<U>>
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
pub use new_fn_hook_element::ssr_and_csr as new_fn_hook_element;

pub struct FnHookElement<HookData: HookPollNextUpdate + HookUnmount + Default, U> {
    use_hook: U,
    _phantom: PhantomData<HookData>,
}

impl<HookData: HookPollNextUpdate + HookUnmount + Default, U, S>
    FnHookElement<HookData, fn_wrapper::FnMutWithContextAndState<U, S>>
{
    #[allow(non_snake_case)]
    pub fn FnMut<Ctx>(use_hook: U) -> Self
    where
        U: for<'a> FnMut(Pin<&'a mut HookData>, ContextAndState<'a, Ctx, S>) -> Rendered<S>,
        S: RenderState<Ctx>,
    {
        Self {
            use_hook: fn_wrapper::FnMutWithContextAndState(use_hook, PhantomData),
            _phantom: PhantomData,
        }
    }
}

impl<HookData: HookPollNextUpdate + HookUnmount + Default, U>
    FnHookElement<HookData, fn_wrapper::FnMutOutputElement<U>>
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
    pub struct FnMutHookElementState<HookData, U, S>
    where
        HookData: HookPollNextUpdate,
        HookData: HookUnmount,
    {
        #[pin]
        hook_data: HookData,
        #[pin]
        state: LazyPinned<S>,
        use_hook: U,
        initialized: bool,
    }
);

impl<HookData: HookPollNextUpdate + HookUnmount, U, S> FnMutHookElementState<HookData, U, S> {
    pub fn impl_unmount<Ctx>(self: Pin<&mut Self>)
    where
        S: RenderState<Ctx>,
    {
        let this = self.project();
        this.hook_data.unmount();
        lazy_pinned_state_unmount(this.state);
    }
}

pub mod fn_wrapper {
    use std::marker::PhantomData;

    pub struct FnMutWithContextAndState<U, S>(pub U, pub PhantomData<S>);
    pub struct FnMutOutputElement<U>(pub U);
}

pub trait UseRender<InnerHook, Ctx> {
    type State: RenderState<Ctx>;

    fn use_render<'a>(
        &'a mut self,
        inner_hook: Pin<&'a mut InnerHook>,
        cs: ContextAndState<'a, Ctx, Self::State>,
    ) -> Rendered<Self::State>;
}

impl<InnerHook, Ctx, U, S> UseRender<InnerHook, Ctx> for fn_wrapper::FnMutWithContextAndState<U, S>
where
    S: RenderState<Ctx>,
    U: for<'a> FnMut(Pin<&'a mut InnerHook>, ContextAndState<'a, Ctx, S>) -> Rendered<S>,
{
    type State = S;

    #[inline]
    fn use_render<'a>(
        &'a mut self,
        inner_hook: Pin<&'a mut InnerHook>,
        cs: ContextAndState<'a, Ctx, S>,
    ) -> Rendered<S> {
        (self.0)(inner_hook, cs)
    }
}

impl<InnerHook, Ctx, U, E: UpdateRenderState<Ctx>> UseRender<InnerHook, Ctx>
    for fn_wrapper::FnMutOutputElement<U>
where
    U: FnMut(Pin<&mut InnerHook>) -> E,
{
    type State = <E as UpdateRenderState<Ctx>>::State;

    #[inline]
    fn use_render<'a>(
        &'a mut self,
        inner_hook: Pin<&'a mut InnerHook>,
        cs: ContextAndState<'a, Ctx, <E as UpdateRenderState<Ctx>>::State>,
    ) -> Rendered<<E as UpdateRenderState<Ctx>>::State> {
        let element = (self.0)(inner_hook);
        cs.render(element)
    }
}

impl<HookData: HookPollNextUpdate + HookUnmount, U, Ctx> RenderState<Ctx>
    for FnMutHookElementState<HookData, U, U::State>
where
    Ctx: HookContext,
    U: UseRender<HookData, Ctx>,
{
    #[inline]
    fn unmount(self: Pin<&mut Self>) {
        self.impl_unmount()
    }

    fn poll_reactive(
        self: Pin<&mut Self>,
        ctx: &mut Ctx,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        let mut this = self.project();

        let res = if *this.initialized {
            this.hook_data.as_mut().poll_next_update(cx)
        } else {
            std::task::Poll::Ready(true)
        };

        let old_context_data = Ctx::get_context_data(ctx);

        let r = lazy_pinned_state_poll_reactive(this.state.as_mut(), ctx, cx);

        match (res, r) {
            (std::task::Poll::Ready(false), std::task::Poll::Ready(())) => {
                std::task::Poll::Ready(())
            }
            (std::task::Poll::Ready(true), _) => {
                Ctx::replace_context_data(ctx, old_context_data);

                let cs = ContextAndState::new(ctx, this.state);
                let _: Rendered<U::State> = this.use_hook.use_render(this.hook_data, cs);

                *this.initialized = true;
                cx.waker().wake_by_ref();
                std::task::Poll::Pending
            }
            _ => std::task::Poll::Pending,
        }
    }
}

impl<HookData: HookPollNextUpdate + HookUnmount, U, Ctx> UpdateRenderState<Ctx>
    for FnHookElement<HookData, U>
where
    HookData: Default,
    Ctx: HookContext,
    U: UseRender<HookData, Ctx>,
{
    type State = FnMutHookElementState<HookData, U, U::State>;

    fn initialize_render_state(self, _ctx: &mut Ctx) -> Self::State {
        // TODO: eagerly initialize for Unpin HookData
        FnMutHookElementState {
            hook_data: Default::default(),
            state: LazyPinned(None),
            use_hook: self.use_hook,
            initialized: false,
        }
    }

    fn update_render_state(self, ctx: &mut Ctx, state: std::pin::Pin<&mut Self::State>) {
        let state = state.project();

        let cs = ContextAndState::new(ctx, state.state);
        let mut use_hook = self.use_hook;
        let _: Rendered<U::State> = use_hook.use_render(state.hook_data, cs);
        *state.initialized = true;
        *state.use_hook = use_hook;
    }
}
