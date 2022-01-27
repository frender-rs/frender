use std::{borrow::Cow, fmt::Debug};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PropsBridge {
    component_name: Cow<'static, str>,
    render_fn: Box<dyn FnOnce() -> Option<react_sys::Element>>,
}

impl PropsBridge {
    pub fn new<F: 'static + FnOnce() -> Option<react_sys::Element>>(
        func: F,
        component_name: Cow<'static, str>,
    ) -> Self {
        Self {
            component_name,
            render_fn: Box::new(func) as Box<dyn FnOnce() -> Option<react_sys::Element>>,
        }
    }
    pub fn new_anonymous<F: 'static + FnOnce() -> Option<react_sys::Element>>(func: F) -> Self {
        Self::new(func, Cow::Borrowed("Anonymous Component"))
    }
}

#[wasm_bindgen]
impl PropsBridge {
    #[wasm_bindgen(getter)]
    pub fn component_name(&self) -> String {
        self.component_name.to_string()
    }
}

impl Debug for PropsBridge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PropsBridge")
            .field("component_name", &self.component_name)
            .finish()
    }
}

impl PropsBridge {
    pub fn render(self) -> Option<react_sys::Element> {
        (self.render_fn)()
    }
}

#[wasm_bindgen]
extern "C" {
    pub type JsProps;
    pub type NodeFromJs;

    #[wasm_bindgen(method, getter, js_name = "__frenderPropsBridge")]
    pub fn props_bridge(this: &JsProps) -> Option<PropsBridge>;

    #[wasm_bindgen(method, setter, js_name = "__frenderPropsBridge")]
    pub fn set_props_bridge(this: &JsProps, v: Option<PropsBridge>);

    #[wasm_bindgen(method, setter, js_name = "key")]
    fn _set_key(this: &JsProps, v: JsValue);

    #[wasm_bindgen(method, getter)]
    pub fn children(this: &JsProps) -> Option<NodeFromJs>;
}

impl JsProps {
    pub fn set_key(&self, v: Option<JsValue>) {
        if let Some(v) = v {
            self._set_key(v);
        }
    }
}

impl crate::Node for NodeFromJs {
    fn as_react_node_js(&self) -> JsValue {
        AsRef::<JsValue>::as_ref(self).clone()
    }
}

fn create_element_with_bridge<C: crate::Component>(c: C) -> react_sys::Element {
    // react_sys::
    todo!()
}

fn impl_bridge_rust_only_props(js_props: crate::JsProps) -> JsValue {
    let children = js_props.children();

    if children.is_some() {
        panic!("rust only component should not accept children from js");
    }

    let bridge = js_props.props_bridge();

    let el = bridge.unwrap().render();

    if let Some(el) = el {
        el.into()
    } else {
        JsValue::NULL
    }
}

pub type ClosureBridgeRustOnlyComponent = Closure<dyn Fn(JsProps) -> JsValue>;

pub fn closure_to_bridge_rust_only_component() -> ClosureBridgeRustOnlyComponent {
    Closure::wrap(Box::new(impl_bridge_rust_only_props) as Box<dyn Fn(crate::JsProps) -> JsValue>)
}
