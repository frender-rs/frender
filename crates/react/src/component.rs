pub trait Props {
    type InitialBuilder;
    type ValidBuilder;

    fn init_builder() -> Self::InitialBuilder;
    fn from_builder(builder: Self::ValidBuilder) -> Self;
}

pub trait Component {
    type ElementType: crate::Element;
    type Props;

    fn create_with_props(props: Self::Props) -> Self;
    fn render(self) -> Self::ElementType;
}
