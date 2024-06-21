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

    impl<R: ?Sized, S: RenderState<R>> RenderState<R> for State<S> {
        fn unmount(self: Pin<&mut Self>, renderer: &mut R) {
            S::unmount(self.project().inner, renderer)
        }

        fn state_unmount(self: Pin<&mut Self>) {
            S::state_unmount(self.project().inner)
        }

        fn poll_render(
            self: Pin<&mut Self>,
            renderer: &mut R,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<()> {
            S::poll_render(self.project().inner, renderer, cx)
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
        type RenderState<R: frender_html::RenderHtml + ?Sized> = State<E::RenderState<R>>;

        fn render_update<Renderer: frender_html::RenderHtml + ?Sized>(
            //
            self,
            renderer: &mut Renderer,
            render_state: Pin<&mut Self::RenderState<Renderer>>,
        ) where
            Self: Sized,
        {
            let render_state = render_state.project().inner;
            if let Some(this) = self.0 {
                E::render_update(this, renderer, render_state)
            } else {
                render_state.unmount(renderer)
            }
        }

        fn render_update_force_reposition<Renderer: frender_html::RenderHtml + ?Sized>(
            //
            self,
            renderer: &mut Renderer,
            render_state: Pin<&mut Self::RenderState<Renderer>>,
        ) where
            Self: Sized,
        {
            let render_state = render_state.project().inner;
            if let Some(this) = self.0 {
                E::render_update_force_reposition(this, renderer, render_state)
            } else {
                render_state.unmount(renderer)
            }
        }

        fn render_update_maybe_reposition<Renderer: frender_html::RenderHtml + ?Sized>(
            //
            self,
            renderer: &mut Renderer,
            render_state: Pin<&mut Self::RenderState<Renderer>>,
            force_reposition: bool,
        ) {
            let render_state = render_state.project().inner;
            if let Some(this) = self.0 {
                E::render_update_maybe_reposition(this, renderer, render_state, force_reposition)
            } else {
                render_state.unmount(renderer)
            }
        }

        type UnpinnedRenderState<R: frender_html::RenderHtml + ?Sized> =
            State<E::UnpinnedRenderState<R>>;

        fn unpinned_render_update<Renderer: frender_html::RenderHtml + ?Sized>(
            //
            self,
            renderer: &mut Renderer,
            render_state: &mut Self::UnpinnedRenderState<Renderer>,
        ) where
            Self: Sized,
        {
            let render_state = &mut render_state.inner;
            if let Some(this) = self.0 {
                E::unpinned_render_update(this, renderer, render_state)
            } else {
                Pin::new(render_state).unmount(renderer)
            }
        }

        fn unpinned_render_update_force_reposition<Renderer: frender_html::RenderHtml + ?Sized>(
            //
            self,
            renderer: &mut Renderer,
            render_state: &mut Self::UnpinnedRenderState<Renderer>,
        ) where
            Self: Sized,
        {
            let render_state = &mut render_state.inner;
            if let Some(this) = self.0 {
                E::unpinned_render_update_force_reposition(this, renderer, render_state)
            } else {
                Pin::new(render_state).unmount(renderer)
            }
        }

        fn unpinned_render_update_maybe_reposition<Renderer: frender_html::RenderHtml + ?Sized>(
            //
            self,
            renderer: &mut Renderer,
            render_state: &mut Self::UnpinnedRenderState<Renderer>,
            force_reposition: bool,
        ) {
            let render_state = &mut render_state.inner;
            if let Some(this) = self.0 {
                E::unpinned_render_update_maybe_reposition(
                    this,
                    renderer,
                    render_state,
                    force_reposition,
                )
            } else {
                Pin::new(render_state).unmount(renderer)
            }
        }
    }
}
