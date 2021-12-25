use wasm_bindgen::JsValue;

use crate::Element;

/// Corresponding to [`ReactHTMLElement`]
///
/// [`ReactHTMLElement`]: https://github.com/DefinitelyTyped/DefinitelyTyped/blob/54d540ab4deb2588c0eff39dadf370cbf0a2dee4/types/react/v16/index.d.ts#L176
pub struct IntrinsicElement {
    tag: String,
    props: Option<js_sys::Object>,
    children_args: Option<js_sys::Array>,
    ref_el: Option<react_sys::MutableRefObjectOptionalHtmlElement>,
    key: Option<JsValue>,
}

impl crate::Node for IntrinsicElement {
    #[inline]
    fn as_react_node_js(&self) -> JsValue {
        self.as_react_element().into()
    }
}

impl super::Element for IntrinsicElement {
    fn as_react_element(&self) -> react_sys::Element {
        let mut props: Option<js_sys::Object> = None;

        if let Some(p) = &self.props {
            let props = props.get_or_insert_with(|| js_sys::Object::new());
            js_sys::Object::assign(props, p);
        }

        if let Some(key) = &self.key {
            let props = props.get_or_insert_with(|| js_sys::Object::new());
            js_sys::Reflect::set(props, "key", key);
        }

        if let Some(ref_el) = &self.ref_el {
            let props = props.get_or_insert_with(|| js_sys::Object::new());
            js_sys::Reflect::set(props, "ref", ref_el);
        }

        react_sys::create_element_intrinsic(
            &self.tag,
            props.unwrap_or(&JsValue::NULL),
            if let Some(children_args) = self.children_args {
                &children_args
            } else {
                &js_sys::Array::new()
            },
        )
    }
}
