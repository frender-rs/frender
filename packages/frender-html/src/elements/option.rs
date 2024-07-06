use std::pin::Pin;

use crate::{Element, HtmlRenderContext, RenderState, RenderStateOfContext, UnpinnedRenderStateOfContext};

macro_rules! update_option {
    ($_self:ident . $method:ident ($ctx:ident, $state:ident $(, $arg:expr)? )) => {
        if let Some(this) = $_self {
            this.$method($ctx, $state $(, $arg)?);
        } else {
            <_ as RenderState<_>>::unmount($state, frender_dom::render::RenderContext::renderer_mut($ctx))
        }
    };
}

macro_rules! unpinned_update_option {
    ($_self:ident . $method:ident ($ctx:ident, $state:ident $(, $arg:expr)? )) => {
        if let Some(this) = $_self {
            this.$method($ctx, $state $(, $arg)?);
        } else {
            <_ as RenderState<_>>::unmount(
                Pin::new($state),
                frender_dom::render::RenderContext::renderer_mut($ctx)
            )
        }
    };
}

impl<E: Element> Element for Option<E> {
    type RenderStateKind = E::RenderStateKind;

    fn render_update<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>)
    where
        Self: Sized,
    {
        update_option!(self.render_update(render_context, render_state))
    }

    fn render_update_force_reposition<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>)
    where
        Self: Sized,
    {
        update_option!(self.render_update_force_reposition(render_context, render_state))
    }

    fn render_update_maybe_reposition<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>, force_reposition: bool) {
        update_option!(self.render_update_maybe_reposition(
            //
            render_context,
            render_state,
            force_reposition
        ))
    }

    fn unpinned_render_update<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: &mut UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>)
    where
        Self: Sized,
    {
        unpinned_update_option!(self.unpinned_render_update(render_context, render_state))
    }

    fn unpinned_render_update_force_reposition<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: &mut UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>)
    where
        Self: Sized,
    {
        unpinned_update_option!(self.unpinned_render_update_force_reposition(render_context, render_state))
    }

    fn unpinned_render_update_maybe_reposition<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: &mut UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>, force_reposition: bool) {
        unpinned_update_option!(self.unpinned_render_update_maybe_reposition(
            //
            render_context,
            render_state,
            force_reposition
        ))
    }
}
