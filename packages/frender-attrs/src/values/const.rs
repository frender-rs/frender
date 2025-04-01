use frender_common::define_phantom_wrapper;

use crate::Attributes;

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;

pub trait HasConstAttributes {
    type Attributes: imp::ConstAttributesAttributes;
    const ATTRIBUTES: Self::Attributes;
}

define_phantom_wrapper!(
    #[always_derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ConstAttributes<T: ?Sized + HasConstAttributes>;
);

impl<T: ?Sized + HasConstAttributes> crate::sealed::Attributes for ConstAttributes<T> {}
impl<T: ?Sized + HasConstAttributes> Attributes for ConstAttributes<T> {}

mod imp {
    use crate::parser::attrs::AttributesForRendering;

    #[cfg(feature = "csr")]
    use super::csr::CsrConstAttributes;
    #[cfg(feature = "ssr")]
    use super::ssr::SsrConstAttributes;

    #[cfg(feature = "csr")]
    #[cfg(feature = "ssr")]
    pub trait ConstAttributesAttributes: CsrConstAttributes + SsrConstAttributes {}

    #[cfg(feature = "csr")]
    #[cfg(not(feature = "ssr"))]
    pub trait ConstAttributesAttributes: CsrConstAttributes {}

    #[cfg(not(feature = "csr"))]
    #[cfg(feature = "ssr")]
    pub trait ConstAttributesAttributes: SsrConstAttributes {}

    #[cfg(not(feature = "csr"))]
    #[cfg(not(feature = "ssr"))]
    pub trait ConstAttributesAttributes {}

    impl<const ATTRS: usize, const SSR_STRING_CAP: usize> ConstAttributesAttributes
        for AttributesForRendering<'_, ATTRS, SSR_STRING_CAP>
    {
    }
}

#[macro_export]
macro_rules! impl_HasConstAttributes_for {
    (impl <$(__)?> $($rest:tt)*) => {
        $crate::impl_HasConstAttributes_for! {
            impl $($rest)*
        }
    };
    (
        impl $for_ty:ty {
            const $NAME:tt: _ = $const_expr:expr;
        }
    ) => {
        $crate::impl_HasConstAttributes_for! {
            impl $for_ty {
                const $NAME: &'static $crate::values::r#const::__private::str = $const_expr;
            }
        }
    };
    (
        impl $for_ty:ty {
            const $NAME:tt: $str_ty:ty = $const_expr:expr;
        }
    ) => {
        const _: () = {
            $crate::__expand_if_underscore_else! { $NAME {} {
                impl $for_ty {
                    const $NAME: $str_ty = $const_expr;
                }
            }}

            enum __FrenderConstValueMarker {}

            impl $crate::values::r#const::__private::HasConstValue for __FrenderConstValueMarker {
                type Value = $str_ty;
                const VALUE: Self::Value = $crate::__expand_if_underscore_else!($NAME {
                    $const_expr
                } {
                    <$for_ty>::$NAME
                });
            }

            const __FRENDER_CONST_INFO: $crate::values::r#const::__private::Info =
                $crate::values::r#const::__private::into_info::<__FrenderConstValueMarker>();
            type __FrenderConstInfo = $crate::values::r#const::__private::ConstInfo<
                { __FRENDER_CONST_INFO.0[0] },
                { __FRENDER_CONST_INFO.0[1] },
                { __FRENDER_CONST_INFO.0[2] },
                { __FRENDER_CONST_INFO.0[3] },
            >;



            impl $crate::values::r#const::HasConstAttributes for $for_ty {
                type Attributes =
                    $crate::values::r#const::__private::IntoAttributes<$str_ty, __FrenderConstInfo>;
                const ATTRIBUTES: Self::Attributes =
                    $crate::values::r#const::__private::into_attributes::<
                        __FrenderConstValueMarker,
                        __FrenderConstInfo,
                    >();
            }
        };
    };
}

#[doc(hidden)]
pub mod __private {
    pub use str;

    pub use super::expr::{
        into_attributes, into_info, ConstInfo, HasConstValue, Info, IntoAttributes,
    };

    #[doc(hidden)]
    #[macro_export]
    macro_rules! __expand_if_underscore_else {
        (_     {$($then:tt)*}   $else:tt   ) => {$($then)*};
        ($t:tt    $then:tt   {$($else:tt)*}) => {$($else)*};
    }
}

mod expr {
    use std::marker::PhantomData;

    use crate::parser::attrs::{
        parse_str, parse_str_with_ssr, AttributesForRendering, AttributesWithInfo,
    };

    pub trait HasConstValue {
        type Value;
        const VALUE: Self::Value;
    }

    const INFO_LEN: usize = 4;
    pub struct Info(pub [usize; INFO_LEN]);

    pub enum ConstInfo<
        const INFO_0: usize = 0,
        const INFO_1: usize = 0,
        const INFO_2: usize = 0,
        const INFO_3: usize = 0,
    > {}

    impl Info {
        const fn new() -> Self {
            Self([0; INFO_LEN])
        }
    }

    pub trait ConstIntoInfo {
        type IntoInfo<M: ?Sized + HasConstValue<Value = Self>>: ?Sized + HasConstValue<Value = Info>;
    }

    impl<'a> ConstIntoInfo for &'a str {
        type IntoInfo<M: ?Sized + HasConstValue<Value = Self>> = IntoInfo<Self, M>;
    }

    pub struct IntoInfo<V, M: ?Sized + HasConstValue<Value = V>>(Never, PhantomData<M>);

    impl<'a, M: ?Sized + HasConstValue<Value = &'a str>> HasConstValue for IntoInfo<&'a str, M> {
        type Value = Info;
        const VALUE: Self::Value = parse_str_as_info(M::VALUE);
    }

    pub trait ConstIntoAttributesWithInfo<Info> {
        type Attributes: super::imp::ConstAttributesAttributes;
        type IntoAttributes<M: ?Sized + HasConstValue<Value = Self>>: ?Sized
            + HasConstValue<Value = Self::Attributes>;
    }

    const fn parse_str_as_info(s: &str) -> Info {
        let parsed = parse_str::<0, 0, 0>(s);

        let mut info = Info::new();

        info.0[0] = parsed.len();
        info.0[1] = parsed.max_csr_cap();
        info.0[2] = parsed.max_ssr_cap();
        info.0[3] = parsed.ssr_string_len();

        info
    }

    impl<
            'a,
            const ATTRS: usize,
            const CSR: usize,
            const SSR: usize,
            const SSR_STRING_CAP: usize,
        > ConstIntoAttributesWithInfo<ConstInfo<ATTRS, CSR, SSR, SSR_STRING_CAP>> for &'a str
    {
        type Attributes = AttributesForRendering<'a, ATTRS, SSR_STRING_CAP>;
        type IntoAttributes<MarkerV: ?Sized + HasConstValue<Value = &'a str>> =
            IntoAttributesWithInfo<Self, MarkerV, ConstInfo<ATTRS, CSR, SSR, SSR_STRING_CAP>>;
    }

    impl<
            'a,
            const ATTRS: usize,
            const CSR: usize,
            const SSR: usize,
            const SSR_STRING_CAP: usize,
            MarkerV: ?Sized + HasConstValue<Value = &'a str>,
        > HasConstValue
        for IntoAttributesWithInfo<&'a str, MarkerV, ConstInfo<ATTRS, CSR, SSR, SSR_STRING_CAP>>
    {
        type Value = AttributesForRendering<'a, ATTRS, SSR_STRING_CAP>;
        const VALUE: Self::Value = Self::ATTRIBUTES_WITH_INFO.to_attributes_for_rendering();
    }

    impl<
            'a,
            const ATTRS: usize,
            const CSR: usize,
            const SSR: usize,
            const SSR_STRING_CAP: usize,
            MarkerV: ?Sized + HasConstValue<Value = &'a str>,
        > IntoAttributesWithInfo<&'a str, MarkerV, ConstInfo<ATTRS, CSR, SSR, SSR_STRING_CAP>>
    {
        const ATTRIBUTES_WITH_INFO: AttributesWithInfo<'a, ATTRS, CSR, SSR, SSR_STRING_CAP> =
            parse_str_with_ssr::<ATTRS, CSR, SSR, SSR_STRING_CAP>(MarkerV::VALUE);
    }

    enum Never {}
    pub struct IntoAttributesWithInfo<V, MarkerV: ?Sized + HasConstValue<Value = V>, Info>(
        Never,
        PhantomData<MarkerV>,
        PhantomData<Info>,
    );

    pub const fn into_info<M: ?Sized + HasConstValue>() -> Info
    where
        M::Value: ConstIntoInfo,
    {
        <<M::Value as ConstIntoInfo>::IntoInfo<M> as HasConstValue>::VALUE
    }

    pub type IntoAttributes<T, Info> = <T as ConstIntoAttributesWithInfo<Info>>::Attributes;
    pub const fn into_attributes<M: ?Sized + HasConstValue, Info>() -> IntoAttributes<M::Value, Info>
    where
        M::Value: ConstIntoAttributesWithInfo<Info>,
    {
        <<M::Value as ConstIntoAttributesWithInfo<Info>>::IntoAttributes<M> as HasConstValue>::VALUE
    }
}

mod exprs {
    #[cfg(feature = "csr")]
    mod csr;
    #[cfg(feature = "ssr")]
    mod ssr;
}

#[cfg(test)]
mod tests;
