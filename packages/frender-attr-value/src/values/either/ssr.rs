use crate::{ssr::SsrAttrValue, AttrValueKind};

use super::EitherAttrValue;

impl<A: SsrAttrValue<AK>, B: SsrAttrValue<AK>, AK: AttrValueKind> SsrAttrValue<AK>
    for EitherAttrValue<A, B>
{
    type HtmlAttributeValue =
        async_str_iter::either::IterEither<A::HtmlAttributeValue, B::HtmlAttributeValue>;

    fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
        match this {
            Self::A(this) => SsrAttrValue::maybe_into_html_attribute_value(this)
                .map(async_str_iter::either::IterEither::Left),
            Self::B(this) => SsrAttrValue::maybe_into_html_attribute_value(this)
                .map(async_str_iter::either::IterEither::Right),
        }
    }
}
