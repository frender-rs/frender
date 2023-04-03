use std::{borrow::Cow, future::IntoFuture};

use crate::{Element, RenderState};

#[derive(Debug, Clone)]
pub enum NextNodePosition<'a> {
    FirstChildOf(Cow<'a, web_sys::Element>),
    InsertAfter(Cow<'a, web_sys::Node>),
}

impl NextNodePosition<'_> {
    pub fn as_borrowed(&self) -> NextNodePosition<'_> {
        match self {
            NextNodePosition::FirstChildOf(v) => NextNodePosition::FirstChildOf(Cow::Borrowed(v)),
            NextNodePosition::InsertAfter(v) => NextNodePosition::InsertAfter(Cow::Borrowed(v)),
        }
    }

    pub fn as_js_debug(&self) -> (&'static str, &web_sys::Node) {
        match self {
            NextNodePosition::FirstChildOf(v) => ("FirstChildOf", v),
            NextNodePosition::InsertAfter(v) => ("InsertAfter", v),
        }
    }
}

impl<'a> NextNodePosition<'a> {
    pub fn set_as_insert_after(&mut self, node: Cow<'a, web_sys::Node>) {
        *self = Self::InsertAfter(node);
    }

    pub fn add_node(&mut self, node: Cow<'a, web_sys::Node>) {
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
pub struct CsrContext<'a> {
    pub document: Cow<'a, web_sys::Document>,
    pub next_node_position: NextNodePosition<'a>,
}

impl CsrContext<'_> {
    pub fn new(document: web_sys::Document, root_parent: web_sys::Element) -> Self {
        Self {
            document: Cow::Owned(document),
            next_node_position: NextNodePosition::FirstChildOf(Cow::Owned(root_parent)),
        }
    }

    pub fn as_borrowed(&self) -> CsrContext<'_> {
        CsrContext {
            document: Cow::Borrowed(&self.document),
            next_node_position: self.next_node_position.as_borrowed(),
        }
    }

    pub fn with_next_node_position<'a>(
        &'a self,
        next_node_position: NextNodePosition<'a>,
    ) -> CsrContext<'a> {
        CsrContext {
            document: Cow::Borrowed(&self.document),
            next_node_position,
        }
    }

    pub async fn render_element<E: Element>(
        &mut self,
        element: E,
        stop: impl IntoFuture<Output = ()>,
    ) {
        let state = element.into_csr_state(&mut self.as_borrowed());

        futures_lite::pin!(state);

        let stop = crate::utils::reentrant(stop.into_future());

        futures_lite::pin!(stop);

        futures_lite::future::or(
            stop.as_mut(),
            std::future::poll_fn(|cx| state.as_mut().poll_csr(&mut self.as_borrowed(), cx)),
        )
        .await;

        web_sys::console::log_1(&"stopped or non-dynamic".into());

        stop.await;

        web_sys::console::log_1(&"stopped".into());

        state.unmount();
    }
}
