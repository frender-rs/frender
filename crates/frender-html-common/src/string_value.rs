pub trait StringValue: AsRef<str> {}

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
