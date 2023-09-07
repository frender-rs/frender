use std::borrow::Cow;
use std::pin::Pin;

use crate::RenderState;

#[derive(Default)]
pub struct ElementAndMounted<E> {
    pub element: E,
    pub mounted: bool,
}

impl<E: wasm_bindgen::JsCast + AsRef<web_sys::Element>> ElementAndMounted<E> {
    pub fn update_maybe_reposition(
        &mut self,
        ctx: &mut crate::CsrContext,
        update: impl FnOnce(&E, &mut crate::CsrContext),
        force_reposition: bool,
    ) {
        use crate::NextNodePosition;

        let force_reposition = force_reposition;
        let ElementAndMounted { element, mounted } = self;
        let node: &web_sys::Element = element.as_ref();

        {
            let mut ctx = ctx.as_borrowed();
            ctx.next_node_position = NextNodePosition::FirstChildOf(Cow::Borrowed(node));
            update(element, &mut ctx)
        };

        let node: web_sys::Node = node.clone().into();
        if *mounted && !force_reposition {
            debug_assert!(node.parent_node().is_some());
            ctx.next_node_position.set_as_insert_after(Cow::Owned(node));
        } else {
            if *mounted
                && match &ctx.next_node_position {
                    NextNodePosition::FirstChildOf(parent) => parent.first_child(),
                    NextNodePosition::InsertAfter(previous) => previous.next_sibling(),
                }
                .map_or(false, |c| node == c)
            {
                ctx.next_node_position.set_as_insert_after(Cow::Owned(node));
                return;
            }

            debug_assert!(force_reposition || node.parent_node().is_none());
            ctx.next_node_position.add_node(Cow::Owned(node));
            *mounted = true;
        }
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
        initialize_state: impl FnOnce(&E, &mut crate::CsrContext) -> S,
        ctx: &mut crate::CsrContext,
        tag: &str,
    ) -> Self
    where
        E: wasm_bindgen::JsCast + AsRef<web_sys::Element>,
    {
        use wasm_bindgen::JsCast;

        use crate::NextNodePosition;

        let element = ctx
            .document
            .create_element(tag)
            .unwrap()
            .dyn_into::<E>()
            .unwrap();
        let node: &web_sys::Element = element.as_ref();

        let ret = {
            let mut ctx = ctx.as_borrowed();
            ctx.next_node_position = NextNodePosition::FirstChildOf(Cow::Borrowed(node));
            initialize_state(&element, &mut ctx)
        };
        ctx.next_node_position
            .add_node(Cow::Owned(node.clone().into()));

        Self {
            element_and_mounted: ElementAndMounted {
                element,
                mounted: true,
            },
            render_state: ret,
        }
    }
}

impl<E: AsRef<web_sys::Element>, S: RenderState> RenderState
    for IntrinsicComponentRenderState<E, S>
{
    fn unmount(self: Pin<&mut Self>) {
        let this = self.project();
        let ElementAndMounted { element, mounted } = this.element_and_mounted;
        if *mounted {
            *mounted = false;
            element.as_ref().remove();
            this.render_state.state_unmount();
        }
    }

    #[inline]
    fn state_unmount(self: std::pin::Pin<&mut Self>) {
        let this = self.project();
        if this.element_and_mounted.mounted {
            this.render_state.state_unmount();
        }
    }

    #[inline]
    fn poll_csr(
        self: Pin<&mut Self>,
        ctx: &mut crate::CsrContext,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        let this = self.project();
        let element = this.element_and_mounted.element.as_ref();

        let result;
        {
            let mut ctx = ctx.with_next_node_position(crate::NextNodePosition::FirstChildOf(
                Cow::Borrowed(element),
            ));
            result = S::poll_csr(this.render_state, &mut ctx, cx);
        }

        let node: &web_sys::Node = element.as_ref();
        let node = node.clone();

        ctx.next_node_position.set_as_insert_after(Cow::Owned(node));

        result
    }
}
