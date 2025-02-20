use frender_dom::ssr::{IntoSpaceAndHtmlAttributesOrEmpty, SsrComponent};
use frender_form_control::textarea::SsrTextAreaValue;
use frender_ssr::html::tag::AssertTagName;

use crate::cs::textarea;

type Element<Attrs, Children> = frender_ssr::html::element::NormalElement<
    //
    AssertTagName<&'static str>,
    Attrs,
    Children,
>;

impl<Children> SsrComponent<Children> for textarea::Marker
where
    Children: SsrTextAreaValue,
{
    type OneElement<Attrs: IntoSpaceAndHtmlAttributesOrEmpty> = Element<
        //
        Attrs::SpaceAndHtmlAttributesOrEmpty,
        Children::IntoSsrTextAreaValue,
    >;

    fn ssr_component<Attrs: IntoSpaceAndHtmlAttributesOrEmpty>(self, attrs: Attrs, children: Children) -> Self::OneElement<Attrs> {
        use frender_dom::ssr::HasIntrinsicComponentTagSsr;
        frender_ssr::html::element::NormalElement::new(Self::ASSERT_TAG_NAME, attrs.into_space_and_html_attributes_or_empty(), Children::into_ssr_text_area_value(children))
    }
}
