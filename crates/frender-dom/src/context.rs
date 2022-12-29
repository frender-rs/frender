use std::future::IntoFuture;

use frender_core::{RenderState, UpdateRenderState};

#[derive(Debug, Clone)]
pub enum NextNodePosition {
    FirstChildOf(web_sys::Element),
    InsertAfter(web_sys::Node),
}

impl NextNodePosition {
    pub fn set_as_insert_after(&mut self, node: web_sys::Node) {
        *self = Self::InsertAfter(node);
    }

    pub fn add_node(&mut self, node: web_sys::Node) {
        match self {
            NextNodePosition::FirstChildOf(parent) => parent.prepend_with_node_1(&node).unwrap(),
            NextNodePosition::InsertAfter(pre) => {
                pre.parent_node()
                    .unwrap()
                    .insert_before(&node, pre.next_sibling().as_ref())
                    .unwrap();
            }
        }

        *self = NextNodePosition::InsertAfter(node);
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Dom {
    pub document: web_sys::Document,
    pub next_node_position: NextNodePosition,
}

impl Dom {
    pub fn new(document: web_sys::Document, root_parent: web_sys::Element) -> Self {
        Self {
            document,
            next_node_position: NextNodePosition::FirstChildOf(root_parent),
        }
    }

    #[inline]
    pub fn add_element_and_children<'a>(
        &'a mut self,
        element: web_sys::Element,
        add_children: impl FnOnce(&'a mut Self),
    ) {
        let mut old_position = std::mem::replace(
            &mut self.next_node_position,
            NextNodePosition::FirstChildOf(element.clone()),
        );

        add_children(self);

        old_position.add_node(element.into())
    }

    #[inline]
    pub fn with_position<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> R {
        let old_position = self.next_node_position.clone();

        let out = f(self);

        self.next_node_position = old_position;

        out
    }

    #[inline]
    pub fn with_new_position<R>(
        &mut self,
        new_position: NextNodePosition,
        f: impl FnOnce(&mut Self) -> R,
    ) -> R {
        let old_position = std::mem::replace(&mut self.next_node_position, new_position);
        let out = f(self);

        self.next_node_position = old_position;

        out
    }

    pub async fn render_element<E: UpdateRenderState<Dom> + Copy>(
        &mut self,
        element: E,
        stop: impl IntoFuture<Output = ()>,
    ) {
        self.render_get_element(|| element, stop).await
    }

    pub async fn render_get_element<E: UpdateRenderState<Dom>>(
        &mut self,
        mut get_element: impl FnMut() -> E,
        stop: impl IntoFuture<Output = ()>,
    ) {
        let state = E::State::new_uninitialized();
        futures_lite::pin!(state);

        let root_position = self.next_node_position.clone();

        get_element().update_render_state(self, state.as_mut());

        let stop = crate::utils::reentrant(stop.into_future());

        futures_lite::pin!(stop);

        futures_lite::future::or(stop.as_mut(), async {
            while futures_lite::future::zip(
                std::future::poll_fn(|cx| state.as_mut().poll_reactive(cx)),
                gloo::timers::future::TimeoutFuture::new(0),
            )
            .await
            .0
            {
                web_sys::console::log_1(&"update_render_state".into());
                self.next_node_position = root_position.clone();
                get_element().update_render_state(self, state.as_mut());
            }
        })
        .await;

        web_sys::console::log_1(&"stopped or non-dynamic".into());

        stop.await;

        web_sys::console::log_1(&"stopped".into());

        state.unmount();
    }
}
