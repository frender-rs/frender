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
pub mod __private {
    #[doc(hidden)]
    pub use str;

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
