pub use self::into_static_str::{IntoStaticStr, ToStaticStr};

mod into_static_str;

#[cfg(test)]
mod asserts {
    use super::IntoStaticStr;

    #[test]
    const fn test()
    where
        for<'a> &'a str: AsRef<str> + IntoStaticStr,
        for<'a> std::borrow::Cow<'a, str>: AsRef<str> + IntoStaticStr,
        for<'a> &'a std::rc::Rc<str>: AsRef<str> + IntoStaticStr,
        for<'a> &'a std::sync::Arc<str>: AsRef<str> + IntoStaticStr,
    {
    }

    const _: () = test();
}
