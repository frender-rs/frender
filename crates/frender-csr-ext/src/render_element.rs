use std::future::Future;

use frender_html::{dom::ProvideRenderContext, Element, RenderHtml, RenderState};

pin_project_lite::pin_project!(
    pub struct RenderElement<P: ProvideRenderContext, E: Element, Stop = std::future::Pending<()>>
    where
        P::Renderer: RenderHtml,
    {
        p: P,
        element: Option<E>,
        #[pin]
        state:
            <E::RenderStateKind as frender_html::RenderStateKindPinned>::RenderState<P::Renderer>,
        #[pin]
        stop: Stop,
    }
);

impl<P: ProvideRenderContext, E: Element> RenderElement<P, E>
where
    P::Renderer: RenderHtml,
{
    pub fn new(render_context: P, element: E) -> Self {
        Self::new_with_stop(render_context, element, std::future::pending())
    }
}

impl<P: ProvideRenderContext, E: Element, Stop> RenderElement<P, E, Stop>
where
    P::Renderer: RenderHtml,
{
    pub fn new_with_stop(render_context: P, element: E, stop: Stop) -> Self {
        Self {
            p: render_context,
            element: Some(element),
            state: Default::default(),
            stop,
        }
    }
}

impl<P: ProvideRenderContext, E: Element, Stop: Future<Output = ()>> std::future::Future
    for RenderElement<P, E, Stop>
where
    P::Renderer: RenderHtml,
{
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut this = self.project();

        if let Some(element) = this.element.take() {
            this.p.provide_render_context(|render_context| {
                element.render_update(render_context, this.state.as_mut())
            });
        }

        if let std::task::Poll::Pending = this.state.as_mut().poll_render(this.p.renderer_mut(), cx)
        {
            return std::task::Poll::Pending;
        }

        if let std::task::Poll::Ready(()) = this.stop.poll(cx) {
            this.state.unmount(this.p.renderer_mut());
            return std::task::Poll::Ready(());
        }

        std::task::Poll::Pending
    }
}
