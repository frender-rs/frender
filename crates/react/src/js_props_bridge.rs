use std::{borrow::Cow, fmt::Debug};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PropsBridge {
    component_name: Cow<'static, str>,
    render_fn: Box<dyn Fn() -> Option<react_sys::Element>>,
}

impl PropsBridge {
    pub fn new<F: 'static + Fn() -> Option<react_sys::Element>>(
        func: F,
        component_name: Cow<'static, str>,
    ) -> Self {
        Self {
            component_name,
            render_fn: Box::new(func) as Box<dyn Fn() -> Option<react_sys::Element>>,
        }
    }
    pub fn new_anonymous<F: 'static + Fn() -> Option<react_sys::Element>>(func: F) -> Self {
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
    pub fn render(&self) -> Option<react_sys::Element> {
        (&self.render_fn)()
    }
}

#[wasm_bindgen]
extern "C" {
    pub type JsProps;
    pub type NodeFromJs;

    #[wasm_bindgen(structural, method, getter, js_name = "__frenderPropsBridge")]
    pub fn props_bridge(this: &JsProps) -> Option<usize>;

    #[wasm_bindgen(structural, method, setter, js_name = "__frenderPropsBridge")]
    pub fn set_props_bridge(this: &JsProps, v: Option<usize>);

    #[wasm_bindgen(structural, method, getter, js_name = "__frenderDebugComponentName")]
    pub fn debug_component_name(this: &JsProps) -> JsValue;

    #[wasm_bindgen(structural, method, setter, js_name = "__frenderDebugComponentName")]
    pub fn set_debug_component_name(this: &JsProps, v: &JsValue);

    #[wasm_bindgen(structural, method, getter, js_name = "__frenderDebugProps")]
    pub fn debug_props(this: &JsProps) -> JsValue;

    #[wasm_bindgen(structural, method, setter, js_name = "__frenderDebugProps")]
    pub fn set_debug_props(this: &JsProps, v: &JsValue);

    #[wasm_bindgen(structural, method, setter, js_name = "key")]
    fn _set_key(this: &JsProps, v: &JsValue);

    #[wasm_bindgen(structural, method, getter)]
    pub fn children(this: &JsProps) -> Option<NodeFromJs>;
}

impl JsProps {
    pub fn set_key(&self, v: Option<&JsValue>) {
        if let Some(v) = v {
            self._set_key(v);
        }
    }
}

impl crate::Node for NodeFromJs {
    fn as_react_node_js(&self) -> crate::AnyNode {
        crate::AnyNode(AsRef::<JsValue>::as_ref(self).clone())
    }
}

fn impl_bridge_rust_only_props(js_props: crate::JsProps) -> JsValue {
    let children = js_props.children();

    if children.is_some() {
        panic!("rust only component should not accept children from js");
    }

    let bridge = js_props.props_bridge().unwrap();

    let ref_bridge = react_sys::use_ref_usize(bridge);

    let old_bridge = ref_bridge.current();
    if old_bridge != bridge {
        let _valid = unsafe { forgotten::try_free_with_usize(old_bridge) };

        #[cfg(debug_assertions)]
        assert!(_valid, "invalid js props bridge: failed to free");

        ref_bridge.set_current(bridge);
    }

    let render = unsafe {
        forgotten::try_get_with_usize::<Box<dyn Fn() -> Option<react_sys::Element>>>(&bridge)
    };

    let render = render.expect_throw("invalid js props bridge: failed to get");

    let el = render();

    if let Some(el) = el {
        el.into()
    } else {
        JsValue::NULL
    }
}

type ClosureBridgeRustOnlyComponent = Closure<dyn Fn(JsProps) -> JsValue>;

fn closure_to_bridge_rust_only_component() -> ClosureBridgeRustOnlyComponent {
    Closure::wrap(Box::new(impl_bridge_rust_only_props) as Box<dyn Fn(crate::JsProps) -> JsValue>)
}

pub fn bridge_rust_only_component<E: crate::AsNullableElement, F: 'static + Fn() -> E>(
    use_render: F,
    key: Option<&JsValue>,
    debug_component_name: Option<&JsValue>,
    debug_props: Option<&JsValue>,
) -> react_sys::Element {
    thread_local! {
        static ADAPTER_FN: ClosureBridgeRustOnlyComponent = closure_to_bridge_rust_only_component();
    }

    ADAPTER_FN.with(|comp_fn| {
        use wasm_bindgen::JsCast;

        let obj = js_sys::Object::new();
        let props: &crate::JsProps = obj.unchecked_ref();

        props.set_key(key);

        #[cfg(debug_assertions)]
        if let Some(debug_component_name) = debug_component_name {
            props.set_debug_component_name(debug_component_name);
        }

        #[cfg(debug_assertions)]
        if let Some(debug_props) = debug_props {
            props.set_debug_props(debug_props);
        }

        let k = forgotten::forget(Box::new(move || use_render().as_nullable_element())
            as Box<dyn Fn() -> Option<react_sys::Element>>);

        let k = k.into_shared();
        let k = k.as_usize();

        props.set_props_bridge(Some(*k));

        react_sys::create_element_no_children(comp_fn.as_ref(), props.as_ref())
    })
}
