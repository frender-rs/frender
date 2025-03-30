use std::borrow::Borrow;

use frender_reactive_value::{
    impl_known_NonReactiveValueStr_v_0_1_0, impl_known_StaticBorrowStr_v_0_1_0,
    impl_known_UncachedNonReactiveValueStr_v_0_1_0,
};

#[cfg(feature = "csr")]
use frender_reactive_value::{
    non_reactive::{CachedNonReactiveValue, UncachedNonReactiveValue},
    value_kind::KindOfTempRef,
};
#[cfg(feature = "ssr")]
use {async_str_iter::IntoAsyncStrIterator, frender_reactive_value::ssr::SsrStr};

#[cfg(feature = "csr")]
#[cfg(feature = "ssr")]
pub(crate) trait KnownStr: CachedNonReactiveValue<KindOfTempRef<str>> + SsrStr {}

#[cfg(feature = "csr")]
#[cfg(not(feature = "ssr"))]
pub(crate) trait KnownStr: CachedNonReactiveValue<KindOfTempRef<str>> {}

#[cfg(not(feature = "csr"))]
#[cfg(feature = "ssr")]
pub(crate) trait KnownStr: SsrStr {}

#[cfg(not(feature = "csr"))]
#[cfg(not(feature = "ssr"))]
pub(crate) trait KnownStr {}

#[cfg(feature = "csr")]
#[cfg(feature = "ssr")]
trait KnownStaticStr:
    'static + Borrow<str> + IntoAsyncStrIterator + CachedNonReactiveValue<KindOfTempRef<str>>
{
}

#[cfg(feature = "csr")]
#[cfg(not(feature = "ssr"))]
trait KnownStaticStr: 'static + Borrow<str> + CachedNonReactiveValue<KindOfTempRef<str>> {}

#[cfg(feature = "ssr")]
#[cfg(not(feature = "csr"))]
trait KnownStaticStr: 'static + Borrow<str> + IntoAsyncStrIterator {}

#[cfg(not(feature = "ssr"))]
#[cfg(not(feature = "csr"))]
trait KnownStaticStr: 'static + Borrow<str> {}

impl_known_StaticBorrowStr_v_0_1_0!(KnownStaticStr);

impl_known_NonReactiveValueStr_v_0_1_0!(
    trait StaticBorrowStr = KnownStaticStr;
    trait UncachedNonReactiveValueStr = KnownUncachedStr;
    trait __ = KnownStr;
);

#[cfg(feature = "csr")]
#[cfg(feature = "ssr")]
trait KnownUncachedStr: UncachedNonReactiveValue<KindOfTempRef<str>> + SsrStr {}

#[cfg(feature = "csr")]
#[cfg(not(feature = "ssr"))]
trait KnownUncachedStr: UncachedNonReactiveValue<KindOfTempRef<str>> {}

#[cfg(not(feature = "csr"))]
#[cfg(feature = "ssr")]
trait KnownUncachedStr: SsrStr {}

#[cfg(not(feature = "csr"))]
#[cfg(not(feature = "ssr"))]
trait KnownUncachedStr {}

impl_known_UncachedNonReactiveValueStr_v_0_1_0! {KnownUncachedStr}
