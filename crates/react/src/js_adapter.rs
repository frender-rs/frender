use crate::WriteRef;
use std::{any::Any, rc::Rc};
use wasm_bindgen::JsValue;

#[derive(Debug, Clone)]
pub struct JsAdapterComponentProps {
    pub js_component: JsValue,
    pub js_props: Option<js_sys::Object>,
    pub js_children: Option<crate::Children>,
    pub to_persist: Option<Rc<dyn Any>>,
}

pub trait IntoJsAdapterComponentProps {
    fn into_js_adapter_props(self) -> JsAdapterComponentProps;
}

pub struct JsAdapterComponent(JsAdapterComponentProps);

impl crate::Props for JsAdapterComponentProps {
    type InitialBuilder = ();

    fn init_builder() -> Self::InitialBuilder {}
}

impl crate::Component for JsAdapterComponent {
    type Props = JsAdapterComponentProps;
    type ElementType = react_sys::Element;

    fn use_render(&self) -> Self::ElementType {
        let props = &self.0;

        let a = crate::use_ref_with::<dyn Any, _>(|| (Rc::new(()) as Rc<dyn Any>));
        if let Some(to_persist) = &props.to_persist {
            a.set_current(Rc::clone(to_persist));
        }

        let null = JsValue::NULL;
        let js_props = props
            .js_props
            .as_ref()
            .map(|obj| obj.as_ref())
            .unwrap_or(&null);

        let el = if let Some(children_args) = &props.js_children {
            match children_args {
                crate::Children::Single(v) => react_sys::create_element(
                    &props.js_component,
                    js_props,
                    &js_sys::Array::of1(v.as_ref()),
                ),
                crate::Children::StaticMultiple(children_args) => {
                    react_sys::create_element(&props.js_component, js_props, children_args)
                }
            }
        } else {
            react_sys::create_element_no_children(&props.js_component, js_props)
        };

        el
    }

    fn new_with_props(props: Self::Props) -> Self
    where
        Self: Sized,
    {
        Self(props)
    }

    fn call_create_element(self, key: Option<JsValue>) -> react_sys::Element {
        thread_local! {
            static ADAPTER_FN: crate::ClosureBridgeRustOnlyComponent = crate::closure_to_bridge_rust_only_component();
        }

        ADAPTER_FN.with(|comp_fn| {
            use crate::AsNullableElement;
            use wasm_bindgen::JsCast;

            let obj = js_sys::Object::new();
            let props: &crate::JsProps = obj.unchecked_ref();

            props.set_key(key);

            web_sys::console::log_1(&JsValue::from_str("set props bridge"));

            let k = forgotten::forget(Box::new(move || self.use_render().as_nullable_element())
                as Box<dyn Fn() -> Option<react_sys::Element>>);

            let k = k.into_shared();
            let k = k.as_usize();

            props.set_props_bridge(Some(*k));

            react_sys::create_element_no_children(comp_fn.as_ref(), props.as_ref())
        })
    }
}
