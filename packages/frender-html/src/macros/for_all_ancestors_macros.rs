macro_rules! define_for_all_ancestors_macro {
    ([$dollar:tt] $trait_name:ident $({$all_ancestors:ident})*) => {
        macro_rules! $trait_name {
            ([$dollar($call:tt)+]{$dollar($prepend:tt)*}{$dollar($append:tt)*}) => {
                $dollar($call)+ {
                    $dollar($prepend)*
                    {
                        $($all_ancestors)*
                    }
                    $dollar($append)*
                }
            };
        }

        pub(crate) use $trait_name;
    };
}

macro_rules! for_all_ancestors_macros {
    (expand_item $expand_item:tt) => { crate::macros::expand_item_simple! $expand_item };
    (
        extends($($extends:ident)*)
        $(special_super_traits($($($special_super_traits:ident),+ $(,)?)?))?
        vis($vis:vis)
        trait_name($trait_name:ident)
        $(trait_bounds $trait_bounds:tt)?
        $(define(
            $(tags = ($($tags:ident $({$($tag_info:tt)*})?),* $(,)?))?
            $(,)?
        ))?
        $(verbatim_trait_items($($verbatim_trait_items:tt)*))?
        $(impl_for_web(
            $(only_for_types!($($impl_for_web_only_for_types:ty),* $(,)?);)?
            $(verbatim_trait_items!($($verbatim_trait_items_impl_web:tt)*);)?
        ))?
        fns($(
            $(#$fn_attr:tt)*
            fn $fn_name:ident $fn_args:tt $fn_body_or_semi:tt
        )*)
    ) => {
        ::frender_common::expand! {
            // expand only when there is a tag
            if ($($($($tags)*)?)?) {
                crate::html::props_macros::$trait_name! { for_all_ancestors {
                    prepend { [$] $trait_name }
                    wrap {} prepend { crate::macros::for_all_ancestors_macros::define_for_all_ancestors_macro! }
                }}
            }
        }
    };
}

pub(crate) use {define_for_all_ancestors_macro, for_all_ancestors_macros};
