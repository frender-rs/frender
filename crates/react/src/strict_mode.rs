use wasm_bindgen::JsValue;

pub struct StrictMode(crate::OptionalChildrenProps);

impl crate::Component for StrictMode {
    type Props = crate::OptionalChildrenProps;
    type ElementType = react_sys::Element;

    fn use_render(&self) -> Self::ElementType {
        crate::create_element_js::create_element_with_js_value(
            &react_sys::StrictMode,
            None,
            self.0.children.as_ref(),
            None,
        )
    }

    fn new_with_props(props: Self::Props) -> Self
    where
        Self: Sized,
    {
        Self(props)
    }

    fn call_create_element(self, key: Option<JsValue>) -> Self::ElementType {
        crate::create_element_js::create_element_with_js_value(
            &react_sys::StrictMode,
            None,
            self.0.children.as_ref(),
            key,
        )
    }
}
