use ::frender_ssr::html::attr::{AssertSpaceAndHtmlAttributeName, SpaceAndHtmlAttribute};
use frender_attr_value::ssr::SsrAttrValue;

use crate::has_const_attr_name::HasConstAttrNameSsr;

use super::HasAttrValueKind;

pub type SpaceAndHtmlAttributesOrEmpty<V, VK> = ::async_str_iter::option::IterOption<SpaceAndHtmlAttribute<AssertSpaceAndHtmlAttributeName<&'static str>, <V as SsrAttrValue<VK>>::HtmlAttributeValue>>;

pub(crate) fn into_space_and_html_attributes_or_empty<PM: HasConstAttrNameSsr + HasAttrValueKind, V: SsrAttrValue<PM::AttrValueKind>>(value: V) -> SpaceAndHtmlAttributesOrEmpty<V, PM::AttrValueKind> {
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
    ::async_str_iter::IntoAsyncStrIterator::into_async_str_iterator(V::maybe_into_html_attribute_value(value).map(|v| SpaceAndHtmlAttribute(PM::ASSERT_SPACE_AND_HTML_ATTRIBUTE_NAME, v)))
}
