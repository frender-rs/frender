#[cfg(not(feature = "macros_not_expanded"))]
macro_rules! define_nothing {
    ($($t:tt)*) => {};
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
        update_with($set_attribute_ident:ident, custom_type!($custom_type:ty), impl_with! $impl_with:tt $(,)?)
        value($value:ident)
        type($maybe_ty:ty)
    ) => {
        fn $set_attribute_ident(&mut self, renderer: &mut Renderer, $value: $custom_type);
    };
}

macro_rules! impl_behavior_fn_update_with {
    (
        update_with($set_attribute_ident:ident $(, $(web_sys_name = $web_sys_name:ident $(,)?)?)? )
        value($value:ident)
        type($maybe_ty:ty)
        trait_name($trait_name:ident $($only_for_types:tt)?)
    ) => {
        fn $set_attribute_ident(&mut self, _: &mut Renderer, $value: $maybe_ty) {
            ::frender_common::expand! {
                { $($($web_sys_name)?)? } or ($set_attribute_ident)
                prepend(
                    ::frender_common::expand! { if ($($only_for_types)?) {
                        self.0
                    } else {
                        AsRef::<::web_sys::$trait_name>::as_ref(&self.0)
                    }}.
                )
                append(
                    ($value)
                )
            }

        }
    };
    (
        update_with($set_attribute_ident:ident, custom_type!($custom_type:ty), impl_with! $impl_with:tt $(,)?)
        value($value:ident)
        type($maybe_ty:ty)
        trait_name($trait_name:ident $($only_for_types:tt)?)
    ) => {
        fn $set_attribute_ident(&mut self, _: &mut Renderer, $value: $custom_type) {
            ::frender_common::expand! { if ($($only_for_types)?) {
                self.0
            } else {
                AsRef::<::web_sys::$trait_name>::as_ref(&self.0)
            }}.$set_attribute_ident($value)
        }
    };
}

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
            crate::macros::define_behavior_fn_update_with! {
                update_with $update_with
                value($value)
                type($maybe_ty)
            }
        )?
    };
    ($fn_name:ident $fn_args:tt $fn_body_or_semi:tt) => {};
}

macro_rules! impl_behavior_fn {
    ($fn_name:ident ($value:ident : event![
        $event_trait_name:ident,
        $event_type_name:literal,
        $event_type_ident:ident,
        $event_type_listener_ident:ident $(,)?
    ]); $trait_name:tt) => {
    };
    ($fn_name:ident ($value:ident : attr_value![$maybe_ty:ty]) {
        $(alias! $alias:tt;)?
        $(attr_name! $attr_name:tt;)?
        $(update_with! $update_with:tt;)?
    } $trait_name:tt) => {
        $(
            crate::macros::impl_behavior_fn_update_with! {
                update_with $update_with
                value($value)
                type($maybe_ty)
                trait_name $trait_name
            }
        )?
    };
    ($fn_name:ident $fn_args:tt $fn_body_or_semi:tt $trait_name:tt) => {};
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

                        $(crate::macros::define_behavior_fn!{
                            $fn_name $fn_args $fn_body_or_semi
                        })*
                    }
                }
            }
        }
    };
}

macro_rules! imp_element_proxy_attrs {
    (expand_item $expand_item:tt) => {
        crate::macros::expand_item_and_prepend_expanded! {
            $expand_item
            {
                use frender_dom::csr::OnEvent;
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
        self::event_types_macros::$trait_name! {
            args { OnEvent Renderer }
            do {
                prepend {
                    impl<
                        Renderer: ?Sized,
                        E: ?Sized + frender_dom::csr::behaviors::Element<Renderer>,
                    > self::behaviors::$trait_name<Renderer> for crate::ElementProxyAttrs<E>
                    where Self:
                        $(self::behaviors::$extends<Renderer> +)*
                        $($($(self::behaviors::$special_super_traits<Renderer> + )+ )?)?
                }
                append {
                    $($($trait_bounds)*)?
                    {
                        $(crate::element_proxy_attrs::csr::macros::impl_behavior_fn! {
                            $fn_name $fn_args $fn_body_or_semi ($trait_name)
                        })*
                    }
                }
            }
        }
    };
}

pub(crate) use imp_element_proxy_attrs;

macro_rules! imp_web {
    (expand_item $expand_item:tt) => {
        crate::macros::expand_item_and_prepend_expanded! {
            $expand_item
            {
                use crate::shims::prelude::*;
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
        // if `impl_for_web`
        ::frender_common::expand! { if ($( ! $($($verbatim_trait_items_impl_web)*)?)?) {
            ::frender_common::expand! { if ($($( ! $($impl_for_web_only_for_types)*)?)?) {
                ::frender_common::expand! { while ($($($({$impl_for_web_only_for_types})*)?)?) {
                    prepend(impl<Renderer: ?Sized + ::frender_dom::csr::web::Renderer> self::behaviors::$trait_name<Renderer> for ::frender_dom::csr::web::Node<)
                    append( > $($(where Self: $(self::behaviors::$special_super_traits<Renderer> + )+ )?)? {
                        $($($($verbatim_trait_items_impl_web)*)?)?

                        ::frender_common::expand! { while ($({$fn_name $fn_args $fn_body_or_semi})*) {
                            append( ($trait_name ($($($($impl_for_web_only_for_types)*)?)?)) )
                            wrap {}
                            prepend(crate::macros::impl_behavior_fn!)
                        }}
                    })
                }}
            } else {
                impl<
                    Renderer: ?Sized + ::frender_dom::csr::web::Renderer,
                    E: AsRef<::web_sys::$trait_name> + AsRef<::web_sys::EventTarget>
                > self::behaviors::$trait_name<Renderer> for ::frender_dom::csr::web::Node<E>
                where Self:
                    $(self::behaviors::$extends<Renderer> +)*
                    $($($(self::behaviors::$special_super_traits<Renderer> + )+ )?)?
                    $($($trait_bounds)*)?
                {
                    $($($($verbatim_trait_items_impl_web)*)?)?

                    $(crate::macros::impl_behavior_fn! {
                        $fn_name $fn_args $fn_body_or_semi ($trait_name)
                    })*
                }
            }}
        }}
    };
}

pub(crate) use imp_web;

macro_rules! behaviors_prelude {
    (expand_item $expand_item:tt) => {
        crate::macros::expand_item_and_prepend_expanded! {
            $expand_item
            {
                #![allow(non_snake_case)]
                #![allow(unused_imports)]
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
            $vis use super::super::behaviors::$trait_name as _;

            ::frender_common::expand! {
                while ($({$extends})* $($($({$special_super_traits})+)?)?) {
                    prepend( $vis use super:: )
                    append( ::*;)
                }
            }
        }
    };
}

macro_rules! behavior_type_traits {
    (expand_item $expand_item:tt) => {
        crate::macros::expand_item_and_prepend_expanded! {
            $expand_item
            {
                use frender_common::convert::IdentityAs;
                use frender_dom::csr::UiHandle;

                use crate::csr::behavior_type::OnEventType;
                use super::{event_types, behaviors};
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
        super::event_types_macros::$trait_name! {
            args { OnEventType }
            do {
                prepend {
                    $vis trait $trait_name:
                    crate::csr::behavior_type::UiHandleType +
                    $($extends +)*
                    $($($($special_super_traits +)+)?)?
                }
                append {
                    {
                        type $trait_name<Renderer: ?Sized + super::RenderHtml>: behaviors::$trait_name<Renderer>
                            + IdentityAs<Self::OfBehaviorType<Renderer>>
                            + IdentityAs<Self::UiHandle<Renderer>>
                            + UiHandle<
                                Renderer,
                                Unmounted = Self::UnmountedUiHandle<Renderer>,
                            >
                        ;
                    }
                }
            }
        }
    };
}

macro_rules! tag_custom_content_model {
    ({custom_content_model} {$($custom_content_model:tt)*} $or:tt) => {
        $($custom_content_model)*
    };
    ({} $custom_content_model:tt {$($or:tt)*}) => {
        $($or)*
    };
}

macro_rules! tag_and_props_markers {
    (expand_item $expand_item:tt) => {
        crate::macros::expand_item_and_prepend_expanded! {
            $expand_item
            {
                #![allow(non_camel_case_types)]
            }
        }
    };
    (
        extends($($extends:ident)*)
        $(special_super_traits($($special_super_traits:ident),* $(,)?))?
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
        pub struct $trait_name;

        $($($(
            pub struct $tags;
        )*)?)?
    };
}

macro_rules! props {
    (expand_item $expand_item:tt) => {
        crate::macros::expand_item_and_prepend_expanded! {
            $expand_item
            {
                #![allow(non_snake_case)]
                #![allow(non_camel_case_types)]
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
            ::frender_common::expand! { if ($($fn_name)*) {
                #[allow(unused_imports)]
                use super::super::*;

                use crate::intrinsic::Property;
                use super::super::prop_markers::$trait_name as _prop_markers;
            }}

            ::frender_common::expand! {
                while (
                    $({$extends})*
                    $($($({$special_super_traits})+)?)?
                ) {
                    prepend(
                        #[allow(unused_imports)]
                        $vis use super::
                    )
                    append( ::*; )
                }
            }

            $(
                crate::macros::parse_fn_args_as_bounds! { $fn_args do {
                    duplex_concat(
                        {
                            prepend(
                                // #[derive(Debug)]
                                pub struct $fn_name<V:
                            )
                            append(
                                >(pub V);
                            )
                        }
                        {
                            prepend(
                                impl<V:
                            )
                            append(
                                > Property for $fn_name<V> {
                                    type PropertyMarker = _prop_markers::$fn_name;
                                }
                            )
                        }
                    )
                }}
            )*
        }
    };
}

macro_rules! props_implementations {
    // ($($t:tt)*)=>{compile_error!{stringify!($($t)*)}};
    (expand_item $expand_item:tt) => { crate::macros::expand_item_simple! $expand_item };
    (
        extends($($extends:ident)*)
        $(special_super_traits($($($special_super_traits:ident),+ $(,)?)?))?
        vis($vis:vis)
        trait_name($trait_name:ident)
        $(trait_bounds $trait_bounds:tt)?
        $(define(
            $(tags = ($($tag:ident $({$($tag_info:tt)*})?),* $(,)?))?
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
        const _: () = {
            #[cfg(feature = "csr")]
            #[allow(unused_imports)]
            use frender_common::convert::FromMut as _;

            #[allow(unused_imports)]
            use self::{props::$trait_name as props, prop_markers::$trait_name as prop_markers};

            #[cfg(feature = "csr")]
            use self::behaviors_prelude::$trait_name::*;

            const _: () = {
                use crate::intrinsic::{TagOrPropsMarker, PropsMarker};
                #[allow(unused_imports)]
                use crate::intrinsic::TagMarker;

                use self::markers::{
                    $trait_name,
                    $($($($tag,)*)?)?
                };

                impl TagOrPropsMarker for $trait_name {}
                impl PropsMarker for $trait_name {}

                $($($(
                    impl TagOrPropsMarker for $tag {}
                    impl TagMarker for $tag {
                        type PropsMarker = $trait_name;
                    }
                )*)?)?
            };

            $(
                self::prop_markers::expand_if_conflicted_name_or_else! { $fn_name {
                    crate::macros::parse_fn_args_as_bounds! { $fn_args do {
                        prepend { impl<V: }
                        append {
                            > crate::intrinsic::PropertyValue<prop_markers::$fn_name> for V {
                                type Property = props::$fn_name<V>;

                                fn wrapped_into_property(this: V) -> Self::Property {
                                    props::$fn_name(this)
                                }
                            }
                        }
                    }}
                }{}}
            )*

            $(
                crate::macros::impl_attribute!{ $fn_name $fn_args $fn_body_or_semi $trait_name }
            )*
        };
    };
}

macro_rules! tag_implementations {
    (expand_item $expand_item:tt) => {
        crate::macros::expand_item_and_prepend_expanded! { $expand_item {
            use frender_dom::HasIntrinsicComponentTag;

            #[cfg(feature = "ssr")]
            use ::frender_ssr::html::tag::AssertTagName;
            #[cfg(feature = "csr")]
            use frender_dom::csr::UnmountedUiHandle;

            #[cfg(feature = "ssr")]
            use frender_dom::ssr::SsrComponentNormalElement;

            #[cfg(feature = "csr")]
            use crate::csr::{
                element::HtmlRenderContext,
                behavior_type::{BehaviorType, UiHandleType},
                component::CsrComponentNormalElement,
            };

            #[cfg(feature = "csr")]
            use self::behavior_type_traits::*;
        } }
    };
    (
        extends($($extends:ident)*)
        $(special_super_traits($($special_super_traits:ident),* $(,)?))?
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
        $($($(
            use self::markers::$tags;

            #[cfg(feature = "csr")]
            impl BehaviorType for $tags {
                type OfBehaviorType<Renderer: ?Sized + RenderHtml> = Renderer::$tags;
            }

            #[cfg(feature = "csr")]
            impl UiHandleType for $tags {
                type UnmountedUiHandle<Renderer: ?Sized + RenderHtml> = <Renderer::$tags as UiHandle<Renderer>>::Unmounted;
                type UiHandle<Renderer: ?Sized + RenderHtml> = Renderer::$tags;

                fn create_unmounted_ui_handle_of_type<R: ?Sized + RenderHtml>(renderer: &mut R) -> <Self::UiHandle<R> as UiHandle<R>>::Unmounted {
                    R::$tags(renderer)
                }

                // fn create_and_mount_ui_handle_of_type<Ctx: ?Sized + HtmlRenderContext>(render_context: &mut Ctx) -> Self::UiHandle<Ctx::Renderer> {
                //     let unmounted = <Ctx::Renderer as RenderHtml>::$tags(render_context.renderer_mut());
                //     render_context.map_mut_render_context(|render_context| unmounted.mount(render_context))
                // }
            }

            impl HasIntrinsicComponentTag for $tags {
                const INTRINSIC_COMPONENT_TAG: &'static str = stringify!($tags);
            }

            crate::macros::tag_custom_content_model! {{$($($tag_info)*)?}{}{
                #[cfg(feature = "ssr")]
                impl SsrComponentNormalElement for $tags {}
                #[cfg(feature = "csr")]
                impl CsrComponentNormalElement for $tags {}
            }}

            #[cfg(feature = "csr")]
            crate::macros::impl_BehaviorTypeTrait! {
                $tags {$trait_name}
            }

            #[cfg(feature = "csr")]
            crate::html::for_all_ancestors_macros::$trait_name! {
                [crate::macros::impl_BehaviorTypeTrait!]
                {$tags} // prepend
                {} // append
            }
        )*)?)?
    };
}

macro_rules! impl_BehaviorTypeTrait {
    ($tag:ident {$($BehaviorTypeTrait:ident)*}) => {$(
        impl $BehaviorTypeTrait for $tag {
            type $BehaviorTypeTrait<Renderer: ?Sized + RenderHtml> = Renderer::$tag;
        }
    )*};
}

macro_rules! on_event_implementations {
    (expand_item $expand_item:tt) => {
        #[cfg(feature = "csr")]
        const _: () = {
            use crate::csr::behavior_type::OnEventType;

            use self::{behavior_type_traits::*, event_types::*, RenderHtml};

            crate::macros::expand_item_simple! $expand_item
        };
    };
    (
        extends($($extends:ident)*)
        $(special_super_traits($($($special_super_traits:ident),+ $(,)?)?))?
        vis($vis:vis)
        trait_name($trait_name:ident)
        $(trait_bounds $trait_bounds:tt)?
        $(define(
            $(tags = ($($tag:ident $({$($tag_info:tt)*})?),* $(,)?))?
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
        $(
            crate::macros::impl_OnEventType! {
                $fn_name
                $fn_args
                $trait_name
            }
        )*
    };
}

macro_rules! impl_OnEventType {
    ($fn_name:ident ($value:ident : event! $event:tt) $trait_name:ident) => {
        impl<BT: $trait_name> OnEventType<$fn_name> for BT {
            type OnEvent<Renderer: ?Sized + RenderHtml> = <Self as $trait_name>::$trait_name<Renderer>;
        }
    };
    ($fn_name:ident $($fn_rest:tt)*) => {};
}

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

                crate::macros::define_conflicted_names! { $expand_item }
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

macro_rules! impl_HasConstAttrName {
    (
        fn_name($fn_name:ident)
        $(attr_name($attr_name:expr))?
    ) => {
        impl crate::has_const_attr_name::HasConstAttrName for prop_markers::$fn_name {
            const ATTR_NAME: &str = ::frender_common::expand!({$($attr_name)?} or (stringify!($fn_name)));
        }

        #[cfg(feature = "ssr")]
        impl crate::has_const_attr_name::HasConstAttrNameSsr for prop_markers::$fn_name {
            const ASSERT_SPACE_AND_HTML_ATTRIBUTE_NAME: frender_ssr::html::attr::AssertSpaceAndHtmlAttributeName<&'static str>
                = frender_ssr::html::attr::AssertSpaceAndHtmlAttributeName::new_from_str(::core::concat!(
                    " ",
                    ::frender_common::expand!({$($attr_name)?} or (stringify!($fn_name)))
                ));
        }
    };
}

macro_rules! impl_attribute {
    ($fn_name:ident ($value:ident : event![
        $event_trait_name:ident,
        $event_type_name:literal,
        $event_type_ident:ident,
        $event_type_listener_ident:ident $(,)?
    ]); $trait_name:tt) => {
        impl<
            H: frender_common::HandleEvent<dyn crate::dom::event::$event_trait_name> + 'static,
            F: frender_common::MaybeHandleEvent<dyn crate::dom::event::$event_trait_name, HandleEvent = H> + 'static,
        > crate::into_property::IntoProperty
            for props::$fn_name::<F>
        {
            type IntoProperty = crate::event_listener::Property<crate::html::event_types::$fn_name, F>;
            fn into_property(Self(this): Self) -> Self::IntoProperty {
                crate::event_listener::Property::new(this)
            }
        }
    };
    ($fn_name:ident ($value:ident : attr_value![$($maybe_ty:tt)*]) ; $trait_name:ident) => {
        crate::macros::impl_attribute! {$fn_name ($value : attr_value![$($maybe_ty)*]) {} $trait_name }
    };
    ($fn_name:ident ($value:ident : attr_value![&$($maybe_ty:tt)*]) $maybe:tt $trait_name:ident) => {
        crate::macros::impl_attribute! {
            $fn_name ($value : attr_value![$($maybe_ty)*]) $maybe $trait_name
            ref_value_kind(&)
        }
    };
    ($fn_name:ident ($value:ident : attr_value![$($maybe_ty:tt)*]) {
        $(alias! $alias:tt;)?
        $(attr_name!($attr_name:expr);)?
        $(update_with! $update_with:tt;)?
    } $trait_name:ident $(ref_value_kind($ref_value_kind:tt))?) => {
        impl<
            V: frender_attr_value::AttrValue<$($maybe_ty)*>,
        > crate::into_property::IntoProperty
            for props::$fn_name<V>
        {
            type IntoProperty = crate::attr_value::Property<prop_markers::$fn_name, V>;
            fn into_property(Self(this): Self) -> Self::IntoProperty {
                crate::attr_value::Property::new(this)
            }
        }

        crate::macros::impl_HasConstAttrName! {
            fn_name($fn_name)
            $(attr_name($attr_name))?
        }

        impl crate::attr_value::HasAttrValueKind for prop_markers::$fn_name {
            type AttrValueKind = $($maybe_ty)*;
        }

        #[cfg(feature = "csr")]
        crate::macros::impl_attr_value_for_prop_marker! {
            update_with($($update_with)?)
            prop_marker(prop_markers::$fn_name)
            trait_name($trait_name)
            value($value)
            ref_value_kind($($ref_value_kind)?)
            value_kind($($maybe_ty)*)
        }

        #[cfg(feature = "ssr")]
        impl<
            V: frender_attr_value::AttrValue<$($maybe_ty)*>,
        > crate::dom::ssr::IntoSpaceAndHtmlAttributesOrEmpty
            for props::$fn_name<V>
        {
            type SpaceAndHtmlAttributesOrEmpty = crate::attr_value::ssr::SpaceAndHtmlAttributesOrEmpty<V, $($maybe_ty)*>;

            fn into_space_and_html_attributes_or_empty(self) -> Self::SpaceAndHtmlAttributesOrEmpty {
                crate::attr_value::ssr::into_space_and_html_attributes_or_empty::<prop_markers::$fn_name, V>(self.0)
            }
        }

    };
    ($fn_name:ident ($value:ident : children! $children:tt) $fn_body_or_semi:tt $trait_name:ident) => {
        // children is not an attribute
    };
    ($fn_name:ident ($value:ident : bounds![$bounds:ident]) $(;)? $({
        $(attr_name!($attr_name:expr);)?
        $(impl_with!($($impl_with:tt)*);)?
    })? $trait_name:ident) => {
        crate::macros::impl_HasConstAttrName! {
            fn_name($fn_name)
            $($(attr_name($attr_name))?)?
        }

        impl<
            V: crate::impl_bounds::$bounds::Bounds,
        > crate::into_property::IntoProperty
            for props::$fn_name<V>
        {
            type IntoProperty = crate::impl_bounds::$bounds::Property<prop_markers::$fn_name, V>;
            fn into_property(Self(this): Self) -> Self::IntoProperty {
                Self::IntoProperty::new(this)
            }
        }

        #[cfg(feature = "ssr")]
        impl<
            V: crate::impl_bounds::$bounds::Bounds,
        > crate::dom::ssr::IntoSpaceAndHtmlAttributesOrEmpty
            for props::$fn_name<V>
        {
            type SpaceAndHtmlAttributesOrEmpty = crate::impl_bounds::$bounds::ssr::Output<V>;

            fn into_space_and_html_attributes_or_empty(self) -> Self::SpaceAndHtmlAttributesOrEmpty {
                crate::impl_bounds::$bounds::ssr::output::<prop_markers::$fn_name, _>(
                    crate::impl_bounds::$bounds::ssr::into_haevoe(
                        self.0
                    )
                )
            }
        }
    };
    // custom impl
    ($fn_name:ident ($value:ident : custom_with_bounds![impl $($bounds:tt)+]); $trait_name:tt) => {
        const _: () = {
            fn asserts_csr<
                V: $($bounds)+,
                ET: $crate::html::behavior_type_traits::$trait_name,
            >(v: V) -> impl $crate::UpdateElementNonReactive<ET> {
                super::attributes::$fn_name(v)
            }

            fn asserts_ssr<V: $($bounds)+>(v: V) -> impl $crate::dom::component::IntoSpaceAndHtmlAttributesOrEmpty {
                super::attributes::$fn_name(v)
            }
        };
    };
    ($fn_name:ident ($value:ident : set_ref![$ty:ty]) $(;)? $({
        $(attr_name!($attr_name:expr);)?
        $(impl_with!($($impl_with:tt)*);)?
    })? $trait_name:ident) => {
        crate::impl_bounds! {
            props::$fn_name(
                prop_marker(prop_markers::$fn_name),
                bounds as crate::impl_bounds::SetRef<$ty>,
                element as $trait_name,
                attr_name = ::frender_common::expand!({$($($attr_name)?)?} or (stringify!($fn_name))),
                $($($($impl_with)*)?)?
            )
        }
    };
}

macro_rules! dom_api_value_from_value {
    (
        value_kind(bool)
        value($value:expr)
    ) => {
        () = $value;
        true
    };
    (
        value_kind($value_kind:ty)
        value($value:expr)
    ) => {
        $value
    };
}

macro_rules! impl_attr_value_dom_api_for_prop_marker {
    (
        $(custom_type($custom_type:ty))?
        $(DomApiValue($DomApiValue:ty))?
        $(dom_api_value_from_value(|$dom_api_value_from_value:pat_param| $dom_api_value:expr))?
        update(|$element:pat_param, $renderer:pat_param $(,)?| $update:expr)
        prop_marker($prop_marker:ty)
        trait_name($trait_name:ident)
        value($value:ident)
        ref_value_kind($($ref_value_kind:tt)?)
        value_kind($($value_kind:tt)*)
    ) => {
        impl<BT: behavior_type_traits::$trait_name> crate::attr_value::csr::HasDomApi<BT> for $prop_marker {
            type DomApiValue<'a> = frender_common::expand![
                {$($DomApiValue)?}
                or ($($custom_type)?)
                or (
                    $($ref_value_kind 'a)?
                    $($value_kind)*
                )
            ];

            frender_common::expand! {
                {$(
                    fn dom_api_value_from_value($dom_api_value_from_value: <Self::AttrValueKind as frender_attr_value::csr::ValueKind>::Value<'_>) -> Self::DomApiValue<'_> {
                        $dom_api_value
                    }
                )?} or (
                    fn dom_api_value_from_value(value: <Self::AttrValueKind as frender_attr_value::csr::ValueKind>::Value<'_>) -> Self::DomApiValue<'_> {
                        crate::macros::dom_api_value_from_value! {
                            value_kind($($value_kind)*)
                            value(value)
                        }
                    }
                )
            }

            fn set_attribute_value<R: ?Sized + RenderHtml>(b: &mut BT::OfBehaviorType<R>, $renderer: &mut R, $value: Self::DomApiValue<'_>) {
                let $element = <BT::$trait_name<R>>::from_mut(b);
                $update
            }
        }
    };
}

macro_rules! impl_attr_value_for_prop_marker {
    (
        update_with()
        prop_marker($prop_marker:ty)
        trait_name($trait_name:ident)
        value($value:ident)
        ref_value_kind($($ref_value_kind:tt)?)
        value_kind($($value_kind:tt)*)
    ) => {
        impl crate::property_common::UseSpecRemoveAttrOfBehaviorType for $prop_marker {}
        impl<BT: behavior_type_traits::$trait_name> crate::property_common::HasSpecRemoveAttrOfBehaviorType<BT> for $prop_marker {
            type SpecRemoveAttrOfBehaviorType = crate::property_common::SpecRemoveAttrOfElementTypeWithAttrName<Self>;
        }

        impl crate::attr_value::csr::UseSpecUpdateAttrValueOfBehaviorType for $prop_marker {}
        impl<BT: behavior_type_traits::$trait_name> crate::attr_value::csr::HasSpecUpdateAttrValueOfBehaviorType<BT> for $prop_marker {
            type SpecUpdateAttrValueOfBehaviorType = crate::attr_value::csr::SpecUpdateAttrValueOfElementWithAttrName<Self>;
        }
    };
    (
        update_with(($set_attribute_ident:ident $(, $(web_sys_name = $web_sys_name:ident $(,)?)?)?))
        prop_marker($prop_marker:ty)
        trait_name($trait_name:ident)
        value($value:ident)
        ref_value_kind($($ref_value_kind:tt)?)
        value_kind($($value_kind:tt)*)
    ) => {
        crate::macros::impl_attr_value_for_prop_marker! {
            update_with((
                $set_attribute_ident,
                impl_with!(
                    update = |element, renderer| element.$set_attribute_ident(renderer, $value)
                ),
            ))
            prop_marker($prop_marker)
            trait_name($trait_name)
            value($value)
            ref_value_kind($($ref_value_kind)?)
            value_kind($($value_kind)*)
        }
    };
    (
        update_with((
            $set_attribute_ident:ident,
            $(
                custom_type!($custom_type:ty),
            )?
            impl_with!(
                $(DomApiValue![$DomApiValue:ty],)?
                $(dom_api_value_from_value = |$dom_api_value_from_value:pat_param| $dom_api_value:expr,)?
                update = |$element:pat_param, $renderer:pat_param $(,)?| $update:expr $(,)?
            ) $(,)?
        ))
        prop_marker($prop_marker:ty)
        trait_name($trait_name:ident)
        value($value:ident)
        ref_value_kind($($ref_value_kind:tt)?)
        value_kind($($value_kind:tt)*)
    ) => {
        impl crate::property_common::UseSpecRemoveAttrOfBehaviorType for $prop_marker {}
        impl<BT: behavior_type_traits::$trait_name> crate::property_common::HasSpecRemoveAttrOfBehaviorType<BT> for $prop_marker {
            type SpecRemoveAttrOfBehaviorType = crate::attr_value::csr::SpecRemoveAttrWithDomApi<Self>;
        }
        impl crate::attr_value::csr::UseSpecUpdateAttrValueOfBehaviorType for $prop_marker {}
        impl<BT: behavior_type_traits::$trait_name> crate::attr_value::csr::HasSpecUpdateAttrValueOfBehaviorType<BT> for $prop_marker {
            type SpecUpdateAttrValueOfBehaviorType = crate::attr_value::csr::SpecUpdateAttrWithDomApi<Self>;
        }

        crate::macros::impl_attr_value_dom_api_for_prop_marker! {
            $(custom_type($custom_type))?
            $(DomApiValue($DomApiValue))?
            $(dom_api_value_from_value(|$dom_api_value_from_value| $dom_api_value))?
            update(|$element, $renderer| $update)
            prop_marker($prop_marker)
            trait_name($trait_name)
            value($value)
            ref_value_kind($($ref_value_kind)?)
            value_kind($($value_kind)*)
        }
    };
    (
        update_with((
            $set_attribute_ident:ident,
            $(custom_type!($custom_type:ty),)?
            impl_with!(
                $(DomApiValue![$DomApiValue:ty],)?
                $(dom_api_value_from_value = |$dom_api_value_from_value:pat_param| $dom_api_value:expr,)?
                update = |$element:pat_param, $renderer:pat_param $(,)?| $update:expr
                , remove = |$element_remove:pat_param, $renderer_remove:pat_param $(,)?| $remove:expr
            ) $(,)?
        ))
        prop_marker($prop_marker:ty)
        trait_name($trait_name:ident)
        value($value:ident)
        ref_value_kind($($ref_value_kind:tt)?)
        value_kind($value_kind:ty)
    ) => {
        impl<BT: behavior_type_traits::$trait_name> crate::property_common::RemoveAttrOfBehaviorType<BT> for $prop_marker {
            fn remove_attr_of_behavior_type<R: ?Sized + RenderHtml>(
                //
                b: &mut BT::OfBehaviorType<R>,
                $renderer_remove: &mut R,
            ) {
                let $element_remove = <BT::$trait_name<R>>::from_mut(b);
                $remove
            }
        }
        impl crate::attr_value::csr::UseSpecUpdateAttrValueOfBehaviorType for $prop_marker {}
        impl<BT: behavior_type_traits::$trait_name> crate::attr_value::csr::HasSpecUpdateAttrValueOfBehaviorType<BT> for $prop_marker {
            type SpecUpdateAttrValueOfBehaviorType = crate::attr_value::csr::SpecUpdateAttrWithDomApi<Self>;
        }

        crate::macros::impl_attr_value_dom_api_for_prop_marker! {
            $(custom_type($custom_type))?
            $(DomApiValue($DomApiValue))?
            $(dom_api_value_from_value(|$dom_api_value_from_value| $dom_api_value))?
            update(|$element, $renderer| $update)
            prop_marker($prop_marker)
            trait_name($trait_name)
            value($value)
            ref_value_kind($($ref_value_kind)?)
            value_kind($value_kind)
        }
    };
}

macro_rules! RenderHtml {
    (expand_item {
        (
            $(#$item_attrs:tt)*
            $vis:vis $item_type:ident $item_name:ident {
                additional_bounds!($(dyn $($additional_bounds:tt)+)?);
                $($items:tt)*
            }
        )
        {$($item_body_expanded:tt)*}
    }) => {
        $(#$item_attrs)*
        $vis $item_type $item_name
        $(: $($additional_bounds)+)?
        {
            $($items)*
            $($item_body_expanded)*
        }
    };
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
        $($($(
            #[allow(non_camel_case_types)]
            type $tags: self::behaviors::$trait_name<Self> + UiHandle<Self, Unmounted: ProvideMutMounted<Self>> + 'static;
            fn $tags(&mut self) -> <Self::$tags as UiHandle<Self>>::Unmounted;
        )*)?)?
    };
}

macro_rules! expand_nested_traits {
    (
        // already expanded tokens
        {$($expanded:tt)*}
        // rest tokens
        {$({
            {
                extends $extends:tt
            }
            ($(
                $vis:vis trait $trait_name:ident {
                    $(special_super_traits! $special_super_traits:tt;)?
                    $(trait_bounds! $trait_bounds:tt;)?
                    $(define! $define:tt;)?

                    $(verbatim_trait_items! $verbatim_trait_items:tt;)?

                    $(impl_for_web! $impl_for_web:tt;)?

                    $(
                        $(#$fn_attr:tt)*
                        fn $fn_name:ident $fn_args:tt $fn_body_or_semi:tt
                    )*

                    $(sub_traits! $sub_traits:tt ;)?
                }
            )+)
        })+}
        do $commands:tt
    ) => {
        crate::macros::expand_nested_traits! {
            {
                $($expanded)*
                $($({
                    extends $extends
                    $(special_super_traits $special_super_traits)?
                    vis($vis)
                    trait_name($trait_name)
                    $(trait_bounds $trait_bounds)?
                    $(define $define)?
                    $(verbatim_trait_items $verbatim_trait_items)?
                    $(impl_for_web $impl_for_web)?
                    fns($(
                        $(#$fn_attr)*
                        fn $fn_name $fn_args $fn_body_or_semi
                    )*)
                })+)+
            } {$($($({
                {
                    extends($trait_name)
                }
                $sub_traits
            })?)+)+} do $commands
        }
    };
    ({ $($expanded:tt)* } {} do $commands:tt) => {
        $crate::expand! {
            { $($expanded)* }
            do $commands
        }
    };
}

macro_rules! define_item_and_traverse_traits {
    (
        $t:tt // {}
        $($macro_name:ident $macro_expand_item:tt)*
    ) => {
        $(
            crate::macros::$macro_name! {
                expand_item {
                    $macro_expand_item
                    {
                        $crate::expand! {
                            $t for_each {
                                wrap{}
                                prepend(crate::macros::$macro_name!)
                            }
                        }
                    }
                }
            }
        )*
    };
}

macro_rules! def_intrinsic_component_props {
    (
        mod __ {$(
            #[$item_macro:ident]
            $(# $item_attrs:tt)*
            $item_vis:vis $item_type:ident $item_name:tt $item_body_or_semi:tt
        )*}

        $($t:tt)*
    ) => {
        crate::macros::expand_nested_traits! {
            {}{{{extends()}($($t)*)}} do {
                wrap {} // { ... }
                append(
                    $($item_macro (
                        $(# $item_attrs)*
                        $item_vis $item_type $item_name $item_body_or_semi
                    ))*
                )
                wrap {} prepend( crate::macros::define_item_and_traverse_traits! )
            }
        }
    };
}

macro_rules! event_types {
    (expand_item $expand_item:tt) => { crate::macros::expand_item_simple! $expand_item };
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
        $(
            crate::macros::event_type! {
                $fn_name $fn_args $fn_body_or_semi $trait_name
            }
        )*
    };
}

macro_rules! event_type {
    ($fn_name:ident ($value:ident : event![
        $event_trait_name:ident,
        $event_type_name:literal,
        $event_type_ident:ident,
        $event_type_listener_ident:ident $(,)?
    ]); $trait_name:ident) => {
        pub use super::prop_markers::$trait_name::$fn_name;

        impl ::frender_dom::HasEventTypeName for $fn_name {
            const EVENT_TYPE_NAME: &'static str = $event_type_name;
        }

        #[cfg(feature = "web")]
        impl ::frender_events::web::JsCastEventType for $fn_name {
            type JsEventTarget = web_sys::$trait_name;
            type JsCastEvent = web_sys::$event_trait_name;

            fn js_event_as_event(event: &Self::JsCastEvent) -> &Self::Event {
                ::frender_events::web::Event::new_from_ref(event)
            }
        }

        ::frender_dom::event_types::type_traits_impl::$event_trait_name! {
            $fn_name,
            $trait_name,
            $event_trait_name
        }
    };
    ($fn_name:ident $fn_args:tt $fn_body_or_semi:tt $trait_name:tt) => {};
}

macro_rules! event_types_macros {
    (expand_item $expand_item:tt) => { crate::macros::expand_item_simple! $expand_item };
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
        crate::macros::event_names::expand_macro! {
            {$(
                { $fn_name $fn_args $fn_body_or_semi }
            )*}
            $trait_name
        }

        pub(crate) use $trait_name;
    };
}

macro_rules! unwrap_brace_concat {
    (
        {
            $({$($content:tt)*})*
        } then $commands:tt
    ) => {
        ::frender_common::expand! {
            {$($($content)*)*}
            do $commands
        }
    };
}

#[cfg(any(feature = "macros_not_expanded", test))]
pub(crate) mod define_props_macro;
#[cfg(feature = "macros_not_expanded")]
pub(crate) use define_props_macro::props_macros;

#[cfg(not(feature = "macros_not_expanded"))]
pub(crate) use define_nothing as props_macros;

#[cfg(feature = "macros_not_expanded")]
pub(crate) mod props_builders;
#[cfg(feature = "macros_not_expanded")]
pub(crate) use props_builders::props_builders;

#[cfg(not(feature = "macros_not_expanded"))]
pub(crate) use define_nothing as props_builders;

#[cfg(feature = "macros_not_expanded")]
pub(crate) mod for_all_ancestors_macros;
#[cfg(feature = "macros_not_expanded")]
pub(crate) use for_all_ancestors_macros::for_all_ancestors_macros;

#[cfg(not(feature = "macros_not_expanded"))]
pub(crate) use define_nothing as for_all_ancestors_macros;

macro_rules! components {
    (expand_item $expand_item:tt) => {
        crate::macros::expand_item_and_prepend_expanded! {
            $expand_item
            {
                #![allow(non_snake_case)]

                use frender_common::Empty;
            }
        }
    };
    (
        extends($($extends:ident)*)
        $(special_super_traits($($($special_super_traits:ident),+ $(,)?)?))?
        vis($vis:vis)
        trait_name($Props:ident)
        $(trait_bounds $trait_bounds:tt)?
        $(define(
            $(tags = ($($tag:ident $({$($tag_info:tt)*})?),* $(,)?))?
            $(,)?
        ))?
        $(verbatim_trait_items $verbatim_trait_items:tt)?
        $(impl_for_web $impl_for_web:tt)?
        fns $fns:tt
    ) => {
        $vis mod $Props {
            use crate::{intrinsic::Intrinsic, Empty};
            use super::super::markers;

            pub use markers::$Props as Marker;

            pub type Props<C, A, P> = Intrinsic<markers::$Props, C, A, P>;
            pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);

            pub mod tags {
                $($($(
                    pub use super::super::$tag;
                )*)?)?
            }
        }

        $($($(
            // The returned type is zero-sized, so it's likely to be optimized.
            $vis const fn $tag() -> $tag::Element<Empty, (), ()> {
                $tag::ELEMENT
            }

            $vis mod $tag {
                use crate::{intrinsic::Intrinsic, Empty};
                use super::super::markers;

                pub use markers::$tag as Marker;

                pub use super::$Props::{self as props, Props, PROPS};

                pub type Element<C, A, P> = Intrinsic<markers::$tag, C, A, P>;
                pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
            }
        )*)?)?
    };
}

/// `children` is excluded
macro_rules! extract_attr_builder_fn_names {
    ({children $fn_body_or_semi:tt} do $commands:tt) => {
        $crate::expand! { {} do $commands }
    };
    ({$fn_name:ident ;} do $commands:tt) => {
        $crate::expand! { { {$fn_name} } do $commands }
    };
    ({$fn_name:ident {
        alias!($($alias:ident),* $(,)?);
        $($other:ident ! $other_macro:tt;)*
    }} do $commands:tt) => {
        $crate::expand! { { {$fn_name} $({$alias})* } do $commands }
    };
    ({$fn_name:ident {
        $($other:ident ! $other_macro:tt;)*
    }} do $commands:tt) => {
        $crate::expand! { { {$fn_name} } do $commands }
    };
}

macro_rules! parse_fn_args_as_bounds {
    (($value:ident : event![
        $event_trait_name:ident,
        $event_type_name:literal,
        $event_type_ident:ident,
        $event_type_listener_ident:ident $(,)?
    ]) do $commands:tt) => {
        $crate::expand! {
            { frender_common::MaybeHandleEvent<dyn $crate::dom::event::$event_trait_name> + 'static }
            do $commands
        }
    };
    (($value:ident : attr_value![&$maybe_ty:ty]) do $commands:tt) => {
        $crate::expand! {
            { frender_attr_value::AttrValue::<$maybe_ty> }
            do $commands
        }
    };
    (($value:ident : attr_value![$maybe_ty:ty]) do $commands:tt) => {
        $crate::expand! {
            { frender_attr_value::AttrValue::<$maybe_ty> }
            do $commands
        }
    };
    (($value:ident : children![$(impl $($bounds:tt)+)?]) do $commands:tt) => {
        $crate::expand! {
            { $($($bounds)+)? }
            do $commands
        }
    };
    (($value:ident : children![impl $($bounds:tt)+]) do $commands:tt) => {
        $crate::expand! {
            { $($bounds)+ }
            do $commands
        }
    };
    (($value:ident : bounds![ $($mod_path_start:ident)? $(:: $mod_path:ident)*  $($(::)? <$($ty:ty),* $(,)?>)?]) do $commands:tt) => {
        $crate::expand! {
            { $($mod_path_start)? $(:: $mod_path)* ::Bounds $(::<$($ty),*>)? }
            do $commands
        }
    };
    (($value:ident : custom_with_bounds![impl $($bounds:tt)+]) do $commands:tt) => {
        $crate::expand! {
            { $($bounds)+ }
            do $commands
        }
    };
    (($value:ident : set_ref![ $ty:ty ]) do $commands:tt) => {
        $crate::expand! {
            { FnOnce(&$ty) }
            do $commands
        }
    };
    ($fn_args:tt do $commands:tt) => {
        compile_error! { stringify!($fn_args) }
    };
}

macro_rules! parse_fn_args_as_whether_pinned_state {
    (($value:ident : event![
        $event_trait_name:ident,
        $event_type_name:literal,
        $event_type_ident:ident,
        $event_type_listener_ident:ident $(,)?
    ]) $yes:tt $no:tt $commands:tt) => {
        ::frender_common::expand! {
            $yes do $commands
        }
    };
    ($fn_args:tt $yes:tt $no:tt $commands:tt) => {
        ::frender_common::expand! {
            $no do $commands
        }
    };
}

macro_rules! extract_only_children_or {
    ($t:tt $do:tt $or:tt) => {
        crate::macros::extract_only_children_or! {
            @ $t
            []
            { $do $or }
        }
    };
    (@{ { children $children:tt } $($t:tt)* } [$($resolved_children:tt)*] $do_or:tt) => {
        crate::macros::extract_only_children_or! {
            @{$($t)*}
            [$($resolved_children)* $children]
            $do_or
        }
    };
    (@{ { $other_name:ident $other:tt } { children $children:tt } $($t:tt)* } [$($resolved_children:tt)*] $do_or:tt) => {
        crate::macros::extract_only_children_or! {
            @{$($t)*}
            [$($resolved_children)* $children]
            $do_or
        }
    };
    (@{ { $other_name:ident $other:tt } { $other_name1:ident $other1:tt } { children $children:tt } $($t:tt)* } [$($resolved_children:tt)*] $do_or:tt) => {
        crate::macros::extract_only_children_or! {
            @{$($t)*}
            [$($resolved_children)* $children]
            $do_or
        }
    };
    (@{ { $other_name:ident $other:tt } { $other_name1:ident $other1:tt } { $other_name2:ident $other2:tt } $($t:tt)* } $resolved_children:tt $do_or:tt) => {
        crate::macros::extract_only_children_or! {
            @{$($t)*}
            $resolved_children
            $do_or
        }
    };
    (@{ { $other_name:ident $other:tt } { $other_name1:ident $other1:tt } $($t:tt)* } $resolved_children:tt $do_or:tt) => {
        crate::macros::extract_only_children_or! {
            @{$($t)*}
            $resolved_children
            $do_or
        }
    };
    (@{ { $other_name:ident $other:tt } $($t:tt)* } $resolved_children:tt $do_or:tt) => {
        crate::macros::extract_only_children_or! {
            @{$($t)*}
            $resolved_children
            $do_or
        }
    };
    (@{} [] { $do:tt $or:tt }) => {
        $crate::expand! $or
    };
    (@{} [$children:tt] { $do:tt $or:tt }) => {
        $crate::expand! { $children do $do }
    };
    (@{} $more_than_one_children:tt { $do:tt $or:tt }) => {
        ::core::compile_error! { "More than one `fn children` found in the same level" }
    };
}

macro_rules! expand_item_simple {
    (
        ($vis:vis expand_without_submodule ! {})
        { $($expanded:tt)* }
    ) => {
        $($expanded)*
    };
    (
        (
            $(#$item_attrs:tt)*
            $vis:vis expand_const_block ! {}
        )
        { $($expanded:tt)* }
    ) => {
        $(#$item_attrs)*
        const _: () = {
            $($expanded)*
        };
    };
    (
        (
            $(#$item_attrs:tt)*
            $vis:vis $item_type:ident $item_name:ident ;
        )
        $item_body_expanded:tt
    ) => {
        $(#$item_attrs)*
        $vis $item_type $item_name
        $item_body_expanded
    };
    (
        (
            $(#$item_attrs:tt)*
            $vis:vis $item_type:ident $item_name:ident {}
        )
        $item_body_expanded:tt
    ) => {
        $(#$item_attrs)*
        $vis $item_type $item_name
        $item_body_expanded
    };
    (
        (
            $(#$item_attrs:tt)*
            $vis:vis $item_type:ident $item_name:ident
            { $($item_body:tt)* }
        )
        {
            #!$inner_attr:tt
            $($item_body_expanded:tt)*
        }
    ) => {
        crate::macros::expand_item_simple! {
            (
                $(#$item_attrs)*
                $vis $item_type $item_name
                {
                    #!$inner_attr

                    $($item_body)*
                }
            )
            { $($item_body_expanded)* }
        }
    };
    (
        (
            $(#$item_attrs:tt)*
            $vis:vis $item_type:ident $item_name:ident
            { $($item_body:tt)* }
        )
        { $($item_body_expanded:tt)* }
    ) => {
        $(#$item_attrs)*
        $vis $item_type $item_name {
            $($item_body)*
            $($item_body_expanded)*
        }
    };
}

macro_rules! expand_item_and_prepend_expanded {
    (
        {
            $expand_item:tt
            { $($item_body_expanded:tt)* }
        }
        { $($prepend:tt)* }
    ) => {
        crate::macros::expand_item_simple! {
            $expand_item
            {
                $($prepend)*
                $($item_body_expanded)*
            }
        }
    };
}

pub(crate) use {
    behavior_type_traits, behaviors, behaviors_prelude, components, def_intrinsic_component_props, define_behavior_fn, define_behavior_fn_update_with, define_conflicted_names, define_item_and_traverse_traits,
    dom_api_value_from_value, event_type, event_types, event_types_macros, expand_item_and_prepend_expanded, expand_item_simple, expand_nested_traits, extract_attr_builder_fn_names, extract_only_children_or,
    impl_BehaviorTypeTrait, impl_HasConstAttrName, impl_OnEventType, impl_attr_value_dom_api_for_prop_marker, impl_attr_value_for_prop_marker, impl_attribute, impl_behavior_fn, impl_behavior_fn_update_with,
    on_event_implementations, parse_fn_args_as_bounds, parse_fn_args_as_whether_pinned_state, prop_markers, props, props_implementations, tag_and_props_markers, tag_custom_content_model, tag_implementations,
    unwrap_brace_concat, RenderHtml,
};

pub(crate) mod event_names;

pub(crate) mod test;
pub(crate) use test::test;

#[cfg(test)]
mod tests;
