use std::{mem::MaybeUninit, pin::Pin, task::Poll};

use frender_html::RenderHtml;

pub trait RenderState<R> {
    fn unmount(self: Pin<&mut Self>, renderer: &mut R);

    fn state_unmount(self: Pin<&mut Self>);

    fn poll_render(
        self: Pin<&mut Self>,
        renderer: &mut R,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()>;
}

pub trait Element: frender_ssr::SsrElement + frender_ssr::IntoAsyncStrIterator {
    type RenderState<R: RenderHtml>: RenderState<R> + Default;

    #[cfg(feature = "render_into")]
    fn render_into<'s, Renderer: RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: PinMutMaybeUninit<'s, Self::RenderState<Renderer>>,
    ) -> Pin<&'s mut Self::RenderState<Renderer>>;

    fn render_update<Renderer: RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: Pin<&mut Self::RenderState<Renderer>>,
    ) where
        Self: Sized,
    {
        self.render_update_maybe_reposition(renderer, render_state, false)
    }

    /// The element needs to be repositioned (re-add to the ctx)
    fn render_update_force_reposition<Renderer: RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: Pin<&mut Self::RenderState<Renderer>>,
    ) where
        Self: Sized,
    {
        self.render_update_maybe_reposition(renderer, render_state, true)
    }

    fn render_update_maybe_reposition<Renderer: RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: Pin<&mut Self::RenderState<Renderer>>,
        force_reposition: bool,
    );
}
