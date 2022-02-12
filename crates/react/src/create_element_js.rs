use std::borrow::Cow;

use wasm_bindgen::{JsCast, JsValue};

use crate::Children;

pub(crate) fn create_element_with_js_value(
    component_type: &JsValue,
    props_without_children: Option<&js_sys::Object>,
    children: Option<&Children>,
    key: Option<JsValue>,
) -> react_sys::Element {
    let mut props_without_children = props_without_children.map(Cow::Borrowed);

    if let Some(key) = key {
        // TODO: whether need to deep clone the props object?
        let obj = props_without_children.get_or_insert_with(|| Cow::Owned(js_sys::Object::new()));
        let obj: &crate::JsProps = obj.unchecked_ref();
        obj.set_key(Some(key));
    }

    const NULL: &JsValue = &JsValue::NULL;
    let props_without_children = props_without_children
        .as_ref()
        .map_or(NULL, |p| p.as_ref().as_ref());

    if let Some(children) = children {
        react_sys::create_element(
            component_type,
            props_without_children,
            &children.as_js_array(),
        )
    } else {
        react_sys::create_element_no_children(component_type, props_without_children)
    }
}
