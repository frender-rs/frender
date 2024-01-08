use futures_lite::Future;

use frender_html::{Element, RenderHtml, RenderState};

pin_project_lite::pin_project!(
    pub struct RenderElement<R: RenderHtml, E: Element, Stop = std::future::Pending<()>> {
        renderer: R,
        element: Option<E>,
        #[pin]
        state: E::RenderState<(), R>,
        #[pin]
        stop: Stop,
        root_element: (),
    }
);

impl<R: RenderHtml, E: Element> RenderElement<R, E> {
    pub fn new(renderer: R, element: E) -> Self {
        Self::new_with_stop(renderer, element, std::future::pending())
    }
}

impl<R: RenderHtml, E: Element, Stop> RenderElement<R, E, Stop> {
    pub fn new_with_stop(renderer: R, element: E, stop: Stop) -> Self {
        Self {
            renderer,
            element: Some(element),
            state: Default::default(),
            stop,
            root_element: (),
        }
    }
}

impl<R: RenderHtml, E: Element, Stop: Future<Output = ()>> std::future::Future
    for RenderElement<R, E, Stop>
{
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut this = self.project();

        if let Some(element) = this.element.take() {
            element.render_update(this.root_element, this.renderer, this.state.as_mut())
        }

        if let std::task::Poll::Pending =
            this.state
                .as_mut()
                .poll_render(this.root_element, this.renderer, cx)
        {
            return std::task::Poll::Pending;
        }

        if let std::task::Poll::Ready(()) = this.stop.poll(cx) {
            this.state.unmount(this.root_element, this.renderer);
            return std::task::Poll::Ready(());
        }

        std::task::Poll::Pending
    }
}
