#[cfg(feature = "ssr")]
mod ssr {
    use frender_ssr::SsrElement;

    /// In ssr, `Preserved<Option<_>>` acts exactly as `Option<_>`.
    impl<E: SsrElement> SsrElement for super::super::Preserved<Option<E>> {
        type HtmlChildren = <Option<E> as SsrElement>::HtmlChildren;

        fn into_html_children(self) -> Self::HtmlChildren {
            self.0.into_html_children()
        }
    }
}

#[cfg(feature = "csr")]
pin_project_lite::pin_project!(
    #[derive(Debug, Default)]
    pub struct State<S> {
        #[pin]
        inner: S,
    }
);

#[cfg(feature = "csr")]
mod csr {
    use std::pin::Pin;

    use frender_csr::RenderState;

    use super::State;

    impl<PEH: ?Sized, R: ?Sized, S: RenderState<PEH, R>> RenderState<PEH, R> for State<S> {
        fn unmount(self: Pin<&mut Self>, peh: &mut PEH, renderer: &mut R) {
            S::unmount(self.project().inner, peh, renderer)
        }

        fn state_unmount(self: Pin<&mut Self>) {
            S::state_unmount(self.project().inner)
        }

        fn poll_render(
            self: Pin<&mut Self>,
            peh: &mut PEH,
            renderer: &mut R,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<()> {
            S::poll_render(self.project().inner, peh, renderer, cx)
        }
    }
}

#[cfg(feature = "html")]
mod html {
    use std::pin::Pin;

    use frender_html::{Element, RenderState};

    use super::super::Preserved;
    use super::State;

    impl<E: Element> Element for Preserved<Option<E>> {
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
            let render_state = render_state.project().inner;
            if let Some(this) = self.0 {
                E::render_update(this, parent_elements_handle, renderer, render_state)
            } else {
                render_state.unmount(parent_elements_handle, renderer)
            }
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
            let render_state = render_state.project().inner;
            if let Some(this) = self.0 {
                E::render_update_force_reposition(
                    this,
                    parent_elements_handle,
                    renderer,
                    render_state,
                )
            } else {
                render_state.unmount(parent_elements_handle, renderer)
            }
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
            let render_state = render_state.project().inner;
            if let Some(this) = self.0 {
                E::render_update_maybe_reposition(
                    this,
                    parent_elements_handle,
                    renderer,
                    render_state,
                    force_reposition,
                )
            } else {
                render_state.unmount(parent_elements_handle, renderer)
            }
        }

        type UnpinnedRenderState<PEH: ?Sized, R: frender_html::RenderHtml + ?Sized> =
            State<E::UnpinnedRenderState<PEH, R>>;

        fn unpinned_render_update<PEH: ?Sized, Renderer: frender_html::RenderHtml + ?Sized>(
            //
            self,
            parent_elements_handle: &mut PEH,
            renderer: &mut Renderer,
            render_state: &mut Self::UnpinnedRenderState<PEH, Renderer>,
        ) where
            Self: Sized,
        {
            let render_state = &mut render_state.inner;
            if let Some(this) = self.0 {
                E::unpinned_render_update(this, parent_elements_handle, renderer, render_state)
            } else {
                Pin::new(render_state).unmount(parent_elements_handle, renderer)
            }
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
            let render_state = &mut render_state.inner;
            if let Some(this) = self.0 {
                E::unpinned_render_update_force_reposition(
                    this,
                    parent_elements_handle,
                    renderer,
                    render_state,
                )
            } else {
                Pin::new(render_state).unmount(parent_elements_handle, renderer)
            }
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
            let render_state = &mut render_state.inner;
            if let Some(this) = self.0 {
                E::unpinned_render_update_maybe_reposition(
                    this,
                    parent_elements_handle,
                    renderer,
                    render_state,
                    force_reposition,
                )
            } else {
                Pin::new(render_state).unmount(parent_elements_handle, renderer)
            }
        }
    }
}
