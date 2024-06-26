use std::pin::Pin;

use crate::{Element, RenderHtml, RenderState};

macro_rules! update_option {
    ($_self:ident . $method:ident ($ctx:ident, $state:ident $(, $arg:expr)? )) => {
        if let Some(this) = $_self {
            this.$method($ctx, $state $(, $arg)?);
        } else {
            <E::RenderState<Renderer> as RenderState<_>>::unmount($state, frender_dom::render::RenderContext::renderer_mut($ctx))
        }
    };
}

macro_rules! unpinned_update_option {
    ($_self:ident . $method:ident ($ctx:ident, $state:ident $(, $arg:expr)? )) => {
        if let Some(this) = $_self {
            this.$method($ctx, $state $(, $arg)?);
        } else {
            <E::UnpinnedRenderState<Renderer> as RenderState<_>>::unmount(
                Pin::new($state),
                frender_dom::render::RenderContext::renderer_mut($ctx)
            )
        }
    };
}

impl<E: Element> Element for Option<E> {
    type RenderState<R: RenderHtml + ?Sized> = E::RenderState<R>;

    fn render_update<Renderer: RenderHtml + ?Sized>(
        //
        self,
        renderer: &mut Renderer::RenderContext<'_>,
        render_state: Pin<&mut Self::RenderState<Renderer>>,
    ) where
        Self: Sized,
    {
        update_option!(self.render_update(renderer, render_state))
    }

    fn render_update_force_reposition<Renderer: RenderHtml + ?Sized>(
        //
        self,
        renderer: &mut Renderer::RenderContext<'_>,
        render_state: Pin<&mut Self::RenderState<Renderer>>,
    ) where
        Self: Sized,
    {
        update_option!(self.render_update_force_reposition(renderer, render_state))
    }

    fn render_update_maybe_reposition<Renderer: RenderHtml + ?Sized>(
        //
        self,
        renderer: &mut Renderer::RenderContext<'_>,
        render_state: Pin<&mut Self::RenderState<Renderer>>,
        force_reposition: bool,
    ) {
        update_option!(self.render_update_maybe_reposition(
            //
            renderer,
            render_state,
            force_reposition
        ))
    }

    type UnpinnedRenderState<R: RenderHtml + ?Sized> = E::UnpinnedRenderState<R>;

    fn unpinned_render_update<Renderer: RenderHtml + ?Sized>(
        //
        self,
        renderer: &mut Renderer::RenderContext<'_>,
        render_state: &mut Self::UnpinnedRenderState<Renderer>,
    ) where
        Self: Sized,
    {
        unpinned_update_option!(self.unpinned_render_update(renderer, render_state))
    }

    fn unpinned_render_update_force_reposition<Renderer: RenderHtml + ?Sized>(
        //
        self,
        renderer: &mut Renderer::RenderContext<'_>,
        render_state: &mut Self::UnpinnedRenderState<Renderer>,
    ) where
        Self: Sized,
    {
        unpinned_update_option!(self.unpinned_render_update_force_reposition(renderer, render_state))
    }

    fn unpinned_render_update_maybe_reposition<Renderer: RenderHtml + ?Sized>(
        //
        self,
        renderer: &mut Renderer::RenderContext<'_>,
        render_state: &mut Self::UnpinnedRenderState<Renderer>,
        force_reposition: bool,
    ) {
        unpinned_update_option!(self.unpinned_render_update_maybe_reposition(
            //
            renderer,
            render_state,
            force_reposition
        ))
    }
}
