#[non_exhaustive]
pub struct FragmentProps {
    pub children: Option<crate::AnyNode>,
}

pub struct Fragment;

impl crate::Component for Fragment {
    type Props = crate::NoProps;
    type ElementType = react_sys::Element;

    fn use_render(&self) -> Self::ElementType {
        todo!()
    }

    fn new_with_props(props: Self::Props) -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn call_create_element(self, key: Option<wasm_bindgen::JsValue>) -> Self::ElementType {
        todo!()
    }
}
