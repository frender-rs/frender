use frender_macro_rules::impl_many;

use crate::{
    strings::{CsrStr, NonReactiveStr},
    value_kind::{KindOfOwned, ValueKind},
    IntoStaticStrCache, TempStr, ToAsRefStr,
};

use super::{
    non_reactive::{Uncached, UncachedNonReactiveValue},
    ReactiveValue, ReactiveValueIntoElement,
};

pub trait ReactiveValueWithKind: ReactiveValue<Self::ReactiveValueKind> {
    type ReactiveValueKind: ?Sized + ValueKind;

    /// You may not need this method if `Self` already implements Element
    fn into_element(self) -> ReactiveValueIntoElement<Self>
    where
        Self: Sized,
    {
        ReactiveValueIntoElement(self)
    }
}

pub trait UncachedNonReactiveValueWithKind:
    UncachedNonReactiveValue<Self::UncachedNonReactiveValueKind>
{
    type UncachedNonReactiveValueKind: ?Sized + ValueKind;
}

impl<T: UncachedNonReactiveValueWithKind> ReactiveValueWithKind for Uncached<T> {
    type ReactiveValueKind = T::UncachedNonReactiveValueKind;
}

trait KnownCopy: 'static + Copy + PartialEq {}

impl<T: KnownCopy> UncachedNonReactiveValueWithKind for T {
    type UncachedNonReactiveValueKind = KindOfOwned<T>;
}

impl<T: KnownCopy> ReactiveValueWithKind for T {
    type ReactiveValueKind = KindOfOwned<T>;
}

impl_many!(
    impl<__> KnownCopy
        for each_of![
            //
            &'static str,
            i8,
            u8,
            i16,
            u16,
            i32,
            u32,
            i64,
            u64,
            i128,
            u128,
            isize,
            usize,
            f32,
            f64,
            char,
        ]
    {
    }
);

impl<S: ToAsRefStr> UncachedNonReactiveValueWithKind for TempStr<S> {
    type UncachedNonReactiveValueKind = str;
}

/// [`where TempStr<S>: CsrStr`](crate::strings::CsrStr)
impl<S: IntoStaticStrCache> ReactiveValueWithKind for TempStr<S> {
    type ReactiveValueKind = str;
}

impl<S: CsrStr> ReactiveValueWithKind for NonReactiveStr<S> {
    type ReactiveValueKind = str;
}

mod alloc {
    use std::{borrow::Cow, rc::Rc, sync::Arc};

    use crate::value_kind::{KindOfOwned, KindOfRef, KindOfStaticRefOrTempOwned};

    use super::{ReactiveValueWithKind, UncachedNonReactiveValueWithKind};

    impl UncachedNonReactiveValueWithKind for String {
        type UncachedNonReactiveValueKind = KindOfOwned<String>;
    }
    /// The default kind uses String as cache and TempStr<&str> as value.
    impl ReactiveValueWithKind for String {
        type ReactiveValueKind = str;
    }

    impl<T: ?Sized + 'static + ToOwned> UncachedNonReactiveValueWithKind for Cow<'static, T> {
        type UncachedNonReactiveValueKind = KindOfOwned<Cow<'static, T>>;
    }
    impl<T: ?Sized + 'static + ToOwned + PartialEq> ReactiveValueWithKind for Cow<'static, T> {
        type ReactiveValueKind = KindOfStaticRefOrTempOwned<T>;
    }

    impl<T: ?Sized + 'static> UncachedNonReactiveValueWithKind for Rc<T> {
        type UncachedNonReactiveValueKind = KindOfOwned<Rc<T>>;
    }
    impl<T: ?Sized + 'static + PartialEq> ReactiveValueWithKind for Rc<T> {
        type ReactiveValueKind = KindOfRef<Self>;
    }

    impl<T: ?Sized + 'static> UncachedNonReactiveValueWithKind for Arc<T> {
        type UncachedNonReactiveValueKind = KindOfOwned<Arc<T>>;
    }
    impl<T: ?Sized + 'static + PartialEq> ReactiveValueWithKind for Arc<T> {
        type ReactiveValueKind = KindOfRef<Self>;
    }
}
