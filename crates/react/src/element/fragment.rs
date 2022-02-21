use wasm_bindgen::{JsCast, JsValue};

use crate::IntoElement;

#[derive(Debug, Clone)]
pub struct FragmentElement {
    pub key: Option<crate::Key>,
    pub children: Option<crate::Children>,
}

impl FragmentElement {
    #[inline]
    pub(crate) fn unsafe_create_element_js(self) -> react_sys::Element {
        let mut props = None;

        if let Some(key) = self.key {
            let obj = props.get_or_insert_with(|| js_sys::Object::new());
            let obj: &crate::JsProps = obj.unchecked_ref();
            obj.set_key(Some(key.as_ref()));
        }

        const NULL: &JsValue = &JsValue::NULL;
        let props = props.as_ref().map_or(NULL, AsRef::as_ref);

        let el = if let Some(children) = self.children {
            react_sys::create_element(
                &react_sys::Fragment,
                props,
                &children.unsafe_into_js_array(),
            )
        } else {
            react_sys::create_element_no_children(&react_sys::Fragment, props)
        };

        el
    }
}

impl IntoElement for FragmentElement {
    #[inline]
    fn into_element(self) -> crate::Element {
        crate::Element::fragment(self)
    }
}

impl Into<crate::Element> for FragmentElement {
    #[inline]
    fn into(self) -> crate::Element {
        self.into_element()
    }
}

impl crate::Node for FragmentElement {
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
        crate::AnyNode::Element(self.into_element())
    }

    #[inline]
    fn into_react_children_js(self) -> Option<crate::Children> {
        Some(crate::Children::from_single(self.into_react_node_js()))
    }
}
