use std::pin::Pin;

use crate::RenderState;

impl<PEH: ?Sized, R: ?Sized, S: RenderState<PEH, R>> RenderState<PEH, R> for Option<S> {
    fn unmount(mut self: Pin<&mut Self>, peh: &mut PEH, renderer: &mut R) {
        let this = self.as_mut().as_pin_mut();
        match this {
            Some(state) => {
                S::unmount(state, peh, renderer);
            }
            None => return,
        }
        self.set(None)
    }

    fn state_unmount(mut self: std::pin::Pin<&mut Self>) {
        let _ = self.as_mut().as_pin_mut().map(S::state_unmount);
    }

    fn poll_render(
        //
        self: Pin<&mut Self>,
        peh: &mut PEH,
        renderer: &mut R,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        match self.as_pin_mut() {
            Some(s) => S::poll_render(s, peh, renderer, cx),
            None => std::task::Poll::Ready(()),
        }
    }
}
