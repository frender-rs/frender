pub use frender_core;

mod ctx_and_state;
mod element;
mod hook_context;

pub use ctx_and_state::*;
pub use element::*;
pub use hook_context::*;

#[cfg(feature = "dom")]
pub use frender_dom;

#[cfg(feature = "ssr")]
pub use frender_ssr;

#[doc(hidden)]
pub mod __private {
    pub use hooks_core;
    pub use hooks_core::transform_hook_fn_body_as_closure;
    pub use syn_lite::parse_item_fn;

    pub use frender_core::{RenderState, UpdateRenderState};

    #[cfg(feature = "csr")]
    pub use frender_dom::Dom as csr;

    #[cfg(feature = "ssr")]
    pub use frender_ssr::AnySsrContext as ssr;
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_component_fn_options_parsed {
    (
        component_options! {
            ctxs! { $($ctxs:ident)* }
        }
        item_fn! {
            outer_attrs! { $($outer_attrs:tt)* }
            vis! { $vis:vis }
            sig! {
                ident! { $ident:ident }
                generics! {
                    params! { $($generics:tt)* }
                    impl_generics! { $($impl_generics:tt)* }
                    type_generics! { $($type_generics:tt)* }
                    params_name! $params_name:tt
                }
                paren_inputs! { $paren_inputs:tt }
                output! {}
                where_clause! { $($where_clause:tt)* }
            }
            inner_attrs! { $($inner_attrs:tt)* }
            stmts! $stmts:tt
        }
    ) => {
        $($outer_attrs)*
        #[allow(non_snake_case)]
        $vis fn $ident $paren_inputs -> $crate::Element![$($ctxs),*] {
            $($inner_attrs)*

            $crate::new_fn_hook_element $(:: $ctxs)* (
                $crate::__private::transform_hook_fn_body_as_closure! {
                    []
                    $stmts
                }
            )
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_component_fn_parse_options {
    (
        {
            #[component $(($($ctx_path:ident),* $(,)?))?]
            $($outer_attrs:tt)*
        }
        [$($item_fn_rest:tt)*]
    ) => {
        $crate::__impl_component_fn_options_parsed! {
            component_options! {
                ctxs! { $($($ctx_path)* )? }
            }
            item_fn! {
                outer_attrs! { $($outer_attrs)* }
                $($item_fn_rest)*
            }
        }
    };
    (
        $outer_attrs:tt
        [$($item_fn_rest:tt)*]
    ) => {
        $crate::__impl_component_fn_options_parsed! {
            component_options! {
                ctxs! {}
            }
            item_fn! {
                outer_attrs! $outer_attrs
                $($item_fn_rest)*
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_component_fn_item_fn_parsed {
    (
        item_fn! {
            outer_attrs! $outer_attrs:tt
            $($item_fn_rest:tt)*
        }
        rest! {}
    ) => {
        $crate::__impl_component_fn_parse_options! {
            $outer_attrs
            [ $($item_fn_rest)* ]
        }
    };
}

#[macro_export]
macro_rules! component_fn {
    (
        $($item_fn:tt)*
    ) => {
        $crate::__private::parse_item_fn! {
            { $($item_fn)* }
            => $crate::__impl_component_fn_item_fn_parsed!
        }
    };
}

#[macro_export]
macro_rules! Element {
    (
        $($p:ident ,)+
        $($rest:tt)*
    ) => {
        $crate::__impl_element_type! {
            [$($p),+]
            [$($p),+]
            $($rest)*
        }
    };
    (
        $($p:ident),+
        $($rest:tt)*
    ) => {
        $crate::__impl_element_type! {
            [$($p),+]
            [$($p),+]
            $($rest)*
        }
    };
    ($($rest:tt)*) => {
        $crate::__impl_element_type! {
            []
            []
            $($rest)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_element_type {
    ([][] $($rest:tt)*) => {
        $crate::__impl_element_type! {[ssr,csr][ssr,csr] $($rest)*}
    };
    ([ssr][$ssr:ident]) => {
        impl for<'ssr> $crate::__private::UpdateRenderState<
            $crate::__private::$ssr<'ssr>,
            State = impl $crate::__private::RenderState
        >
    };
    ([csr][$csr:ident]) => {
        impl $crate::__private::UpdateRenderState<
            $crate::__private::$csr,
            State = impl $crate::__private::RenderState
        >
    };
    ([ssr,csr][$ssr:ident,$csr:ident]) => {
        impl for<'ssr> $crate::__private::UpdateRenderState<
            $crate::__private::$ssr<'ssr>,
            State = impl $crate::__private::RenderState
        > + $crate::__private::UpdateRenderState<
            $crate::__private::$csr,
            State = impl $crate::__private::RenderState
        >
    };
    ([csr,ssr][$csr:ident,$ssr:ident] $($rest:tt)*) => {
        $crate::__impl_element_type! {[$ssr,$csr][$ssr,$csr] $($rest)*}
    };
}
