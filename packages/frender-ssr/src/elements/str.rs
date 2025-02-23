use frender_reactive_value::temp_str::TempStr;

use crate::{temp_str::IntoStaticStr, SsrElement};

/// The `AsRef<str>` bounds implies
/// for<T: KnownStr + 'static + PartialEq> T: CachedNonReactiveValue<str>
trait KnownStr: async_str_iter::IntoAsyncStrIterator + AsRef<str> {}

frender_common::impl_many!(
    impl<__> KnownStr
        for each_of![
            //
            &str,
            String,
            std::borrow::Cow<'_, str>,
            std::rc::Rc<str>,
            std::sync::Arc<str>,
        ]
    {
    }
);

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

impl<S> SsrElement for TempStr<S>
where
    S: IntoStaticStr,
{
    type HtmlChildren = frender_ssr_html::encode::Encode<
        frender_ssr_html::escape_safe::Safe,
        async_str_iter::any_str::IterAnyStr<S::StaticStr>,
    >;

    fn into_html_children(self) -> Self::HtmlChildren {
        Self::HtmlChildren::new(
            frender_ssr_html::escape_safe::Safe,
            async_str_iter::any_str::IterAnyStr::new(self.0.into_static_str()),
        )
    }
}
