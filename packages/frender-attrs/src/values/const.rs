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
}

#[macro_export]
macro_rules! impl_HasConstAttributes_for {
    ($($imp:tt)*) => {
        $crate::values::r#const::__private::impl_HasConstValue_for! {
            const _: () = {
                trait __ = $crate::values::r#const::HasConstAttributes;
                const ATTRIBUTES: Self::Attributes;
                type For = $crate::values::r#const::__private::ForAttributes;
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
    pub use super::input::ForAttributes;
}

mod input;

mod imps;

#[cfg(test)]
mod tests;
