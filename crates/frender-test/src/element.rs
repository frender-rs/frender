pub use dom_token_list::DomTokenList;

use std::{
    borrow::Cow,
    cell::RefCell,
    rc::{Rc, Weak},
};

use indexmap::IndexMap;

use crate::text::Text;

#[derive(Debug, Clone)]
pub enum Node {
    Text(Text),
    Element(Element),
}

impl Node {
    pub(crate) fn parent(&self) -> Option<WeakElement> {
        match self {
            Node::Text(text) => text.parent(),
            Node::Element(element) => element.parent(),
        }
    }

    pub fn as_text(&self) -> Option<&Text> {
        if let Self::Text(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_element(&self) -> Option<&Element> {
        if let Self::Element(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub(crate) fn is_same_node(&self, node: &Node) -> bool {
        match (self, node) {
            (Node::Text(a), Node::Text(b)) => a.is_same_text(b),
            (Node::Element(a), Node::Element(b)) => a.is_same_element(b),
            _ => false,
        }
    }

    pub(crate) fn try_into_text(self) -> Result<Text, Self> {
        if let Self::Text(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub(crate) fn try_into_element(self) -> Result<Element, Self> {
        if let Self::Element(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    fn set_parent(&self, parent: Option<WeakElement>) {
        match self {
            Node::Text(this) => this.set_parent(parent),
            Node::Element(this) => this.set_parent(parent),
        }
    }
}

#[derive(Debug)]
pub(crate) enum Cursor {
    FirstChildOf(Element),
    After {
        //
        node: Node,
        children_index_hint: usize,
    },
}

impl Cursor {
    pub(crate) fn cloned_cursor(&self) -> Self {
        match self {
            Cursor::FirstChildOf(this) => Cursor::FirstChildOf(this.clone()),
            Cursor::After {
                node,
                children_index_hint,
            } => Cursor::After {
                node: node.clone(),
                children_index_hint: *children_index_hint,
            },
        }
    }
    pub(crate) fn current_node(&self) -> Option<Node> {
        match self {
            Cursor::FirstChildOf(parent) => parent.inner.borrow().children.first().cloned(),
            Cursor::After {
                node,
                children_index_hint,
            } => {
                let parent = node
                    .parent()
                    .expect("cursor node should have parent")
                    .upgrade()
                    .expect("cursor node's parent has been dropped");
                let children = &parent.inner.borrow().children;

                let node = children[*children_index_hint..]
                    .iter()
                    .find(|n| n.is_same_node(node))
                    .or_else(|| {
                        children[..*children_index_hint]
                            .iter()
                            .find(|n| n.is_same_node(node))
                    })
                    .expect("node should be in its parent's children");

                Some(node.clone())
            }
        }
    }
}

#[derive(Debug)]
pub(crate) enum AttrValue {
    Absent,
    Empty,
    String(String),
}

#[derive(Debug)]
struct ElementInner {
    parent: Option<WeakElement>,
    tag: Cow<'static, str>,
    attrs: IndexMap<Cow<'static, str>, AttrValue>,
    children: Vec<Node>,
}

impl ElementInner {
    fn data_cloned(&self) -> ElementData {
        ElementData {
            tag: self.tag.clone(),
            attrs: self
                .attrs
                .iter()
                .filter_map(|(name, value)| {
                    let name = name.clone();

                    match value {
                        AttrValue::Absent => None,
                        AttrValue::Empty => Some((name, None)),
                        AttrValue::String(s) => Some((name, Some(s.clone()))),
                    }
                })
                .collect(),
            children: self.children.clone(),
        }
    }
}

pub struct ElementData {
    pub tag: Cow<'static, str>,
    pub attrs: Vec<(Cow<'static, str>, Option<String>)>,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct Element {
    inner: Rc<RefCell<ElementInner>>,
}

impl Element {
    pub fn data_cloned(&self) -> ElementData {
        self.inner.borrow().data_cloned()
    }

    pub(crate) fn new_dummy() -> Self {
        Self {
            inner: Rc::new(RefCell::new(ElementInner {
                parent: None,
                tag: Cow::Borrowed(""),
                attrs: Default::default(),
                children: vec![],
            })),
        }
    }

    pub(crate) fn new_with_tag(tag: impl Into<Cow<'static, str>>) -> Self {
        Self {
            inner: Rc::new(RefCell::new(ElementInner {
                parent: None,
                tag: tag.into(),
                attrs: Default::default(),
                children: vec![],
            })),
        }
    }

    pub(crate) fn is_same_element(&self, other: &Element) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }

    fn parent(&self) -> Option<WeakElement> {
        self.inner.borrow().parent.clone()
    }

    pub(crate) fn children(&self) -> Vec<Node> {
        self.inner.borrow().children.clone()
    }

    pub(crate) fn position_of_child(&self, node: &Node) -> usize {
        self.inner
            .borrow()
            .children
            .iter()
            .position(|n| node.is_same_node(n))
            .expect("node should be a child")
    }

    pub(crate) fn remove_child(&self, child: &Node) -> Node {
        let pos = self.position_of_child(child);
        let node = self.inner.borrow_mut().children.remove(pos);

        debug_assert!(node.is_same_node(child));

        node.set_parent(None);

        node
    }

    fn set_parent(&self, parent: Option<WeakElement>) {
        self.inner.borrow_mut().parent = parent
    }

    pub(crate) fn prepend_child(&self, node: Node) {
        self.insert_child_at(node, 0)
    }

    pub(crate) fn insert_child_after(&self, node: Node, after: &Node) {
        let pos = self.position_of_child(after);
        self.insert_child_at(node, pos + 1)
    }

    fn insert_child_at(&self, node: Node, at: usize) {
        assert!(node.parent().is_none());
        node.set_parent(Some(self.downgrade()));
        self.inner.borrow_mut().children.insert(at, node);
    }

    fn downgrade(&self) -> WeakElement {
        WeakElement {
            inner: Rc::downgrade(&self.inner),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct WeakElement {
    inner: Weak<RefCell<ElementInner>>,
}
impl WeakElement {
    pub(crate) fn upgrade(&self) -> Option<Element> {
        self.inner.upgrade().map(|inner| Element { inner })
    }
}

mod dom {
    use crate::renderer::Renderer;

    use super::{Element, Node};

    use frender_html::dom::behaviors;

    impl behaviors::Node<Renderer> for Element {
        fn log_self(&self, _: &mut Renderer) {
            eprintln!("{:?}", self)
        }

        fn cursor_is_at_self(&self, renderer: &Renderer) -> bool {
            renderer
                .cursor
                .current_node()
                .as_ref()
                .and_then(Node::as_element)
                .map_or(false, |e| e.is_same_element(self))
        }

        fn move_cursor_after_self(&mut self, renderer: &mut Renderer) {
            renderer.move_cursor_after_node(Node::Element(self.clone()))
        }

        fn readd_self(&mut self, renderer: &mut Renderer, force_reposition: bool) {
            renderer.readd_node(
                std::borrow::Cow::Owned(Node::Element(self.clone())),
                force_reposition,
            )
        }

        fn remove_self(&mut self, renderer: &mut Renderer) {
            todo!()
        }
    }

    impl behaviors::Element<Renderer> for Element {
        fn move_cursor_at_the_first_child_of_self(&mut self, renderer: &mut Renderer) {
            renderer.cursor = super::Cursor::FirstChildOf(self.clone());
        }

        fn set_attribute(&mut self, renderer: &mut Renderer, name: &str, value: &str) {
            todo!()
        }

        fn remove_attribute(&mut self, renderer: &mut Renderer, name: &str) {
            todo!()
        }

        fn set_inner_html(&mut self, renderer: &mut Renderer, value: &str) {
            todo!()
        }

        fn as_node_ref(&self) -> &(dyn 'static + frender_html::dom::node_ref::traits::Element) {
            todo!()
        }
    }

    impl behaviors::HtmlElement<Renderer> for Element {
        fn set_inner_text(&mut self, renderer: &mut Renderer, value: &str) {
            todo!()
        }

        fn as_node_ref(&self) -> &(dyn 'static + frender_html::dom::node_ref::traits::HtmlElement) {
            todo!()
        }
    }

    impl behaviors::ElementWithClassList<Renderer> for Element {
        type ClassList<'a> = super::DomTokenList
        where
            Self: 'a,
            Renderer: 'a;

        fn class_list<'a>(&'a mut self, renderer: &'a mut Renderer) -> Self::ClassList<'a> {
            todo!()
        }
    }

    impl behaviors::ElementWithRelList<Renderer> for Element {
        type RelList<'a> = super::DomTokenList
        where
            Self: 'a,
            Renderer: 'a;

        fn rel_list<'a>(&'a mut self, renderer: &'a mut Renderer) -> Self::RelList<'a> {
            todo!()
        }
    }
}

mod dom_token_list {
    use frender_html::DomToken;

    pub struct DomTokenList {}

    impl frender_html::DomTokenList for DomTokenList {
        fn set_value(&mut self, value: &str) {
            todo!()
        }

        fn add_1(&mut self, token: DomToken) {
            todo!()
        }

        fn remove_1(&mut self, token: DomToken) {
            todo!()
        }

        fn replace(&mut self, old_token: DomToken, new_token: DomToken) {
            todo!()
        }
    }
}

mod event_listener {
    use frender_html::dom::{event_types::EventType, HasEventTypeName, OnEvent, RegisterOrUpdate};

    use crate::renderer::Renderer;

    use super::Element;

    #[derive(Debug)]
    pub struct EventListener<F> {
        f: Option<F>,
    }

    impl<F> Unpin for EventListener<F> {}

    impl<F> Default for EventListener<F> {
        fn default() -> Self {
            Self { f: None }
        }
    }

    impl<F> RegisterOrUpdate<Element, Renderer, F> for EventListener<F> {
        fn register_or_update(
            self: std::pin::Pin<&mut Self>,
            node: &mut Element,
            renderer: &mut Renderer,
            f: F,
        ) {
            todo!()
        }
    }

    impl<ET: HasEventTypeName + EventType> OnEvent<Renderer, ET> for Element {
        type EventListener<
            F: frender_html::dom::HandleEvent<
                    <ET as frender_html::dom::event_types::EventType>::Event,
                > + 'static,
        > = EventListener<F>;

        type EventListenerUnpinned<
            F: frender_html::dom::HandleEvent<
                    <ET as frender_html::dom::event_types::EventType>::Event,
                > + 'static,
        > = EventListener<F>;
    }
}

mod form_control {
    use frender_html::form_control::element::FormControlElement;

    use crate::renderer::Renderer;

    use super::Element;

    impl FormControlElement<str, Renderer> for Element {
        fn set_default_value(&mut self, renderer: &mut Renderer, value: &str) {
            todo!()
        }

        fn set_value(&mut self, renderer: &mut Renderer, value: &str) {
            todo!()
        }

        fn remove_value(&mut self, renderer: &mut Renderer) {
            todo!()
        }

        type OnValueChangeEventListener<
            F: frender_html::form_control::value::HandleFormControlValue<str> + 'static,
        > = ();

        fn on_value_change<
            F: frender_html::form_control::value::HandleFormControlValue<str> + 'static,
        >(
            &mut self,
            renderer: &mut Renderer,
            state: &mut Self::OnValueChangeEventListener<F>,
            f: F,
        ) {
            todo!()
        }
    }

    impl FormControlElement<f64, Renderer> for Element {
        fn set_default_value(&mut self, renderer: &mut Renderer, value: &f64) {
            todo!()
        }

        fn set_value(&mut self, renderer: &mut Renderer, value: &f64) {
            todo!()
        }

        fn remove_value(&mut self, renderer: &mut Renderer) {
            todo!()
        }

        type OnValueChangeEventListener<
            F: frender_html::form_control::value::HandleFormControlValue<f64> + 'static,
        > = ();

        fn on_value_change<
            F: frender_html::form_control::value::HandleFormControlValue<f64> + 'static,
        >(
            &mut self,
            renderer: &mut Renderer,
            state: &mut Self::OnValueChangeEventListener<F>,
            f: F,
        ) {
            todo!()
        }
    }

    impl FormControlElement<bool, Renderer> for Element {
        fn set_default_value(&mut self, renderer: &mut Renderer, value: &bool) {
            todo!()
        }

        fn set_value(&mut self, renderer: &mut Renderer, value: &bool) {
            todo!()
        }

        fn remove_value(&mut self, renderer: &mut Renderer) {
            todo!()
        }

        type OnValueChangeEventListener<
            F: frender_html::form_control::value::HandleFormControlValue<bool> + 'static,
        > = ();

        fn on_value_change<
            F: frender_html::form_control::value::HandleFormControlValue<bool> + 'static,
        >(
            &mut self,
            renderer: &mut Renderer,
            state: &mut Self::OnValueChangeEventListener<F>,
            f: F,
        ) {
            todo!()
        }
    }
}
