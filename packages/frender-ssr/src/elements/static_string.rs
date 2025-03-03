use std::borrow::Borrow;

use frender_reactive_value::impl_known_StaticBorrowStr_v_0_1_0;

use crate::SsrElement;

/// The `Borrow<str>` bound implies
/// for<T: KnownStr + PartialEq> T: CachedNonReactiveValue<str>
trait KnownStr: 'static + async_str_iter::IntoAsyncStrIterator + Borrow<str> {}

impl_known_StaticBorrowStr_v_0_1_0! {KnownStr}

impl<T: KnownStr> SsrElement for T {
    type HtmlChildren = frender_ssr_html::encode::Encode<
        frender_ssr_html::escape_safe::Safe,
        <Self as async_str_iter::IntoAsyncStrIterator>::IntoAsyncStrIterator,
    >;

    fn into_html_children(self) -> Self::HtmlChildren {
        Self::HtmlChildren::new(
            frender_ssr_html::escape_safe::Safe,
            async_str_iter::IntoAsyncStrIterator::into_async_str_iterator(self),
        )
    }
}

impl super::KnownCopySsrElement for &'static str {}
