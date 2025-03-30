use frender_ssr_html::assert::SpaceAndHtmlAttributesOrEmpty;

pub trait SsrAttributes {
    type IntoSsrAttributes: SpaceAndHtmlAttributesOrEmpty;
    fn into_ssr_attributes(self) -> Self::IntoSsrAttributes;
}
