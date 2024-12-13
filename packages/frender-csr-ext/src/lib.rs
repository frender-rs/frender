pub use frender_html::{CsrComponent, CsrElement};
pub use into_render_element_ext::IntoRenderElementExt;
pub use render_element::RenderElement;

mod render_element;

mod into_render_element_ext {
    use std::{future::Future, pin::pin};

    use frender_html::{
        dom::ProvideRenderContext,
        experimental::{
            PinMutRenderInitStates, PinnedRenderStateKind, PinnedRenderStateKindPollRender,
            RenderStates,
        },
        CsrElement as Element, RenderHtml, StateUnmount,
    };

    pub trait IntoRenderElementExt: ProvideRenderContext {
        fn into_render_element<E: Element>(self, element: E) -> crate::RenderElement<Self, E>
        where
            Self: Sized,
            Self::Renderer: RenderHtml,
        {
            crate::RenderElement::new(self, element)
        }

        fn render_element<E: Element>(&mut self, element: E) -> crate::RenderElement<&mut Self, E>
        where
            Self::Renderer: RenderHtml,
        {
            crate::RenderElement::new(self, element)
        }

        /// The caller could then unmount the ui handle or just drop it without unmounting.
        fn into_render_element_until_non_reactive<E: Element>(
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
                let mut non_reactive_state = pin!(
                    <<E::RenderStateKind as PinnedRenderStateKind>::PinnedNonReactiveState<
                        Self::Renderer,
                    >>::default()
                );
                let mut reactive_state = pin!(
                    <<E::RenderStateKind as PinnedRenderStateKind>::PinnedReactiveState>::default()
                );
                let mut ui_handle = self.provide_render_context(|render_context| {
                    element.pinned_render_init(
                        render_context,
                        PinMutRenderInitStates {
                            non_reactive_state: non_reactive_state.as_mut(),
                            reactive_state: reactive_state.as_mut(),
                        },
                    )
                });

                std::future::poll_fn(|cx| {
                    <E::RenderStateKind as PinnedRenderStateKindPollRender>::pinned_poll_render(
                        self.renderer_mut(),
                        RenderStates {
                            ui_handle: &mut ui_handle,
                            non_reactive_state: non_reactive_state.as_mut(),
                            reactive_state: reactive_state.as_mut(),
                        },
                        cx,
                    )
                })
                .await;

                // now the ui is no longer reactive

                // The unmount order is the same as EitherElement accidentally
                // (This is not considered as a feature and might change in the future)
                non_reactive_state.set(Default::default());
                reactive_state.as_mut().state_unmount();
                reactive_state.set(Default::default());

                ui_handle
            }
        }

        fn render_element_until_non_reactive<E: Element>(
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
}
