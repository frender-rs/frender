macro_rules! prop_markers {
    (expand_item {
        $expand_item:tt
        {$($item_body_expanded:tt)*}
    }) => {
        crate::macros::expand_item_simple! {
            $expand_item
            {
                #![allow(non_snake_case)]
                #![allow(non_camel_case_types)]
                #![allow(unused_imports)]

                use crate::intrinsic::{AllowAttributeName, Intrinsic, PropertyValue};

                $($item_body_expanded)*

                crate::macros::prop_markers::define_conflicted_names! { $expand_item }
            }
        }
    };
    (
        extends($($extends:ident)*)
        $(special_super_traits($($($special_super_traits:ident),+ $(,)?)?))?
        vis($vis:vis)
        trait_name($trait_name:ident)
        $(trait_bounds $trait_bounds:tt)?
        $(define $define:tt)?
        // $(define(
        //     Props: $Props:ident
        //     $(, components: ($($components:ident),* $(,)?))?
        //     $(,)?
        // ))?
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
        $vis mod $trait_name {
            ::frender_common::expand! {
                while (
                    $({$extends})*
                    $($($({$special_super_traits})+)?)?
                ) {
                    prepend { pub use super:: }
                    append { ::*; }
                }
            }
            $(
                pub enum $fn_name {}
            )*
        }
    };
}

macro_rules! define_conflicted_names {
    ((
        $(#$item_attrs:tt)*
        $item_vis:vis mod $item_name:ident {
            $mod_vis:vis mod $conflicted_names:ident {
                $(#!$mod_attrs:tt)*

                $(
                    $vis:vis enum $conflicted_name:ident {}
                )*
            }
        }
    )) => {
        $(
            impl<M: AllowAttributeName<self::$conflicted_names::$conflicted_name>, C, A, P> Intrinsic<M, C, A, P> {
                $vis fn $conflicted_name<T: PropertyValue<M::AttributeMarker>>(
                    self,
                    value: T,
                ) -> Intrinsic<M, C, (A, T::Property), P> {
                    Self::with_attribute_appended(self, T::wrapped_into_property(value))
                }
            }
        )*

        macro_rules! expand_if_conflicted_name_or_else {
            $(
                ($conflicted_name $_if:tt $_else:tt) => {
                    frender_common::expand! {$_if}
                };
            )*
            ($not_conflicted_name:ident $_if:tt $_else:tt) => {
                frender_common::expand! {$_else}
            };
        }

        pub(super) use expand_if_conflicted_name_or_else;
    };
}

pub(crate) use {define_conflicted_names, prop_markers};
