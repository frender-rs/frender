use frender_ssr::{
    html::{assert::SpaceAndHtmlAttributesOrEmpty, tag::AssertTagName},
    SsrElement,
};
pub use props::{ElementProps, IntoElementProps};

mod props;

pub trait HasIntrinsicComponentTag {
    const INTRINSIC_COMPONENT_TAG: &'static str;
    const ASSERT_TAG_NAME: AssertTagName<&'static str>;
}

pub trait HasIntrinsicElementType {
    type IntrinsicElementType: IntrinsicElementType;
}

pub trait IntrinsicElementType {}

pub trait SsrComponentNormalElement: HasIntrinsicComponentTag {}

pub trait SsrComponent<Attrs: IntoSpaceAndHtmlAttributesOrEmpty, Children>:
    HasIntrinsicComponentTag
{
    type OneElement: frender_ssr::html::assert::OneElement;
    fn ssr_component(attrs: Attrs, children: Children) -> Self::OneElement;
}

impl<C, Attrs: IntoSpaceAndHtmlAttributesOrEmpty, Children: SsrElement>
    SsrComponent<Attrs, Children> for C
where
    C: SsrComponentNormalElement,
{
    type OneElement = frender_ssr::html::element::NormalElement<
        AssertTagName<&'static str>,
        <Attrs as IntoSpaceAndHtmlAttributesOrEmpty>::SpaceAndHtmlAttributesOrEmpty,
        Children::HtmlChildren,
    >;

    fn ssr_component(attrs: Attrs, children: Children) -> Self::OneElement {
        frender_ssr::html::element::NormalElement::new(
            C::ASSERT_TAG_NAME,
            attrs.into_space_and_html_attributes_or_empty(),
            SsrElement::into_html_children(children),
        )
    }
}

pub struct IntrinsicElement<C, P: IntoElementProps>(pub C, pub P);

pub trait IntoSpaceAndHtmlAttributesOrEmpty {
    type SpaceAndHtmlAttributesOrEmpty: SpaceAndHtmlAttributesOrEmpty;
    fn into_space_and_html_attributes_or_empty(self) -> Self::SpaceAndHtmlAttributesOrEmpty;
}

impl IntoSpaceAndHtmlAttributesOrEmpty for () {
    type SpaceAndHtmlAttributesOrEmpty = async_str_iter::empty::Empty;

    fn into_space_and_html_attributes_or_empty(self) -> Self::SpaceAndHtmlAttributesOrEmpty {
        async_str_iter::empty::Empty
    }
}

impl<A: IntoSpaceAndHtmlAttributesOrEmpty, B: IntoSpaceAndHtmlAttributesOrEmpty>
    IntoSpaceAndHtmlAttributesOrEmpty for (A, B)
{
    type SpaceAndHtmlAttributesOrEmpty = async_str_iter::chain::Chain<
        A::SpaceAndHtmlAttributesOrEmpty,
        B::SpaceAndHtmlAttributesOrEmpty,
    >;

    fn into_space_and_html_attributes_or_empty(self) -> Self::SpaceAndHtmlAttributesOrEmpty {
        async_str_iter::chain::Chain::new(
            self.0.into_space_and_html_attributes_or_empty(),
            self.1.into_space_and_html_attributes_or_empty(),
        )
    }
}

mod ssr {
    use frender_ssr::SsrElement;

    use super::{ElementProps, IntoElementProps, IntoSpaceAndHtmlAttributesOrEmpty, SsrComponent};

    impl<C, P: IntoElementProps> SsrElement for super::IntrinsicElement<C, P>
    where
        P::Attrs: IntoSpaceAndHtmlAttributesOrEmpty,
        C: SsrComponent<P::Attrs, P::Children>,
    {
        type HtmlChildren = C::OneElement;

        fn into_html_children(self) -> Self::HtmlChildren {
            let ElementProps {
                children,
                attributes,
            } = P::into_element_props(self.1);

            C::ssr_component(attributes, children)
        }
    }
}
