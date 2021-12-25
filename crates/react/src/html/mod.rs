mod aria;
mod attrs;
mod css;
mod dom;

#[allow(non_camel_case_types)]
pub struct div {}

pub struct DivProps {}

pub struct DivElement {}

// impl HtmlElement for DivElement {}

impl crate::Component for div {
    type ElementType = DivElement;

    type Props;

    fn render(props: Self::Props) -> Self::ElementType {
        todo!()
    }
}
