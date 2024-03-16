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

    impl<
            PEH: ?Sized,
            Renderer: ?Sized,
            L: RenderState<PEH, Renderer>,
            R: RenderState<PEH, Renderer>,
        > RenderState<PEH, Renderer> for State<L, R>
    {
        fn unmount(self: Pin<&mut Self>, peh: &mut PEH, renderer: &mut Renderer) {
            let this = self.project();
            match this.left_is_mounted {
                Some(true) => L::unmount(this.left, peh, renderer),
                Some(false) => R::unmount(this.right, peh, renderer),
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
            peh: &mut PEH,
            renderer: &mut Renderer,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<()> {
            let this = self.project();
            match this.left_is_mounted {
                Some(true) => L::poll_render(this.left, peh, renderer, cx),
                Some(false) => R::poll_render(this.right, peh, renderer, cx),
                None => std::task::Poll::Ready(()),
            }
        }
    }
}

#[cfg(feature = "html")]
mod html {
    use either::Either;
    use frender_html::{Element, RenderState};

    use super::super::Preserved;
    use super::State;

    macro_rules! update {
        ($element:expr, $method:ident, $arg1:expr, $arg2:expr, $render_state:expr $(, $($arg4:expr $(,)?)?)?) => {{
            let render_state = $render_state.project();
            match *render_state.left_is_mounted {
                Some(true) => match $element {
                    Either::Left(this) => {
                        LE::$method(this, $arg1, $arg2, render_state.left, $($($arg4)?)?)
                    }
                    Either::Right(this) => {
                        render_state.left.unmount($arg1, $arg2);
                        *render_state.left_is_mounted = Some(false);
                        RE::$method(this, $arg1, $arg2, render_state.right, $($($arg4)?)?)
                    }
                },
                Some(false) => match $element {
                    Either::Left(this) => {
                        render_state.right.unmount($arg1, $arg2);
                        *render_state.left_is_mounted = Some(true);
                        LE::$method(this, $arg1, $arg2, render_state.left, $($($arg4)?)?)
                    }
                    Either::Right(this) => {
                        RE::$method(this, $arg1, $arg2, render_state.right, $($($arg4)?)?)
                    }
                },
                None => match $element {
                    Either::Left(this) => {
                        *render_state.left_is_mounted = Some(true);
                        LE::$method(this, $arg1, $arg2, render_state.left, $($($arg4)?)?)
                    }
                    Either::Right(this) => {
                        *render_state.left_is_mounted = Some(false);
                        RE::$method(this, $arg1, $arg2, render_state.right, $($($arg4)?)?)
                    }
                },
            }
        }};
    }

    macro_rules! update_unpinned {
        ($element:expr, $method:ident, $arg1:expr, $arg2:expr, $render_state:ident $(, $($arg4:expr $(,)?)?)?) => {
            match $render_state.left_is_mounted {
                Some(true) => match $element {
                    Either::Left(this) => {
                        LE::$method(this, $arg1, $arg2, &mut $render_state.left, $($($arg4)?)?)
                    }
                    Either::Right(this) => {
                        std::pin::Pin::new(&mut $render_state.left).unmount($arg1, $arg2);
                        $render_state.left_is_mounted = Some(false);
                        RE::$method(this, $arg1, $arg2, &mut $render_state.right, $($($arg4)?)?)
                    }
                },
                Some(false) => match $element {
                    Either::Left(this) => {
                        std::pin::Pin::new(&mut $render_state.right).unmount($arg1, $arg2);
                        $render_state.left_is_mounted = Some(true);
                        LE::$method(this, $arg1, $arg2, &mut $render_state.left, $($($arg4)?)?)
                    }
                    Either::Right(this) => {
                        RE::$method(this, $arg1, $arg2, &mut $render_state.right, $($($arg4)?)?)
                    }
                },
                None => match $element {
                    Either::Left(this) => {
                        $render_state.left_is_mounted = Some(true);
                        LE::$method(this, $arg1, $arg2, &mut $render_state.left, $($($arg4)?)?)
                    }
                    Either::Right(this) => {
                        $render_state.left_is_mounted = Some(false);
                        RE::$method(this, $arg1, $arg2, &mut $render_state.right, $($($arg4)?)?)
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
        type RenderState<PEH: ?Sized, R: frender_html::RenderHtml + ?Sized> =
            State<LE::RenderState<PEH, R>, RE::RenderState<PEH, R>>;

        fn render_update<PEH: ?Sized, Renderer: frender_html::RenderHtml + ?Sized>(
            //
            self,
            parent_elements_handle: &mut PEH,
            renderer: &mut Renderer,
            render_state: std::pin::Pin<&mut Self::RenderState<PEH, Renderer>>,
        ) where
            Self: Sized,
        {
            update!(
                self.0,
                render_update,
                parent_elements_handle,
                renderer,
                render_state,
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
            render_state: std::pin::Pin<&mut Self::RenderState<PEH, Renderer>>,
        ) where
            Self: Sized,
        {
            update!(
                self.0,
                render_update_force_reposition,
                parent_elements_handle,
                renderer,
                render_state,
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
            render_state: std::pin::Pin<&mut Self::RenderState<PEH, Renderer>>,
            force_reposition: bool,
        ) {
            update!(
                self.0,
                render_update_maybe_reposition,
                parent_elements_handle,
                renderer,
                render_state,
                force_reposition,
            )
        }

        type UnpinnedRenderState<PEH: ?Sized, R: frender_html::RenderHtml + ?Sized> =
            State<LE::UnpinnedRenderState<PEH, R>, RE::UnpinnedRenderState<PEH, R>>;

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
            update_unpinned!(
                self.0,
                unpinned_render_update_maybe_reposition,
                parent_elements_handle,
                renderer,
                render_state,
                force_reposition,
            )
        }

        fn unpinned_render_update<PEH: ?Sized, Renderer: frender_html::RenderHtml + ?Sized>(
            //
            self,
            parent_elements_handle: &mut PEH,
            renderer: &mut Renderer,
            render_state: &mut Self::UnpinnedRenderState<PEH, Renderer>,
        ) where
            Self: Sized,
        {
            update_unpinned!(
                self.0,
                unpinned_render_update,
                parent_elements_handle,
                renderer,
                render_state,
            )
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
            update_unpinned!(
                self.0,
                unpinned_render_update_force_reposition,
                parent_elements_handle,
                renderer,
                render_state,
            )
        }
    }
}
