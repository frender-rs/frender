use crate::WriteRef;
use std::{any::Any, rc::Rc};
use wasm_bindgen::JsValue;

#[derive(Debug, Clone)]
pub struct JsAdapterComponentProps {
    pub js_component: JsValue,
    pub js_props: Option<js_sys::Object>,
    pub js_children: Option<js_sys::Array>,
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

    fn use_render(self) -> Self::ElementType {
        let props = self.0;

        let a = crate::use_ref::<dyn Any, _>(|| (Rc::new(()) as Rc<dyn Any>));
        if let Some(to_persist) = props.to_persist {
            a.set_current(to_persist);
        }

        let null = JsValue::NULL;
        let js_props = props
            .js_props
            .as_ref()
            .map(|obj| obj.as_ref())
            .unwrap_or(&null);

        let el = if let Some(children_args) = &props.js_children {
            react_sys::create_element(&props.js_component, js_props, children_args)
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

    fn call_create_element(self, key: Option<JsValue>) -> react_sys::Element
    where
        Self: Sized,
    {
        thread_local! {
            static ADAPTER_FN: crate::ClosureBridgeRustOnlyComponent = crate::closure_to_bridge_rust_only_component();
        }

        ADAPTER_FN.with(|comp_fn| {
            use crate::AsNullableElement;
            use wasm_bindgen::JsCast;

            let obj = js_sys::Object::new();
            let props: &crate::JsProps = obj.dyn_ref().unwrap();

            props.set_key(key);

            props.set_props_bridge(Some(crate::PropsBridge::new(
                move || self.use_render().as_nullable_element(),
                "CommonHtmlComponent".into(),
            )));

            react_sys::create_element_no_children(comp_fn.as_ref(), props.as_ref())
        })
    }
}
