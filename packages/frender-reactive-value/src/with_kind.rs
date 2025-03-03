use frender_common::impl_many;

use crate::{
    static_or_temp_ref::StaticOrTempRef,
    temp_into_static::{
        IntoStaticCache, IntoStaticWithKind, TempIntoStatic, UncachedTempIntoStatic,
    },
    temp_ref::TempRef,
    value_kind::{KindOfOwned, KindOfStaticOrTempRef, KindOfTempRef, ValueKind},
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

impl<T: ?Sized + 'static> UncachedNonReactiveValueWithKind for TempRef<'_, T> {
    type UncachedNonReactiveValueKind = KindOfTempRef<T>;
}

impl<T: IntoStaticWithKind + UncachedTempIntoStatic<T::IntoStaticValue>>
    UncachedNonReactiveValueWithKind for TempIntoStatic<T>
{
    type UncachedNonReactiveValueKind = KindOfTempRef<T::IntoStaticValue>;
}

impl<T: IntoStaticWithKind + IntoStaticCache<T::IntoStaticValue>> ReactiveValueWithKind
    for TempIntoStatic<T>
{
    type ReactiveValueKind = KindOfTempRef<T::IntoStaticValue>;
}

impl<T: ?Sized + 'static> UncachedNonReactiveValueWithKind for StaticOrTempRef<'_, T> {
    type UncachedNonReactiveValueKind = KindOfStaticOrTempRef<T>;
}
impl<T: ?Sized + 'static + ToOwned + PartialEq> ReactiveValueWithKind for StaticOrTempRef<'_, T> {
    type ReactiveValueKind = KindOfStaticOrTempRef<T>;
}

mod alloc {
    use std::{borrow::Cow, rc::Rc, sync::Arc};

    use crate::value_kind::{KindOfOwned, KindOfStaticOrTempRef, KindOfTempRef};

    use super::{ReactiveValueWithKind, UncachedNonReactiveValueWithKind};

    impl UncachedNonReactiveValueWithKind for String {
        type UncachedNonReactiveValueKind = KindOfOwned<String>;
    }
    /// The default kind uses String as cache and &str as value.
    impl ReactiveValueWithKind for String {
        type ReactiveValueKind = KindOfTempRef<str>;
    }

    impl<T: ?Sized + 'static + ToOwned> UncachedNonReactiveValueWithKind for Cow<'static, T> {
        type UncachedNonReactiveValueKind = KindOfOwned<Cow<'static, T>>;
    }
    impl<T: ?Sized + 'static + ToOwned + PartialEq> ReactiveValueWithKind for Cow<'static, T> {
        type ReactiveValueKind = KindOfStaticOrTempRef<T>;
    }

    impl<T: ?Sized + 'static> UncachedNonReactiveValueWithKind for Rc<T> {
        type UncachedNonReactiveValueKind = KindOfOwned<Rc<T>>;
    }
    impl<T: ?Sized + 'static + PartialEq> ReactiveValueWithKind for Rc<T> {
        type ReactiveValueKind = KindOfTempRef<Self>;
    }

    impl<T: ?Sized + 'static> UncachedNonReactiveValueWithKind for Arc<T> {
        type UncachedNonReactiveValueKind = KindOfOwned<Arc<T>>;
    }
    impl<T: ?Sized + 'static + PartialEq> ReactiveValueWithKind for Arc<T> {
        type ReactiveValueKind = KindOfTempRef<Self>;
    }
}
