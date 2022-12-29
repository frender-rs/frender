use std::{any::Any, pin::Pin};

use frender_core::{RenderState, UpdateRenderState};
use frender_ssr::{AsyncWrite, SsrContext};
use hooks::Hook;

use crate::{
    ContextAndState, HookElementPollTillEnd, HookElementWithNoProps, HookElementWithOwnedProps,
    HookElementWithRefProps, HookStatePollOnce, HookStateWithNoProps, HookStateWithRefProps,
};

impl<F2, F, H, S: RenderState + 'static, W: AsyncWrite + Unpin> UpdateRenderState<SsrContext<W>>
    for HookElementWithNoProps<F, F2>
where
    F: FnOnce() -> H,
    H: for<'a> Hook<
        (ContextAndState<'a, SsrContext<W>, dyn Any>,),
        Value = ContextAndState<'a, SsrContext<W>, S>,
    >,
{
    type State = HookStateWithNoProps<H, SsrContext<W>, S>;

    fn update_render_state(self, ctx: &mut SsrContext<W>, state: Pin<&mut Self::State>) {
        let state = state.project();
        *state.ctx = Some(SsrContext {
            writer: ctx.writer.take(),
        });
        let hook = state.hook.use_hook((self.with_dom,));
        hook.use_hook((ContextAndState::new(ctx, state.render_state),));
    }
}

impl<F, HDom, H, S: RenderState + 'static, Props, W: AsyncWrite + Unpin>
    UpdateRenderState<SsrContext<W>>
    for HookElementPollTillEnd<HookElementWithRefProps<HDom, F, Props>>
where
    F: FnOnce() -> H,
    H: for<'a, 'props> Hook<
        (ContextAndState<'a, SsrContext<W>, dyn Any>, &'props Props),
        Value = ContextAndState<'a, SsrContext<W>, S>,
    >,
{
    type State = HookStateWithRefProps<H, SsrContext<W>, S, Props>;

    fn update_render_state(self, ctx: &mut SsrContext<W>, state: Pin<&mut Self::State>) {
        let state = state.project();
        let (_, props) = state
            .ctx_and_props
            .insert((std::mem::take(ctx), self.0.props));
        let hook = state.hook.use_hook((self.0.with_ssr,));
        hook.use_hook((ContextAndState::new(ctx, state.render_state), props));
    }
}

impl<F, FSsr, H, S: RenderState + 'static, Props, W: AsyncWrite + Unpin>
    UpdateRenderState<SsrContext<W>> for HookElementWithRefProps<F, FSsr, Props>
where
    F: FnOnce() -> H,
    H: for<'a, 'props> Hook<
        (ContextAndState<'a, SsrContext<W>, dyn Any>, &'props Props),
        Value = ContextAndState<'a, SsrContext<W>, S>,
    >,
{
    type State = HookStatePollOnce<H, S>;

    fn update_render_state(self, ctx: &mut SsrContext<W>, state: Pin<&mut Self::State>) {
        let state = state.project();
        let hook = state.hook.use_hook((self.with_dom,));
        hook.use_hook((ContextAndState::new(ctx, state.render_state), &self.props));
    }
}

impl<F, F1, H, S: RenderState + 'static, Props, W: AsyncWrite + Unpin>
    UpdateRenderState<SsrContext<W>> for HookElementWithOwnedProps<F1, F, Props>
where
    F: FnOnce() -> H,
    H: for<'a> Hook<
        (ContextAndState<'a, SsrContext<W>, dyn Any>, Props),
        Value = ContextAndState<'a, SsrContext<W>, S>,
    >,
{
    type State = HookStatePollOnce<H, S>;

    fn update_render_state(self, ctx: &mut SsrContext<W>, state: Pin<&mut Self::State>) {
        let state = state.project();
        let hook = state.hook.use_hook((self.with_ssr,));
        hook.use_hook((ContextAndState::new(ctx, state.render_state), self.props));
    }
}
