macro_rules! type_prop {
    ($fn_name:ident ($value:ident : event![
        $event_trait_name:ident,
        $event_type_name:literal,
        $event_type_ident:ident,
        $event_type_listener_ident:ident $(,)?
    ]);) => {
        type $fn_name<V: frender_dom::MaybeHandleEvent<dyn $crate::dom::event::$event_trait_name> + 'static, BT: BehaviorTypeTrait>:
            UnpinnedRenderWithBehavior<BT> +
            PinnedRenderWithBehavior<BT>
        ;
    };
    ($fn_name:ident ($value:ident : attr_value![&$($maybe_ty:tt)*]) $fn_body_or_semi:tt) => {
        crate::macros::test::type_prop! {
            $fn_name ($value : attr_value![$($maybe_ty)*]) $fn_body_or_semi
        }
    };
    ($fn_name:ident ($value:ident : attr_value![$($maybe_ty:tt)*]) $fn_body_or_semi:tt) => {
        type $fn_name<V: frender_attr_value::AttrValue<$($maybe_ty)*>, BT: BehaviorTypeTrait>:
            UnpinnedRenderWithBehavior<BT>
        ;
    };
    ($fn_name:ident ($value:ident : children! $children:tt) $fn_body_or_semi:tt) => {
        // TODO: test children
    };
    ($fn_name:ident ($value:ident : bounds![$($bounds:tt)+]) $(;)? $({
        $(attr_name!($attr_name:expr);)?
        $(impl_with!($($impl_with:tt)*);)?
    })?) => {
        // TODO: test bounds
    };
    ($fn_name:ident ($value:ident : set_ref![$ty:ty]) $(;)? $({
        $(attr_name!($attr_name:expr);)?
        $(impl_with!($($impl_with:tt)*);)?
    })?) => {
        // TODO: test set_ref
    };
}

macro_rules! impl_prop {
    ($fn_name:ident ($value:ident : event![
        $event_trait_name:ident,
        $event_type_name:literal,
        $event_type_ident:ident,
        $event_type_listener_ident:ident $(,)?
    ]);) => {
        type $fn_name<V: frender_dom::MaybeHandleEvent<dyn $crate::dom::event::$event_trait_name> + 'static, BT: BehaviorTypeTrait> =
            props::$fn_name<V>
        ;
    };
    ($fn_name:ident ($value:ident : attr_value![&$($maybe_ty:tt)*]) $fn_body_or_semi:tt) => {
        crate::macros::test::impl_prop! {
            $fn_name ($value : attr_value![$($maybe_ty)*]) $fn_body_or_semi
        }
    };
    ($fn_name:ident ($value:ident : attr_value![$($maybe_ty:tt)*]) $fn_body_or_semi:tt) => {
        type $fn_name<V: frender_attr_value::AttrValue<$($maybe_ty)*>, BT: BehaviorTypeTrait> =
            props::$fn_name<V>
        ;
    };
    ($fn_name:ident ($value:ident : children! $children:tt) $fn_body_or_semi:tt) => {
        // TODO: test children
    };
    ($fn_name:ident ($value:ident : bounds![$($bounds:tt)+]) $(;)? $({
        $(attr_name!($attr_name:expr);)?
        $(impl_with!($($impl_with:tt)*);)?
    })?) => {
        // TODO: test bounds
    };
    ($fn_name:ident ($value:ident : set_ref![$ty:ty]) $(;)? $({
        $(attr_name!($attr_name:expr);)?
        $(impl_with!($($impl_with:tt)*);)?
    })?) => {
        // TODO: test set_ref
    };
}

macro_rules! test {
    (expand_item $expand_item:tt) => {
        crate::macros::expand_item_and_prepend_expanded! {
            $expand_item
            {
                #![cfg(feature = "components")]

                #![allow(non_camel_case_types)]
                // #![allow(non_snake_case)]
                // #![allow(unused_imports)]

                #[cfg(feature = "csr")]
                use crate::update_element::{UnpinnedRenderWithBehavior, PinnedRenderWithBehavior};

                use super::*;
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
        #[cfg(feature = "csr")]
        const _: () = {
            use super::{behavior_type_traits::$trait_name as BehaviorTypeTrait, props::$trait_name as props};
            trait Test {
                $(
                    crate::macros::test::type_prop! {
                        $fn_name $fn_args $fn_body_or_semi
                    }
                )*
            }

            impl Test for () {
                $(
                    crate::macros::test::impl_prop! {
                        $fn_name $fn_args $fn_body_or_semi
                    }
                )*
            }
        };
    };
}

pub(crate) use {impl_prop, test, type_prop};

#[cfg(feature = "components")]
#[cfg(feature = "csr")]
mod test_id {
    use frender_attr_value::AttrValue;

    use crate::update_element::UnpinnedRenderWithBehavior;

    const _: () = {
        trait Test {
            type id<V: AttrValue<str>, BT: crate::html::behavior_type_traits::Element>: UnpinnedRenderWithBehavior<BT>;
        }

        impl Test for () {
            type id<V: AttrValue<str>, BT: crate::html::behavior_type_traits::Element> = crate::html::props::Element::id<V>;
        }
    };
}
