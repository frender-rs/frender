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

macro_rules! parse_update_with {
    (match ($set_attribute_ident:ident $(, $(web_sys_name = $web_sys_name:ident $(,)?)?)?) {
        simple => $do_simple:tt
        impl_with => $do_impl_with:tt
    }) => {
        ::frender_common::expand! { { $set_attribute_ident } do $do_simple }
    };
    (match ($set_attribute_ident:ident, custom_type!($custom_type:ty), impl_with! $impl_with:tt $(,)?) {
        simple => $do_simple:tt
        impl_with => $do_impl_with:tt
    }) => {
        ::frender_common::expand! { { $set_attribute_ident $impl_with } do $do_impl_with }
    };
}

macro_rules! parse_impl_with {
    ($set_attribute_ident:ident (
        update = |$element:pat_param, $renderer:pat_param $(,)?| $update:expr
        $(, remove = $($t:tt)*)?
    ) as update(
        ValueType($ValueType:ty)
        value($value:pat_param)
        element_type($element_type:ty)
    )) => {
        |$element: &mut $element_type, $renderer: &mut _, _, $value: $ValueType| $update
    };
    ($set_attribute_ident:ident (
        update = |$_element:pat_param, $_renderer:pat_param $(,)?| $update:expr
        $(,)?
    ) as remove(
        element_type($element_type:ty)
    )) => {
        crate::dom::behaviors::Element::remove_attribute
    };
    ($set_attribute_ident:ident (
        update = |$_element:pat_param, $_renderer:pat_param $(,)?| $update:expr,
        remove = |$element:pat_param, $renderer:pat_param $(,)?| $remove:expr $(,)?
    ) as remove(
        element_type($element_type:ty)
    )) => {
        |$element: &mut $element_type, $renderer: &mut _, _| $remove
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
    (expand_item $expand_item:tt) => { crate::macros::expand_item_simple! $expand_item };
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
        $vis trait $trait_name<Renderer: ?Sized> :
            $($extends<Renderer> +)*
            $($($($special_super_traits<Renderer> +)+)?)?
            $($($trait_bounds)*)?
        {
            $($($verbatim_trait_items)*)?

            $(crate::macros::define_behavior_fn!{
                $fn_name $fn_args $fn_body_or_semi
            })*
        }

        #[cfg(feature="ElementProxyAttrs")]
        impl<
            Renderer: ?Sized,
            E: ?Sized + frender_dom::behaviors::Element<Renderer>,
        > $trait_name<Renderer> for crate::ElementProxyAttrs<E>
        where Self:
            $($extends<Renderer> +)*
            $($($($special_super_traits<Renderer> + )+ )?)?
            $($($trait_bounds)*)?
        {
            $(crate::element_proxy_attrs::macros::impl_behavior_fn! {
                $fn_name $fn_args $fn_body_or_semi ($trait_name)
            })*
        }

        // if `impl_for_web`
        #[cfg(feature = "web")]
        ::frender_common::expand! { if ($( ! $($($verbatim_trait_items_impl_web)*)?)?) {
            ::frender_common::expand! { if ($($( ! $($impl_for_web_only_for_types)*)?)?) {
                ::frender_common::expand! { while ($($($({$impl_for_web_only_for_types})*)?)?) {
                    prepend(impl<Renderer: ?Sized + ::frender_dom::csr::web::Renderer> $trait_name<Renderer> for ::frender_dom::csr::web::Node<)
                    append( > $($(where Self: $($special_super_traits<Renderer> + )+ )?)? {
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
                > $trait_name<Renderer> for ::frender_dom::csr::web::Node<E>
                where Self:
                    $($extends<Renderer> +)*
                    $($($($special_super_traits<Renderer> + )+ )?)?
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
        $vis trait $trait_name:
            crate::BehaviorType +
            $($extends +)*
            $($($($special_super_traits +)+)?)?
        {
            type $trait_name<Renderer: ?Sized + super::RenderHtml>: super::behaviors::$trait_name<Renderer>
                + ::frender_common::convert::IdentityAs<Self::NodeOfBehaviorType<Renderer>>
            ;
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
                use ::frender_ssr::html::tag::AssertTagName;
                use crate::{
                    dom::component::{HasIntrinsicComponentTag, SsrComponentNormalElement},
                    BehaviorType, CreateNode, CsrComponentNormalElement, RenderHtml,
                };
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
            #[allow(non_camel_case_types)]
            pub struct $tags;
        )*)?)?

        $($($(
            impl BehaviorType for $tags {
                type NodeOfBehaviorType<Renderer: ?Sized + RenderHtml> = Renderer::$tags;
            }

            impl HasIntrinsicComponentTag for $tags {
                const INTRINSIC_COMPONENT_TAG: &'static str = stringify!($tags);
                const ASSERT_TAG_NAME: AssertTagName<&'static str> =
                    AssertTagName::new_from_str(Self::INTRINSIC_COMPONENT_TAG);
            }
            impl CreateNode for $tags {
                fn create_node<R: super::RenderHtml + ?::core::marker::Sized>(renderer: &mut R) -> <Self as super::behavior_type_traits::Node>::Node<R> {
                    renderer.$tags()
                }
            }
            crate::macros::tag_custom_content_model! {{$($($tag_info)*)?}{}{
                impl SsrComponentNormalElement for $tags {}
                impl CsrComponentNormalElement for $tags {}
            }}

            crate::html::props_builders::$trait_name! { for_all_ancestors {
                prepend { {$trait_name} }
                for_each {
                    duplex_concat (
                        {
                            prepend {
                                impl super::behavior_type_traits::
                            }
                            append {
                                for $tags
                            }
                        }
                        {
                            prepend {
                                type
                            }
                            append {
                                <Renderer: ?Sized + RenderHtml> = Renderer::$tags;
                            }
                            wrap {}
                        }
                    )
                }
            }}
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

            #[allow(unused_imports)]
            use super::super::*;

            ::frender_common::expand! { if ($($fn_name)*) {
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
                                #[derive(Debug)]
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
            #[allow(unused_imports)]
            use self::{props::$trait_name as props, prop_markers::$trait_name as prop_markers};

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
                ($conflicted_name $if:tt $else:tt) => {
                    frender_common::expand! {$if}
                };
            )*
            ($not_conflicted_name:ident $if:tt $else:tt) => {
                frender_common::expand! {$else}
            };
        }

        pub(super) use expand_if_conflicted_name_or_else;
    };
}

macro_rules! impl_attribute {
    ($fn_name:ident ($value:ident : event![
        $event_trait_name:ident,
        $event_type_name:literal,
        $event_type_ident:ident,
        $event_type_listener_ident:ident $(,)?
    ]); $trait_name:tt) => {
        crate::impl_bounds! {
            props::$fn_name(
                prop_marker(prop_markers::$fn_name),
                #[event(self::event_type_helpers::$fn_name)]
                bounds as crate::impl_bounds::MaybeHandleEvent,
                element as $trait_name,
                attr_name = __,
            )
        }
    };
    ($fn_name:ident ($value:ident : attr_value![$($maybe_ty:tt)*]) ; $trait_name:ident) => {
        crate::macros::impl_attribute! {$fn_name ($value : attr_value![$($maybe_ty)*]) {} $trait_name }
    };
    // TODO: remove
    ($fn_name:ident ($value:ident : attr_value![&$($maybe_ty:tt)*]) $maybe:tt $trait_name:ident) => {
        crate::macros::impl_attribute! {$fn_name ($value : attr_value![$($maybe_ty)*]) $maybe $trait_name }
    };
    ($fn_name:ident ($value:ident : attr_value![$maybe_ty:ty]) {
        $(alias! $alias:tt;)?
        $(attr_name!($attr_name:expr);)?
        $(update_with! $update_with:tt;)?
    } $trait_name:ident) => {
        crate::impl_bounds! {
            props::$fn_name(
                prop_marker(prop_markers::$fn_name),
                bounds as crate::impl_bounds::AttrValue<$maybe_ty>,
                element as $trait_name,
                attr_name = ::frender_common::expand!({$($attr_name)?} or (stringify!($fn_name))),
                csr {
                    update: ::frender_common::expand! {
                        if ($($update_with)?) {
                                crate::macros::parse_update_with!(match $($update_with)? {
                                    simple => {
                                        prepend {
                                            |v| el.
                                        }
                                        append {
                                            (renderer, v), v
                                        }
                                        wrap ()
                                        prepend {
                                            |el: &mut ET::$trait_name<Renderer>, renderer: &mut _, _, v: <$maybe_ty as frender_attr_value::csr::ValueKind>::Value<'_>|
                                                <$maybe_ty as crate::attr::SetAttributeWithDomApi>::set_attribute_with_dom_api
                                        }
                                    }
                                    impl_with => {
                                        append( as update(
                                            ValueType(<$maybe_ty as frender_attr_value::csr::ValueKind>::Value<'_>)
                                            value($value)
                                            element_type(ET::$trait_name<Renderer>)
                                        ))
                                        wrap {}
                                        prepend( crate::macros::parse_impl_with! )
                                    }
                                })
                        } else {
                            <$maybe_ty as crate::attr::SetAttribute>::set_attribute
                        }
                    },
                    remove: ::frender_common::expand! {
                        if ($($update_with)?) {
                            crate::macros::parse_update_with!(match $($update_with)? {
                                // RemoveAttributeWithDomApi::remove_attribute_with_dom_api(DomApi {})
                                simple => {
                                    prepend {
                                        element,
                                        renderer,
                                        attr_name,
                                        api_set: <_>::
                                    }
                                    wrap {}
                                    prepend {
                                        crate::attr::DomApi
                                    }
                                    wrap ()
                                    prepend {
                                        |element: &mut ET::$trait_name<Renderer>, renderer: &mut _, attr_name: &_|
                                            <$maybe_ty as crate::attr::RemoveAttributeWithDomApi>::remove_attribute_with_dom_api
                                    }
                                }
                                impl_with => {
                                    append( as remove(element_type(ET::$trait_name<Renderer>)))
                                    wrap {}
                                    prepend( crate::macros::parse_impl_with! )
                                }
                            })
                        } else {
                            crate::dom::behaviors::Element::remove_attribute
                        }
                    },
                },
            )
        }
    };
    ($fn_name:ident ($value:ident : children! $children:tt) $fn_body_or_semi:tt $trait_name:ident) => {
        // children is not an attribute
    };
    ($fn_name:ident ($value:ident : bounds![$($bounds:tt)+]) $(;)? $({
        $(attr_name!($attr_name:expr);)?
        $(impl_with!($($impl_with:tt)*);)?
    })? $trait_name:ident) => {
        crate::impl_bounds! {
            props::$fn_name(
                prop_marker(prop_markers::$fn_name),
                bounds as $($bounds)+,
                element as $trait_name,
                attr_name = ::frender_common::expand!({$($($attr_name)?)?} or (stringify!($fn_name))),
                $($($($impl_with)*)?)?
            )
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
            type $tags: self::behaviors::$trait_name<Self> + 'static;
            fn $tags(&mut self) -> Self::$tags;
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
        #[allow(unused_imports)]
        use super::behaviors::$trait_name;

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
        #[allow(non_camel_case_types)]
        pub enum $fn_name {}

        impl ::frender_dom::HasEventTypeName for $fn_name {
            const EVENT_TYPE_NAME: &'static str = $event_type_name;
        }

        #[cfg(feature = "web")]
        impl ::frender_dom::csr::web::JsCastEventType for $fn_name {
            type JsEventTarget = web_sys::$trait_name;
            type JsCastEvent = web_sys::$event_trait_name;

            fn js_event_as_event(event: &Self::JsCastEvent) -> &Self::Event {
                frender_dom::csr::web::Event::new_from_ref(event)
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

macro_rules! event_type_helpers {
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
            crate::macros::event_type_helper! {
                $fn_name $fn_args $fn_body_or_semi $trait_name { super::super::behaviors }
            }
        )*
    };
}

macro_rules! event_type_helper {
    ($fn_name:ident ($value:ident : event![
        $event_trait_name:ident,
        $event_type_name:literal,
        $event_type_ident:ident,
        $event_type_listener_ident:ident $(,)?
    ]); $trait_name:ident {$($path_to_mod_behaviors:tt)+}) => {
        pub mod $fn_name {
            pub use ::frender_dom::event::$event_trait_name as Event;

            pub type EventListenerOf<E, R, F> = <E as ::frender_dom::OnEvent<R, super::super::event_types::$fn_name>>::EventListener<F>;
            pub type UnpinnedEventListenerOf<E, R, F> = <E as ::frender_dom::OnEvent<R, super::super::event_types::$fn_name>>::EventListenerUnpinned<F>;

            // pub const EVENT_TYPE_NAME: &'static str = <super::super::event_types::$fn_name as ::frender_dom::HasEventTypeName>::EVENT_TYPE_NAME;
        }
    };
    ($fn_name:ident $fn_args:tt $fn_body_or_semi:tt $trait_name:tt $path:tt) => {};
}

macro_rules! macro_props_builders {
    (expand_item $expand_item:tt) => {
        crate::macros::expand_item_and_prepend_expanded! {
            $expand_item
            {
                use super::*;
                use super::prop_markers::conflicted_names;

                use crate::intrinsic::{Intrinsic, AllowAttribute, AllowAttributeWithPinnedState, AllowAttributeName, AllowChildren};
            }
        }
    };
    (
        extends $extends:tt
        $(special_super_traits $special_super_traits:tt)?
        vis $vis:tt
        trait_name($trait_name:ident)
        $($rest:ident $rest_paren:tt)*
    ) => {
        crate::macros::props_builders::define! {
            extends $extends
            $(special_super_traits $special_super_traits)?
            vis $vis
            trait_name($trait_name)
            $($rest $rest_paren)*
        }
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

#[cfg(feature = "macros_not_expanded")]
pub(crate) mod define_props_macro;

#[cfg(feature = "macros_not_expanded")]
pub(crate) mod props_builders;

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
            { frender_dom::MaybeHandleEvent<dyn $crate::dom::event::$event_trait_name> + 'static }
            do $commands
        }
    };
    (($value:ident : attr_value![&$maybe_ty:ty]) do $commands:tt) => {
        $crate::expand! {
            { $crate::impl_bounds::AttrValue::Bounds::<$maybe_ty> }
            do $commands
        }
    };
    (($value:ident : attr_value![$maybe_ty:ty]) do $commands:tt) => {
        $crate::expand! {
            { $crate::impl_bounds::AttrValue::Bounds::<$maybe_ty> }
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
    event_type, event_type_helper, event_type_helpers, event_types, expand_item_and_prepend_expanded, expand_item_simple, expand_nested_traits, extract_attr_builder_fn_names, extract_only_children_or, impl_attribute,
    impl_behavior_fn, impl_behavior_fn_update_with, macro_props_builders as props_builders, parse_fn_args_as_bounds, parse_fn_args_as_whether_pinned_state, parse_impl_with, parse_update_with, prop_markers, props,
    props_implementations, tag_and_props_markers, tag_custom_content_model, unwrap_brace_concat, RenderHtml,
};

#[cfg(test)]
mod tests;
