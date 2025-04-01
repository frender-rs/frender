use crate::{
    html::{bool_to_str, AttrKindOfContentEditable},
    known::KnownStr,
    ssr::SsrAttrValue,
    AttrKindOfStr, IntoAttrValue,
};

use super::{CachedSomeBool, CachedSomeStr, EmptyAsContentEditable};

// TODO: optimize escaping
impl SsrAttrValue<AttrKindOfContentEditable> for CachedSomeBool {
    type HtmlAttributeValue =
        <<&'static str as IntoAttrValue<AttrKindOfStr>>::IntoAttrValue as SsrAttrValue<
            AttrKindOfStr,
        >>::HtmlAttributeValue;

    fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
        let this = this.0;
        let this = bool_to_str(this);

        let this = <&'static str as IntoAttrValue<AttrKindOfStr>>::into_attr_value(this);

        <_ as SsrAttrValue<AttrKindOfStr>>::maybe_into_html_attribute_value(this)
    }
}

impl SsrAttrValue<AttrKindOfContentEditable> for EmptyAsContentEditable {
    type HtmlAttributeValue = async_str_iter::empty::Empty;

    fn maybe_into_html_attribute_value(Self: Self) -> Option<Self::HtmlAttributeValue> {
        Some(async_str_iter::empty::Empty)
    }
}

impl<V: KnownStr> SsrAttrValue<AttrKindOfContentEditable> for CachedSomeStr<V> {
    type HtmlAttributeValue = <Self as SsrAttrValue<AttrKindOfStr>>::HtmlAttributeValue;

    fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
        <Self as SsrAttrValue<AttrKindOfStr>>::maybe_into_html_attribute_value(this)
    }
}
