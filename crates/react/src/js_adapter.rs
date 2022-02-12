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

        let el = crate::create_element_js::create_element_with_js_value(
            &props.js_component,
            props.js_props.as_ref(),
            props.js_children.as_ref(),
            None,
        );

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

            let k = forgotten::forget(Box::new(move || self.use_render().as_nullable_element())
                as Box<dyn Fn() -> Option<react_sys::Element>>);

            let k = k.into_shared();
            let k = k.as_usize();

            props.set_props_bridge(Some(*k));

            react_sys::create_element_no_children(comp_fn.as_ref(), props.as_ref())
        })
    }
}
