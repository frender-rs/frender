use std::pin::Pin;

use frender_core::RenderState;

use crate::props::IntrinsicComponentPollReactive;

pub struct ElementAndMounted<E> {
    pub element: E,
    pub mounted: bool,
}

impl<E: wasm_bindgen::JsCast + AsRef<web_sys::Element>> ElementAndMounted<E> {
    pub fn update(&mut self, ctx: &mut crate::Dom, update: impl FnOnce(&mut E, &mut crate::Dom)) {
        crate::utils::dom::update_element(self, ctx, update)
    }
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

    pub fn initialize_with_tag(
        initialize_state: impl FnOnce(&mut E, &mut crate::Dom) -> S,
        ctx: &mut crate::Dom,
        tag: &str,
    ) -> Self
    where
        E: wasm_bindgen::JsCast + AsRef<web_sys::Element>,
    {
        let (element_and_mounted, render_state) =
            crate::utils::dom::initialize_element_with_tag(ctx, tag, initialize_state);

        Self {
            element_and_mounted,
            render_state,
        }
    }
}

impl<E: AsRef<web_sys::Element>, S: IntrinsicComponentPollReactive> RenderState<crate::Dom>
    for IntrinsicComponentRenderState<E, S>
{
    fn unmount(self: Pin<&mut Self>) {
        let this = self.project();
        let ElementAndMounted { element, mounted } = this.element_and_mounted;
        if *mounted {
            *mounted = false;
            element.as_ref().remove();
            this.render_state.intrinsic_component_unmount(); // TODO: unmount_when_parent_unmounted
        }
    }

    #[inline]
    fn poll_reactive(
        self: Pin<&mut Self>,
        ctx: &mut crate::Dom,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        let this = self.project();
        let element = this.element_and_mounted.element.as_ref().clone();

        let node: &web_sys::Node = element.as_ref();
        let node = node.clone();

        ctx.next_node_position = crate::NextNodePosition::FirstChildOf(element);
        let result = S::intrinsic_component_poll_reactive(this.render_state, ctx, cx);

        ctx.next_node_position.set_as_insert_after(node);

        result
    }
}
