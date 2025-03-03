use std::borrow::Borrow;

use frender_reactive_value::{
    impl_known_NonReactiveValueStr_v_0_1_0, impl_known_StaticBorrowStr_v_0_1_0,
    non_reactive::CachedNonReactiveValue, value_kind::KindOfTempRef,
};

trait KnownStaticStr: 'static + PartialEq + Borrow<str> {}
impl_known_StaticBorrowStr_v_0_1_0! {KnownStaticStr}

pub trait KnownNonReactive: CachedNonReactiveValue<KindOfTempRef<str>> {}

impl_known_NonReactiveValueStr_v_0_1_0!(
    trait StaticBorrowStr = KnownStaticStr;
    trait __ = KnownNonReactive;
);
