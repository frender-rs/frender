use std::borrow::Cow;

pub use frender_events::web::{Event, JsCastEventType};

pub use frender_html_common::web::DomTokenList;

pub use self::cursor_place_holder::CursorPlaceholder;

use wasm_bindgen::UnwrapThrowExt as _;

use crate::render::RenderWithContext;

pub mod event_listener;

mod cursor_place_holder;

#[derive(Debug)]
pub struct Node<N>(pub N);

pub trait Renderer: for<'a> RenderWithContext<RenderContext<'a> = RenderContext<'a, Self>> {
    fn document(&self) -> Cow<web_sys::Document>;

    fn cursor_is_at_node(render_context: &Self::RenderContext<'_>, node: &web_sys::Node) -> bool
    where
        Self: RenderWithContext;

    /// See [`crate::behaviors::Node::readd_self`].
    fn readd_node(
        render_context: &mut Self::RenderContext<'_>,
        node: &web_sys::Node,
        force_reposition: bool,
    ) where
        Self: RenderWithContext;

    fn remove_node(&mut self, node: &web_sys::Node);

    fn with_render_context_at_first_child_of_element<R>(
        &mut self,
        parent: &web_sys::Element,
        f: impl FnOnce(&mut RenderContext<'_, Self>) -> R,
    ) -> R {
        let mut cursor = Cursor {
            position: CursorPosition::FirstChildOf(Cow::Borrowed(parent)),
            skipped: false,
        };
        f(&mut RenderContext {
            renderer: self,
            cursor: &mut cursor,
        })
    }
}

enum CursorPosition<'a> {
    After(Cow<'a, web_sys::Node>),
    FirstChildOf(Cow<'a, web_sys::Element>),
}

pub struct Cursor<'a> {
    position: CursorPosition<'a>,
    skipped: bool,
}

impl<'a> Cursor<'a> {
    pub fn first_child_of(el: Cow<'a, web_sys::Element>) -> Self {
        Self {
            position: CursorPosition::FirstChildOf(el),
            skipped: false,
        }
    }

    fn after(node: Cow<'a, web_sys::Node>) -> Self {
        Self {
            position: CursorPosition::After(node),
            skipped: false,
        }
    }
}

pub struct RenderContext<'a, R: ?Sized> {
    pub renderer: &'a mut R,
    pub cursor: &'a mut Cursor<'a>,
}

impl<'a> Cursor<'a> {
    pub fn cursor_is_at_node(&self, node: &web_sys::Node) -> bool {
        match &self.position {
            CursorPosition::FirstChildOf(parent) => parent.first_child(),
            CursorPosition::After(previous) => previous.next_sibling(),
        }
        .map_or(false, |c| *node == c)
    }

    pub fn readd_node(&mut self, node: &web_sys::Node, force_reposition: bool) {
        if force_reposition {
            match &self.position {
                CursorPosition::FirstChildOf(parent) => {
                    // web_sys::console::log_2(&"FirstChildOf".into(), parent);

                    parent.prepend_with_node_1(node).unwrap_throw()
                }
                CursorPosition::After(pre) => {
                    // web_sys::console::log_2(&"InsertAfter".into(), pre);

                    pre.parent_node()
                        .expect_throw("the previous node should have a parent node")
                        .insert_before(node, pre.next_sibling().as_ref())
                        .unwrap_throw();
                }
            }
        } else {
            // TODO: check position
        }

        self.position = CursorPosition::After(Cow::Owned(node.clone()));
        self.skipped = false;
    }
}

impl<'a, R: ?Sized + RenderWithContext> crate::render::RenderContext for RenderContext<'a, R> {
    type Renderer = R;

    fn renderer_mut(&mut self) -> &mut Self::Renderer {
        self.renderer
    }

    fn log_cursor(&mut self) {
        let (kind, node, cur) = match &self.cursor.position {
            CursorPosition::FirstChildOf(node) => (
                "FirstChildOf",
                AsRef::<web_sys::Node>::as_ref(node.as_ref()),
                node.first_child(),
            ),
            CursorPosition::After(node) => ("After", node.as_ref(), node.next_sibling()),
        };

        web_sys::console::log_5(
            &"cursor=".into(),
            &kind.into(),
            node,
            &"=".into(),
            &cur.into(),
        );
    }

    fn mark_cursor_skipped(&mut self) {
        self.cursor.skipped = true;
    }
}

impl<
        N: AsRef<ET::JsEventTarget> + AsRef<web_sys::EventTarget>,
        Renderer: ?Sized,
        ET: JsCastEventType + 'static,
    > crate::OnEvent<Renderer, ET> for Node<N>
{
    type EventListener<F: frender_common::HandleEvent<ET::Event> + 'static> =
        event_listener::MaybeEventListenerOfType<F, ET>;

    type EventListenerUnpinned<F: frender_common::HandleEvent<ET::Event> + 'static> =
        event_listener::unpinned::MaybeEventListenerOfType<F, ET>;
}
