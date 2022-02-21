use crate::Keyed;

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

pub trait UseRender {
    type RenderArg: Props;
    /// This allows implementor type to specify
    /// a custom type as the return type of the methods.
    ///
    /// With this associated type param, some special components
    /// can return special elements with special traits.
    /// For example, [`table`](crate::html::table) component
    /// returns
    type RenderOutput;
    fn use_render(&self, props: &Self::RenderArg) -> Self::RenderOutput;
}

pub trait Component {
    type Props: Props;
    /// Output of `create_element`.
    /// Many components may return `Option<Element>` in [`UseRender::use_render`]
    /// while return `Element` in create_element.
    /// Thus this type parameter exists.
    type Element;

    fn create_element(self, props: Self::Props, key: Option<crate::Key>) -> Self::Element;
}

pub trait UseRenderStatic {
    type RenderArg: Props;
    /// See [`UseRender::Output`]
    type RenderOutput;
    fn use_render(props: &Self::RenderArg) -> Self::RenderOutput;
}

pub trait ComponentStatic {
    type Props: Props;
    /// See [`Component::Element`]
    type Element;

    fn create_element(props: Self::Props, key: Option<crate::Key>) -> Self::Element;
}

impl<T: UseRenderStatic> UseRender for T {
    type RenderArg = <T as UseRenderStatic>::RenderArg;
    type RenderOutput = <T as UseRenderStatic>::RenderOutput;

    #[inline]
    fn use_render(&self, props: &Self::RenderArg) -> Self::RenderOutput {
        T::use_render(props)
    }
}

impl<T: ComponentStatic> Component for T {
    type Props = <T as ComponentStatic>::Props;
    type Element = <T as ComponentStatic>::Element;

    #[inline]
    fn create_element(self, props: Self::Props, key: Option<crate::Key>) -> Self::Element {
        T::create_element(props, key)
    }
}
