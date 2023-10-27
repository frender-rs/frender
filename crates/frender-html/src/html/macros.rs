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
    ($fn_name:ident $fn_args:tt ;) => {};
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
                <crate::event_types::event_types::$fn_name as crate::event::HasEventTypeName>::EVENT_TYPE_NAME,
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
    ($fn_name:ident $fn_args:tt ; $trait_name:tt) => {};
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
            type $trait_name<Renderer: ?Sized + super::RenderHtml>: super::$mod_behaviors::$trait_name<Renderer>;
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
        )*)?)?

        ::frender_common::expand! { while ($($($({$tags})*)?)?) {
            prepend( impl_all_traits { super } for )
            wrap {}
            prepend( super::$mod_attributes::$trait_name! )
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
        pub(crate) use $trait_name::$trait_name;

        $vis mod $trait_name {
            $(
                // #[derive(Debug)]
                pub struct $fn_name<V>(pub V);
            )*

            macro_rules! $trait_name {
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
                            append (!{
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
                        append(::$mod_attributes::$trait_name! {
                            impl $prepend_path for $for_tag
                        })
                    }
                    ::frender_common::expand! {
                        $prepend_path
                        append(::$mod_attributes::$trait_name! {
                            impl_all_super_traits $prepend_path for $for_tag
                        })
                    }
                };
            }
            pub(crate) use $trait_name;
        }
    };
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

pub(crate) use attributes;
pub(crate) use behavior_type_traits;
pub(crate) use behaviors;
pub(crate) use behaviors_prelude;
pub(crate) use def_intrinsic_component_props;
pub(crate) use define_behavior_fn;
pub(crate) use define_behavior_fn_update_with;
pub(crate) use define_item_and_traverse_traits;
pub(crate) use impl_behavior_fn;
pub(crate) use impl_behavior_fn_update_with;
pub(crate) use tags;
pub(crate) use traverse_traits;
pub(crate) use RenderHtml;
