use std::rc::Rc;

use wasm_bindgen::{closure::Closure, JsValue, UnwrapThrowExt};

use crate::IntoElement;

pub type DynUseRenderFn = dyn Fn() -> Option<crate::Element>;

fn impl_bridge_rust_only_component(js_props: crate::JsProps) -> JsValue {
    #[cfg(debug_assertions)]
    let children = js_props.children();
    #[cfg(debug_assertions)]
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

    let render = unsafe { forgotten::try_get_with_usize::<Box<DynUseRenderFn>>(&bridge) };

    let render = render.expect_throw("invalid js props bridge: failed to get");

    let el = render();

    if let Some(el) = el {
        el.unsafe_into_js_element().into()
    } else {
        JsValue::NULL
    }
}

type ClosureBridgeRustOnlyComponent = Closure<dyn Fn(crate::JsProps) -> JsValue>;

fn closure_to_bridge_rust_only_component() -> ClosureBridgeRustOnlyComponent {
    Closure::wrap(
        Box::new(impl_bridge_rust_only_component) as Box<dyn Fn(crate::JsProps) -> JsValue>
    )
}

pub struct UseRenderElement {
    pub use_render: Rc<Box<DynUseRenderFn>>,
    pub key: Option<crate::Key>,
    pub debug_component_name: Option<JsValue>,
    pub debug_props: Option<JsValue>,
}

impl UseRenderElement {
    #[inline]
    pub fn wrap_use_render<E: crate::IntoOptionalElement, F: 'static + Fn() -> E>(
        use_render: F,
        key: Option<crate::Key>,
        debug_component_name: Option<JsValue>,
        debug_props: Option<JsValue>,
    ) -> Self {
        let use_render = move || use_render().into_optional_element();
        Self {
            use_render: Rc::new(Box::new(use_render)),
            key,
            debug_component_name,
            debug_props,
        }
    }
}

impl std::fmt::Debug for UseRenderElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UseRenderElement")
            .field("use_render", &"Rc<RustClosure>")
            .field("key", &self.key)
            .field("debug_component_name", &self.debug_component_name)
            .field("debug_props", &self.debug_props)
            .finish()
    }
}

impl Clone for UseRenderElement {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            use_render: self.use_render.clone(),
            key: self.key.clone(),
            debug_component_name: self.debug_component_name.clone(),
            debug_props: self.debug_props.clone(),
        }
    }
}

impl UseRenderElement {
    /// Note that the created element must not be cloned
    /// or used multiple times as react element.
    ///
    /// `use_render` will be forgotten,
    /// and it will be dropped automatically once changed
    #[inline]
    pub(crate) fn unsafe_create_element_js(self) -> react_sys::Element {
        let Self {
            use_render,
            key,
            debug_component_name,
            debug_props,
        } = self;

        thread_local! {
            static ADAPTER_FN: ClosureBridgeRustOnlyComponent = closure_to_bridge_rust_only_component();
        }

        ADAPTER_FN.with(|comp_fn| {
            use wasm_bindgen::JsCast;

            let obj = js_sys::Object::new();
            let props: &crate::JsProps = obj.unchecked_ref();

            props.set_key(key.map(Into::into).as_ref());

            #[cfg(debug_assertions)]
            if let Some(debug_component_name) = debug_component_name {
                props.set_debug_component_name(&debug_component_name);
            }

            #[cfg(debug_assertions)]
            if let Some(debug_props) = debug_props {
                props.set_debug_props(&debug_props);
            }

            let k = forgotten::forget_rc(use_render);

            let k = k.into_shared();
            let k = k.as_usize();

            props.set_props_bridge(Some(*k));

            react_sys::create_element_no_children(comp_fn.as_ref(), props.as_ref())
        })
    }
}

impl crate::IntoElement for UseRenderElement {
    #[inline]
    fn into_element(self) -> crate::Element {
        crate::Element::bridge_use_render_element(self)
    }
}

impl Into<crate::Element> for UseRenderElement {
    #[inline]
    fn into(self) -> crate::Element {
        self.into_element()
    }
}
