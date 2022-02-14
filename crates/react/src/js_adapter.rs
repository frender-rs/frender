use std::{any::Any, borrow::Cow, rc::Rc};
use wasm_bindgen::JsValue;

#[derive(Debug, Clone)]
pub struct JsAdapterComponentProps {
    pub debug_component_name: Option<JsValue>,
    pub debug_props: Option<JsValue>,
    pub js_component: JsValue,
    pub js_props: Option<js_sys::Object>,
    pub js_children: Option<crate::Children>,
    pub to_persist: Option<Rc<dyn Any>>,
}

pub trait IntoJsAdapterComponentProps {
    fn into_js_adapter_props(self) -> JsAdapterComponentProps;
}

pub struct JsAdapterComponent(pub JsAdapterComponentProps);

impl crate::Props for JsAdapterComponentProps {
    type InitialBuilder = ();

    fn init_builder() -> Self::InitialBuilder {}
}

impl crate::Component for JsAdapterComponent {
    type Props = JsAdapterComponentProps;
    type ElementType = react_sys::Element;

    fn use_render(&self) -> Self::ElementType {
        let props = &self.0;

        // TODO: check if required to use_ref
        // let a = crate::use_ref_with::<dyn Any, _>(|| (Rc::new(()) as Rc<dyn Any>));
        // if let Some(to_persist) = &props.to_persist {
        //     a.set_current(Rc::clone(to_persist));
        // }

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

    fn call_create_element(mut self, key: Option<&JsValue>) -> react_sys::Element {
        let debug_component_name = self.0.debug_component_name.take();
        let debug_props = self.0.debug_props.take();
        crate::bridge_rust_only_component(
            move || self.use_render(),
            key,
            debug_component_name.as_ref(),
            debug_props.as_ref(),
        )
    }
}
