use frender_dom::Empty;

use crate::{
    csr::element::{self, CsrElement, HtmlRenderContext},
    html::RenderHtml,
    kinds::{KindOfNoState, RenderInitNothing},
};

impl CsrElement for Empty {
    type RenderStateKind = KindOfNoState;
    type PinnedRenderInit<R: ?Sized + RenderHtml> = RenderInitNothing;

    fn pinned_render_init<Renderer: ?Sized + RenderHtml>(
        //
        self,
        _: &mut Renderer,
    ) -> (
        //
        element::PinnedStateOfKind<Renderer, Self::RenderStateKind>,
        Self::PinnedRenderInit<Renderer>,
    ) {
        ((), RenderInitNothing)
    }

    fn pinned_render_init_by_reusing<Ctx: ?Sized + HtmlRenderContext>(
        self,
        _: &mut Ctx,
        _: std::pin::Pin<&mut element::PinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>>,
        (): element::PinnedUnmountedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
    ) -> element::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
    }

    fn pinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        _: &mut Renderer,
        _: std::pin::Pin<&mut element::PinnedStateOfKind<Renderer, Self::RenderStateKind>>,
        (): &mut element::PinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
    }

    fn unpinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        _: &mut Ctx,
    ) -> (
        //
        element::UnpinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        element::UnpinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
    ) {
        ((), ())
    }

    fn unpinned_render_init_by_reusing<Ctx: ?Sized + HtmlRenderContext>(
        self,
        _: &mut Ctx,
        (): &mut element::UnpinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        (): element::UnpinnedUnmountedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
    ) -> element::UnpinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
    }

    fn unpinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        _: &mut Renderer,
        (): &mut element::UnpinnedStateOfKind<Renderer, Self::RenderStateKind>,
        (): &mut element::UnpinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
    }
}
