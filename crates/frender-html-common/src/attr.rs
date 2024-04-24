use async_str_iter::IntoAsyncStrIterator;
use frender_ssr_html::assert::HtmlAttributeEqValueOrEmpty;

use crate::StringValue;

pub enum HtmlAttributeValue<S> {
    String(S),
    /// If a boolean attribute is present, its value is true, and if it's absent, its value is false.
    ///
    /// HTML defines restrictions on the allowed values of boolean attributes. Please see
    /// [boolean_attributes]
    ///
    /// [boolean_attributes]: https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes#boolean_attributes
    BooleanTrue,
}

pub trait MaybeIntoHtmlAttributeEqValueOrEmpty<AttributeType: ?Sized> {
    type HtmlAttributeEqValueOrEmpty: HtmlAttributeEqValueOrEmpty;

    /// `None` indicates this attributes is not present
    fn maybe_into_html_attribute_eq_value_or_empty(
        this: Self,
    ) -> Option<Self::HtmlAttributeEqValueOrEmpty>;
}

impl<S: StringValue> MaybeIntoHtmlAttributeEqValueOrEmpty<str> for S {
    type HtmlAttributeEqValueOrEmpty =
        frender_ssr_html::attr_value::AttrEqValue<async_str_iter::any_str::IterAnyStr<S>>;

    fn maybe_into_html_attribute_eq_value_or_empty(
        this: Self,
    ) -> Option<Self::HtmlAttributeEqValueOrEmpty> {
        Some(Self::HtmlAttributeEqValueOrEmpty::new(
            async_str_iter::any_str::AnyStr(this).into_async_str_iterator(),
        ))
    }
}

impl<S: std::borrow::Borrow<str>> MaybeIntoHtmlAttributeEqValueOrEmpty<str>
    for frender_common::TempStr<S>
{
    type HtmlAttributeEqValueOrEmpty =
        <String as MaybeIntoHtmlAttributeEqValueOrEmpty<str>>::HtmlAttributeEqValueOrEmpty;

    fn maybe_into_html_attribute_eq_value_or_empty(
        this: Self,
    ) -> Option<Self::HtmlAttributeEqValueOrEmpty> {
        <String as MaybeIntoHtmlAttributeEqValueOrEmpty<str>>::maybe_into_html_attribute_eq_value_or_empty(

            this.0.borrow().to_owned()
        )
    }
}

/// The attribute is absent
impl<AttributeType: ?Sized> MaybeIntoHtmlAttributeEqValueOrEmpty<AttributeType> for () {
    type HtmlAttributeEqValueOrEmpty = async_str_iter::never::Never;

    fn maybe_into_html_attribute_eq_value_or_empty(
        (): Self,
    ) -> Option<Self::HtmlAttributeEqValueOrEmpty> {
        None
    }
}

impl<AttributeType: ?Sized, V: MaybeIntoHtmlAttributeEqValueOrEmpty<AttributeType>>
    MaybeIntoHtmlAttributeEqValueOrEmpty<AttributeType> for Option<V>
{
    type HtmlAttributeEqValueOrEmpty = V::HtmlAttributeEqValueOrEmpty;

    fn maybe_into_html_attribute_eq_value_or_empty(
        this: Self,
    ) -> Option<Self::HtmlAttributeEqValueOrEmpty> {
        this.and_then(V::maybe_into_html_attribute_eq_value_or_empty)
    }
}

frender_common::impl_many!(
    impl<__> MaybeIntoHtmlAttributeEqValueOrEmpty<Self>
        for each_of![i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64]
    {
        type HtmlAttributeEqValueOrEmpty =
            <String as MaybeIntoHtmlAttributeEqValueOrEmpty<str>>::HtmlAttributeEqValueOrEmpty;

        fn maybe_into_html_attribute_eq_value_or_empty(
            this: Self,
        ) -> Option<Self::HtmlAttributeEqValueOrEmpty> {
            <String as MaybeIntoHtmlAttributeEqValueOrEmpty<str>>::maybe_into_html_attribute_eq_value_or_empty(this.to_string())
        }
    }
);

impl MaybeIntoHtmlAttributeEqValueOrEmpty<bool> for bool {
    type HtmlAttributeEqValueOrEmpty = async_str_iter::empty::Empty;

    fn maybe_into_html_attribute_eq_value_or_empty(
        this: Self,
    ) -> Option<Self::HtmlAttributeEqValueOrEmpty> {
        this.then_some(async_str_iter::empty::Empty)
    }
}

#[cfg(feature = "either")]
mod either {
    use either::Either;

    use super::MaybeIntoHtmlAttributeEqValueOrEmpty;

    impl<
            L: MaybeIntoHtmlAttributeEqValueOrEmpty<AttributeType>,
            R: MaybeIntoHtmlAttributeEqValueOrEmpty<AttributeType>,
            AttributeType: ?Sized,
        > MaybeIntoHtmlAttributeEqValueOrEmpty<AttributeType> for Either<L, R>
    {
        type HtmlAttributeEqValueOrEmpty = async_str_iter::either::IterEither<
            L::HtmlAttributeEqValueOrEmpty,
            R::HtmlAttributeEqValueOrEmpty,
        >;

        fn maybe_into_html_attribute_eq_value_or_empty(
            this: Self,
        ) -> Option<Self::HtmlAttributeEqValueOrEmpty> {
            match this {
                Either::Left(this) => {
                    MaybeIntoHtmlAttributeEqValueOrEmpty::maybe_into_html_attribute_eq_value_or_empty(this)
                        .map(async_str_iter::either::IterEither::Left)
                }
                Either::Right(this) => {
                    MaybeIntoHtmlAttributeEqValueOrEmpty::maybe_into_html_attribute_eq_value_or_empty(this)
                        .map(async_str_iter::either::IterEither::Right)
                }
            }
        }
    }
}
