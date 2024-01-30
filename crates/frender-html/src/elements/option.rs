use std::pin::Pin;

use crate::{Element, RenderHtml, RenderState};

macro_rules! update_option {
    ($_self:ident . $method:ident ($peh:ident, $ctx:ident, $state:ident $(, $arg:expr)? )) => {
        if let Some(this) = $_self {
            this.$method($peh, $ctx, $state $(, $arg)?);
        } else {
            <E::RenderState<PEH, Renderer> as RenderState<_, _>>::unmount($state, $peh, $ctx)
        }
    };
}

macro_rules! unpinned_update_option {
    ($_self:ident . $method:ident ($peh:ident, $ctx:ident, $state:ident $(, $arg:expr)? )) => {
        if let Some(this) = $_self {
            this.$method($peh, $ctx, $state $(, $arg)?);
        } else {
            <E::UnpinnedRenderState<PEH, Renderer> as RenderState<_, _>>::unmount(
                Pin::new($state),
                $peh,
                $ctx
            )
        }
    };
}

impl<E: Element> Element for Option<E> {
    type RenderState<PEH: ?Sized, R: RenderHtml + ?Sized> = E::RenderState<PEH, R>;

    #[cfg(feature = "render_into")]
    fn render_into<'s, Renderer: RenderHtml>(
        //
        self,
        peh: &mut PEH,
        renderer: &mut Renderer,
        render_state: PinMutMaybeUninit<'s, Self::RenderState<PEH, Renderer>>,
    ) -> Pin<&'s mut Self::RenderState<PEH, Renderer>> {
        match self {
            Some(this) => Some(this.render_into(renderer)),
            None => render_state.write(None),
        }
    }

    fn render_update<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(
        //
        self,
        peh: &mut PEH,
        renderer: &mut Renderer,
        render_state: Pin<&mut Self::RenderState<PEH, Renderer>>,
    ) where
        Self: Sized,
    {
        update_option!(self.render_update(peh, renderer, render_state))
    }

    fn render_update_force_reposition<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(
        //
        self,
        peh: &mut PEH,
        renderer: &mut Renderer,
        render_state: Pin<&mut Self::RenderState<PEH, Renderer>>,
    ) where
        Self: Sized,
    {
        update_option!(self.render_update_force_reposition(peh, renderer, render_state))
    }

    fn render_update_maybe_reposition<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(
        //
        self,
        peh: &mut PEH,
        renderer: &mut Renderer,
        render_state: Pin<&mut Self::RenderState<PEH, Renderer>>,
        force_reposition: bool,
    ) {
        update_option!(self.render_update_maybe_reposition(
            //
            peh,
            renderer,
            render_state,
            force_reposition
        ))
    }

    type UnpinnedRenderState<PEH: ?Sized, R: RenderHtml + ?Sized> = E::UnpinnedRenderState<PEH, R>;

    fn unpinned_render_update<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(
        //
        self,
        peh: &mut PEH,
        renderer: &mut Renderer,
        render_state: &mut Self::UnpinnedRenderState<PEH, Renderer>,
    ) where
        Self: Sized,
    {
        unpinned_update_option!(self.unpinned_render_update(peh, renderer, render_state))
    }

    fn unpinned_render_update_force_reposition<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(
        //
        self,
        peh: &mut PEH,
        renderer: &mut Renderer,
        render_state: &mut Self::UnpinnedRenderState<PEH, Renderer>,
    ) where
        Self: Sized,
    {
        unpinned_update_option!(self.unpinned_render_update_force_reposition(peh, renderer, render_state))
    }

    fn unpinned_render_update_maybe_reposition<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(
        //
        self,
        peh: &mut PEH,
        renderer: &mut Renderer,
        render_state: &mut Self::UnpinnedRenderState<PEH, Renderer>,
        force_reposition: bool,
    ) {
        unpinned_update_option!(self.unpinned_render_update_maybe_reposition(
            //
            peh,
            renderer,
            render_state,
            force_reposition
        ))
    }
}
