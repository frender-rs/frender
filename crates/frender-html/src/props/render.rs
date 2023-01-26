use std::pin::Pin;

use frender_core::RenderState;

pub trait IntrinsicComponent {
    const INTRINSIC_TAG: &'static str;
}

pub struct FieldData<'a, Data, State, Element> {
    pub data: Data,
    pub state: State,
    pub element: &'a Element,
    pub dom_element: &'a ::web_sys::Element,
    pub children_ctx: &'a mut ::frender_dom::Dom,
}

pub struct ElementAndMounted<E> {
    pub element: E,
    pub mounted: bool,
}

pin_project_lite::pin_project! {
    pub struct IntrinsicComponentRenderState<E, S> {
        pub element_and_mounted: Option<ElementAndMounted<E>>,
        #[pin]
        pub render_state: S,
    }
}

impl<E, S> IntrinsicComponentRenderState<E, S> {
    pub fn pin_project(self: Pin<&mut Self>) -> (&mut Option<ElementAndMounted<E>>, Pin<&mut S>) {
        let this = self.project();
        (this.element_and_mounted, this.render_state)
    }
}

pub trait IntrinsicComponentPollReactive {
    fn intrinsic_component_poll_reactive(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<bool>;
}

impl<E: AsRef<web_sys::Element>, S: Default + IntrinsicComponentPollReactive> RenderState
    for IntrinsicComponentRenderState<E, S>
{
    fn new_uninitialized() -> Self {
        Self {
            element_and_mounted: None,
            render_state: S::default(),
        }
    }

    fn unmount(self: Pin<&mut Self>) {
        match self.project().element_and_mounted {
            Some(ElementAndMounted { element, mounted }) if *mounted => {
                *mounted = false;
                element.as_ref().remove();
            }
            _ => {}
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
