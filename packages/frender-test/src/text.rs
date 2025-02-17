use std::{borrow::Cow, cell::RefCell, rc::Rc};

use frender_html::dom::ui_handle::{UiHandle, UnmountedUiHandle};

use crate::{
    element::{Node, WeakElement},
    renderer::{RenderContext, Renderer},
};

#[derive(Debug)]
struct TextInner {
    parent: Option<WeakElement>,
    text: String,
}

#[derive(Debug, Clone)]
pub struct Text {
    inner: Rc<RefCell<TextInner>>,
}

impl ToString for Text {
    fn to_string(&self) -> String {
        self.inner.borrow().text.clone()
    }
}

impl Text {
    fn new(text: String) -> Self {
        Self {
            inner: Rc::new(RefCell::new(TextInner { parent: None, text })),
        }
    }

    pub(crate) fn update(&mut self, text: String) {
        self.inner.borrow_mut().text = text;
    }

    pub(crate) fn is_same_text(&self, cur: &Text) -> bool {
        Rc::ptr_eq(&self.inner, &cur.inner)
    }

    pub(crate) fn parent(&self) -> Option<WeakElement> {
        self.inner.borrow().parent.clone()
    }

    pub(crate) fn set_parent(&self, parent: Option<WeakElement>) {
        self.inner.borrow_mut().parent = parent
    }
}

impl frender_html::dom::behaviors::Node<Renderer> for Text {
    fn log_self(&self, _: &mut Renderer) {
        eprintln!("{:?}", self)
    }

    fn warn_self_with_message(&self, _: &mut Renderer, message: &str) {
        eprintln!("WARNING: {:?} {}", self, message)
    }

    fn cursor_is_at_self(&self, renderer: &crate::renderer::RenderContext) -> bool {
        renderer.cursor_is_at(|node| matches!(node, Node::Text(t) if t.is_same_text(self)))
    }

    fn readd_self(
        &mut self,
        render_context: &mut crate::renderer::RenderContext<'_>,
        force_reposition: bool,
    ) {
        render_context.readd_node(Cow::Owned(Node::Text(self.clone())), force_reposition)
    }

    fn check_and_move_cursor_after_self(
        &self,
        render_context: &mut crate::renderer::RenderContext<'_>,
    ) {
        assert!(self.cursor_is_at_self(render_context));
        render_context.readd_node(Cow::Owned(Node::Text(self.clone())), false)
    }

    fn remove_self(&mut self, _: &mut Renderer) {
        self.parent()
            .expect("text node should have a parent")
            .upgrade()
            .expect("text node's parent should not have been dropped")
            .remove_child(&Node::Text(self.clone()));
    }
}

pub struct UnmountedText(Text);

impl UnmountedText {
    pub(crate) fn new(text: String) -> Self {
        Self(Text::new(text))
    }
}

impl UnmountedUiHandle<Renderer> for UnmountedText {
    type Mounted = Text;

    fn mount(self, render_context: &mut RenderContext) -> Self::Mounted {
        render_context.readd_node(Cow::Owned(Node::Text(self.0.clone())), true);
        self.0
    }
}

impl UiHandle<Renderer> for Text {
    type Unmounted = UnmountedText;

    fn unmount(self, renderer: &mut Renderer) -> Self::Unmounted {
        self.parent()
            .expect("text node should have a parent")
            .upgrade()
            .expect("text node's parent should not have been dropped")
            .remove_child(&Node::Text(self.clone()));
        UnmountedText(self)
    }

    fn reposition(&mut self, render_context: &mut RenderContext) {
        render_context.readd_node(Cow::Owned(Node::Text(self.clone())), true)
    }

    fn check_and_move_cursor(&self, render_context: &mut RenderContext) {
        render_context.readd_node(Cow::Owned(Node::Text(self.clone())), false)
    }

    fn assert_cursor_is_at_self(&self, render_context: &RenderContext) {
        assert!(render_context
            .cursor_is_at(|node| matches!(node, Node::Text(t) if t.is_same_text(self))))
    }
}

impl frender_html::html::behaviors::Node<Renderer> for Text {}
