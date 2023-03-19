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
    pub use syn_lite::{expand_or, parse_item_fn};

    pub use frender_core::{RenderState, UpdateRenderState};

    #[cfg(feature = "csr")]
    pub use frender_dom::Dom as csr;

    pub mod main {
        #[cfg(all(feature = "csr", feature = "spawn"))]
        pub use frender_dom::spawn_mount_to_dom_element;
    }

    #[cfg(feature = "ssr")]
    pub use frender_ssr::AnySsrContext as ssr;
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_unexpected {
    () => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_main_fn {
    (
        vis! { $vis:vis }
        component_fn_name! { $name:ident }
        main! {
            $main:ident
            $((
                get_dom_element
                $(( $($get_dom_element:tt)* ))?
                $( = $get_dom_element_id:literal )?
                $(,)?
            ))?
            $(= $element_id:literal)?
        }
    ) => {
        $vis fn $main () {
            $crate::__private::main::spawn_mount_to_dom_element(
                $name(),
                $crate::__private::expand_or!(
                    [
                        $($($get_dom_element)*)?
                        $($get_dom_element_id)?
                        $($element_id)?
                    ]
                    "frender-root"
                )
            )
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_component_fn_options_parsed {
    (
        component_options! {
            ctxs! { $($ctxs:ident)* }
            mains! { $($mains:tt)* }
            other_items! { $($other_items:tt)* }
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
                output! { $(-> $output_ty:ty)? }
                where_clause! { $($where_clause:tt)* }
            }
            inner_attrs! { $($inner_attrs:tt)* }
            stmts! $stmts:tt
        }
    ) => {
        $($outer_attrs)*
        #[allow(non_snake_case)]
        $vis fn $ident $paren_inputs ->
            $crate::__private::expand_or![[$($output_ty)?] $crate::Element![$($ctxs),*]]
        {
            $($inner_attrs)*

            $crate::new_fn_hook_element $(:: $ctxs)* (
                $crate::__private::transform_hook_fn_body_as_closure! {
                    []
                    $stmts
                }
            )
        }

        $(
            $crate::__impl_main_fn! {
                vis! { $vis }
                component_fn_name! { $ident }
                main! $mains
            }
        )*

        $($other_items)*
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_parse_component_options {
    // END
    (
        ($(,)?) ($(,)?) // inputs
        $options:tt
        $item_fn:tt
    ) => {
        $crate::__impl_component_fn_options_parsed! {
            component_options! $options
            item_fn!           $item_fn
        }
    };
    // , ...
    (
        (, $($inputs:tt)+) $_inputs:tt
        $options:tt
        $item_fn:tt
    ) => {
        $crate::__impl_parse_component_options! {
            ($($inputs)*) ($($inputs)*)
            $options
            $item_fn
        }
    };
    // main = "element-id" ...
    (
        ( main        =         $_main_id:literal $($_inputs:tt)*)
        ($main:ident $assign:tt $main_id:literal  $($inputs:tt)* )
        {
            ctxs! $ctxs:tt
            mains! { $($mains:tt)* }
            other_items! $other_items:tt
        }
        $item_fn:tt
    ) => {
        $crate::__impl_parse_component_options! {
            ($($inputs)*) ($($inputs)*)
            {
                ctxs! $ctxs
                mains! {
                    $($mains)*
                    { $main $assign $main_id }
                }
                other_items! $other_items
            }
            $item_fn
        }
    };
    // main() ...
    (
        (main ($($_main_options:tt)*) $($_inputs:tt)*)
        ($main:ident $main_options:tt $($inputs:tt)* )
        {
            ctxs! $ctxs:tt
            mains! { $($mains:tt)* }
            other_items! $other_items:tt
        }
        $item_fn:tt
    ) => {
        $crate::__impl_parse_component_options! {
            ($($inputs)*) ($($inputs)*)
            {
                ctxs! $ctxs
                mains! {
                    $($mains)*
                    { $main $main_options }
                }
                other_items! $other_items
            }
            $item_fn
        }
    };
    // main ...
    (
        ( main       $($_inputs:tt)*)
        ($main:ident $($inputs:tt)* )
        {
            ctxs! $ctxs:tt
            mains! { $($mains:tt)* }
            other_items! $other_items:tt
        }
        $item_fn:tt
    ) => {
        $crate::__impl_parse_component_options! {
            ($($inputs)*) ($($inputs)*)
            {
                ctxs! $ctxs
                mains! {
                    $($mains)*
                    { $main }
                }
                other_items! $other_items
            }
            $item_fn
        }
    };
    // $ctx_path ...
    (
        ($ctx_path:ident $($inputs:tt)*) $_inputs:tt
        {
            ctxs! { $($ctxs:tt)* }
            mains! $mains:tt
            other_items! $other_items:tt
        }
        $item_fn:tt
    ) => {
        $crate::__impl_parse_component_options! {
            ($($inputs)*) ($($inputs)*)
            {
                ctxs! { $($ctxs)* $ctx_path }
                mains! $mains
                other_items! $other_items
            }
            $item_fn
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_component_fn_parse_options {
    (
        {
            #[component $(())?]
            $($outer_attrs:tt)*
        }
        [$($item_fn_rest:tt)*]
    ) => {
        $crate::__impl_component_fn_options_parsed! {
            component_options! {
                ctxs! {}
                mains! {}
                other_items! {}
            }
            item_fn! {
                outer_attrs! { $($outer_attrs)* }
                $($item_fn_rest)*
            }
        }
    };
    (
        {
            #[component $component_options:tt]
            $($outer_attrs:tt)*
        }
        [$($item_fn_rest:tt)*]
    ) => {
        $crate::__impl_parse_component_options! {
            $component_options $component_options
            {
                ctxs! {}
                mains! {}
                other_items! {}
            }
            {
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
                mains! {}
                other_items! {}
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
