use futures_lite::Future;

use frender_html::{RenderHtml, RenderState};

pin_project_lite::pin_project!(
    pub struct RenderElement<R, State, Stop = std::future::Pending<()>> {
        renderer: R,
        #[pin]
        state: State,
        #[pin]
        stop: Stop,
    }
);

impl<R, State: Default> RenderElement<R, State> {
    pub fn new(renderer: R) -> Self {
        Self {
            renderer,
            state: Default::default(),
            stop: std::future::pending(),
        }
    }
}

impl<R, State: Default, Stop> RenderElement<R, State, Stop> {
    pub fn new_with_stop(renderer: R, stop: Stop) -> Self {
        Self {
            renderer,
            state: Default::default(),
            stop,
        }
    }
}

impl<R: RenderHtml, State: RenderState<R>, Stop> RenderElement<R, State, Stop> {
    pub fn update_with_element<E: crate::Element<RenderState<R> = State>>(
        self: std::pin::Pin<&mut Self>,
        element: E,
    ) {
        let this = self.project();
        element.render_update(this.renderer, this.state)
    }
}

impl<R, State: RenderState<R>, Stop: Future<Output = ()>> std::future::Future
    for RenderElement<R, State, Stop>
{
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut this = self.project();

        if let std::task::Poll::Pending = this.state.as_mut().poll_render(this.renderer, cx) {
            return std::task::Poll::Pending;
        }

        if let std::task::Poll::Ready(()) = this.stop.poll(cx) {
            this.state.unmount(this.renderer);
            return std::task::Poll::Ready(());
        }

        std::task::Poll::Pending
    }
}
