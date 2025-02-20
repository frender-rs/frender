use frender_dom::ssr::{HasIntrinsicComponentTagSsr, IntoSpaceAndHtmlAttributesOrEmpty, SsrComponent};
use frender_dom::Empty;
use frender_ssr::html::tag::AssertTagName;

use crate::html::markers as tags;

frender_common::impl_many!(
    impl<__> SsrComponent<Empty>
        for each_of![
            tags::area,
            tags::base,
            tags::br,
            tags::col,
            tags::embed,
            tags::hr,
            tags::img,
            // tags::input, // input is special
            tags::link,
            tags::meta,
            tags::source,
            tags::track,
            tags::wbr,
        ]
    {
        type OneElement<Attrs: IntoSpaceAndHtmlAttributesOrEmpty> = frender_ssr::html::element::VoidElement<AssertTagName<&'static str>, <Attrs as IntoSpaceAndHtmlAttributesOrEmpty>::SpaceAndHtmlAttributesOrEmpty>;

        fn ssr_component<Attrs: IntoSpaceAndHtmlAttributesOrEmpty>(self, attrs: Attrs, Empty: Empty) -> Self::OneElement<Attrs> {
            Self::OneElement::<Attrs>::new(Self::ASSERT_TAG_NAME, attrs.into_space_and_html_attributes_or_empty())
        }
    }
);
