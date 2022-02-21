use frender_macros::custom_element;

pub type StrictModeProps = crate::OptionalChildrenProps;

pub struct StrictMode;

#[custom_element(react = "crate")]
pub struct StrictModeElement(crate::JsBridgeElement);

impl crate::UseRenderStatic for StrictMode {
    type RenderArg = StrictModeProps;
    type RenderOutput = StrictModeElement;

    #[inline]
    fn use_render(props: &Self::RenderArg) -> Self::RenderOutput {
        StrictModeElement(crate::JsBridgeElement {
            js_component_type: crate::JsComponentType::Any(react_sys::StrictMode.clone()),
            js_props_without_children: None,
            children: props.children.clone(),
            key: None,
        })
    }
}

impl crate::ComponentStatic for StrictMode {
    type Props = StrictModeProps;
    type Element = StrictModeElement;

    #[inline]
    fn create_element(props: Self::Props, key: Option<crate::Key>) -> Self::Element {
        StrictModeElement(crate::JsBridgeElement {
            js_component_type: crate::JsComponentType::Any(react_sys::StrictMode.clone()),
            js_props_without_children: None,
            children: props.children,
            key,
        })
    }
}
