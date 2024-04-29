use async_str_iter::IntoAsyncStrIterator;

pub trait StringValue: AsRef<str> + IntoAsyncStrIterator {}

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
    }
);
