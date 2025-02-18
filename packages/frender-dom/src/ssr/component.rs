use frender_ssr::{
    html::{assert::SpaceAndHtmlAttributesOrEmpty, tag::AssertTagName},
    SsrElement,
};

use crate::tag::HasIntrinsicComponentTag;

pub trait HasIntrinsicComponentTagSsr: HasIntrinsicComponentTag {
    const ASSERT_TAG_NAME: AssertTagName<&'static str>;
}

impl<T: ?Sized + HasIntrinsicComponentTag> HasIntrinsicComponentTagSsr for T {
    const ASSERT_TAG_NAME: AssertTagName<&'static str> =
        AssertTagName::new_from_str(Self::INTRINSIC_COMPONENT_TAG);
}

pub trait SsrComponentNormalElement: HasIntrinsicComponentTag {}

pub trait SsrComponent<Children>: HasIntrinsicComponentTag {
    type OneElement<Attrs: IntoSpaceAndHtmlAttributesOrEmpty>: frender_ssr::html::assert::OneElement;
    fn ssr_component<Attrs: IntoSpaceAndHtmlAttributesOrEmpty>(
        self,
        attrs: Attrs,
        children: Children,
    ) -> Self::OneElement<Attrs>;
}

impl<C, Children: SsrElement> SsrComponent<Children> for C
where
    C: SsrComponentNormalElement,
{
    type OneElement<Attrs: IntoSpaceAndHtmlAttributesOrEmpty> =
        frender_ssr::html::element::NormalElement<
            AssertTagName<&'static str>,
            <Attrs as IntoSpaceAndHtmlAttributesOrEmpty>::SpaceAndHtmlAttributesOrEmpty,
            Children::HtmlChildren,
        >;

    fn ssr_component<Attrs: IntoSpaceAndHtmlAttributesOrEmpty>(
        self,
        attrs: Attrs,
        children: Children,
    ) -> Self::OneElement<Attrs> {
        frender_ssr::html::element::NormalElement::new(
            C::ASSERT_TAG_NAME,
            attrs.into_space_and_html_attributes_or_empty(),
            SsrElement::into_html_children(children),
        )
    }
}

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
