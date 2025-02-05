use std::marker::PhantomData;

use frender_common::convert::FromMut;
use frender_dom::render::RenderIntoTextKnownKind;
use frender_dom::render::{RenderContextRenderTextFrom as _, RenderIntoTextKnown, RenderTextFrom, RenderWithContext};

use crate::{HtmlRenderContext, RenderHtml};

use super::{StatelessRender, StatelessRenderStateKind, StatelessUiHandleOfKind};

enum Never {}
pub struct Kind<V: 'static + RenderIntoTextKnown>(Never, PhantomData<V>);

impl<V: 'static + RenderIntoTextKnown> StatelessRenderStateKind for Kind<V> {
    type UiHandle<R: ?Sized + RenderHtml> = <V::RenderTextFromSelf<R> as RenderTextFrom<V>>::Text;
}

// known text ('static)

impl<T: RenderIntoTextKnown> StatelessRender for T {
    type StatelessRenderStateKind = Kind<T::StaticRenderIntoTextKnown>;

    fn stateless_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
    ) -> StatelessUiHandleOfKind<Ctx::Renderer, Self::StatelessRenderStateKind> {
        // Ctx -> impl RenderContextRenderTextFrom<Self>
        render_context.map_mut_render_context(|render_context: &mut <Ctx::Renderer as RenderWithContext>::RenderContext<'_>| {
            let render_context: &mut <T::RenderTextFromSelf<Ctx::Renderer> as RenderWithContext>::RenderContext<'_> = FromMut::from_mut(render_context);

            let text: <T::RenderIntoTextKnownKind as RenderIntoTextKnownKind>::RenderIntoText<Ctx::Renderer> = render_context.render_text_from(self);

            text
        })
    }

    fn stateless_render_update<R: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut R,
        ui_handle: &mut StatelessUiHandleOfKind<R, Self::StatelessRenderStateKind>,
    ) {
        let renderer = T::RenderTextFromSelf::<R>::from_mut(renderer);
        renderer.update_text_from(ui_handle, self);
    }
}
