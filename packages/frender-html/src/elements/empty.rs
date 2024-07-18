use std::pin::Pin;

use frender_dom::Empty;

use crate::{kinds::KindOfNoState, Element, HtmlRenderContext, RenderStateOfContext};

impl Element for Empty {
    type RenderStateKind = KindOfNoState;

    fn render_update<Ctx: ?Sized + HtmlRenderContext>(self, _: &mut Ctx, _: Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>) {}
    fn render_update_force_reposition<Ctx: ?Sized + HtmlRenderContext>(self, _: &mut Ctx, _: Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>) {}
    fn render_update_maybe_reposition<Ctx: ?Sized + HtmlRenderContext>(self, _: &mut Ctx, _: Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>, _: bool) {}

    crate::impl_unpinned_render_for_unpin! {}
}
