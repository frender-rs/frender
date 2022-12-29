use std::{any::Any, pin::Pin};

use frender_core::{RenderState, UpdateRenderState};
use frender_dom::Dom;
use hooks::Hook;

use crate::{
    ContextAndState, HookElementPollTillEnd, HookElementWithNoProps, HookElementWithOwnedProps,
    HookElementWithRefProps, HookStatePollOnce, HookStateWithNoProps, HookStateWithRefProps,
};

impl<F, F2, H, S: RenderState + 'static, Props> UpdateRenderState<Dom>
    for HookElementWithOwnedProps<F, F2, Props>
where
    F: FnOnce() -> H,
    H: for<'a> Hook<
        (ContextAndState<'a, Dom, dyn Any>, Props),
        Value = ContextAndState<'a, Dom, S>,
    >,
{
    type State = HookStatePollOnce<H, S>;

    fn update_render_state(self, ctx: &mut Dom, state: Pin<&mut Self::State>) {
        let state = state.project();
        let hook = state.hook.use_hook((self.with_dom,));
        hook.use_hook((ContextAndState::new(ctx, state.render_state), self.props));
    }
}

impl<F2, F, H, S: RenderState + 'static> UpdateRenderState<Dom> for HookElementWithNoProps<F, F2>
where
    F: FnOnce() -> H,
    H: for<'a> Hook<(ContextAndState<'a, Dom, dyn Any>,), Value = ContextAndState<'a, Dom, S>>,
{
    type State = HookStateWithNoProps<H, Dom, S>;

    fn update_render_state(self, ctx: &mut Dom, state: Pin<&mut Self::State>) {
        let state = state.project();
        *state.ctx = Some(ctx.clone());
        let hook = state.hook.use_hook((self.with_dom,));
        hook.use_hook((ContextAndState::new(ctx, state.render_state),));
    }
}

impl<F, HSsr, H, S: RenderState + 'static, Props> UpdateRenderState<Dom>
    for HookElementPollTillEnd<HookElementWithRefProps<F, HSsr, Props>>
where
    F: FnOnce() -> H,
    H: for<'a, 'props> Hook<
        (ContextAndState<'a, Dom, dyn Any>, &'props Props),
        Value = ContextAndState<'a, Dom, S>,
    >,
{
    type State = HookStateWithRefProps<H, Dom, S, Props>;

    fn update_render_state(self, ctx: &mut Dom, state: Pin<&mut Self::State>) {
        let state = state.project();
        let (_, props) = state.ctx_and_props.insert((ctx.clone(), self.0.props));
        let hook = state.hook.use_hook((self.0.with_dom,));
        hook.use_hook((ContextAndState::new(ctx, state.render_state), props));
    }
}

impl<F, FSsr, H, S: RenderState + 'static, Props> UpdateRenderState<Dom>
    for HookElementWithRefProps<F, FSsr, Props>
where
    F: FnOnce() -> H,
    H: for<'a, 'props> Hook<
        (ContextAndState<'a, Dom, dyn Any>, &'props Props),
        Value = ContextAndState<'a, Dom, S>,
    >,
{
    type State = HookStatePollOnce<H, S>;

    fn update_render_state(self, ctx: &mut Dom, state: Pin<&mut Self::State>) {
        let state = state.project();
        let hook = state.hook.use_hook((self.with_dom,));
        hook.use_hook((ContextAndState::new(ctx, state.render_state), &self.props));
    }
}
