use std::pin::Pin;

use frender_core::RenderState;

pub trait IntrinsicComponent {
    const INTRINSIC_TAG: &'static str;
}

pub struct ElementAndMounted<E> {
    pub element: E,
    pub mounted: bool,
}

pin_project_lite::pin_project! {
    pub struct IntrinsicComponentRenderState<E, S> {
        pub element_and_mounted: ElementAndMounted<E>,
        #[pin]
        pub render_state: S,
    }
}

impl<E, S> IntrinsicComponentRenderState<E, S> {
    pub fn pin_project(self: Pin<&mut Self>) -> (&mut ElementAndMounted<E>, Pin<&mut S>) {
        let this = self.project();
        (this.element_and_mounted, this.render_state)
    }

    pub fn initialize_with_tag<D: crate::props::UpdateElement<E, State = S>>(
        data: D,
        ctx: &mut frender_dom::Dom,
        tag: &str,
    ) -> Self
    where
        E: wasm_bindgen::JsCast + AsRef<web_sys::Element>,
    {
        let (element_and_mounted, render_state) =
            crate::utils::dom::initialize_element_with_tag(ctx, tag, |element, children_ctx| {
                <D as crate::props::UpdateElement<E>>::initialize_state(data, element, children_ctx)
            });

        Self {
            element_and_mounted,
            render_state,
        }
    }
}

pub trait IntrinsicComponentPollReactive {
    fn intrinsic_component_poll_reactive(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<bool>;
}

impl<E: AsRef<web_sys::Element>, S: IntrinsicComponentPollReactive> RenderState
    for IntrinsicComponentRenderState<E, S>
{
    fn unmount(self: Pin<&mut Self>) {
        let ElementAndMounted { element, mounted } = self.project().element_and_mounted;
        if *mounted {
            *mounted = false;
            element.as_ref().remove();
        }
    }

    #[inline]
    fn poll_reactive(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<bool> {
        S::intrinsic_component_poll_reactive(self.project().render_state, cx)
    }
}
