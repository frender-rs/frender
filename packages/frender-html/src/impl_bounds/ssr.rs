use ::async_str_iter::{option::IterOption, IntoAsyncStrIterator};
use frender_ssr::html::{
    assert::HtmlAttributeEqValueOrEmpty,
    attr::{AssertSpaceAndHtmlAttributeName, SpaceAndHtmlAttribute},
};

use crate::has_const_attr_name::{HasConstAttrName, HasConstAttrNameSsr};

/// `Haevoe` means [`HtmlAttributeEqValueOrEmpty`].
pub type SpaceAndHtmlAttributes<Haevoe> = SpaceAndHtmlAttribute<AssertSpaceAndHtmlAttributeName<&'static str>, Haevoe>;

pub type SpaceAndHtmlAttributesOrEmpty<Haevoe> = IterOption<SpaceAndHtmlAttributes<Haevoe>>;

pub(crate) fn into_space_and_html_attributes<PM: HasConstAttrNameSsr, Haevoe: HtmlAttributeEqValueOrEmpty>(haevoe: Haevoe) -> SpaceAndHtmlAttributes<Haevoe> {
    #[cfg(test)]
    const {
        let spaced = PM::ASSERT_SPACE_AND_HTML_ATTRIBUTE_NAME.as_inner_str().as_bytes();
        match spaced {
            [b' ', a @ ..] => {
                let b = PM::ATTR_NAME.as_bytes();
                assert!(a.len() == b.len());
                let mut i = 0;
                while i < a.len() {
                    assert!(a[i] == b[i]);
                    i += 1;
                }
            }
            _ => panic!(),
        }
    };
    SpaceAndHtmlAttribute(PM::ASSERT_SPACE_AND_HTML_ATTRIBUTE_NAME, haevoe)
}

pub(crate) fn into_space_and_html_attributes_or_empty<PM: HasConstAttrNameSsr, Haevoe: HtmlAttributeEqValueOrEmpty>(haevoe: Option<Haevoe>) -> SpaceAndHtmlAttributesOrEmpty<Haevoe> {
    haevoe.map(into_space_and_html_attributes::<PM, Haevoe>).into_async_str_iterator()
}
