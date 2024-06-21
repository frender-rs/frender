#[cfg(feature = "csr")]
pub use csr::State;

pub struct PinBoxState<E>(pub E);

#[cfg(feature = "ssr")]
mod ssr {
    use frender_ssr::SsrElement;

    use super::PinBoxState;

    impl<E: SsrElement> SsrElement for PinBoxState<E> {
        type HtmlChildren = E::HtmlChildren;

        fn into_html_children(self) -> Self::HtmlChildren {
            self.0.into_html_children()
        }
    }
}

#[cfg(feature = "csr")]
mod csr {
    use frender_csr::RenderState;

    #[derive(Debug)]
    pub struct State<S>(pub std::pin::Pin<Box<S>>);

    impl<S: Default> Default for State<S> {
        fn default() -> Self {
            Self(Box::pin(S::default()))
        }
    }

    impl<S: RenderState<R>, R: ?Sized> RenderState<R> for State<S> {
        fn unmount(self: std::pin::Pin<&mut Self>, renderer: &mut R) {
            self.get_mut().0.as_mut().unmount(renderer)
        }

        fn state_unmount(self: std::pin::Pin<&mut Self>) {
            self.get_mut().0.as_mut().state_unmount()
        }

        fn poll_render(
            self: std::pin::Pin<&mut Self>,
            renderer: &mut R,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<()> {
            self.get_mut().0.as_mut().poll_render(renderer, cx)
        }
    }
}

#[cfg(feature = "html")]
mod html {
    use std::pin::Pin;

    use frender_html::Element;

    use super::PinBoxState;
    use super::State;

    impl<E: Element> Element for PinBoxState<E> {
        type RenderState<R: frender_html::RenderHtml + ?Sized> = State<E::RenderState<R>>;

        fn render_update<Renderer: frender_html::RenderHtml + ?Sized>(
            //
            self,
            renderer: &mut Renderer,
            render_state: Pin<&mut Self::RenderState<Renderer>>,
        ) where
            Self: Sized,
        {
            self.0
                .render_update(renderer, render_state.get_mut().0.as_mut())
        }

        fn render_update_force_reposition<Renderer: frender_html::RenderHtml + ?Sized>(
            //
            self,
            renderer: &mut Renderer,
            render_state: Pin<&mut Self::RenderState<Renderer>>,
        ) where
            Self: Sized,
        {
            self.0
                .render_update_force_reposition(renderer, render_state.get_mut().0.as_mut())
        }

        fn render_update_maybe_reposition<Renderer: frender_html::RenderHtml + ?Sized>(
            //
            self,
            renderer: &mut Renderer,
            render_state: Pin<&mut Self::RenderState<Renderer>>,
            force_reposition: bool,
        ) {
            self.0.render_update_maybe_reposition(
                renderer,
                render_state.get_mut().0.as_mut(),
                force_reposition,
            )
        }

        type UnpinnedRenderState<R: frender_html::RenderHtml + ?Sized> = E::UnpinnedRenderState<R>;

        fn unpinned_render_update<Renderer: frender_html::RenderHtml + ?Sized>(
            //
            self,
            renderer: &mut Renderer,
            render_state: &mut Self::UnpinnedRenderState<Renderer>,
        ) where
            Self: Sized,
        {
            self.0.unpinned_render_update(renderer, render_state)
        }

        fn unpinned_render_update_force_reposition<Renderer: frender_html::RenderHtml + ?Sized>(
            //
            self,
            renderer: &mut Renderer,
            render_state: &mut Self::UnpinnedRenderState<Renderer>,
        ) where
            Self: Sized,
        {
            self.0
                .unpinned_render_update_force_reposition(renderer, render_state)
        }

        fn unpinned_render_update_maybe_reposition<Renderer: frender_html::RenderHtml + ?Sized>(
            //
            self,
            renderer: &mut Renderer,
            render_state: &mut Self::UnpinnedRenderState<Renderer>,
            force_reposition: bool,
        ) {
            self.0
                .unpinned_render_update_maybe_reposition(renderer, render_state, force_reposition)
        }
    }
}
