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
        update = |$element:pat_param, $renderer:pat_param $(,)?| $update:expr,
        remove = $($t:tt)*
    ) as update(
        value($value:pat_param)
        element_type($element_type:ty)
    )) => {
        |$element: &mut $element_type, $renderer: &mut _, _, $value: &_| $update
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
        type $event_type_ident: crate::event::$event_trait_name + 'static;
        type $event_type_listener_ident;

        fn $fn_name(
            &mut self,
            renderer: &mut Renderer,
            listener: impl FnMut(&Self::$event_type_ident) + 'static,
        ) -> Self::$event_type_listener_ident;
    };
    ($fn_name:ident ($value:ident : maybe![$maybe_ty:ty]) {
        $(alias! $alias:tt;)?
        $(attr_name! $attr_name:tt;)?
        $(update_with! $update_with:tt;)?
    }) => {
        $(
            crate::html::macros::define_behavior_fn_update_with! {
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
        type $event_type_ident = crate::csr::web::Event<::web_sys::$event_trait_name>;
        type $event_type_listener_ident = gloo_events::EventListener;

        fn $fn_name(
            &mut self,
            _: &mut Renderer,
            mut listener: impl FnMut(&Self::$event_type_ident) + 'static,
        ) -> Self::$event_type_listener_ident {
            let element: &web_sys::Element = self.0.as_ref();

            ::gloo_events::EventListener::new(
                element,
                <crate::html::event_types::$fn_name as crate::event::HasEventTypeName>::EVENT_TYPE_NAME,
                move |event| {
                    use wasm_bindgen::JsCast;
                    let event = event.unchecked_ref();
                    listener(crate::csr::web::Event::new_from_ref(event))
                },
            )
        }
    };
    ($fn_name:ident ($value:ident : maybe![$maybe_ty:ty]) {
        $(alias! $alias:tt;)?
        $(attr_name! $attr_name:tt;)?
        $(update_with! $update_with:tt;)?
    } $trait_name:tt) => {
        $(
            crate::html::macros::impl_behavior_fn_update_with! {
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
    (expand_item $vis:vis $item_type:ident $item_name:ident {} {$($item_body_expanded:tt)*}) => {
        $vis $item_type $item_name {
            use crate::shims::prelude::*;

            $($item_body_expanded)*
        }
    };
    (
        common_data($common_data:tt)
        extends($($extends:ident)*)
        special_super_traits($($($special_super_traits:ident)+)?)
        special_inter_traits $special_inter_traits:tt
        vis($vis:vis)
        trait_name($trait_name:ident)
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
            $($($special_super_traits<Renderer> +)+)?
        {
            $($($verbatim_trait_items)*)?

            $(crate::html::macros::define_behavior_fn!{
                $fn_name $fn_args $fn_body_or_semi
            })*
        }

        // if `impl_for_web`
        #[cfg(feature = "web")]
        ::frender_common::expand! { if ($( ! $($($verbatim_trait_items_impl_web)*)?)?) {
            ::frender_common::expand! { if ($($( ! $($impl_for_web_only_for_types)*)?)?) {
                ::frender_common::expand! { while ($($($({$impl_for_web_only_for_types})*)?)?) {
                    prepend(impl<Renderer: ?Sized + crate::csr::web::Renderer> $trait_name<Renderer> for crate::csr::web::Node<)
                    append( > $(where Self: $($special_super_traits<Renderer> + )+ )? {
                        $($($($verbatim_trait_items_impl_web)*)?)?

                        ::frender_common::expand! { while ($({$fn_name $fn_args $fn_body_or_semi})*) {
                            append( ($trait_name ($($($($impl_for_web_only_for_types)*)?)?)) )
                            wrap {}
                            prepend(crate::html::macros::impl_behavior_fn!)
                        }}
                    })
                }}
            } else {
                impl<
                    Renderer: ?Sized + crate::csr::web::Renderer,
                    E: AsRef<::web_sys::$trait_name> $(+ AsRef<::web_sys::$extends>)*
                > $trait_name<Renderer> for crate::csr::web::Node<E>
                $(where Self: $($special_super_traits<Renderer> + )+ )?
                {
                    $($($($verbatim_trait_items_impl_web)*)?)?

                    $(crate::html::macros::impl_behavior_fn! {
                        $fn_name $fn_args $fn_body_or_semi ($trait_name)
                    })*
                }
            }}
        }}
    };
}

macro_rules! behaviors_prelude {
    (expand_item $vis:vis $item_type:ident $item_name:ident {} $item_body_expanded:tt) => { $vis $item_type $item_name $item_body_expanded };
    (
        common_data({
            root_path $root_path:tt
            item_names(
                behaviors($mod_behaviors:ident)
                behaviors_prelude($mod_behaviors_prelude:ident)
                attributes($mod_attributes:ident)
                behavior_type_traits($mod_behavior_type_traits:ident)
                tags($mod_tags:ident)
                event_types($mod_event_types:ident)
                event_type_helpers($mod_event_type_helpers:ident)
                components($mod_components:ident)
                RenderHtml($RenderHtml:ident)
            )
        })
        extends($($extends:ident)*)
        special_super_traits($($($special_super_traits:ident)+)?)
        special_inter_traits $special_inter_traits:tt
        vis($vis:vis)
        trait_name($trait_name:ident)
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
            $vis use super::super::$mod_behaviors::$trait_name as _;

            ::frender_common::expand! {
                while ($({$extends})* $($({$special_super_traits})+)?) {
                    prepend( $vis use super:: )
                    append( ::*;)
                }
            }
        }
    };
}

macro_rules! behavior_type_traits {
    (expand_item $vis:vis $item_type:ident $item_name:ident {} $item_body_expanded:tt) => { $vis $item_type $item_name $item_body_expanded };
    (
        common_data({
            root_path $root_path:tt
            item_names(
                behaviors($mod_behaviors:ident)
                behaviors_prelude($mod_behaviors_prelude:ident)
                attributes($mod_attributes:ident)
                behavior_type_traits($mod_behavior_type_traits:ident)
                tags($mod_tags:ident)
                event_types($mod_event_types:ident)
                event_type_helpers($mod_event_type_helpers:ident)
                components($mod_components:ident)
                RenderHtml($RenderHtml:ident)
            )
        })
        extends($($extends:ident)*)
        special_super_traits($($($special_super_traits:ident)+)?)
        special_inter_traits $special_inter_traits:tt
        vis($vis:vis)
        trait_name($trait_name:ident)
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
            $($extends +)*
            $($($special_super_traits +)+)?
        {
            type $trait_name<Renderer: ?Sized + super::$RenderHtml>: super::$mod_behaviors::$trait_name<Renderer>
                $(  + crate::convert::IdentityAs<Self::$extends             <Renderer>>)*
                $($(+ crate::convert::IdentityAs<Self::$special_super_traits<Renderer>>)+)?
            ;

            fn from_identity_mut_root<Renderer: ?Sized + super::$RenderHtml>(
                root: &mut ::frender_common::expand![
                    {{$trait_name} $({$extends})*} get {-1}
                    prepend(Self::)
                    append(<Renderer>)
                ]
            ) -> &mut Self::$trait_name<Renderer>;
        }
    };
}

macro_rules! tags {
    (expand_item $vis:vis $item_type:ident $item_name:ident {} $item_body_expanded:tt) => { $vis $item_type $item_name $item_body_expanded };
    (
        common_data({
            root_path $root_path:tt
            item_names(
                behaviors($mod_behaviors:ident)
                behaviors_prelude($mod_behaviors_prelude:ident)
                attributes($mod_attributes:ident)
                behavior_type_traits($mod_behavior_type_traits:ident)
                tags($mod_tags:ident)
                event_types($mod_event_types:ident)
                event_type_helpers($mod_event_type_helpers:ident)
                components($mod_components:ident)
                RenderHtml($RenderHtml:ident)
            )
        })
        extends($($extends:ident)*)
        special_super_traits($($special_super_traits:ident)*)
        special_inter_traits $special_inter_traits:tt
        vis($vis:vis)
        trait_name($trait_name:ident)
        $(define(
            Props: $Props:ident
            $(, tags: ($($tags:ident),* $(,)?))?
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
            pub struct $tags;
            impl crate::renderer::HasIntrinsicComponentTag for $tags {
                const INTRINSIC_COMPONENT_TAG: &'static str = stringify!($tags);
            }
            impl crate::renderer::CreateNode for $tags {
                fn create_node<R: super::$RenderHtml>(renderer: &mut R) -> <Self as super::$mod_behavior_type_traits::Node>::Node<R> {
                    renderer.$tags()
                }
            }
        )*)?)?

        ::frender_common::expand! { while ($($($({$tags})*)?)?) {
            prepend( impl_all_traits { super } for )
            wrap {}
            prepend( super::$mod_attributes::$trait_name::helper_macro! )
        }}
    };
}

macro_rules! attributes {
    (expand_item $vis:vis $item_type:ident $item_name:ident {} $item_body_expanded:tt) => { $vis $item_type $item_name $item_body_expanded };
    (
        common_data({
            root_path $root_path:tt
            item_names(
                behaviors($mod_behaviors:ident)
                behaviors_prelude($mod_behaviors_prelude:ident)
                attributes($mod_attributes:ident)
                behavior_type_traits($mod_behavior_type_traits:ident)
                tags($mod_tags:ident)
                event_types($mod_event_types:ident)
                event_type_helpers($mod_event_type_helpers:ident)
                components($mod_components:ident)
                RenderHtml($RenderHtml:ident)
            )
        })
        extends($($extends:ident)*)
        special_super_traits($($($special_super_traits:ident)+)?)
        special_inter_traits($($special_inter_traits:ident)*)
        vis($vis:vis)
        trait_name($trait_name:ident)
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
            $vis mod attributes {
                ::frender_common::expand! {
                    while (
                        $({$extends})*
                        $($({$special_super_traits})+)?
                        $({$special_inter_traits})*
                    ) {
                        prepend( $vis use super::super:: )
                        append( ::attributes::*; )
                    }
                }

                $($vis use super::$fn_name;)*
            }

            $(
                #[derive(Debug, Default)]
                pub struct $fn_name<V>(pub V);
            )*

            ::frender_common::expand! {
                while ($({ $fn_name $fn_args $fn_body_or_semi $trait_name })*) {
                    wrap {}
                    prepend( crate::html::macros::impl_attribute! )
                }
            }

            macro_rules! helper_macro {
                (fns_without_attributes $commands:tt) => {
                    ::frender_common::expand! {
                        {$({ fn $fn_name $fn_args $fn_body_or_semi })*}
                        do $commands
                    }
                };
                (fn_names $commands:tt) => {
                    ::frender_common::expand! {
                        {$({ $fn_name })*}
                        do $commands
                    }
                };
                (impl $prepend_path:tt for $for_tag:ident ) => {
                    ::frender_common::expand! {
                        { impl }
                        append $prepend_path
                        append(::$mod_behavior_type_traits::$trait_name for $for_tag {
                            ::frender_common::expand! {
                                { type $trait_name<Renderer: ?Sized + }
                                append $prepend_path
                                append(
                                    ::$RenderHtml > = Renderer::$for_tag;
                                )
                            }
                            ::frender_common::expand! {
                                { fn from_identity_mut_root<Renderer: ?Sized + }
                                append $prepend_path
                                append(
                                    ::$RenderHtml >(
                                        root: &mut ::frender_common::expand![
                                            {{$trait_name} $({$extends})*} get {-1}
                                            prepend(Self::)
                                            append(<Renderer>)
                                        ]
                                    ) -> &mut Self::$trait_name<Renderer> {
                                        root
                                    }
                                )
                            }
                        })
                    }
                };
                (impl_all_super_traits $prepend_path:tt for $for_tag:ident) => {
                    ::frender_common::expand! {
                        while (
                            $({$extends})*
                            $($({$special_super_traits})+)?
                            $({$special_inter_traits})*
                        ) {
                            // Node
                            prepend {::$mod_attributes::}
                            prepend $prepend_path
                            append (::helper_macro!{
                                impl $prepend_path for $for_tag
                            })
                        }
                    }

                    // ::frender_common::expand! {
                    //     {$({$extends})* $($({$special_super_traits})+)?}
                    //     get_or_exit {0} // Node
                    //     prepend {::$mod_attributes::}
                    //     prepend $prepend_path
                    //     append (!{
                    //         impl_all_super_traits $prepend_path for $for_tag
                    //     })
                    // }
                };
                (impl_all_traits $prepend_path:tt for $for_tag:ident) => {
                    ::frender_common::expand! {
                        $prepend_path
                        append(::$mod_attributes::$trait_name::helper_macro! {
                            impl $prepend_path for $for_tag
                        })
                    }
                    ::frender_common::expand! {
                        $prepend_path
                        append(::$mod_attributes::$trait_name::helper_macro! {
                            impl_all_super_traits $prepend_path for $for_tag
                        })
                    }
                };
            }
            pub(crate) use helper_macro;
        }
    };
}

macro_rules! impl_attribute {
    ($fn_name:ident ($value:ident : event![
        $event_trait_name:ident,
        $event_type_name:literal,
        $event_type_ident:ident,
        $event_type_listener_ident:ident $(,)?
    ]); $trait_name:tt) => {};
    ($fn_name:ident ($value:ident : maybe![$($maybe_ty:tt)*]) ; $trait_name:ident) => {
        crate::html::macros::impl_attribute! {$fn_name ($value : maybe![$($maybe_ty)*]) {} $trait_name }
    };
    ($fn_name:ident ($value:ident : maybe![&$maybe_ty:ty]) {
        $(alias! $alias:tt;)?
        $(attr_name!($attr_name:expr);)?
        $(update_with! $update_with:tt;)?
    } $trait_name:ident) => {
        crate::impl_bounds! {
            self::$fn_name(
                bounds as crate::impl_bounds::MaybeValue<$maybe_ty>,
                element as $trait_name,
                attr_name = ::frender_common::expand!({$($attr_name)?} or (stringify!($fn_name))),
                csr {
                    update: ::frender_common::expand! {
                        if ($($update_with)?) {
                                crate::html::macros::parse_update_with!(match $($update_with)? {
                                    simple => {
                                        prepend(|el: &mut ET::$trait_name<Renderer>, renderer: &mut _, _, v: &_| el.)
                                        append( (renderer, v) )
                                    }
                                    impl_with => {
                                        append( as update(value($value) element_type(ET::$trait_name<Renderer>)))
                                        wrap {}
                                        prepend( crate::html::macros::parse_impl_with! )
                                    }
                                })
                        } else {
                            crate::impl_bounds::MaybeValue::csr::default_update
                        }
                    },
                    remove: ::frender_common::expand! {
                        if ($($update_with)?) {
                            crate::html::macros::parse_update_with!(match $($update_with)? {
                                simple => {
                                    reset {}
                                    {crate::impl_bounds::MaybeValue::csr::default_remove}
                                }
                                impl_with => {
                                    append( as remove(element_type(ET::$trait_name<Renderer>)))
                                    wrap {}
                                    prepend( crate::html::macros::parse_impl_with! )
                                }
                            })
                        } else {
                            crate::impl_bounds::MaybeValue::csr::default_remove
                        }
                    },
                },
            )
        }
    };
    ($fn_name:ident ($value:ident : maybe![$maybe_ty:ty]) {
        $(alias! $alias:tt;)?
        $(attr_name!($attr_name:expr);)?
        $(update_with! $update_with:tt;)?
    } $trait_name:ident) => {
        crate::impl_bounds! {
            self::$fn_name(
                bounds as crate::impl_bounds::MaybeValue<$maybe_ty>,
                element as $trait_name,
                attr_name = ::frender_common::expand!({$($attr_name)?} or (stringify!($fn_name))),
                csr {
                    update: ::frender_common::expand! {
                        if ($($update_with)?) {
                                crate::html::macros::parse_update_with!(match $($update_with)? {
                                    simple => {
                                        prepend(|el: &mut ET::$trait_name<Renderer>, renderer: &mut _, _, v: &_| el.)
                                        append( (renderer, *v) )
                                    }
                                    impl_with => {
                                        append( as update(value(&$value) element_type(ET::$trait_name<Renderer>)))
                                        wrap {}
                                        prepend( crate::html::macros::parse_impl_with! )
                                    }
                                })
                        } else {
                            crate::impl_bounds::MaybeValue::csr::default_update
                        }
                    },
                    remove: ::frender_common::expand! {
                        if ($($update_with)?) {
                            crate::html::macros::parse_update_with!(match $($update_with)? {
                                simple => {
                                    reset {}
                                    {crate::impl_bounds::MaybeValue::csr::default_remove}
                                }
                                impl_with => {
                                    append( as remove(element_type(ET::$trait_name<Renderer>)))
                                    wrap {}
                                    prepend( crate::html::macros::parse_impl_with! )
                                }
                            })
                        } else {
                            crate::impl_bounds::MaybeValue::csr::default_remove
                        }
                    },
                },
            )
        }
    };
    ($fn_name:ident $fn_args:tt $fn_body_or_semi:tt $trait_name:tt) => {};
}

macro_rules! RenderHtml {
    (expand_item $vis:vis $item_type:ident $item_name:ident {
        additional_bounds!($(dyn $($additional_bounds:tt)+)?);
        $($items:tt)*
    } {$($item_body_expanded:tt)*}) => {
        $vis $item_type $item_name
        $(: $($additional_bounds)+)?
        {
            $($items)*
            $($item_body_expanded)*
        }
    };
    (
        common_data({
            root_path $root_path:tt
            item_names(
                behaviors($mod_behaviors:ident)
                behaviors_prelude($mod_behaviors_prelude:ident)
                attributes($mod_attributes:ident)
                behavior_type_traits($mod_behavior_type_traits:ident)
                tags($mod_tags:ident)
                event_types($mod_event_types:ident)
                event_type_helpers($mod_event_type_helpers:ident)
                components($mod_components:ident)
                RenderHtml($RenderHtml:ident)
            )
        })
        extends($($extends:ident)*)
        special_super_traits($($($special_super_traits:ident)+)?)
        special_inter_traits $special_inter_traits:tt
        vis($vis:vis)
        trait_name($trait_name:ident)
        $(define(
            Props: $Props:ident
            $(, tags: ($($tags:ident),* $(,)?))?
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
            type $tags: super::$mod_behaviors::$trait_name<Self>;
            fn $tags(&mut self) -> Self::$tags;
        )*)?)?
    };
}

macro_rules! traverse_traits {
    (
        $common_data:tt {
            $(@extends($($extends:ident)*))?
            $(@inherited_special_super_traits($($inherited_special_super_traits:ident)*))?
            $vis:vis trait $trait_name:ident {
                $(special_super_traits!($($special_super_traits:ident),* $(,)?);)?
                $(special_inter_traits!($($special_inter_traits:ident),* $(,)?);)?
                $(define! $define:tt;)?

                $(verbatim_trait_items! $verbatim_trait_items:tt;)?

                $(impl_for_web! $impl_for_web:tt;)?

                $(
                    $(#$fn_attr:tt)*
                    fn $fn_name:ident $fn_args:tt $fn_body_or_semi:tt
                )*

                $(sub_traits! $sub_traits:tt ;)?
            }
        } $commands:tt
    ) => {
        ::frender_common::expand! {{{
            common_data($common_data)
            extends($($($extends)*)?)
            special_super_traits($($($special_super_traits)*)? $($($inherited_special_super_traits)*)?)
            special_inter_traits($($($special_inter_traits)*)?)
            vis($vis)
            trait_name($trait_name)
            $(define $define)?
            $(verbatim_trait_items $verbatim_trait_items)?
            $(impl_for_web $impl_for_web)?
            fns($(
                $(#$fn_attr)*
                fn $fn_name $fn_args $fn_body_or_semi
            )*)
        }} do $commands }

        ::frender_common::expand! { if ($($sub_traits)?) {
            crate::for_each_trait! { $($sub_traits)? {
                prepend(
                    @extends($trait_name $($($extends)*)?)
                    @inherited_special_super_traits($($($special_super_traits)*)? $($($inherited_special_super_traits)*)?)
                )
                wrap {}
                prepend($common_data)
                append($commands)
                wrap {}
                prepend(
                    crate::html::macros::traverse_traits!
                )
            }}
        }}
    };
}

macro_rules! define_item_and_traverse_traits {
    (
        $prepend:tt // {}
        $t:tt // {}
        $($macro_name:ident ($vis:vis $item_type:ident $item_name:ident $item_body:tt))*
    ) => {
        $(
            crate::html::macros::$macro_name! {
                expand_item $vis $item_type $item_name $item_body
                {
                    crate::html::macros::traverse_traits! {
                        $prepend
                        $t
                        { prepend(crate::html::macros::$macro_name!) }
                    }
                }
            }
        )*
    };
}

macro_rules! def_intrinsic_component_props {
    (
        #[root_path]
        use $($root_path_1:ident)? $(::$root_path_2:ident)*;

        mod items {$(
            #[$item_macro:ident]
            $item_vis:vis $item_type:ident $item_name:ident { $($item_body:tt)* }
        )*}

        $($t:tt)*
    ) => {
        crate::html::macros::define_item_and_traverse_traits! {
            {
                root_path($($root_path_1)? $(::$root_path_2)*)
                item_names( $( $item_macro ($item_name) )+ )
            }
            { $($t)* }
            $($item_macro (
                $item_vis $item_type $item_name { $($item_body)* }
            ))*
        }
    };
}

macro_rules! event_types {
    (expand_item $vis:vis $item_type:ident $item_name:ident {} $item_body_expanded:tt) => { $vis $item_type $item_name $item_body_expanded };
    (
        common_data({
            root_path $root_path:tt
            item_names(
                behaviors($mod_behaviors:ident)
                behaviors_prelude($mod_behaviors_prelude:ident)
                attributes($mod_attributes:ident)
                behavior_type_traits($mod_behavior_type_traits:ident)
                tags($mod_tags:ident)
                event_types($mod_event_types:ident)
                event_type_helpers($mod_event_type_helpers:ident)
                components($mod_components:ident)
                RenderHtml($RenderHtml:ident)
            )
        })
        extends($($extends:ident)*)
        special_super_traits($($($special_super_traits:ident)+)?)
        special_inter_traits $special_inter_traits:tt
        vis($vis:vis)
        trait_name($trait_name:ident)
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
        use super::$mod_behaviors::$trait_name;

        $(
            crate::html::macros::event_type! {
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
        pub enum $fn_name {}

        impl crate::event::HasEventTypeName for $fn_name {
            const EVENT_TYPE_NAME: &'static str = $event_type_name;
        }

        crate::event::type_traits_impl::$event_trait_name! {
            $fn_name,
            $trait_name,
            $event_type_ident,
            $event_type_listener_ident
        }
    };
    ($fn_name:ident $fn_args:tt $fn_body_or_semi:tt $trait_name:tt) => {};
}

macro_rules! event_type_helpers {
    (expand_item $vis:vis $item_type:ident $item_name:ident {} $item_body_expanded:tt) => { $vis $item_type $item_name $item_body_expanded };
    (
        common_data({
            root_path $root_path:tt
            item_names(
                behaviors($mod_behaviors:ident)
                behaviors_prelude($mod_behaviors_prelude:ident)
                attributes($mod_attributes:ident)
                behavior_type_traits($mod_behavior_type_traits:ident)
                tags($mod_tags:ident)
                event_types($mod_event_types:ident)
                event_type_helpers($mod_event_type_helpers:ident)
                components($mod_components:ident)
                RenderHtml($RenderHtml:ident)
            )
        })
        extends($($extends:ident)*)
        special_super_traits($($($special_super_traits:ident)+)?)
        special_inter_traits $special_inter_traits:tt
        vis($vis:vis)
        trait_name($trait_name:ident)
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
            crate::html::macros::event_type_helper! {
                $fn_name $fn_args $fn_body_or_semi $trait_name { super::super::$mod_behaviors }
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
            pub use crate::event::$event_trait_name as Event;

            pub type EventOf        <E, R> = <E as $($path_to_mod_behaviors)+::$trait_name<R>>::$event_type_ident;
            pub type EventListenerOf<E, R> = <E as $($path_to_mod_behaviors)+::$trait_name<R>>::$event_type_listener_ident;

            pub fn on<
                'event,
                E: ?Sized + $($path_to_mod_behaviors)+::$trait_name<R>,
                R: ?Sized,
                C: 'static
                    + Clone
                    + for<'e> frender_events::callable::Callable<(&'e (dyn Event + 'event),), Output = ()>,
            >(
                element: &mut E,
                renderer: &mut R,
                callable: &C,
            ) -> E::$event_type_listener_ident {
                E::$fn_name(element, renderer, {
                    let callable = (*callable).clone();
                    move |ev: &_| frender_events::callable::Callable::call_fn(&callable, (ev,))
                })
            }
        }
    };
    ($fn_name:ident $fn_args:tt $fn_body_or_semi:tt $trait_name:tt $path:tt) => {};
}

macro_rules! components {
    (expand_item $vis:vis $item_type:ident $item_name:ident {} $item_body_expanded:tt) => { $vis $item_type $item_name $item_body_expanded };
    (
        common_data({
            root_path $root_path:tt
            item_names(
                behaviors($mod_behaviors:ident)
                behaviors_prelude($mod_behaviors_prelude:ident)
                attributes($mod_attributes:ident)
                behavior_type_traits($mod_behavior_type_traits:ident)
                tags($mod_tags:ident)
                event_types($mod_event_types:ident)
                event_type_helpers($mod_event_type_helpers:ident)
                components($mod_components:ident)
                RenderHtml($RenderHtml:ident)
            )
        })
        extends($($extends:ident)*)
        special_super_traits($($($special_super_traits:ident)+)?)
        special_inter_traits($($special_inter_traits:ident)*)
        vis($vis:vis)
        trait_name($trait_name:ident)
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
            pub mod data_struct {
                // #[allow(unused_imports)]
                // use super::super::*;

                #[derive(Debug, Clone, Copy, Default)]
                pub struct $trait_name<Children = (), Attrs = ()> {
                    pub props: $crate::component::ElementProps<Children, Attrs>,
                }

                impl<Children, Attrs> $crate::component::IntoElementProps for $trait_name<Children, Attrs> {
                    type Children = Children;
                    type Attrs = Attrs;

                    fn into_element_props(this: Self) -> $crate::component::ElementProps<Children, Attrs> {
                        this.props
                    }
                }
            }

            pub mod building_struct {
                pub struct $trait_name<Children = (), Attrs = ()>(pub super::Data<Children, Attrs>);
            }

            pub use building_struct::$trait_name as Building;
            pub use data_struct::$trait_name as Data;
            pub type DataInitial = data_struct::$trait_name;
            pub mod prelude {}

            #[inline(always)]
            pub fn build<Children, Attrs>(
                building: Building<Children, Attrs>,
            ) -> Data<Children, Attrs> {
                building.0
            }
            pub use build as valid;

            pub use super::super::$mod_attributes::$trait_name::attributes;

            macro_rules! impl_props_builder_fns {
                // only children is overridable
                (impl_children $overrides:tt) => {

                };
                (impl_self_attrs) => {
                    $(
                        // #fn_attr
                        // fn #attr_builder_fn_name<V: #parse_fn_args_as_bounds>(
                        //     v: V
                        // ) -> super::Building<Children, (Attrs, super::props::$fn_name<V>)> {
                        //     super::Building(super::Data {
                        //         props: self.0.props.chain_prop(super::props::$fn_name(v)),
                        //     })
                        // }
                        crate::html::macros::parse_fn_args_as_bounds! {
                            $fn_args
                            do {
                                wrap()
                                prepend(
                                    prepend(
                                        $(#$fn_attr)*
                                        fn
                                    )
                                    append(<V: )
                                    append
                                )
                                append(
                                    append(
                                        >(self, value: V) -> super::Building<Children, (Attrs, super::attributes::$fn_name<V>)> {
                                            super::Building(super::Data {
                                                props: self.0.props.chain_prop(super::attributes::$fn_name(value)),
                                            })
                                        }
                                    )
                                )
                                wrap {}
                                prepend( for_each )
                                wrap {}
                                prepend( {$fn_name $fn_body_or_semi} do )
                                wrap {}
                                prepend( crate::html::macros::extract_attr_builder_fn_names! )
                            }
                        }
                    )*
                };
                (impl_all_attrs) => {
                    ::frender_common::expand! {
                        while (
                            {$trait_name}
                            $({$extends})*
                            $($({$special_super_traits})+)?
                            $({$special_inter_traits})*
                        ) {
                            prepend( super::super:: )
                            append( ::impl_props_builder_fns! { impl_self_attrs } )
                        }
                    }
                };
            }

            pub(crate) use impl_props_builder_fns;

            mod props_builder {
                #[allow(unused_imports)]
                use super::super::super::*;

                impl<Attrs> super::Building<(), Attrs> {

                }
                    // $crate::__impl_children_fns! $fields ;

                impl<Children, Attrs> super::Building<Children, Attrs> {
                    super::impl_props_builder_fns! { impl_all_attrs }
                }
            }
        }

        #[allow(non_snake_case)]
        #[inline(always)]
        $vis fn $trait_name() -> $trait_name::Building {
            $trait_name::Building(Default::default())
        }
    };
}

/// `children` is excluded
macro_rules! extract_attr_builder_fn_names {
    ({children $fn_body_or_semi:tt} do $commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    ({$fn_name:ident ;} do $commands:tt) => {
        ::frender_common::expand! { { {$fn_name} } do $commands }
    };
    ({$fn_name:ident {
        alias!($($alias:ident),* $(,)?);
        $($other:ident ! $other_macro:tt;)*
    }} do $commands:tt) => {
        ::frender_common::expand! { { {$fn_name} $({$alias})* } do $commands }
    };
    ({$fn_name:ident {
        $($other:ident ! $other_macro:tt;)*
    }} do $commands:tt) => {
        ::frender_common::expand! { { {$fn_name} } do $commands }
    };
}

macro_rules! parse_fn_args_as_bounds {
    (($value:ident : event![
        $event_trait_name:ident,
        $event_type_name:literal,
        $event_type_ident:ident,
        $event_type_listener_ident:ident $(,)?
    ]) do $commands:tt) => {
        ::frender_common::expand! {
            { crate::impl_bounds::MaybeHandleEvent::Bounds::<dyn crate::event::$event_trait_name> }
            do $commands
        }
    };
    (($value:ident : maybe![&$maybe_ty:ty]) do $commands:tt) => {
        ::frender_common::expand! {
            { crate::impl_bounds::MaybeValue::Bounds::<$maybe_ty> }
            do $commands
        }
    };
    (($value:ident : maybe![$maybe_ty:ty]) do $commands:tt) => {
        ::frender_common::expand! {
            { crate::impl_bounds::MaybeValue::Bounds::<$maybe_ty> }
            do $commands
        }
    };
    (($value:ident : children![impl $($bounds:tt)+]) do $commands:tt) => {
        ::frender_common::expand! {
            { $($bounds)+ }
            do $commands
        }
    };
    (($value:ident : children![impl $($bounds:tt)+]) do $commands:tt) => {
        ::frender_common::expand! {
            { $($bounds)+ }
            do $commands
        }
    };
    (($value:ident : bounds![ $($mod_path_start:ident)? $(:: $mod_path:ident)*  $($(::)? <$($ty:ty),* $(,)?>)?]) do $commands:tt) => {
        ::frender_common::expand! {
            { $($mod_path_start)? $(:: $mod_path)* ::Bounds $(::<$($ty),*>)? }
            do $commands
        }
    };
    ($fn_args:tt do $commands:tt) => {
        compile_error! { stringify!($fn_args) }
    };
}

pub(crate) use attributes;
pub(crate) use behavior_type_traits;
pub(crate) use behaviors;
pub(crate) use behaviors_prelude;
pub(crate) use components;
pub(crate) use def_intrinsic_component_props;
pub(crate) use define_behavior_fn;
pub(crate) use define_behavior_fn_update_with;
pub(crate) use define_item_and_traverse_traits;
pub(crate) use event_type;
pub(crate) use event_type_helper;
pub(crate) use event_type_helpers;
pub(crate) use event_types;
pub(crate) use extract_attr_builder_fn_names;
pub(crate) use impl_attribute;
pub(crate) use impl_behavior_fn;
pub(crate) use impl_behavior_fn_update_with;
pub(crate) use parse_fn_args_as_bounds;
pub(crate) use parse_impl_with;
pub(crate) use parse_update_with;
pub(crate) use tags;
pub(crate) use traverse_traits;
pub(crate) use RenderHtml;
