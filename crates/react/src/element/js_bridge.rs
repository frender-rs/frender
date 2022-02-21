use wasm_bindgen::{JsCast, JsValue};

#[derive(Debug, Clone)]
pub enum JsComponentType {
    Any(JsValue),
    StaticIntrinsic(&'static str),
}

#[derive(Debug, Clone)]
pub struct JsBridgeElement {
    pub js_component_type: JsComponentType,
    pub js_props_without_children: Option<js_sys::Object>,
    pub children: Option<crate::Children>,
    pub key: Option<crate::Key>,
    // pub to_persist: Option<Rc<dyn Any>>,
}

impl JsBridgeElement {
    pub(crate) fn unsafe_create_element_js(self) -> react_sys::Element {
        let mut props_without_children = self.js_props_without_children;

        if let Some(key) = self.key {
            // TODO: whether need to deep clone the props object?
            let obj = props_without_children.get_or_insert_with(|| js_sys::Object::new());
            let obj: &crate::JsProps = obj.unchecked_ref();
            obj.set_key(Some(key.as_ref()));
        }

        const NULL: &JsValue = &JsValue::NULL;

        let props_without_children = props_without_children.as_ref().map_or(NULL, AsRef::as_ref);

        let el = if let Some(children) = self.children {
            match self.js_component_type {
                JsComponentType::Any(js_component_type) => react_sys::create_element(
                    &js_component_type,
                    props_without_children,
                    &children.unsafe_into_js_array(),
                ),
                JsComponentType::StaticIntrinsic(tag) => react_sys::create_element_intrinsic(
                    tag,
                    props_without_children,
                    &children.unsafe_into_js_array(),
                ),
            }
        } else {
            match self.js_component_type {
                JsComponentType::Any(js_component_type) => react_sys::create_element_no_children(
                    &js_component_type,
                    props_without_children,
                ),
                JsComponentType::StaticIntrinsic(tag) => {
                    react_sys::create_element_intrinsic_no_children(tag, props_without_children)
                }
            }
        };

        el
    }
}

impl Into<crate::Element> for JsBridgeElement {
    #[inline]
    fn into(self) -> crate::Element {
        crate::Element::bridge_js(self)
    }
}

impl crate::IntoElement for JsBridgeElement {
    #[inline]
    fn into_element(self) -> crate::Element {
        self.into()
    }
}

impl crate::Node for JsBridgeElement {
    fn as_react_node_js(&self) -> crate::AnyNode {
        self.clone().as_react_node_js()
    }

    fn as_react_children_js(&self) -> Option<crate::Children> {
        self.clone().into_react_children_js()
    }

    #[inline]
    fn into_react_node_js(self) -> crate::AnyNode {
        crate::AnyNode::Element(self.into())
    }

    #[inline]
    fn into_react_children_js(self) -> Option<crate::Children> {
        Some(crate::Children::from_single(self.into_react_node_js()))
    }
}
