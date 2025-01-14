#[cfg(any(feature = "macros_not_expanded", feature = "ElementProxyAttrs"))]
macro_rules! expand_impl {
    (
        {$($fn_name:tt)*}
        {$name:tt}
        do $commands:tt
    ) => {
        ::frender_common::expand! {
            {$(
                $name<event_types::$fn_name> +
            )*} do $commands
        }
    };
    (
        {$($fn_name:tt)*}
        {$name:tt $generic:tt}
        do $commands:tt
    ) => {
        ::frender_common::expand! {
            {$(
                $name<$generic, event_types::$fn_name> +
            )*} do $commands
        }
    };
}

#[cfg(any(feature = "macros_not_expanded", feature = "ElementProxyAttrs"))]
pub(crate) use expand_impl;

#[cfg(feature = "macros_not_expanded")]
mod not_expanded;

#[cfg(feature = "macros_not_expanded")]
pub(crate) use not_expanded::*;
