use frender_dom::csr::UiHandle;

use crate::{csr::element::HtmlRenderContext, html::RenderHtml};

pub mod text;

pub trait StatelessRenderStateKind {
    type UiHandle<R: ?Sized + RenderHtml>: UiHandle<R>;
}

pub type StatelessUiHandleOfKind<R, K> = <K as StatelessRenderStateKind>::UiHandle<R>;

pub trait StatelessRender {
    type StatelessRenderStateKind: StatelessRenderStateKind;
    fn stateless_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
    ) -> StatelessUiHandleOfKind<Ctx::Renderer, Self::StatelessRenderStateKind>;
    fn stateless_render_update<R: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut R,
        ui_handle: &mut StatelessUiHandleOfKind<R, Self::StatelessRenderStateKind>,
    );
}
