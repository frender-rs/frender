use std::pin::Pin;

use crate::{pin_mut_maybe_uninit::PinMutMaybeUninit, Element, RenderHtml, RenderState};

impl<R, S: RenderState<R>> RenderState<R> for Option<S> {
    fn unmount(mut self: Pin<&mut Self>, renderer: &mut R) {
        let this = self.as_mut().as_pin_mut();
        match this {
            Some(state) => {
                S::unmount(state, renderer);
            }
            None => return,
        }
        self.set(None)
    }

    fn state_unmount(mut self: std::pin::Pin<&mut Self>) {
        let _ = self.as_mut().as_pin_mut().map(S::state_unmount);
    }

    fn poll_render(
        self: Pin<&mut Self>,
        renderer: &mut R,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        match self.as_pin_mut() {
            Some(s) => S::poll_render(s, renderer, cx),
            None => std::task::Poll::Ready(()),
        }
    }
}

macro_rules! update_option {
    ($_self:ident . $method:ident ($ctx:ident, $state:ident $(, $arg:expr)? )) => {
        if let Some(this) = $_self {
            this.$method($ctx, $state $(, $arg)?);
        } else {
            <E::RenderState<Renderer> as RenderState<_>>::unmount($state, $ctx)
        }
    };
}

impl<E: Element> Element for Option<E> {
    type RenderState<R: crate::RenderHtml> = E::RenderState<R>;

    #[cfg(feature = "render_into")]
    fn render_into<'s, Renderer: crate::RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: PinMutMaybeUninit<'s, Self::RenderState<Renderer>>,
    ) -> Pin<&'s mut Self::RenderState<Renderer>> {
        match self {
            Some(this) => Some(this.render_into(renderer)),
            None => render_state.write(None),
        }
    }

    fn render_update<Renderer: crate::RenderHtml>(
        self,
        renderer: &mut Renderer,
        mut render_state: Pin<&mut Self::RenderState<Renderer>>,
    ) where
        Self: Sized,
    {
        update_option!(self.render_update(renderer, render_state))
    }

    fn render_update_force_reposition<Renderer: crate::RenderHtml>(
        self,
        renderer: &mut Renderer,
        mut render_state: Pin<&mut Self::RenderState<Renderer>>,
    ) where
        Self: Sized,
    {
        update_option!(self.render_update_force_reposition(renderer, render_state))
    }

    fn render_update_maybe_reposition<Renderer: crate::RenderHtml>(
        self,
        renderer: &mut Renderer,
        mut render_state: Pin<&mut Self::RenderState<Renderer>>,
        force_reposition: bool,
    ) {
        update_option!(self.render_update_maybe_reposition(
            renderer,
            render_state,
            force_reposition
        ))
    }
}
