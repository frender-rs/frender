use frender_dom::Empty;

use crate::{
    kinds::{KindOfNoState, RenderInitNothing},
    CsrElement, HtmlRenderContext,
};

impl CsrElement for Empty {
    type RenderStateKind = KindOfNoState;

    fn pinned_render_init<Renderer: ?Sized + crate::RenderHtml>(
        //
        self,
        _: &mut Renderer,
    ) -> (
        //
        crate::element::PinnedStateOfKind<Renderer, Self::RenderStateKind>,
        crate::element::PinnedRenderInitOfKind<Renderer, Self::RenderStateKind>,
    ) {
        ((), RenderInitNothing)
    }

    fn pinned_render_init_by_reusing<Ctx: ?Sized + HtmlRenderContext>(
        self,
        _: &mut Ctx,
        _: std::pin::Pin<&mut crate::element::PinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>>,
        (): crate::element::PinnedUnmountedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
    ) -> crate::element::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
    }

    fn pinned_render_update<Renderer: ?Sized + crate::RenderHtml>(
        //
        self,
        _: &mut Renderer,
        _: std::pin::Pin<&mut crate::element::PinnedStateOfKind<Renderer, Self::RenderStateKind>>,
        (): &mut crate::element::PinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
    }

    fn unpinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        _: &mut Ctx,
    ) -> (
        //
        crate::element::UnpinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        crate::element::UnpinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
    ) {
        ((), ())
    }

    fn unpinned_render_init_by_reusing<Ctx: ?Sized + HtmlRenderContext>(
        self,
        _: &mut Ctx,
        (): &mut crate::element::UnpinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        (): crate::element::UnpinnedUnmountedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
    ) -> crate::element::UnpinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
    }

    fn unpinned_render_update<Renderer: ?Sized + crate::RenderHtml>(
        //
        self,
        _: &mut Renderer,
        (): &mut crate::element::UnpinnedStateOfKind<Renderer, Self::RenderStateKind>,
        (): &mut crate::element::UnpinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
    }
}
