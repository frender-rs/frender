pub use self::into_static_str_cache::{IntoStaticStrCache, ToStaticStrCache};

mod into_static_str_cache;

#[derive(Debug, Clone, Copy)]
pub struct TempStr<S>(pub S);

#[cfg(test)]
mod asserts {
    use super::IntoStaticStrCache;

    #[test]
    const fn test()
    where
        for<'a> &'a str: AsRef<str> + IntoStaticStrCache,
        for<'a> std::borrow::Cow<'a, str>: AsRef<str> + IntoStaticStrCache,
        for<'a> &'a std::rc::Rc<str>: AsRef<str> + IntoStaticStrCache,
        for<'a> &'a std::sync::Arc<str>: AsRef<str> + IntoStaticStrCache,
    {
    }

    const _: () = test();
}
