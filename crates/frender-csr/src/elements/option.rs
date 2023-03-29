use std::pin::Pin;

use crate::{Element, RenderState};

impl<S: RenderState + Unpin> RenderState for Option<S> {
    fn unmount(self: Pin<&mut Self>) {
        let this = self.get_mut();
        match this {
            Some(state) => {
                S::unmount(Pin::new(state));
            }
            None => return,
        }
        *this = None;
    }

    fn poll_csr(
        self: Pin<&mut Self>,
        ctx: &mut crate::CsrContext,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        match self.get_mut() {
            Some(s) => S::poll_csr(Pin::new(s), ctx, cx),
            None => std::task::Poll::Ready(()),
        }
    }
}

impl<E: Element> Element for Option<E>
where
    <E as Element>::CsrState: Unpin,
{
    type CsrState = Option<E::CsrState>;

    fn into_csr_state(self, ctx: &mut crate::CsrContext) -> Self::CsrState {
        match self {
            Some(this) => Some(this.into_csr_state(ctx)),
            None => None,
        }
    }

    fn update_csr_state(self, ctx: &mut crate::CsrContext, state: Pin<&mut Self::CsrState>) {
        if let Some(this) = self {
            match state.get_mut() {
                Some(state) => this.update_csr_state(ctx, Pin::new(state)),
                state => *state = Some(this.into_csr_state(ctx)),
            };
        } else {
            <Option<E::CsrState> as RenderState>::unmount(state)
        }
    }
}
