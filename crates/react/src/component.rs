use wasm_bindgen::JsValue;

pub trait PropsBuilder<T> {
    fn build(self) -> T;
}

pub trait Props {
    type InitialBuilder;

    fn init_builder() -> Self::InitialBuilder;
}

pub struct NoProps;

impl PropsBuilder<NoProps> for NoProps {
    #[inline]
    fn build(self) -> NoProps {
        self
    }
}

impl Props for NoProps {
    type InitialBuilder = NoProps;

    #[inline]
    fn init_builder() -> Self::InitialBuilder {
        NoProps
    }
}

pub trait Component {
    type Props: Props;
    type ElementType: crate::AsNullableElement;

    fn use_render(&self) -> Self::ElementType;

    fn new_with_props(props: Self::Props) -> Self
    where
        Self: Sized;

    fn call_create_element(self, key: Option<JsValue>) -> Self::ElementType;
}
