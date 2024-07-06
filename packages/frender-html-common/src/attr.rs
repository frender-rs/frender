use async_str_iter::IntoAsyncStrIterator;
use frender_ssr_html::assert::HtmlAttributeEqValueOrEmpty;

use crate::StringValue;

/// A *html attribute value* is `=value`, `="value"`, `='value'` or empty.
pub trait MaybeIntoHtmlAttributeValue<AttributeType: ?Sized> {
    type HtmlAttributeValue: HtmlAttributeEqValueOrEmpty;

    /// `None` indicates this attributes is not present
    fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue>;
}

impl<S: StringValue> MaybeIntoHtmlAttributeValue<str> for S {
    type HtmlAttributeValue =
        frender_ssr_html::attr_value::AttrEqValue<async_str_iter::any_str::IterAnyStr<S>>;

    fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
        Some(Self::HtmlAttributeValue::new(
            async_str_iter::any_str::AnyStr(this).into_async_str_iterator(),
        ))
    }
}

impl<S: std::borrow::Borrow<str>> MaybeIntoHtmlAttributeValue<str> for frender_common::TempStr<S> {
    type HtmlAttributeValue = <String as MaybeIntoHtmlAttributeValue<str>>::HtmlAttributeValue;

    fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
        <String as MaybeIntoHtmlAttributeValue<str>>::maybe_into_html_attribute_value(
            this.0.borrow().to_owned(),
        )
    }
}

/// The attribute is absent
impl<AttributeType: ?Sized> MaybeIntoHtmlAttributeValue<AttributeType> for () {
    type HtmlAttributeValue = async_str_iter::never::Never;

    fn maybe_into_html_attribute_value((): Self) -> Option<Self::HtmlAttributeValue> {
        None
    }
}

impl<AttributeType: ?Sized, V: MaybeIntoHtmlAttributeValue<AttributeType>>
    MaybeIntoHtmlAttributeValue<AttributeType> for Option<V>
{
    type HtmlAttributeValue = V::HtmlAttributeValue;

    fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
        this.and_then(V::maybe_into_html_attribute_value)
    }
}

frender_common::impl_many!(
    impl<__> MaybeIntoHtmlAttributeValue<Self>
        for each_of![i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64]
    {
        type HtmlAttributeValue = <String as MaybeIntoHtmlAttributeValue<str>>::HtmlAttributeValue;

        fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
            <String as MaybeIntoHtmlAttributeValue<str>>::maybe_into_html_attribute_value(
                this.to_string(),
            )
        }
    }
);

impl MaybeIntoHtmlAttributeValue<bool> for bool {
    type HtmlAttributeValue = async_str_iter::empty::Empty;

    fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
        this.then_some(async_str_iter::empty::Empty)
    }
}

#[cfg(feature = "either")]
mod either {
    use either::Either;

    use super::MaybeIntoHtmlAttributeValue;

    impl<
            L: MaybeIntoHtmlAttributeValue<AttributeType>,
            R: MaybeIntoHtmlAttributeValue<AttributeType>,
            AttributeType: ?Sized,
        > MaybeIntoHtmlAttributeValue<AttributeType> for Either<L, R>
    {
        type HtmlAttributeValue =
            async_str_iter::either::IterEither<L::HtmlAttributeValue, R::HtmlAttributeValue>;

        fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
            match this {
                Either::Left(this) => {
                    MaybeIntoHtmlAttributeValue::maybe_into_html_attribute_value(this)
                        .map(async_str_iter::either::IterEither::Left)
                }
                Either::Right(this) => {
                    MaybeIntoHtmlAttributeValue::maybe_into_html_attribute_value(this)
                        .map(async_str_iter::either::IterEither::Right)
                }
            }
        }
    }
}

/// A trait alias.
pub trait MaybeAttrValue<VK: ?Sized + crate::ValueKind>:
    MaybeIntoHtmlAttributeValue<VK> + crate::MaybeValue<VK>
{
}

impl<
        T: ?Sized + MaybeIntoHtmlAttributeValue<VK> + crate::MaybeValue<VK>,
        VK: ?Sized + crate::ValueKind,
    > MaybeAttrValue<VK> for T
{
}
