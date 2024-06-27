use std::pin::Pin;

use either::Either;

use crate::{render_state::either::EitherRenderState, Element, HtmlRenderContext, RenderHtml, RenderState, RenderStateKind, RenderStateOfContext, UnpinnedRenderStateOfContext};

macro_rules! update_either {
    ($_self:ident . $method:ident($ctx:ident, $state:ident $(, $arg:expr)?)) => {{
        let mut $state = $state.project_inner();
        match $_self {
            Either::Left(e) => {
                if let Either::Right(other_state) = $state.as_mut().as_pin_mut() {
                    other_state.unmount(frender_dom::render::RenderContext::renderer_mut($ctx));
                    $state.set(Either::Left(Default::default()))
                }

                let state = if let Either::Left(state) = $state.as_pin_mut() {
                    state
                } else {
                    unreachable!();
                };

                e.$method($ctx, state $(, $arg)?)
            }
            Either::Right(e) => {
                if let Either::Left(other_state) = $state.as_mut().as_pin_mut() {
                    other_state.unmount(frender_dom::render::RenderContext::renderer_mut($ctx));
                    $state.set(Either::Right(Default::default()))
                }

                let state = if let Either::Right(state) = $state.as_pin_mut() {
                    state
                } else {
                    unreachable!();
                };

                e.$method($ctx, state $(, $arg)?)
            }
        }
    }};
}

macro_rules! unpinned_update_either {
    ($_self:ident . $method:ident($ctx:ident, $state:ident $(, $arg:expr)?)) => {{
        let $state = $state.inner_mut();
        match $_self {
            Either::Left(e) => {
                if let Either::Right(other_state) = $state {
                    Pin::new(other_state).unmount(frender_dom::render::RenderContext::renderer_mut($ctx));
                    *$state = Either::Left(Default::default());
                }

                let state = if let Either::Left(state) = $state {
                    state
                } else {
                    unreachable!();
                };

                e.$method($ctx, state $(, $arg)?)
            }
            Either::Right(e) => {
                if let Either::Left(other_state) = $state {
                    Pin::new(other_state).unmount(frender_dom::render::RenderContext::renderer_mut($ctx));
                    *$state = Either::Right(Default::default());
                }

                let state = if let Either::Right(state) = $state {
                    state
                } else {
                    unreachable!();
                };

                e.$method($ctx, state $(, $arg)?)
            }
        }
    }};
}

pub struct Kind<KA, KB>(super::Kind<(KA, KB)>);

impl<KA: RenderStateKind, KB: RenderStateKind> crate::RenderStateKindPinned for Kind<KA, KB> {
    type RenderState<Renderer: RenderHtml + ?Sized> = EitherRenderState<KA::RenderState<Renderer>, KB::RenderState<Renderer>>;
}
impl<KA: RenderStateKind, KB: RenderStateKind> crate::RenderStateKindUnpinned for Kind<KA, KB> {
    type UnpinnedRenderState<Renderer: RenderHtml + ?Sized> = EitherRenderState<KA::UnpinnedRenderState<Renderer>, KB::UnpinnedRenderState<Renderer>>;
}

impl<L, R> Element for Either<L, R>
where
    L: Element,
    R: Element,
{
    type RenderStateKind = Kind<L::RenderStateKind, R::RenderStateKind>;
    fn render_update<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>) {
        update_either!(self.render_update(render_context, render_state))
    }

    fn render_update_force_reposition<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>) {
        update_either!(self.render_update_force_reposition(render_context, render_state))
    }

    fn render_update_maybe_reposition<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>, force_reposition: bool) {
        update_either!(self.render_update_maybe_reposition(render_context, render_state, force_reposition))
    }

    fn unpinned_render_update<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: &mut UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>) {
        unpinned_update_either!(self.unpinned_render_update(render_context, render_state))
    }

    fn unpinned_render_update_force_reposition<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: &mut UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>) {
        unpinned_update_either!(self.unpinned_render_update_force_reposition(render_context, render_state))
    }

    fn unpinned_render_update_maybe_reposition<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: &mut UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>, force_reposition: bool) {
        unpinned_update_either!(self.unpinned_render_update_maybe_reposition(render_context, render_state, force_reposition))
    }
}
