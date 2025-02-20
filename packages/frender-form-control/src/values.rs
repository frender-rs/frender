#[derive(Debug, Clone, Copy)]
pub struct UncontrolledEmptyDefaultValue;

/// This wrapper proxies [`SsrAttrValue`] and [`SsrTextAreaValue`].
#[derive(Debug)]
pub struct UncontrolledWithDefaultValue<V>(pub V);

#[derive(Debug, Clone, Copy)]
pub enum EitherFormControlValue<A, B> {
    A(A),
    B(B),
}

#[cfg(feature = "either")]
impl<A, B> EitherFormControlValue<A, B> {
    pub(crate) fn from_either(v: either::Either<A, B>) -> Self {
        match v {
            either::Either::Left(this) => Self::A(this),
            either::Either::Right(this) => Self::B(this),
        }
    }
}

#[cfg(todo)] // TODO: is this needed?
mod ssr {
    use frender_attr_value::ssr::SsrAttrValue;

    use super::UncontrolledWithDefaultValue;

    impl<V: SsrAttrValue<AT>, AT: ?Sized> SsrAttrValue<AT> for UncontrolledWithDefaultValue<V> {
        type HtmlAttributeValue = V::HtmlAttributeValue;

        fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
            V::maybe_into_html_attribute_value(this.0)
        }
    }
}
