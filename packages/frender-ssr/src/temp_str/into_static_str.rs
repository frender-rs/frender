use frender_common::impl_many;

pub trait IntoStaticStr {
    type StaticStr: 'static + AsRef<str>;
    fn into_static_str(self) -> Self::StaticStr
    where
        Self: Sized;
}

impl<T: ?Sized + ToStaticStr> IntoStaticStr for &T {
    type StaticStr = T::ToStaticStr;

    fn into_static_str(self) -> Self::StaticStr {
        T::to_static_str(self)
    }
}

pub trait ToStaticStr {
    type ToStaticStr: 'static + AsRef<str>;
    fn to_static_str(&self) -> Self::ToStaticStr;
}

impl ToStaticStr for str {
    type ToStaticStr = String;

    fn to_static_str(&self) -> Self::ToStaticStr {
        self.to_owned()
    }
}

impl_many!(
    impl<__> ToStaticStr
        for each_of![
            //
            std::rc::Rc<str>,
            std::sync::Arc<str>
        ]
    {
        type ToStaticStr = Self;
        fn to_static_str(&self) -> Self::ToStaticStr {
            Self::clone(self)
        }
    }
);

impl IntoStaticStr for std::borrow::Cow<'_, str> {
    type StaticStr = String;

    fn into_static_str(self) -> Self::StaticStr {
        self.into_owned()
    }
}
