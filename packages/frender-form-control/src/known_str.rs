#[cfg(feature = "csr")]
pub(crate) use self::csr::KnownReactiveValueStr;

use frender_common::{define_trait_known_str, reactive_value::non_reactive::Uncached};

define_trait_known_str!(
    pub(crate) trait IsNonReactiveStr = KnownIsNonReactiveStr;
    pub(crate) trait Ssr = KnownSsrStr;
    pub(crate) trait Csr = KnownCsrStr;
);

pub(crate) trait KnownIsNonReactiveStrOrUncached {}

impl<T: KnownIsNonReactiveStr> KnownIsNonReactiveStrOrUncached for T {}
impl<T> KnownIsNonReactiveStrOrUncached for Uncached<T> {}

#[cfg(feature = "csr")]
mod csr {
    use frender_common::reactive_value::{
        non_reactive::{Uncached, UncachedNonReactiveValue},
        ReactiveValue,
    };

    use super::{KnownCsrStr, KnownIsNonReactiveStrOrUncached};

    pub(crate) trait KnownReactiveValueStr:
        KnownIsNonReactiveStrOrUncached + ReactiveValue<str>
    {
    }

    impl<T: KnownCsrStr> KnownReactiveValueStr for T {}
    impl<T: UncachedNonReactiveValue<str>> KnownReactiveValueStr for Uncached<T> {}
}
