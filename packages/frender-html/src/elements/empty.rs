use frender_dom::Empty;

use crate::{element::RenderStates, kinds::KindOfNoState, CsrElement, HtmlRenderContext};

impl CsrElement for Empty {
    type RenderStateKind = KindOfNoState;

    fn pinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        _: &mut Ctx,
        _: crate::element::PinMutRenderInitStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) -> crate::element::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
    }

    fn pinned_render_update<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        _: &mut Ctx,
        _: crate::element::PinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) {
    }

    fn unpinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        _: &mut Ctx,
    ) -> crate::element::UnpinnedRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer> {
        RenderStates {
            ui_handle: (),
            non_reactive_state: (),
            reactive_state: (),
        }
    }

    fn unpinned_render_update<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        _: &mut Ctx,
        _: crate::element::UnpinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) {
    }
}
