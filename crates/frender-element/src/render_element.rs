use futures_lite::Future;

use frender_html::{RenderHtml, RenderState};

pin_project_lite::pin_project!(
    pub struct RenderElement<R, E, State, Stop = std::future::Pending<()>> {
        renderer: R,
        root: E,
        #[pin]
        state: State,
        #[pin]
        stop: Stop,
    }
);

impl<R, E, State: Default> RenderElement<R, E, State> {
    pub fn new(renderer: R, root: E) -> Self {
        Self {
            renderer,
            root,
            state: Default::default(),
            stop: std::future::pending(),
        }
    }
}

impl<R, E, State: Default, Stop> RenderElement<R, E, State, Stop> {
    pub fn new_with_stop(renderer: R, root: E, stop: Stop) -> Self {
        Self {
            renderer,
            root,
            state: Default::default(),
            stop,
        }
    }
}

impl<R: RenderHtml, RE, State: RenderState<RE, R>, Stop> RenderElement<R, RE, State, Stop> {
    pub fn update_with_element<E: crate::Element<RenderState<RE, R> = State>>(
        self: std::pin::Pin<&mut Self>,
        element: E,
    ) {
        let this = self.project();
        element.render_update(this.root, this.renderer, this.state)
    }
}

impl<R, RE, State: RenderState<RE, R>, Stop: Future<Output = ()>> std::future::Future
    for RenderElement<R, RE, State, Stop>
{
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut this = self.project();

        if let std::task::Poll::Pending =
            this.state
                .as_mut()
                .poll_render(this.root, this.renderer, cx)
        {
            return std::task::Poll::Pending;
        }

        if let std::task::Poll::Ready(()) = this.stop.poll(cx) {
            this.state.unmount(this.root, this.renderer);
            return std::task::Poll::Ready(());
        }

        std::task::Poll::Pending
    }
}
