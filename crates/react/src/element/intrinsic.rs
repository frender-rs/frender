use wasm_bindgen::JsValue;

use crate::{AnyNode, AsNullableElement};

/// Corresponding to [`ReactHTMLElement`]
///
/// [`ReactHTMLElement`]: https://github.com/DefinitelyTyped/DefinitelyTyped/blob/54d540ab4deb2588c0eff39dadf370cbf0a2dee4/types/react/v16/index.d.ts#L176
pub struct IntrinsicElement {
    tag: String,
    props: Option<js_sys::Object>,
    children_args: Option<js_sys::Array>,
    js_ref: Option<JsValue>,
    key: Option<JsValue>,
}

impl crate::Node for IntrinsicElement {
    #[inline]
    fn as_react_node_js(&self) -> AnyNode {
        self.as_nullable_element()
            .map_or_else(AnyNode::null, crate::Node::into_react_node_js)
    }
}

impl super::AsNullableElement for IntrinsicElement {
    fn as_nullable_element(&self) -> Option<react_sys::Element> {
        let mut props: Option<js_sys::Object> = None;

        if let Some(p) = &self.props {
            let props = props.get_or_insert_with(|| js_sys::Object::new());
            js_sys::Object::assign(props, p);
        }

        if let Some(key) = &self.key {
            let props = props.get_or_insert_with(|| js_sys::Object::new());
            js_sys::Reflect::set(props, &JsValue::from_str("key"), key);
        }

        if let Some(ref_el) = &self.js_ref {
            let props = props.get_or_insert_with(|| js_sys::Object::new());
            js_sys::Reflect::set(props, &JsValue::from_str("ref"), ref_el);
        }

        let null = JsValue::NULL;
        let props = props.as_ref().map(|obj| obj.as_ref()).unwrap_or(&null);

        let el = if let Some(children_args) = &self.children_args {
            react_sys::create_element_intrinsic(&self.tag, props, children_args)
        } else {
            react_sys::create_element_intrinsic_no_children(&self.tag, props)
        };

        Some(el)
    }
}
