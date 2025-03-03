#[cfg(feature = "csr")]
pub(crate) use self::csr::KnownCsrStr;
#[cfg(feature = "ssr")]
pub(crate) use self::ssr::KnownSsrStr;

use std::borrow::Borrow;

use frender_reactive_value::{
    impl_known_IntoBorrowStr_v_0_1_0, impl_known_IsStr_v_0_1_0, impl_known_StaticBorrowStr_v_0_1_0,
    impl_known_StaticOrIntoStaticStr_v_0_1_0, into_borrow_str::IntoBorrowStr,
    static_or_into_static_str::StaticOrIntoStaticStr,
};

#[cfg(feature = "ssr")]
use async_str_iter::IntoAsyncStrIterator;
#[cfg(feature = "csr")]
use frender_reactive_value::{non_reactive::CachedNonReactiveValue, value_kind::KindOfTempRef};

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

impl_known_IsStr_v_0_1_0!(
    trait BorrowStr = KnownStaticStr;
    trait __ = KnownStr;
);

pub(crate) trait KnownStaticOrIntoStaticStr: StaticOrIntoStaticStr + KnownStr {}

impl_known_StaticOrIntoStaticStr_v_0_1_0!(
    trait StaticBorrowStr = KnownStaticStr;
    trait __ = KnownStaticOrIntoStaticStr;
);

pub(crate) trait KnownIntoBorrowStr: IntoBorrowStr + KnownStr {}

impl_known_IntoBorrowStr_v_0_1_0!(
    trait BorrowStr = KnownStaticStr;
    trait __ = KnownIntoBorrowStr;
);

#[cfg(feature = "csr")]
mod csr {
    use frender_reactive_value::{
        impl_known_NonReactiveValueStr_v_0_1_0, non_reactive::CachedNonReactiveValue,
        value_kind::KindOfTempRef,
    };

    use crate::known::KnownStaticStr;

    use super::KnownStr;

    pub(crate) trait KnownCsrStr:
        KnownStr + CachedNonReactiveValue<KindOfTempRef<str>>
    {
    }

    impl_known_NonReactiveValueStr_v_0_1_0!(
        trait StaticBorrowStr = KnownStaticStr;
        trait __ = KnownCsrStr;
    );
}

#[cfg(feature = "ssr")]
mod ssr {
    use frender_reactive_value::{impl_known_SsrStr_v_0_1_0, ssr::SsrStr};

    use super::{KnownStaticStr, KnownStr};

    pub(crate) trait KnownSsrStr: KnownStr + SsrStr {}

    impl_known_SsrStr_v_0_1_0!(
        trait StaticBorrowStr = KnownStaticStr;
        trait __ = KnownSsrStr;
    );
}
