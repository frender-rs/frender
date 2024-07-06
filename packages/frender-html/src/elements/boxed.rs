use std::pin::Pin;

use crate::{Element, HtmlRenderContext, RenderStateOfContext, UnpinnedRenderStateOfContext};

impl<E: Element> Element for Box<E> {
    type RenderStateKind = E::RenderStateKind;

    fn render_update<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>) {
        E::render_update(*self, render_context, render_state)
    }

    fn render_update_force_reposition<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>) {
        E::render_update_force_reposition(*self, render_context, render_state)
    }

    fn render_update_maybe_reposition<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>, force_reposition: bool) {
        E::render_update_maybe_reposition(*self, render_context, render_state, force_reposition)
    }

    fn unpinned_render_update<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: &mut UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>) {
        E::unpinned_render_update(*self, render_context, render_state)
    }

    fn unpinned_render_update_force_reposition<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: &mut UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>) {
        E::unpinned_render_update_force_reposition(*self, render_context, render_state)
    }

    fn unpinned_render_update_maybe_reposition<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: &mut UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>, force_reposition: bool) {
        E::unpinned_render_update_maybe_reposition(*self, render_context, render_state, force_reposition)
    }
}
