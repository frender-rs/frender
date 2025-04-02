use frender_attrs::{experimental::ssr::SsrAttributes, IntoAttributes};

use crate::ssr::IntoSpaceAndHtmlAttributesOrEmpty;

use super::Attrs;

impl<T: IntoAttributes> IntoSpaceAndHtmlAttributesOrEmpty for Attrs<T> {
    type SpaceAndHtmlAttributesOrEmpty = <T::IntoAttributes as SsrAttributes>::IntoSsrAttributes;

    fn into_space_and_html_attributes_or_empty(self) -> Self::SpaceAndHtmlAttributesOrEmpty {
        T::into_attributes(self.0).into_ssr_attributes()
    }
}
