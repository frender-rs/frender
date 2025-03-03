#[macro_export]
macro_rules! impl_known_StaticBorrowStr_v_0_1_0 {
    ($StaticBorrowStr:path) => {
        const _: () = {
            use frender_reactive_value::non_reactive::__private::{str, Arc, Cow, Rc, String};

            impl $StaticBorrowStr for &'static str {}
            impl $StaticBorrowStr for String {}
            impl $StaticBorrowStr for Cow<'static, str> {}
            impl $StaticBorrowStr for Rc<str> {}
            impl $StaticBorrowStr for Arc<str> {}
        };
    };
}

#[macro_export]
macro_rules! impl_known_NonReactiveValueStr_v_0_1_0 {
    (
        trait StaticBorrowStr = $StaticBorrowStr:path;
        trait __ = $CachedNonReactiveValueStr:path;
    ) => {
        impl<T: $StaticBorrowStr> $CachedNonReactiveValueStr for T {}

        $crate::impl_known_NonReactiveValueStr_v_0_1_0! {$CachedNonReactiveValueStr}
    };
    ($CachedNonReactiveValueStr:path) => {
        const _: () = {
            use $crate::{
                non_reactive::{Uncached, UncachedNonReactiveValue, __private::str},
                static_or_temp_ref::StaticOrTempRef,
                temp_into_static::{IntoStaticCache, TempIntoStatic},
                temp_ref::TempRef,
                value_kind::KindOfTempRef,
            };

            impl $CachedNonReactiveValueStr for TempRef<'_, str> {}

            impl $CachedNonReactiveValueStr for StaticOrTempRef<'_, str> {}

            impl<T: IntoStaticCache<str>> $CachedNonReactiveValueStr for TempIntoStatic<T> {}

            impl<T: UncachedNonReactiveValue<KindOfTempRef<str>>> $CachedNonReactiveValueStr
                for Uncached<T>
            {
            }
        };
    };
}

#[macro_export]
macro_rules! impl_known_StaticOrIntoStaticStr_v_0_1_0 {
    (
        trait StaticBorrowStr = $StaticBorrowStr:path;
        trait __ = $StaticOrIntoStaticStr:path;
    ) => {
        const _: () = {
            use $crate::{
                non_reactive::{Uncached, UncachedNonReactiveValue, __private::str},
                static_or_temp_ref::StaticOrTempRef,
                temp_into_static::{IntoStaticCache, TempIntoStatic},
                value_kind::KindOfTempRef,
            };

            impl<T: $StaticBorrowStr> $StaticOrIntoStaticStr for T {}
        };

        $crate::impl_known_StaticOrIntoStaticStr_v_0_1_0! {$StaticOrIntoStaticStr}
    };
    ($StaticOrIntoStaticStr:path) => {
        const _: () = {
            use $crate::{
                non_reactive::{Uncached, UncachedNonReactiveValue, __private::str},
                static_or_temp_ref::StaticOrTempRef,
                temp_into_static::{IntoStatic, TempIntoStatic},
                value_kind::KindOfTempRef,
            };

            // impl $StaticOrIntoStaticStr for TempRef<'_, str> {}

            impl $StaticOrIntoStaticStr for StaticOrTempRef<'_, str> {}

            impl<T: IntoStatic<str>> $StaticOrIntoStaticStr for TempIntoStatic<T> {}

            // impl<T: $StaticOrIntoStaticStr> $StaticOrIntoStaticStr for Uncached<T> {}
        };
    };
}

#[macro_export]
macro_rules! impl_known_SsrStr_v_0_1_0 {
    (
        trait StaticBorrowStr = $StaticBorrowStr:path;
        trait __ = $SsrStr:path;
    ) => {
        const _: () = {
            use $crate::{
                non_reactive::{Uncached, UncachedNonReactiveValue, __private::str},
                static_or_temp_ref::StaticOrTempRef,
                temp_into_static::{IntoStaticCache, TempIntoStatic},
                value_kind::KindOfTempRef,
            };

            impl<T: $StaticBorrowStr> $SsrStr for T {}
        };

        $crate::impl_known_SsrStr_v_0_1_0! {$SsrStr}
    };
    ($SsrStr:path) => {
        const _: () = {
            use $crate::{
                non_reactive::{Uncached, UncachedNonReactiveValue, __private::str},
                static_or_temp_ref::StaticOrTempRef,
                temp_into_static::{IntoStatic, TempIntoStatic},
                temp_ref::TempRef,
                value_kind::KindOfTempRef,
            };

            impl $SsrStr for TempRef<'_, str> {}

            impl $SsrStr for StaticOrTempRef<'_, str> {}

            impl<T: IntoStatic<str>> $SsrStr for TempIntoStatic<T> {}

            impl<T: UncachedNonReactiveValue<KindOfTempRef<str>> + $crate::ssr::SsrStr> $SsrStr
                for Uncached<T>
            {
            }
        };
    };
}

#[macro_export]
macro_rules! impl_known_IsStr_v_0_1_0 {
    (
        trait BorrowStr = $BorrowStr:path;
        trait __ = $IsStr:path;
    ) => {
        impl<T: $BorrowStr> $IsStr for T {}

        $crate::impl_known_IsStr_v_0_1_0! {$IsStr}
    };
    ($IsStr:path) => {
        $crate::impl_known_IntoBorrowStr_v_0_1_0! {$IsStr}
        const _: () = {
            use $crate::{
                non_reactive::{Uncached, UncachedNonReactiveValue, __private::str},
                value_kind::KindOfTempRef,
            };

            impl<T: UncachedNonReactiveValue<KindOfTempRef<str>>> $IsStr for Uncached<T> {}
        };
    };
}

#[macro_export]
macro_rules! impl_known_IntoBorrowStr_v_0_1_0 {
    (
        trait BorrowStr = $BorrowStr:path;
        trait __ = $IntoBorrowStr:path;
    ) => {
        impl<T: $BorrowStr> $IntoBorrowStr for T {}

        $crate::impl_known_IntoBorrowStr_v_0_1_0! {$IntoBorrowStr}
    };
    ($IntoBorrowStr:path) => {
        const _: () = {
            use $crate::{
                non_reactive::__private::str,
                static_or_temp_ref::StaticOrTempRef,
                temp_into_static::{IntoStatic, TempIntoStatic},
                temp_ref::TempRef,
            };

            impl $IntoBorrowStr for TempRef<'_, str> {}

            impl $IntoBorrowStr for StaticOrTempRef<'_, str> {}

            impl<T: IntoStatic<str>> $IntoBorrowStr for TempIntoStatic<T> {}
        };
    };
}
