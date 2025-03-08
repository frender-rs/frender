macro_rules! define_behavior_fn {
    ($fn_name:ident ($value:ident : event![
        $event_trait_name:ident,
        $event_type_name:literal,
        $event_type_ident:ident,
        $event_type_listener_ident:ident $(,)?
    ]);) => {
    };
    ($fn_name:ident ($value:ident : attr_value![$maybe_ty:ty]) {
        $(alias! $alias:tt;)?
        $(attr_name! $attr_name:tt;)?
        $(update_with! $update_with:tt;)?
    }) => {
        $(
            crate::macros::behaviors::define_behavior_fn_update_with! {
                update_with $update_with
                value($value)
                type($maybe_ty)
            }
        )?
    };
    ($fn_name:ident $fn_args:tt $fn_body_or_semi:tt) => {};
}

macro_rules! define_behavior_fn_update_with {
    (
        update_with($set_attribute_ident:ident $(, $(web_sys_name = $web_sys_name:ident $(,)?)?)? )
        value($value:ident)
        type($maybe_ty:ty)
    ) => {
        fn $set_attribute_ident(&mut self, renderer: &mut Renderer, $value: $maybe_ty);
    };
    (
        update_with($set_attribute_ident:ident, custom_type!($custom_type:ty) $(, impl_with! $impl_with:tt $(,)?)?)
        value($value:ident)
        type($maybe_ty:ty)
    ) => {
        fn $set_attribute_ident(&mut self, renderer: &mut Renderer, $value: $custom_type);
    };
}

macro_rules! behaviors {
    (expand_item $expand_item:tt) => {
        crate::macros::expand_item_and_prepend_expanded! {
            $expand_item
            {
                use frender_dom::csr::OnEvent;

                use super::{*, event_types};
            }
        }
    };
    (
        extends($($extends:ident)*)
        $(special_super_traits($($($special_super_traits:ident),+ $(,)?)?))?
        vis($vis:vis)
        trait_name($trait_name:ident)
        $(trait_bounds($($trait_bounds:tt)*))?
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
        super::event_types_macros::$trait_name! {
            args { OnEvent Renderer }
            do {
                prepend {
                    $vis trait $trait_name<Renderer: ?Sized> :
                    $($extends<Renderer> +)*
                    $($($($special_super_traits<Renderer> +)+)?)?
                }
                append {
                    $($($trait_bounds)*)?
                    {
                        $($($verbatim_trait_items)*)?

                        $(crate::macros::behaviors::define_behavior_fn!{
                            $fn_name $fn_args $fn_body_or_semi
                        })*
                    }
                }
            }
        }
    };
}

pub(crate) use {behaviors, define_behavior_fn, define_behavior_fn_update_with};
