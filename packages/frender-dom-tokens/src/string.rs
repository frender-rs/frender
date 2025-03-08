use frender_reactive_value::{
    impl_known_StaticBorrowStr_v_0_1_0,
    non_reactive::{Uncached, UncachedNonReactiveValue},
    static_or_temp_ref::StaticOrTempRef,
    temp_into_static::{IntoStaticCache, TempIntoStatic, UncachedTempIntoStatic},
    temp_ref::TempRef,
    value_kind::KindOfTempRef,
};

#[cfg(feature = "csr")]
use frender_reactive_value::non_reactive::CachedNonReactiveValue;
#[cfg(feature = "ssr")]
use frender_reactive_value::ssr::SsrStr;

use crate::IntoDomTokens;

#[cfg(feature = "csr")]
#[cfg(feature = "ssr")]
trait KnownNonReactiveStr: CachedNonReactiveValue<KindOfTempRef<str>> + SsrStr {}

#[cfg(feature = "csr")]
#[cfg(not(feature = "ssr"))]
trait KnownNonReactiveStr: CachedNonReactiveValue<KindOfTempRef<str>> {}

#[cfg(not(feature = "csr"))]
#[cfg(feature = "ssr")]
trait KnownNonReactiveStr: SsrStr {}

#[cfg(not(feature = "csr"))]
#[cfg(not(feature = "ssr"))]
trait KnownNonReactiveStr {}

#[cfg(feature = "ssr")]
trait KnownUncachedStr: UncachedNonReactiveValue<KindOfTempRef<str>> + SsrStr {}

#[cfg(not(feature = "ssr"))]
trait KnownUncachedStr: UncachedNonReactiveValue<KindOfTempRef<str>> {}

impl<T: UncachedTempIntoStatic<str>> KnownUncachedStr for TempIntoStatic<T> {}
impl KnownUncachedStr for TempRef<'_, str> {}

impl KnownNonReactiveStr for TempRef<'_, str> {}
impl KnownNonReactiveStr for StaticOrTempRef<'_, str> {}
impl<T: IntoStaticCache<str>> KnownNonReactiveStr for TempIntoStatic<T> {}
impl<T: KnownUncachedStr> KnownNonReactiveStr for Uncached<T> {}

impl_known_StaticBorrowStr_v_0_1_0!(KnownNonReactiveStr);

impl<T: KnownNonReactiveStr> IntoDomTokens for T {
    type IntoDomTokens = imp::NonReactiveStrIntoDomTokens<T>;

    fn into_dom_tokens(self) -> Self::IntoDomTokens {
        imp::NonReactiveStrIntoDomTokens(self)
    }
}

mod imp;
