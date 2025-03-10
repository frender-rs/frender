#[doc(no_inline)]
pub use frender_const_expr::const_marker;

/// An inline expr of [`ConstDeclarationList<impl HasConstDeclarationList>`](type@crate::styles::constness::ConstDeclarationList).
#[doc(hidden)]
#[macro_export]
macro_rules! declaration_value_const {
    (const $($rest:tt)*) => {{
        enum HasConstDeclarationValueStr {}
        $crate::declaration_value::r#const! {
            #[const_marker(HasConstDeclarationNameStr)]
            const $($rest)*
        }
    }};
    (
        #[$const_marker:ident $const_marker_body:tt]
        const $s:tt
    ) => {
        $crate::declaration_value::r#const! {
            #[$const_marker $const_marker_body]
            const $s as _
        }
    };
    (
        #[$const_marker:ident $const_marker_body:tt]
        const $s:tt as $($const_ty:tt)*
    ) => {{
        const CONST_EXPR: $crate::declaration_value::__private::DeclarationValueConstValue::<
            $crate::declaration_value::const_marker::$const_marker!$const_marker_body
        > = {
            impl $crate::declaration_value::__private::HasConstValue for
            $crate::declaration_value::const_marker::$const_marker!$const_marker_body {
                type Value = $crate::__declaration_value_type![$($const_ty)*];
                const VALUE: Self::Value = $s;
            }

            $crate::declaration_value::__private::DeclarationValueConstValue::new_const_value()
        };

        CONST_EXPR
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __declaration_value_type {
    (_) => {
        &'static $crate::declaration_value::__private::str
    };
    ($Ty:ty) => {
        $Ty
    };
}

#[doc(inline)]
pub use declaration_value_const as r#const;

#[doc(hidden)]
pub mod __private {
    pub use crate::styles::constness::const_value::HasConstValue;
    pub use str;

    use crate::{declaration::DeclarationValue, styles::constness::const_value::ConstValue};

    pub type DeclarationValueConstValue<T> = DeclarationValue<ConstValue<T>>;
}
