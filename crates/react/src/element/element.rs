use std::rc::Rc;

use wasm_bindgen::JsValue;

use crate::IntoOptionalElement;

mod inner {
    #[derive(Debug, Clone)]
    pub enum Element {
        /// The element created from js and is same to clone.
        JsElement(react_sys::Element),
        /// bridge a js component so that
        /// it can be created in rust
        JsBridge(crate::JsBridgeElement),
        /// Bridge a rust use_render fn
        UseRender(crate::UseRenderElement),
        /// This could be represented as JsBridge but
        /// extracted out to improve performance
        Fragment(crate::FragmentElement),
    }
}

use inner::Element as ElementInner;

///
/// ### Why not [`react_sys::Element`]
///
/// `react_sys::Element`, created from ReactJs,
///  can be cloned freely.
///
/// However, when it is created from frender component,
/// it contains data from rust,
/// which can't be stored in react_sys::Element.
///
/// Due to the same reason, [`Element`]
/// implements [`From<react_sys::Element>`]
/// but not [`Into<react_sys::Element>`]
#[derive(Clone)]
pub struct Element {
    inner: ElementInner,
}

impl std::fmt::Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl Element {
    #[inline]
    pub fn from_js_element(el: react_sys::Element) -> Self {
        Self {
            inner: ElementInner::JsElement(el),
        }
    }

    #[inline]
    pub(crate) fn unsafe_into_js_element(self) -> react_sys::Element {
        match self.inner {
            ElementInner::JsElement(js) => js,
            ElementInner::JsBridge(br) => br.unsafe_create_element_js(),
            ElementInner::UseRender(re) => re.unsafe_create_element_js(),
            ElementInner::Fragment(fe) => fe.unsafe_create_element_js(),
        }
    }

    #[inline]
    pub fn bridge_use_render<E: IntoOptionalElement, F: 'static + Fn() -> E>(
        use_render: F,
        key: Option<crate::Key>,
        debug_component_name: Option<JsValue>,
        debug_props: Option<JsValue>,
    ) -> Self {
        let bridge = crate::UseRenderElement::wrap_use_render(
            use_render,
            key,
            debug_component_name,
            debug_props,
        );

        Self::bridge_use_render_element(bridge)
    }

    #[inline]
    pub fn bridge_use_render_element(el: crate::UseRenderElement) -> Self {
        Self {
            inner: ElementInner::UseRender(el),
        }
    }

    #[inline]
    pub fn bridge_js(js: crate::JsBridgeElement) -> Self {
        Self {
            inner: ElementInner::JsBridge(js),
        }
    }

    #[inline]
    pub fn fragment(fe: crate::FragmentElement) -> Self {
        Self {
            inner: ElementInner::Fragment(fe),
        }
    }

    #[doc(hidden)]
    #[inline]
    pub fn private_from_element(el: Self) -> Self {
        el
    }
}

/// Marks an [`Element`] is created with key.
#[derive(Debug, Clone)]
pub struct KeyedElement(pub Element);

impl crate::Node for Element {
    #[inline]
    fn as_react_node_js(&self) -> crate::AnyNode {
        self.clone().into_react_node_js()
    }

    #[inline]
    fn as_react_children_js(&self) -> Option<crate::Children> {
        self.clone().into_react_children_js()
    }

    #[inline]
    fn into_react_node_js(self) -> crate::AnyNode {
        crate::AnyNode::Element(self)
    }

    #[inline]
    fn into_react_children_js(self) -> Option<crate::Children> {
        Some(crate::Children::from_single(self.into_react_node_js()))
    }
}

impl From<react_sys::Element> for Element {
    #[inline]
    fn from(el: react_sys::Element) -> Self {
        Self::from_js_element(el)
    }
}
