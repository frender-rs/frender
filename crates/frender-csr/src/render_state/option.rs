use std::pin::Pin;

use crate::RenderState;

impl<R: ?Sized, S: RenderState<R>> RenderState<R> for Option<S> {
    fn unmount(mut self: Pin<&mut Self>, renderer: &mut R) {
        let this = self.as_mut().as_pin_mut();
        match this {
            Some(state) => {
                S::unmount(state, renderer);
            }
            None => return,
        }
        // TODO: is this needed?
        self.set(None)
    }

    fn state_unmount(self: std::pin::Pin<&mut Self>) {
        let _ = self.as_pin_mut().map(S::state_unmount);
    }

    fn poll_render(
        //
        self: Pin<&mut Self>,
        renderer: &mut R,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        match self.as_pin_mut() {
            Some(s) => S::poll_render(s, renderer, cx),
            None => std::task::Poll::Ready(()),
        }
    }

    fn check_and_move_cursor(&self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: crate::render::RenderWithContext,
    {
        if let Some(this) = self {
            this.check_and_move_cursor(render_context)
        }
    }
}
