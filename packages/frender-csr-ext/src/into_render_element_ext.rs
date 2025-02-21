use std::{future::Future, pin::pin};

use frender_csr::{
    experimental::{
        PinnedRenderStateKind, PinnedRenderStateKindPollRender, ProvideRenderContext, RenderHtml,
        RenderInitPinned as _,
    },
    CsrElement, StateUnmount,
};

pub trait IntoRenderElementExt: ProvideRenderContext {
    fn into_render_element<E: CsrElement>(self, element: E) -> crate::RenderElement<Self, E>
    where
        Self: Sized,
        Self::Renderer: RenderHtml,
    {
        crate::RenderElement::new(self, element)
    }

    fn render_element<E: CsrElement>(&mut self, element: E) -> crate::RenderElement<&mut Self, E>
    where
        Self::Renderer: RenderHtml,
    {
        crate::RenderElement::new(self, element)
    }

    /// The caller could then unmount the ui handle or just drop it without unmounting.
    fn into_render_element_until_non_reactive<E: CsrElement>(
        mut self,
        element: E,
    ) -> impl Future<
        Output = <E::RenderStateKind as PinnedRenderStateKind>::PinnedUiHandle<Self::Renderer>,
    >
    where
        Self: Sized,
        Self::Renderer: RenderHtml,
    {
        async move {
            let (state, init) = element.pinned_render_init(self.renderer_mut());
            let mut state = pin!(state);
            let mut ui_handle = self.provide_render_context(|render_context| {
                init.render_init_pinned(render_context, state.as_mut())
            });

            std::future::poll_fn(|cx| {
                <E::RenderStateKind as PinnedRenderStateKindPollRender>::pinned_poll_render(
                    self.renderer_mut(),
                    state.as_mut(),
                    &mut ui_handle,
                    cx,
                )
            })
            .await;

            // now the ui is no longer reactive

            state.state_unmount();

            ui_handle
        }
    }

    fn render_element_until_non_reactive<E: CsrElement>(
        &mut self,
        element: E,
    ) -> impl Future<
        Output = <E::RenderStateKind as PinnedRenderStateKind>::PinnedUiHandle<Self::Renderer>,
    >
    where
        Self: Sized,
        Self::Renderer: RenderHtml,
    {
        self.into_render_element_until_non_reactive(element)
    }
}

impl<R: ?Sized + ProvideRenderContext> IntoRenderElementExt for R {}
