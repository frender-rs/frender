#[macro_export]
macro_rules! attr_value {
    ($($t:tt)+) => {
        $crate::attr_value::syntax::one!(
            @{$crate::attr_value::syntax}
            {$($t)+}
        )
    };
}

#[doc(no_inline)]
pub use attr_value as one;

/// An inline expr of [`ConstAttrValue<impl HasConstAttrValue>`](type@crate::values::r#const::ConstAttrValue).
#[doc(hidden)]
#[macro_export]
macro_rules! attr_value_const {
    ($($t:tt)*) => {
        $crate::r#attr_value::__private::r#const! {
            #[const_impl_mod($crate::r#attr_value::__private::const_impl)]
            $($t)*
        }
    };
}

#[doc(inline)]
pub use attr_value_const as r#const;

#[doc(hidden)]
pub mod syntax {
    pub use frender_const_expr::syntax::*;

    pub mod parsed {
        #[doc(no_inline)]
        pub use frender_const_expr::syntax::parsed::*;

        #[doc(no_inline)]
        pub use crate::values::{EitherAttrValue as Either, Never};

        #[doc(no_inline)]
        pub use super::super::r#const;
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! __attr_value_syntax_empty {
        (@{$($with:tt)*} #{$($attr:tt)*} {()} $braced:tt) => {
            $($attr)*
            $crate::__disallow_empty! $braced
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! __disallow_empty {
        (("attr_value!() doesn't support empty syntax because empty could mean absent or empty string")) => {
            $crate::attr_value::__private::compile_error!("")
        };
    }

    pub use __attr_value_syntax_empty as empty;

    pub mod macros {
        #[doc(hidden)]
        #[macro_export]
        #[deprecated = "`attr_value!(verbatim!(expr))` is equivalent to `expr`"]
        macro_rules! __attr_value_syntax_macros_verbatim {
            ($e:expr) => {
                $e
            };
        }

        #[doc(hidden)]
        #[macro_export]
        #[deprecated = "`attr_value!(attr_value!(..))` is equivalent to `attr_value!(..)`"]
        macro_rules! __attr_value_syntax_macros_attr_value {
            ($($t:tt)*) => {
                $crate::attr_value! {$($t)*}
            };
        }

        #[allow(deprecated)]
        #[doc(inline)]
        pub use {
            __attr_value_syntax_macros_attr_value as attr_value,
            __attr_value_syntax_macros_verbatim as verbatim,
        };
    }
}

#[doc(hidden)]
pub mod __private {
    #[doc(hidden)]
    pub use {::core::compile_error, str};

    #[doc(hidden)]
    pub use frender_const_expr::r#const;
    #[doc(hidden)]
    pub mod const_impl {
        pub use crate::{
            impl_HasConstAttrValue_for as impl_marker_for,
            values::r#const::ConstAttrValue as ConstValue,
        };
    }
}

#[cfg(test)]
mod tests;

#[cfg(any(test, doctest))]
mod doc_tests;
