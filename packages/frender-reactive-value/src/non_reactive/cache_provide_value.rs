use std::borrow::{Borrow, Cow};

pub(super) use self::sealed::{CacheCanNotProvideValue, CacheCanProvideValueMarker};

use crate::{
    temp_ref::TempRef,
    value_kind::{KindOfOwned, KindOfStaticOrTempRef, KindOfTempRef, ValueKind},
};

pub struct CacheCanProvideValue;

mod sealed {
    use super::CacheCanProvideValue;

    pub trait CacheCanProvideValueMarker {}

    pub enum CacheCanNotProvideValue {}

    impl CacheCanProvideValueMarker for CacheCanProvideValue {}
    impl CacheCanProvideValueMarker for CacheCanNotProvideValue {}

    pub trait ConstDefault {
        const DEFAULT: Self;
    }

    impl ConstDefault for CacheCanProvideValue {
        const DEFAULT: Self = Self;
    }
}

pub trait CacheProvideValue<
    VK: ?Sized + ValueKind,
    // This type generic is sealed
    CacheCanProvideValue: sealed::CacheCanProvideValueMarker = self::CacheCanProvideValue,
>
{
    /// The implementation should be cheap (should not alloc or clone).
    fn cache_provide_value_with_marker<Out>(
        &self,
        receive: impl FnOnce(VK::Value<'_>) -> Out,
        marker: CacheCanProvideValue,
    ) -> Out;

    fn cache_provide_value<Out>(&self, receive: impl FnOnce(VK::Value<'_>) -> Out) -> Out
    where
        CacheCanProvideValue: sealed::ConstDefault,
    {
        self.cache_provide_value_with_marker(receive, sealed::ConstDefault::DEFAULT)
    }
}

impl<T: ?Sized, VK: ?Sized + ValueKind> CacheProvideValue<VK, sealed::CacheCanNotProvideValue>
    for T
{
    fn cache_provide_value_with_marker<Out>(
        &self,
        _: impl FnOnce(<VK as ValueKind>::Value<'_>) -> Out,
        marker: sealed::CacheCanNotProvideValue,
    ) -> Out {
        match marker {}
    }
}

impl<T: 'static + Copy> CacheProvideValue<KindOfOwned<T>> for T {
    fn cache_provide_value_with_marker<Out>(
        &self,
        receive: impl FnOnce(<KindOfOwned<T> as ValueKind>::Value<'_>) -> Out,
        _: CacheCanProvideValue,
    ) -> Out {
        receive(*self)
    }
}

impl<T: Borrow<U>, U: ?Sized + 'static> CacheProvideValue<KindOfTempRef<U>> for T {
    fn cache_provide_value_with_marker<Out>(
        &self,
        receive: impl FnOnce(<KindOfTempRef<U> as ValueKind>::Value<'_>) -> Out,
        _: CacheCanProvideValue,
    ) -> Out {
        receive(TempRef(self.borrow()))
    }
}

impl<T: ?Sized + 'static + ToOwned> CacheProvideValue<KindOfStaticOrTempRef<T>>
    for Cow<'static, T>
{
    fn cache_provide_value_with_marker<Out>(
        &self,
        receive: impl FnOnce(<KindOfStaticOrTempRef<T> as ValueKind>::Value<'_>) -> Out,
        _: CacheCanProvideValue,
    ) -> Out {
        receive(From::from(self))
    }
}
