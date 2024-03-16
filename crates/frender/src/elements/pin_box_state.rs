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

    impl<S: RenderState<PEH, R>, PEH: ?Sized, R: ?Sized> RenderState<PEH, R> for State<S> {
        fn unmount(
            self: std::pin::Pin<&mut Self>,
            parent_elements_handle: &mut PEH,
            renderer: &mut R,
        ) {
            self.get_mut()
                .0
                .as_mut()
                .unmount(parent_elements_handle, renderer)
        }

        fn state_unmount(self: std::pin::Pin<&mut Self>) {
            self.get_mut().0.as_mut().state_unmount()
        }

        fn poll_render(
            self: std::pin::Pin<&mut Self>,
            parent_elements_handle: &mut PEH,
            renderer: &mut R,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<()> {
            self.get_mut()
                .0
                .as_mut()
                .poll_render(parent_elements_handle, renderer, cx)
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
        type RenderState<PEH: ?Sized, R: frender_html::RenderHtml + ?Sized> =
            State<E::RenderState<PEH, R>>;

        fn render_update<PEH: ?Sized, Renderer: frender_html::RenderHtml + ?Sized>(
            //
            self,
            parent_elements_handle: &mut PEH,
            renderer: &mut Renderer,
            render_state: Pin<&mut Self::RenderState<PEH, Renderer>>,
        ) where
            Self: Sized,
        {
            self.0.render_update(
                parent_elements_handle,
                renderer,
                render_state.get_mut().0.as_mut(),
            )
        }

        fn render_update_force_reposition<
            PEH: ?Sized,
            Renderer: frender_html::RenderHtml + ?Sized,
        >(
            //
            self,
            parent_elements_handle: &mut PEH,
            renderer: &mut Renderer,
            render_state: Pin<&mut Self::RenderState<PEH, Renderer>>,
        ) where
            Self: Sized,
        {
            self.0.render_update_force_reposition(
                parent_elements_handle,
                renderer,
                render_state.get_mut().0.as_mut(),
            )
        }

        fn render_update_maybe_reposition<
            PEH: ?Sized,
            Renderer: frender_html::RenderHtml + ?Sized,
        >(
            //
            self,
            parent_elements_handle: &mut PEH,
            renderer: &mut Renderer,
            render_state: Pin<&mut Self::RenderState<PEH, Renderer>>,
            force_reposition: bool,
        ) {
            self.0.render_update_maybe_reposition(
                parent_elements_handle,
                renderer,
                render_state.get_mut().0.as_mut(),
                force_reposition,
            )
        }

        type UnpinnedRenderState<PEH: ?Sized, R: frender_html::RenderHtml + ?Sized> =
            E::UnpinnedRenderState<PEH, R>;

        fn unpinned_render_update<PEH: ?Sized, Renderer: frender_html::RenderHtml + ?Sized>(
            //
            self,
            parent_elements_handle: &mut PEH,
            renderer: &mut Renderer,
            render_state: &mut Self::UnpinnedRenderState<PEH, Renderer>,
        ) where
            Self: Sized,
        {
            self.0
                .unpinned_render_update(parent_elements_handle, renderer, render_state)
        }

        fn unpinned_render_update_force_reposition<
            PEH: ?Sized,
            Renderer: frender_html::RenderHtml + ?Sized,
        >(
            //
            self,
            parent_elements_handle: &mut PEH,
            renderer: &mut Renderer,
            render_state: &mut Self::UnpinnedRenderState<PEH, Renderer>,
        ) where
            Self: Sized,
        {
            self.0.unpinned_render_update_force_reposition(
                parent_elements_handle,
                renderer,
                render_state,
            )
        }

        fn unpinned_render_update_maybe_reposition<
            PEH: ?Sized,
            Renderer: frender_html::RenderHtml + ?Sized,
        >(
            //
            self,
            parent_elements_handle: &mut PEH,
            renderer: &mut Renderer,
            render_state: &mut Self::UnpinnedRenderState<PEH, Renderer>,
            force_reposition: bool,
        ) {
            self.0.unpinned_render_update_maybe_reposition(
                parent_elements_handle,
                renderer,
                render_state,
                force_reposition,
            )
        }
    }
}
