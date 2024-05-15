use async_str_iter::IntoAsyncStrIterator;

pub trait StringValue:
    AsRef<str> + IntoAsyncStrIterator<IntoAsyncStrIterator = Self::OneString>
{
    type OneString: frender_ssr_html::assert::OneString;
}

frender_common::impl_many!(
    impl<__> StringValue
        for each_of![
            &str,
            String,
            std::borrow::Cow<'_, str>,
            std::rc::Rc<str>,
            std::sync::Arc<str>,
        ]
    {
        type OneString = <Self as IntoAsyncStrIterator>::IntoAsyncStrIterator;
    }
);
