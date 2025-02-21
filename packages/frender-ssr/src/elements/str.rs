use frender_common::{
    strings::{NonReactiveStr, SsrStr},
    IntoStaticStr,
};

use crate::SsrElement;

pub trait KnownStr: async_str_iter::IntoAsyncStrIterator {}

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

type SsrStrIntoHtmlChildren<S> = frender_ssr_html::encode::Encode<
    frender_ssr_html::escape_safe::Safe,
    async_str_iter::any_str::IterAnyStr<<S as SsrStr>::StaticStr>,
>;

/// <code>where TempStr\<S>: [SsrStr]</code>
impl<S> SsrElement for frender_common::TempStr<S>
where
    S: frender_common::IntoStaticStr,
{
    type HtmlChildren = SsrStrIntoHtmlChildren<Self>;

    fn into_html_children(self) -> Self::HtmlChildren {
        NonReactiveStr(self).into_html_children()
    }
}

/// <code>where NonReactiveStr\<S>: [SsrStr]</code>
impl<S: SsrStr> SsrElement for NonReactiveStr<S> {
    type HtmlChildren = SsrStrIntoHtmlChildren<S>;

    fn into_html_children(self) -> Self::HtmlChildren {
        Self::HtmlChildren::new(
            frender_ssr_html::escape_safe::Safe,
            async_str_iter::any_str::IterAnyStr::new(
                self.0.into_into_static_str().into_static_str(),
            ),
        )
    }
}
