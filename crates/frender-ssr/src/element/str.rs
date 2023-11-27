use crate::SsrElement;

frender_common::impl_many!(
    impl<__> SsrElement
        for each_of![
            //
            &str,
            String,
            std::borrow::Cow<'_, str>,
        ]
    {
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
);
