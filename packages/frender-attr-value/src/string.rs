pub(crate) trait KnownStaticStr: 'static + AsRef<str> + PartialEq {}

frender_common::impl_many!(
    impl<__> KnownStaticStr
        for each_of![
            &'static str,
            String,
            std::borrow::Cow<'static, str>,
            std::rc::Rc<str>,
            std::sync::Arc<str>,
        ]
    {
    }
);
