use std::{borrow::Cow, cell::RefCell, rc::Rc};

use crate::{
    element::{Node, WeakElement},
    renderer::Renderer,
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
    pub(crate) fn new(text: String) -> Self {
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

    fn cursor_is_at_self(&self, renderer: &Renderer) -> bool {
        renderer
            .cursor
            .current_node()
            .as_ref()
            .and_then(Node::as_text)
            .map_or(false, |e| e.is_same_text(self))
    }

    fn move_cursor_after_self(&mut self, renderer: &mut Renderer) {
        renderer.move_cursor_after_node(Node::Text(self.clone()))
    }

    fn readd_self(&mut self, renderer: &mut Renderer, force_reposition: bool) {
        renderer.readd_node(Cow::Owned(Node::Text(self.clone())), force_reposition)
    }

    fn remove_self(&mut self, _: &mut Renderer) {
        self.parent()
            .expect("text node should have a parent")
            .upgrade()
            .expect("text node's parent should not have been dropped")
            .remove_child(&Node::Text(self.clone()));
    }
}

impl frender_html::html::behaviors::Node<Renderer> for Text {}
