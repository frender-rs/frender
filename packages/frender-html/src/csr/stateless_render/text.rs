use std::marker::PhantomData;

use frender_dom::csr::render::{KnownValueForText, TextKind};

use crate::{csr::element::HtmlRenderContext, html::RenderHtml};

use super::{StatelessRender, StatelessRenderStateKind, StatelessUiHandleOfKind};

enum Never {}
pub struct Kind<TK: ?Sized + TextKind>(Never, PhantomData<TK>);

impl<TK: ?Sized + TextKind> StatelessRenderStateKind for Kind<TK> {
    type UiHandle<R: ?Sized + RenderHtml> = TK::Text<R>;
}

impl<T: KnownValueForText> StatelessRender for T {
    type StatelessRenderStateKind = Kind<T::TextKind>;

    fn stateless_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
    ) -> StatelessUiHandleOfKind<Ctx::Renderer, Self::StatelessRenderStateKind> {
        render_context.map_mut_render_context(|render_context| self.render_text_from_self(render_context))
    }

    fn stateless_render_update<R: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut R,
        ui_handle: &mut StatelessUiHandleOfKind<R, Self::StatelessRenderStateKind>,
    ) {
        self.update_text_from_self(renderer, ui_handle)
    }
}
