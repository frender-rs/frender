use std::pin::Pin;

use lazy_pinned::LazyPinned;

use crate::{Element, RenderState};

use super::Preserved;

pin_project_lite::pin_project! {
    pub struct PreservedOptionState<S> {
        #[pin]
        inner: LazyPinned<S>
    }
}

impl<S: RenderState> RenderState for PreservedOptionState<S> {
    fn unmount(self: Pin<&mut Self>) {
        self.project().inner.as_pin_mut().map(S::unmount);
    }

    fn poll_csr(
        self: Pin<&mut Self>,
        ctx: &mut crate::CsrContext,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        self.project()
            .inner
            .as_pin_mut()
            .map_or(std::task::Poll::Ready(()), |s| S::poll_csr(s, ctx, cx))
    }
}

impl<R: Element> Element for Preserved<Option<R>> {
    type CsrState = PreservedOptionState<R::CsrState>;

    fn into_csr_state(self, ctx: &mut crate::CsrContext) -> Self::CsrState {
        PreservedOptionState {
            inner: LazyPinned(self.0.map(|this| R::into_csr_state(this, ctx))),
        }
    }

    fn update_csr_state(self, ctx: &mut crate::CsrContext, state: Pin<&mut Self::CsrState>) {
        if let Some(element) = self.0 {
            state.project().inner.use_pin_or_insert_with_data(
                (element, ctx),
                |(element, ctx), state| {
                    element.update_csr_state(ctx, state);
                },
                |(element, ctx)| element.into_csr_state(ctx),
            );
        } else {
            state.project().inner.as_pin_mut().map(R::CsrState::unmount);
        }
    }
}
