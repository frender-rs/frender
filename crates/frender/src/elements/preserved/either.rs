#[cfg(feature = "ssr")]
mod ssr {
    use either::Either;
    use frender_ssr::SsrElement;

    use super::super::Preserved;

    /// In ssr, `Preserved<Either<_, _>>` acts exactly as `Either<_, _>`.
    impl<L: SsrElement, R: SsrElement> SsrElement for Preserved<Either<L, R>> {
        type HtmlChildren = <Either<L, R> as SsrElement>::HtmlChildren;

        fn into_html_children(self) -> Self::HtmlChildren {
            self.0.into_html_children()
        }
    }
}

#[cfg(feature = "csr")]
pin_project_lite::pin_project!(
    #[derive(Debug, Default)]
    pub struct State<L, R> {
        left_is_mounted: Option<bool>,
        #[pin]
        left: L,
        #[pin]
        right: R,
    }
);

#[cfg(feature = "csr")]
mod csr {
    use std::pin::Pin;

    use frender_csr::RenderState;

    use super::State;

    impl<Renderer: ?Sized, L: RenderState<Renderer>, R: RenderState<Renderer>> RenderState<Renderer>
        for State<L, R>
    {
        fn unmount(self: Pin<&mut Self>, renderer: &mut Renderer) {
            let this = self.project();
            match this.left_is_mounted {
                Some(true) => L::unmount(this.left, renderer),
                Some(false) => R::unmount(this.right, renderer),
                None => return,
            };
            *this.left_is_mounted = None;
        }

        fn state_unmount(self: Pin<&mut Self>) {
            let this = self.project();
            match this.left_is_mounted {
                Some(true) => L::state_unmount(this.left),
                Some(false) => R::state_unmount(this.right),
                None => return,
            };
        }

        fn poll_render(
            self: Pin<&mut Self>,
            renderer: &mut Renderer,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<()> {
            let this = self.project();
            match this.left_is_mounted {
                Some(true) => L::poll_render(this.left, renderer, cx),
                Some(false) => R::poll_render(this.right, renderer, cx),
                None => std::task::Poll::Ready(()),
            }
        }
    }
}

#[cfg(feature = "html")]
mod html {
    use either::Either;
    use frender_html::{dom::render::RenderContext as _, Element, RenderState};

    use super::super::Preserved;
    use super::State;

    macro_rules! update {
        ($element:expr, $method:ident, $arg2:expr, $render_state:expr $(, $($arg4:expr $(,)?)?)?) => {{
            let render_state = $render_state.project();
            match *render_state.left_is_mounted {
                Some(true) => match $element {
                    Either::Left(this) => {
                        LE::$method(this, $arg2, render_state.left, $($($arg4)?)?)
                    }
                    Either::Right(this) => {
                        render_state.left.unmount($arg2.renderer_mut());
                        *render_state.left_is_mounted = Some(false);
                        RE::$method(this, $arg2, render_state.right, $($($arg4)?)?)
                    }
                },
                Some(false) => match $element {
                    Either::Left(this) => {
                        render_state.right.unmount($arg2.renderer_mut());
                        *render_state.left_is_mounted = Some(true);
                        LE::$method(this, $arg2, render_state.left, $($($arg4)?)?)
                    }
                    Either::Right(this) => {
                        RE::$method(this, $arg2, render_state.right, $($($arg4)?)?)
                    }
                },
                None => match $element {
                    Either::Left(this) => {
                        *render_state.left_is_mounted = Some(true);
                        LE::$method(this, $arg2, render_state.left, $($($arg4)?)?)
                    }
                    Either::Right(this) => {
                        *render_state.left_is_mounted = Some(false);
                        RE::$method(this, $arg2, render_state.right, $($($arg4)?)?)
                    }
                },
            }
        }};
    }

    macro_rules! update_unpinned {
        ($element:expr, $method:ident, $arg2:expr, $render_state:ident $(, $($arg4:expr $(,)?)?)?) => {
            match $render_state.left_is_mounted {
                Some(true) => match $element {
                    Either::Left(this) => {
                        LE::$method(this, $arg2, &mut $render_state.left, $($($arg4)?)?)
                    }
                    Either::Right(this) => {
                        std::pin::Pin::new(&mut $render_state.left).unmount($arg2.renderer_mut());
                        $render_state.left_is_mounted = Some(false);
                        RE::$method(this, $arg2, &mut $render_state.right, $($($arg4)?)?)
                    }
                },
                Some(false) => match $element {
                    Either::Left(this) => {
                        std::pin::Pin::new(&mut $render_state.right).unmount($arg2.renderer_mut());
                        $render_state.left_is_mounted = Some(true);
                        LE::$method(this, $arg2, &mut $render_state.left, $($($arg4)?)?)
                    }
                    Either::Right(this) => {
                        RE::$method(this, $arg2, &mut $render_state.right, $($($arg4)?)?)
                    }
                },
                None => match $element {
                    Either::Left(this) => {
                        $render_state.left_is_mounted = Some(true);
                        LE::$method(this, $arg2, &mut $render_state.left, $($($arg4)?)?)
                    }
                    Either::Right(this) => {
                        $render_state.left_is_mounted = Some(false);
                        RE::$method(this, $arg2, &mut $render_state.right, $($($arg4)?)?)
                    }
                },
            }
        };
    }

    impl<LE, RE> Element for Preserved<Either<LE, RE>>
    where
        LE: Element,
        RE: Element,
    {
        type RenderState<R: frender_html::RenderHtml + ?Sized> =
            State<LE::RenderState<R>, RE::RenderState<R>>;

        fn render_update<Renderer: frender_html::RenderHtml + ?Sized>(
            //
            self,
            render_context: &mut Renderer::RenderContext<'_>,
            render_state: std::pin::Pin<&mut Self::RenderState<Renderer>>,
        ) where
            Self: Sized,
        {
            update!(self.0, render_update, render_context, render_state,)
        }

        fn render_update_force_reposition<Renderer: frender_html::RenderHtml + ?Sized>(
            //
            self,
            render_context: &mut Renderer::RenderContext<'_>,
            render_state: std::pin::Pin<&mut Self::RenderState<Renderer>>,
        ) where
            Self: Sized,
        {
            update!(
                self.0,
                render_update_force_reposition,
                render_context,
                render_state,
            )
        }

        fn render_update_maybe_reposition<Renderer: frender_html::RenderHtml + ?Sized>(
            //
            self,
            render_context: &mut Renderer::RenderContext<'_>,
            render_state: std::pin::Pin<&mut Self::RenderState<Renderer>>,
            force_reposition: bool,
        ) {
            update!(
                self.0,
                render_update_maybe_reposition,
                render_context,
                render_state,
                force_reposition,
            )
        }

        type UnpinnedRenderState<R: frender_html::RenderHtml + ?Sized> =
            State<LE::UnpinnedRenderState<R>, RE::UnpinnedRenderState<R>>;

        fn unpinned_render_update_maybe_reposition<Renderer: frender_html::RenderHtml + ?Sized>(
            //
            self,
            render_context: &mut Renderer::RenderContext<'_>,
            render_state: &mut Self::UnpinnedRenderState<Renderer>,
            force_reposition: bool,
        ) {
            update_unpinned!(
                self.0,
                unpinned_render_update_maybe_reposition,
                render_context,
                render_state,
                force_reposition,
            )
        }

        fn unpinned_render_update<Renderer: frender_html::RenderHtml + ?Sized>(
            //
            self,
            render_context: &mut Renderer::RenderContext<'_>,
            render_state: &mut Self::UnpinnedRenderState<Renderer>,
        ) where
            Self: Sized,
        {
            update_unpinned!(self.0, unpinned_render_update, render_context, render_state,)
        }

        fn unpinned_render_update_force_reposition<Renderer: frender_html::RenderHtml + ?Sized>(
            //
            self,
            render_context: &mut Renderer::RenderContext<'_>,
            render_state: &mut Self::UnpinnedRenderState<Renderer>,
        ) where
            Self: Sized,
        {
            update_unpinned!(
                self.0,
                unpinned_render_update_force_reposition,
                render_context,
                render_state,
            )
        }
    }
}
