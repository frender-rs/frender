pub use self::dom::UnmountedElement;
pub use dom_token_list::DomTokenList;

use std::{
    borrow::Cow,
    cell::RefCell,
    rc::{Rc, Weak},
};

use indexmap::IndexMap;

use crate::text::Text;

#[derive(Debug, Clone)]
pub struct CursorPlaceholder {
    inner: Rc<RefCell<Option<WeakElement>>>,
}

impl CursorPlaceholder {
    fn is_same_cursor_placeholder(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }

    pub(crate) fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(None)),
        }
    }

    pub(crate) fn parent(&self) -> Option<WeakElement> {
        self.inner.borrow().clone()
    }

    fn set_parent(&self, parent: Option<WeakElement>) {
        *self.inner.borrow_mut() = parent
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    CursorPlaceholder(CursorPlaceholder),
    Text(Text),
    Element(Element),
}

impl Node {
    pub(crate) fn parent(&self) -> Option<WeakElement> {
        match self {
            Node::CursorPlaceholder(cp) => cp.parent(),
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
            (Node::CursorPlaceholder(a), Node::CursorPlaceholder(b)) => {
                a.is_same_cursor_placeholder(b)
            }
            _ => false,
        }
    }

    fn set_parent(&self, parent: Option<WeakElement>) {
        match self {
            Node::Text(this) => this.set_parent(parent),
            Node::Element(this) => this.set_parent(parent),
            Node::CursorPlaceholder(cp) => cp.set_parent(parent),
        }
    }

    /// Returns `true` if the node is [`CursorPlaceholder`].
    ///
    /// [`CursorPlaceholder`]: Node::CursorPlaceholder
    #[must_use]
    pub fn is_cursor_placeholder(&self) -> bool {
        matches!(self, Self::CursorPlaceholder(..))
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Cursor {
    FirstChildOf(Element),
    After {
        //
        node: Node,
        children_index_hint: usize,
    },
}

impl Cursor {
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

                let node_pos = children[*children_index_hint..]
                    .iter()
                    .position(|n| n.is_same_node(node))
                    .map(|pos| pos + children_index_hint)
                    .or_else(|| {
                        children[..*children_index_hint]
                            .iter()
                            .position(|n| n.is_same_node(node))
                    })
                    .expect("node should be in its parent's children");

                let next_node = children.get(node_pos + 1);
                next_node.cloned()
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
struct StyleValue {
    value: Cow<'static, str>,
    important: bool,
}

impl StyleValue {
    fn to_strings(&self) -> [&str; 2] {
        [
            &self.value,
            if self.important { "!important" } else { "" }.into(),
        ]
    }
}

#[derive(Debug, Default)]
struct ElementAttrs {
    style: IndexMap<Cow<'static, str>, StyleValue>,
    all: IndexMap<Cow<'static, str>, AttrValue>,
}

impl ElementAttrs {
    fn style_value(&self) -> String {
        self.style
            .iter()
            .map(|(name, value)| {
                let mut s = name.clone().into_owned();
                for v in value.to_strings() {
                    s.push_str(v)
                }

                s
            })
            .fold(String::new(), |acc, item| {
                let semi = if acc.is_empty() { "" } else { ";" };
                acc + semi + &item
            })
    }

    fn iter_pair(&self) -> impl '_ + Iterator<Item = (Cow<'static, str>, Option<String>)> {
        self.all.iter().filter_map(|(name, value)| {
            let value = match value {
                AttrValue::Absent => return None,
                AttrValue::Empty => None,
                AttrValue::String(s) => Some(s),
            };

            let n;
            let v;

            match &**name {
                "style" => {
                    debug_assert!(value.is_none());
                    n = "style".into();
                    v = Some(self.style_value());
                }
                _ => {
                    n = name.clone();
                    v = value.cloned();
                }
            }

            Some((n, v))
        })
    }

    fn remove_style_property(&mut self, property: &str) {
        self.style.shift_remove(property);
    }

    fn set_style_property(&mut self, property_name: &str, value: &str, important: bool) {
        self.style.insert(
            property_name.to_owned().into(),
            StyleValue {
                value: value.to_owned().into(),
                important,
            },
        );
    }
}

#[derive(Debug)]
struct ElementInner {
    parent: Option<WeakElement>,
    tag: Cow<'static, str>,
    attrs: ElementAttrs,
    children: Vec<Node>,
}

impl ElementInner {
    fn data_cloned(&self) -> ElementData {
        ElementData {
            tag: self.tag.clone(),
            attrs: self.attrs.iter_pair().collect(),
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

    fn new_with_tag(tag: impl Into<Cow<'static, str>>) -> Self {
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

    pub fn children(&self) -> Vec<Node> {
        self.inner.borrow().children.clone()
    }

    pub(crate) fn try_position_of_child(&self, node: &Node) -> Option<usize> {
        self.inner
            .borrow()
            .children
            .iter()
            .position(|n| node.is_same_node(n))
    }

    pub(crate) fn position_of_child(&self, node: &Node) -> usize {
        self.try_position_of_child(node)
            .unwrap_or_else(|| panic!("node should be a child: {:?}", node))
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

    fn remove_style_property(&self, property: &str) {
        self.inner
            .borrow_mut()
            .attrs
            .remove_style_property(property);
    }

    fn set_style_property(&self, property_name: &str, value: &str, important: bool) {
        self.inner
            .borrow_mut()
            .attrs
            .set_style_property(property_name, value, important)
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

mod cursor_placeholder {
    use std::borrow::Cow;

    use frender_html::dom::csr::{
        behaviors::{self, Node as _},
        render::RenderWithContext,
        UiHandle, UnmountedUiHandle,
    };

    use crate::renderer::{RenderContext, Renderer};

    use super::{CursorPlaceholder, Node};

    pub struct UnmountedCursorPlaceholder(CursorPlaceholder);

    impl UnmountedUiHandle<Renderer> for UnmountedCursorPlaceholder {
        type Mounted = CursorPlaceholder;

        fn mount(self, render_context: &mut RenderContext) -> Self::Mounted
        where
            Renderer: RenderWithContext,
        {
            render_context.readd_node(Cow::Owned(Node::CursorPlaceholder(self.0.clone())), true);
            self.0
        }
    }

    impl UiHandle<Renderer> for CursorPlaceholder {
        type Unmounted = UnmountedCursorPlaceholder;

        fn unmount(self, renderer: &mut Renderer) -> Self::Unmounted {
            self.parent()
                .expect("CursorPlaceholder should have a parent")
                .upgrade()
                .expect("CursorPlaceholder's parent should not have been dropped")
                .remove_child(&Node::CursorPlaceholder(self.clone()));
            UnmountedCursorPlaceholder(self)
        }

        fn reposition(&mut self, render_context: &mut RenderContext)
        where
            Renderer: RenderWithContext,
        {
            render_context.readd_node(Cow::Owned(Node::CursorPlaceholder(self.clone())), true);
        }

        fn check_and_move_cursor(&self, render_context: &mut RenderContext)
        where
            Renderer: RenderWithContext,
        {
            render_context.readd_node(Cow::Owned(Node::CursorPlaceholder(self.clone())), false);
        }

        fn assert_cursor_is_at_self(&self, render_context: &RenderContext)
        where
            Renderer: RenderWithContext,
        {
            assert!(render_context.cursor_is_at(
                |node| matches!(node, Node::CursorPlaceholder(cp) if cp.is_same_cursor_placeholder(self))
            ))
        }
    }

    impl behaviors::Node<Renderer> for CursorPlaceholder {
        fn log_self(&self, _: &mut Renderer) {
            eprintln!("{:?}", self)
        }

        fn warn_self_with_message(&self, _: &mut Renderer, message: &str) {
            eprintln!("WARNING: {:?} {}", self, message)
        }

        fn readd_self(&mut self, render_context: &mut RenderContext, force_reposition: bool) {
            render_context.readd_node(
                Cow::Owned(Node::CursorPlaceholder(self.clone())),
                force_reposition,
            );
        }

        fn check_and_move_cursor_after_self(&self, render_context: &mut RenderContext)
        where
            Renderer: RenderWithContext,
        {
            assert!(self.cursor_is_at_self(render_context));
            render_context.readd_node(Cow::Owned(Node::CursorPlaceholder(self.clone())), false)
        }

        fn cursor_is_at_self(&self, render_context: &RenderContext) -> bool
        where
            Renderer: RenderWithContext,
        {
            render_context.cursor_is_at(
                |node| matches!(node, Node::CursorPlaceholder(cp) if cp.is_same_cursor_placeholder(self)),
            )
        }

        fn remove_self(&mut self, _: &mut Renderer) {
            self.parent()
                .expect("CursorPlaceholder should have a parent")
                .upgrade()
                .expect("CursorPlaceholder's parent should not have been dropped")
                .remove_child(&Node::CursorPlaceholder(self.clone()));
        }
    }

    impl behaviors::NodeRenderSelf<Renderer> for CursorPlaceholder {
        fn render_self(
            render_context: &mut <Renderer as RenderWithContext>::RenderContext<'_>,
        ) -> Self {
            let mut node = Self::new();
            node.readd_self(render_context, true);
            node
        }
    }

    impl behaviors::NodeWithRenderContextAfterSelf<Renderer> for CursorPlaceholder {
        fn with_render_context_after_self<Res>(
            &mut self,
            renderer: &mut Renderer,
            f: impl FnOnce(&mut <Renderer as RenderWithContext>::RenderContext<'_>) -> Res,
        ) -> Res {
            renderer.with_render_context_after_node(Node::CursorPlaceholder(self.clone()), f)
        }
    }
}

mod dom {
    use crate::renderer::{RenderContext, Renderer};

    use super::{Element, Node};

    use frender_html::dom::csr::{
        behaviors, render::RenderWithContext, render_from::str::ValueForStr, ProvideMutMounted,
        UiHandle, UnmountedUiHandle,
    };

    pub struct UnmountedElement(Element);

    impl UnmountedElement {
        pub(crate) fn new_with_tag(tag: impl Into<std::borrow::Cow<'static, str>>) -> Self {
            Self(Element::new_with_tag(tag))
        }
    }

    impl UnmountedUiHandle<Renderer> for UnmountedElement {
        type Mounted = Element;

        fn mount(self, render_context: &mut RenderContext) -> Self::Mounted {
            render_context.readd_node(std::borrow::Cow::Owned(Node::Element(self.0.clone())), true);
            self.0
        }
    }

    impl ProvideMutMounted<Renderer> for UnmountedElement {
        fn provide_mut_mounted<Out>(
            &mut self,
            renderer: &mut Renderer,
            f: impl FnOnce(&mut Renderer, &mut Self::Mounted) -> Out,
        ) -> Out {
            f(renderer, &mut self.0)
        }
    }

    impl UiHandle<Renderer> for Element {
        type Unmounted = UnmountedElement;

        fn unmount(self, renderer: &mut Renderer) -> Self::Unmounted {
            todo!()
        }

        fn reposition(&mut self, render_context: &mut RenderContext)
        where
            Renderer: RenderWithContext,
        {
            render_context.readd_node(std::borrow::Cow::Owned(Node::Element(self.clone())), true)
        }

        fn check_and_move_cursor(&self, render_context: &mut RenderContext)
        where
            Renderer: RenderWithContext,
        {
            render_context.readd_node(std::borrow::Cow::Owned(Node::Element(self.clone())), false)
        }

        fn assert_cursor_is_at_self(&self, render_context: &RenderContext)
        where
            Renderer: RenderWithContext,
        {
            assert!(render_context
                .current_node()
                .as_ref()
                .and_then(Node::as_element)
                .map_or(false, |e| e.is_same_element(self)))
        }
    }

    impl behaviors::Node<Renderer> for Element {
        fn log_self(&self, _: &mut Renderer) {
            eprintln!("{:?}", self)
        }

        fn warn_self_with_message(&self, renderer: &mut Renderer, message: &str) {
            eprintln!("WARNING: {:?} {}", self, message)
        }

        fn cursor_is_at_self(&self, render_context: &crate::renderer::RenderContext<'_>) -> bool {
            render_context
                .current_node()
                .as_ref()
                .and_then(Node::as_element)
                .map_or(false, |e| e.is_same_element(self))
        }

        fn readd_self(
            &mut self,
            render_context: &mut crate::renderer::RenderContext<'_>,
            force_reposition: bool,
        ) where
            Renderer: RenderWithContext,
        {
            render_context.readd_node(
                std::borrow::Cow::Owned(Node::Element(self.clone())),
                force_reposition,
            )
        }

        fn check_and_move_cursor_after_self(
            &self,
            render_context: &mut crate::renderer::RenderContext<'_>,
        ) where
            Renderer: RenderWithContext,
        {
            assert!(self.cursor_is_at_self(render_context));
            render_context.readd_node(std::borrow::Cow::Owned(Node::Element(self.clone())), false)
        }

        fn remove_self(&mut self, renderer: &mut Renderer) {
            todo!()
        }
    }

    impl behaviors::SetInnerHtmlFromStr<Renderer> for Element {
        fn set_inner_html_from_str(&mut self, renderer: &mut Renderer, value: impl ValueForStr) {
            todo!()
        }
    }

    impl behaviors::Element<Renderer> for Element {
        fn set_attribute(&mut self, renderer: &mut Renderer, name: &str, value: &str) {
            todo!()
        }

        fn remove_attribute(&mut self, renderer: &mut Renderer, name: &str) {
            todo!()
        }

        fn as_node_ref(&self) -> &(dyn 'static + frender_html::dom::node_ref::traits::Element) {
            todo!()
        }
    }

    impl behaviors::SetInnerTextFromStr<Renderer> for Element {
        fn set_inner_text_from_str(&mut self, renderer: &mut Renderer, value: impl ValueForStr) {
            todo!()
        }
    }

    impl behaviors::HtmlElement<Renderer> for Element {
        fn as_node_ref(&self) -> &(dyn 'static + frender_html::dom::node_ref::traits::HtmlElement) {
            todo!()
        }
    }

    impl behaviors::ElementWithClassList<Renderer> for Element {
        type ClassList<'a>
            = super::DomTokenList
        where
            Self: 'a,
            Renderer: 'a;

        fn class_list<'a>(&'a mut self, renderer: &'a mut Renderer) -> Self::ClassList<'a> {
            todo!()
        }
    }

    impl behaviors::ElementWithRelList<Renderer> for Element {
        type RelList<'a>
            = super::DomTokenList
        where
            Self: 'a,
            Renderer: 'a;

        fn rel_list<'a>(&'a mut self, renderer: &'a mut Renderer) -> Self::RelList<'a> {
            todo!()
        }
    }

    mod style {
        use frender_html::dom::{csr::behaviors, style::csr::CssStyleDeclaration};

        use crate::{element::Element, renderer::Renderer};

        pub struct ElementRenderStyle<'a>(&'a mut Element);

        impl CssStyleDeclaration for ElementRenderStyle<'_> {
            fn remove_property_str(&mut self, property: &str) {
                self.0.remove_style_property(property)
            }

            fn set_property_str_with_value_str_and_priority(
                &mut self,
                property_name: &str,
                value: &str,
                priority: frender_html::dom::style::csr::Priority,
            ) {
                self.0
                    .set_style_property(property_name, value, priority.is_important())
            }
        }

        impl behaviors::ElementWithStyle<Renderer> for Element {
            type Style<'a>
                = ElementRenderStyle<'a>
            where
                Self: 'a,
                Renderer: 'a;

            fn style<'a>(&'a mut self, _: &'a mut Renderer) -> Self::Style<'a> {
                ElementRenderStyle(self)
            }
        }
    }

    impl behaviors::ElementWithChildren<Renderer> for Element {
        fn with_render_context_at_first_child_of_self<R>(
            &mut self,
            renderer: &mut Renderer,
            f: impl FnOnce(&mut crate::renderer::RenderContext<'_>) -> R,
        ) -> R {
            renderer.with_render_context_at_first_child_of_element(self, f)
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
    use frender_common::HandleEvent;
    use frender_html::{
        csr::experimental::RenderInitPinned,
        dom::{
            csr::{OnEvent, PinnedRegisterUpdate, RegisterUpdate},
            event_types::EventType,
            HasEventTypeName,
        },
    };

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

    #[non_exhaustive]
    pub enum RenderInit {}

    impl<F>
        RenderInitPinned<
            //
            (&mut Element, &mut Renderer),
            EventListener<F>,
        > for RenderInit
    {
        type Output = ();
        fn render_init_pinned(
            self,
            renderer: (&mut Element, &mut Renderer),
            state: std::pin::Pin<&mut EventListener<F>>,
        ) -> Self::Output {
            todo!()
        }
    }

    impl<F> PinnedRegisterUpdate<Element, Renderer, F> for EventListener<F> {
        type PinnedRegisterInit = RenderInit;

        fn pinned_register_init(
            node: &mut Element,
            renderer: &mut Renderer,
            f: F,
        ) -> (Self, Self::PinnedRegisterInit)
        where
            Self: Sized,
        {
            todo!()
        }

        fn pinned_update(
            self: std::pin::Pin<&mut Self>,
            node: &mut Element,
            renderer: &mut Renderer,
            f: F,
        ) {
            todo!()
        }
    }

    #[derive(Debug)]
    pub struct EventListenerUnpinned<F> {
        f: F,
    }

    impl<F> RegisterUpdate<Element, Renderer, F> for EventListenerUnpinned<F> {
        fn register(node: &mut Element, renderer: &mut Renderer, f: F) -> Self {
            Self { f }
        }

        fn update(&mut self, node: &mut Element, renderer: &mut Renderer, f: F) {
            self.f = f
        }
    }

    impl<ET: HasEventTypeName + EventType> OnEvent<Renderer, ET> for Element {
        type EventListener<
            F: HandleEvent<<ET as frender_html::dom::event_types::EventType>::Event> + 'static,
        > = EventListener<F>;

        type EventListenerUnpinned<
            F: HandleEvent<<ET as frender_html::dom::event_types::EventType>::Event> + 'static,
        > = EventListenerUnpinned<F>;
    }
}

mod form_control {
    use std::marker::PhantomData;

    use frender_html::{
        dom::csr::RegisterUpdate,
        form_control::{
            csr::{FormControlElement, HandleFormControlValue},
            FormControlValueKind, KindOfChecked, KindOfValue, KindOfValueAsNumber,
        },
    };

    use crate::renderer::Renderer;

    use super::Element;

    impl FormControlElement<KindOfValue, Renderer> for Element {
        fn set_default_value(&mut self, renderer: &mut Renderer, value: &str) {
            todo!()
        }

        fn remove_default_value(&mut self, renderer: &mut Renderer) {
            todo!()
        }

        fn set_value(&mut self, renderer: &mut Renderer, value: &str) {
            todo!()
        }

        fn remove_value(&mut self, renderer: &mut Renderer) {
            todo!()
        }

        type OnValueChangeEventListenerUnpinned<F: HandleFormControlValue<KindOfValue> + 'static> =
            EventListenerUnpinned<KindOfValue>;

        type OnValueChangeElementUnpinned = Self;
        fn on_value_change_element_unpinned(&mut self) -> &mut Self::OnValueChangeElementUnpinned {
            self
        }

        type OnValueChangeFUnpinned<F: HandleFormControlValue<KindOfValue> + 'static> = F;
    }

    impl FormControlElement<KindOfValueAsNumber, Renderer> for Element {
        fn set_default_value(&mut self, renderer: &mut Renderer, value: f64) {
            todo!()
        }

        fn remove_default_value(&mut self, renderer: &mut Renderer) {
            todo!()
        }

        fn set_value(&mut self, renderer: &mut Renderer, value: f64) {
            todo!()
        }

        fn remove_value(&mut self, renderer: &mut Renderer) {
            todo!()
        }

        type OnValueChangeEventListenerUnpinned<
            F: HandleFormControlValue<KindOfValueAsNumber> + 'static,
        > = EventListenerUnpinned<KindOfValueAsNumber>;

        type OnValueChangeElementUnpinned = Self;
        fn on_value_change_element_unpinned(&mut self) -> &mut Self::OnValueChangeElementUnpinned {
            self
        }

        type OnValueChangeFUnpinned<F: HandleFormControlValue<KindOfValueAsNumber> + 'static> = F;
    }

    impl FormControlElement<KindOfChecked, Renderer> for Element {
        fn set_default_value(&mut self, renderer: &mut Renderer, value: bool) {
            todo!()
        }

        fn remove_default_value(&mut self, renderer: &mut Renderer) {
            todo!()
        }

        fn set_value(&mut self, renderer: &mut Renderer, value: bool) {
            todo!()
        }

        fn remove_value(&mut self, renderer: &mut Renderer) {
            todo!()
        }

        type OnValueChangeEventListenerUnpinned<
            F: HandleFormControlValue<KindOfChecked> + 'static,
        > = EventListenerUnpinned<KindOfChecked>;

        type OnValueChangeElementUnpinned = Self;
        fn on_value_change_element_unpinned(&mut self) -> &mut Self::OnValueChangeElementUnpinned {
            self
        }

        type OnValueChangeFUnpinned<F: HandleFormControlValue<KindOfChecked> + 'static> = F;
    }

    enum Never {}
    pub struct EventListenerUnpinned<VK: ?Sized + FormControlValueKind> {
        _todo: Never,
        _phantom: PhantomData<VK>,
    }

    impl<F: HandleFormControlValue<VK> + 'static, VK: ?Sized + FormControlValueKind>
        RegisterUpdate<Element, Renderer, F> for EventListenerUnpinned<VK>
    {
        fn register(node: &mut Element, renderer: &mut Renderer, f: F) -> Self {
            todo!()
        }

        fn update(&mut self, node: &mut Element, renderer: &mut Renderer, f: F) {
            match self._todo {}
        }
    }
}
