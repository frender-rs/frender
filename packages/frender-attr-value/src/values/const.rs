use frender_common::define_phantom_wrapper;

define_phantom_wrapper!(
    #[always_derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ConstAttrValue<T: ?Sized + HasConstAttrValue>;
);

pub trait HasConstAttrValue {
    type AttrValue: value::ConstAttrValueValue;
    const ATTR_VALUE: Self::AttrValue;
}

#[macro_export]
macro_rules! impl_HasConstAttrValue_for {
    ($($imp:tt)*) => {
        $crate::values::r#const::__private::impl_HasConstValue_for! {
            const _: () = {
                trait __ = $crate::values::r#const::HasConstAttrValue;
                const ATTR_VALUE: Self::AttrValue;
                type For = $crate::values::r#const::__private::ForAttrValue;
            };
            $($imp)*
        }
    };
}

#[doc(hidden)]
pub mod __private {
    #[doc(hidden)]
    pub use frender_const_value::impl_HasConstValue_for;

    #[doc(hidden)]
    pub use super::input::ForAttrValue;
}

mod value {
    use crate::AttrValueKind;

    #[cfg(feature = "csr")]
    use super::csr::ConstAttrValueCsrValue;
    #[cfg(feature = "ssr")]
    use super::ssr::ConstAttrValueSsrValue;

    pub trait ConstAttrValueValue {}

    #[cfg(feature = "csr")]
    #[cfg(feature = "ssr")]
    pub trait ConstAttrValueValueOfKind<AK: AttrValueKind>:
        ConstAttrValueValue + ConstAttrValueCsrValue<AK> + ConstAttrValueSsrValue<AK>
    {
    }

    #[cfg(feature = "csr")]
    #[cfg(not(feature = "ssr"))]
    pub trait ConstAttrValueValueOfKind<AK: AttrValueKind>:
        ConstAttrValueValue + ConstAttrValueCsrValue<AK>
    {
    }

    #[cfg(not(feature = "csr"))]
    #[cfg(feature = "ssr")]
    pub trait ConstAttrValueValueOfKind<AK: AttrValueKind>:
        ConstAttrValueValue + ConstAttrValueSsrValue<AK>
    {
    }

    #[cfg(not(feature = "csr"))]
    #[cfg(not(feature = "ssr"))]
    pub trait ConstAttrValueValueOfKind<AK: AttrValueKind>: ConstAttrValueValue {}
}

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;

mod imps {
    mod double_quoted;
}

mod input;

#[cfg(test)]
mod tests;
