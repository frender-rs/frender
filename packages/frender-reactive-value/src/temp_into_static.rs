pub use self::into_static::{IntoStatic, ToStatic};
pub use self::into_static_cache::{IntoStaticCache, ToStaticCache};
pub use self::uncached_temp_into_static::UncachedTempIntoStatic;
pub use self::with_kind::{IntoStaticWithKind, ToStaticWithKind};

mod cheap_clone_pointer;

mod into_static;
mod into_static_cache;

mod uncached_temp_into_static;

mod with_kind;

#[derive(Debug, Clone, Copy)]
pub struct TempIntoStatic<T>(pub T);

#[cfg(test)]
mod asserts {
    use std::{borrow::Cow, rc::Rc, sync::Arc};

    use super::{IntoStaticCache, IntoStaticWithKind};

    struct _Test
    where
        for<'a> &'a str: IntoStaticWithKind<IntoStaticValue = str> + IntoStaticCache<str>,
        for<'a> &'a Cow<'static, str>:
            IntoStaticWithKind<IntoStaticValue = Cow<'static, str>> + IntoStaticCache<str>,
        for<'a> Cow<'a, str>: IntoStaticWithKind<IntoStaticValue = str> + IntoStaticCache<str>,
        for<'a> &'a Rc<str>: IntoStaticWithKind<IntoStaticValue = Rc<str>> + IntoStaticCache<str>,
        for<'a> &'a Arc<str>: IntoStaticWithKind<IntoStaticValue = Arc<str>> + IntoStaticCache<str>;

    const _: _Test = _Test;
}
