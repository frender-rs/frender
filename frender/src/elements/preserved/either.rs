#[cfg(feature = "csr")]
pub mod csr {
    use std::pin::Pin;

    use either::Either;
    use frender_csr::{Element, RenderState};

    use super::super::Preserved;

    pin_project_lite::pin_project!(
        pub struct State<L, R> {
            left_is_mounted: Option<bool>,
            #[pin]
            left: Option<L>,
            #[pin]
            right: Option<R>,
        }
    );

    impl<L: RenderState, R: RenderState> RenderState for State<L, R> {
        fn unmount(self: Pin<&mut Self>) {
            let this = self.project();
            match this.left_is_mounted {
                Some(true) => this.left.as_pin_mut().map(L::unmount),
                Some(false) => this.right.as_pin_mut().map(R::unmount),
                None => return,
            };
            *this.left_is_mounted = None;
        }

        fn poll_csr(
            self: Pin<&mut Self>,
            ctx: &mut crate::CsrContext,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<()> {
            let this = self.project();
            match this.left_is_mounted {
                Some(true) => this.left.as_pin_mut().unwrap().poll_csr(ctx, cx),
                Some(false) => this.right.as_pin_mut().unwrap().poll_csr(ctx, cx),
                None => std::task::Poll::Ready(()),
            }
        }
    }

    impl<L, R> Element for Preserved<Either<L, R>>
    where
        L: Element,
        R: Element,
    {
        type CsrState = State<L::CsrState, R::CsrState>;

        fn into_csr_state(self, ctx: &mut crate::CsrContext) -> Self::CsrState {
            match self.0 {
                Either::Left(e) => State {
                    left_is_mounted: Some(true),
                    left: Some(e.into_csr_state(ctx)),
                    right: None,
                },
                Either::Right(e) => State {
                    left_is_mounted: Some(true),
                    left: None,
                    right: Some(e.into_csr_state(ctx)),
                },
            }
        }

        fn update_csr_state(self, ctx: &mut crate::CsrContext, state: Pin<&mut Self::CsrState>) {
            let state = state.project();
            match self.0 {
                Either::Left(e) => {
                    if let Some(false) = state.left_is_mounted {
                        state.right.as_pin_mut().unwrap().unmount();
                    }
                    super::super::option::csr::update_or_into_state(e, state.left, ctx);
                    *state.left_is_mounted = Some(true);
                }
                Either::Right(e) => {
                    if let Some(true) = state.left_is_mounted {
                        state.left.as_pin_mut().unwrap().unmount();
                    }

                    super::super::option::csr::update_or_into_state(e, state.right, ctx);

                    *state.left_is_mounted = Some(false);
                }
            }
        }
    }
}

#[cfg(feature = "ssr")]
mod ssr {
    use either::Either;
    use frender_ssr::Element;

    use super::super::Preserved;

    impl<L: Element, R: Element> Element for Preserved<Either<L, R>> {
        type SsrState = <Either<L, R> as Element>::SsrState;

        fn into_ssr_state(self) -> Self::SsrState {
            self.0.into_ssr_state()
        }
    }
}
