#[macro_export]
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

#[macro_export]
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

#[macro_export]
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

#[macro_export]
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

#[macro_export]
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
            crate::define_behavior_fn_update_with! {
                update_with $update_with
                value($value)
                type($maybe_ty)
            }
        )?
    };
    ($fn_name:ident $fn_args:tt $fn_body_or_semi:tt) => {};
}

#[macro_export]
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
            crate::impl_behavior_fn_update_with! {
                update_with $update_with
                value($value)
                type($maybe_ty)
                trait_name $trait_name
            }
        )?
    };
    ($fn_name:ident $fn_args:tt $fn_body_or_semi:tt $trait_name:tt) => {};
}

#[macro_export]
macro_rules! behaviors {
    (expand_item $vis:vis $item_type:ident $item_name:ident {} {$($item_body_expanded:tt)*}) => {
        $vis $item_type $item_name {
            use crate::shims::prelude::*;

            $($item_body_expanded)*
        }
    };
    (
        extends($($extends:ident)*)
        $(special_super_traits($($($special_super_traits:ident),+ $(,)?)?))?
        $(special_inter_traits $special_inter_traits:tt)?
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
            $($($($special_super_traits<Renderer> +)+)?)?
        {
            $($($verbatim_trait_items)*)?

            $(crate::define_behavior_fn!{
                $fn_name $fn_args $fn_body_or_semi
            })*
        }

        // if `impl_for_web`
        #[cfg(feature = "web")]
        ::frender_common::expand! { if ($( ! $($($verbatim_trait_items_impl_web)*)?)?) {
            ::frender_common::expand! { if ($($( ! $($impl_for_web_only_for_types)*)?)?) {
                ::frender_common::expand! { while ($($($({$impl_for_web_only_for_types})*)?)?) {
                    prepend(impl<Renderer: ?Sized + crate::csr::web::Renderer> $trait_name<Renderer> for crate::csr::web::Node<)
                    append( > $($(where Self: $($special_super_traits<Renderer> + )+ )?)? {
                        $($($($verbatim_trait_items_impl_web)*)?)?

                        ::frender_common::expand! { while ($({$fn_name $fn_args $fn_body_or_semi})*) {
                            append( ($trait_name ($($($($impl_for_web_only_for_types)*)?)?)) )
                            wrap {}
                            prepend(crate::impl_behavior_fn!)
                        }}
                    })
                }}
            } else {
                impl<
                    Renderer: ?Sized + crate::csr::web::Renderer,
                    E: AsRef<::web_sys::$trait_name>
                > $trait_name<Renderer> for crate::csr::web::Node<E>
                where Self:
                    $($extends<Renderer> +)*
                    $($($($special_super_traits<Renderer> + )+ )?)?
                {
                    $($($($verbatim_trait_items_impl_web)*)?)?

                    $(crate::impl_behavior_fn! {
                        $fn_name $fn_args $fn_body_or_semi ($trait_name)
                    })*
                }
            }}
        }}
    };
}

#[macro_export]
macro_rules! behaviors_prelude {
    (expand_item $vis:vis $item_type:ident $item_name:ident {} $item_body_expanded:tt) => { $vis $item_type $item_name $item_body_expanded };
    (
        extends($($extends:ident)*)
        $(special_super_traits($($($special_super_traits:ident),+ $(,)?)?))?
        $(special_inter_traits $special_inter_traits:tt)?
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

#[macro_export]
macro_rules! behavior_type_traits {
    (expand_item $vis:vis $item_type:ident $item_name:ident {} $item_body_expanded:tt) => { $vis $item_type $item_name $item_body_expanded };
    (
        extends($($extends:ident)*)
        $(special_super_traits($($($special_super_traits:ident),+ $(,)?)?))?
        $(special_inter_traits $special_inter_traits:tt)?
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
            $($($($special_super_traits +)+)?)?
        {
            type $trait_name<Renderer: ?Sized + super::RenderHtml>: super::behaviors::$trait_name<Renderer>
                $(  + crate::convert::IdentityAs<Self::$extends             <Renderer>>)*
                $($($(+ crate::convert::IdentityAs<Self::$special_super_traits<Renderer>>)+)?)?
            ;

            fn from_identity_mut_root<Renderer: ?Sized + super::RenderHtml>(
                root: &mut super::attributes::$trait_name::helper_macro![
                    use_root_trait_name { super }
                    {
                        prepend(Self::)
                        append(<Renderer>)
                    }
                ]
            ) -> &mut Self::$trait_name<Renderer>;
        }
    };
}

#[macro_export]
macro_rules! tags {
    (expand_item $vis:vis $item_type:ident $item_name:ident {} $item_body_expanded:tt) => { $vis $item_type $item_name $item_body_expanded };
    (
        extends($($extends:ident)*)
        $(special_super_traits($($special_super_traits:ident),* $(,)?))?
        $(special_inter_traits $special_inter_traits:tt)?
        vis($vis:vis)
        trait_name($trait_name:ident)
        $(define(
            $(tags: ($($tags:ident),* $(,)?))?
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
                fn create_node<R: super::RenderHtml>(renderer: &mut R) -> <Self as super::behavior_type_traits::Node>::Node<R> {
                    renderer.$tags()
                }
            }
            impl crate::component::SsrIntrinsicComponent for $tags {
                // TODO: some components are void or self closing
            }
            impl crate::IntrinsicComponent for $tags {
                const INTRINSIC_TAG: &'static str = stringify!($tags);

                // type ElementType = super::::$trait_name;
                // TODO: some components are void or self closing
                type ElementTagType = crate::element_tag_types::EncloseAnyElement;
            }
        )*)?)?

        ::frender_common::expand! { while ($($($({$tags})*)?)?) {
            prepend( impl_all_traits { super } for )
            wrap {}
            prepend( super::attributes::$trait_name::helper_macro! )
        }}
    };
}

#[macro_export]
macro_rules! attributes {
    (expand_item $vis:vis $item_type:ident $item_name:ident {} $item_body_expanded:tt) => { $vis $item_type $item_name $item_body_expanded };
    (
        extends($($extends:ident)*)
        $(special_super_traits($($($special_super_traits:ident),+ $(,)?)?))?
        $(special_inter_traits($($special_inter_traits:ident),* $(,)?    ))?
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
                        $($($({$special_super_traits})+)?)?
                        $($({$special_inter_traits})*)?
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
                    prepend( crate::impl_attribute! )
                }
            }

            macro_rules! helper_macro {
                (use_root_trait_name $prepend_path:tt $commands:tt) => {
                    ::frender_common::expand! {
                        if ($($extends)*) {
                            ::frender_common::expand! {
                                $prepend_path
                                append(::attributes::$($extends)*::helper_macro! {
                                    use_root_trait_name $prepend_path $commands
                                })
                            }
                        } else {
                            ::frender_common::expand! {
                                {$trait_name}
                                do $commands
                            }
                        }
                    }
                };
                (impl $prepend_path:tt for $for_tag:ident ) => {
                    ::frender_common::expand! {
                        { impl }
                        append $prepend_path
                        append(::behavior_type_traits::$trait_name for $for_tag {
                            ::frender_common::expand! {
                                { type $trait_name<Renderer: ?Sized + }
                                append $prepend_path
                                append(
                                    ::RenderHtml > = Renderer::$for_tag;
                                )
                            }
                            ::frender_common::expand! {
                                { fn from_identity_mut_root<Renderer: ?Sized + }
                                append $prepend_path
                                append(
                                    ::RenderHtml >(
                                        root: &mut super::attributes::$trait_name::helper_macro![
                                            use_root_trait_name $prepend_path
                                            {
                                                prepend(Self::)
                                                append(<Renderer>)
                                            }
                                        ]
                                    ) -> &mut Self::$trait_name<Renderer> {
                                        root
                                    }
                                )
                            }
                        })
                    }
                };
                (impl_self_and_all_super $prepend_path:tt for $for_tag:ident) => {
                    ::frender_common::expand! {
                        $prepend_path
                        append(::attributes::$trait_name::helper_macro! {
                            impl $prepend_path for $for_tag
                        })
                    }
                    ::frender_common::expand! {
                        $prepend_path
                        append(::attributes::$trait_name::helper_macro! {
                            impl_all_super_traits $prepend_path for $for_tag
                        })
                    }
                };
                (impl_all_super_traits $prepend_path:tt for $for_tag:ident) => {
                    ::frender_common::expand! {
                        while (
                            $({$extends})* // actually there will be at most one
                        ) {
                            // Node
                            prepend {::attributes::}
                            prepend $prepend_path
                            append (::helper_macro!{
                                impl_self_and_all_super $prepend_path for $for_tag
                            })
                        }
                    }

                    ::frender_common::expand! {
                        while (
                            $($($({$special_super_traits})+)?)?
                            $($({$special_inter_traits})*)?
                        ) {
                            // Node
                            prepend {::attributes::}
                            prepend $prepend_path
                            append (::helper_macro!{
                                impl $prepend_path for $for_tag
                            })
                        }
                    }
                };
                (impl_all_traits $prepend_path:tt for $for_tag:ident) => {
                    ::frender_common::expand! {
                        $prepend_path
                        append(::attributes::$trait_name::helper_macro! {
                            impl $prepend_path for $for_tag
                        })
                    }
                    ::frender_common::expand! {
                        $prepend_path
                        append(::attributes::$trait_name::helper_macro! {
                            impl_all_super_traits $prepend_path for $for_tag
                        })
                    }
                };
            }
            pub(crate) use helper_macro;
        }
    };
}

#[macro_export]
macro_rules! impl_attribute {
    ($fn_name:ident ($value:ident : event![
        $event_trait_name:ident,
        $event_type_name:literal,
        $event_type_ident:ident,
        $event_type_listener_ident:ident $(,)?
    ]); $trait_name:tt) => {
        crate::impl_bounds! {
            self::$fn_name(
                #[event(super::super::event_type_helpers::$fn_name)]
                bounds as crate::impl_bounds::MaybeHandleEvent,
                element as $trait_name,
                attr_name = $event_type_name,
            )
        }
    };
    ($fn_name:ident ($value:ident : maybe![$($maybe_ty:tt)*]) ; $trait_name:ident) => {
        crate::impl_attribute! {$fn_name ($value : maybe![$($maybe_ty)*]) {} $trait_name }
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
                                crate::parse_update_with!(match $($update_with)? {
                                    simple => {
                                        prepend(|el: &mut ET::$trait_name<Renderer>, renderer: &mut _, _, v: &_| el.)
                                        append( (renderer, v) )
                                    }
                                    impl_with => {
                                        append( as update(value($value) element_type(ET::$trait_name<Renderer>)))
                                        wrap {}
                                        prepend( crate::parse_impl_with! )
                                    }
                                })
                        } else {
                            crate::impl_bounds::MaybeValue::csr::default_update
                        }
                    },
                    remove: ::frender_common::expand! {
                        if ($($update_with)?) {
                            crate::parse_update_with!(match $($update_with)? {
                                simple => {
                                    reset {}
                                    {crate::impl_bounds::MaybeValue::csr::default_remove}
                                }
                                impl_with => {
                                    append( as remove(element_type(ET::$trait_name<Renderer>)))
                                    wrap {}
                                    prepend( crate::parse_impl_with! )
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
                                crate::parse_update_with!(match $($update_with)? {
                                    simple => {
                                        prepend(|el: &mut ET::$trait_name<Renderer>, renderer: &mut _, _, v: &_| el.)
                                        append( (renderer, *v) )
                                    }
                                    impl_with => {
                                        append( as update(value(&$value) element_type(ET::$trait_name<Renderer>)))
                                        wrap {}
                                        prepend( crate::parse_impl_with! )
                                    }
                                })
                        } else {
                            crate::impl_bounds::MaybeValue::csr::default_update
                        }
                    },
                    remove: ::frender_common::expand! {
                        if ($($update_with)?) {
                            crate::parse_update_with!(match $($update_with)? {
                                simple => {
                                    reset {}
                                    {crate::impl_bounds::MaybeValue::csr::default_remove}
                                }
                                impl_with => {
                                    append( as remove(element_type(ET::$trait_name<Renderer>)))
                                    wrap {}
                                    prepend( crate::parse_impl_with! )
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

#[macro_export]
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
        extends($($extends:ident)*)
        $(special_super_traits($($($special_super_traits:ident),+ $(,)?)?))?
        $(special_inter_traits $special_inter_traits:tt)?
        vis($vis:vis)
        trait_name($trait_name:ident)
        $(define(
            $(tags: ($($tags:ident),* $(,)?))?
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
            type $tags: super::behaviors::$trait_name<Self>;
            fn $tags(&mut self) -> Self::$tags;
        )*)?)?
    };
}

#[macro_export]
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
                    $(special_inter_traits! $special_inter_traits:tt;)?
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
        $crate::expand_nested_traits! {
            {
                $($expanded)*
                $($({
                    extends $extends
                    $(special_super_traits $special_super_traits)?
                    $(special_inter_traits $special_inter_traits)?
                    vis($vis)
                    trait_name($trait_name)
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

#[macro_export]
macro_rules! define_item_and_traverse_traits {
    (
        $t:tt // {}
        $($macro_name:ident ($vis:vis $item_type:ident $item_name:ident $item_body:tt))*
    ) => {
        $(
            $crate::$macro_name! {
                expand_item $vis $item_type $item_name $item_body
                {
                    $crate::expand! {
                        $t for_each {
                            wrap{}
                            prepend($crate::$macro_name!)
                        }
                    }
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! def_intrinsic_component_props {
    (
        #[expand_html_traits]
        $(#$expand_html_traits_attrs:tt)*
        use $expand_html_traits:ident;

        mod items {$(
            #[$item_macro:ident]
            $item_vis:vis $item_type:ident $item_name:ident { $($item_body:tt)* }
        )*}

        $($t:tt)*
    ) => {
        crate::expand_nested_traits! {
            {}{{{extends()}($($t)*)}} do {
                wrap {} // { ... }
                duplex_concat ({
                    append( do $commands ) // { ... } do $commands
                    wrap {}
                    prepend( $crate::expand! ) // $crate::expand!{ { ... } do $commands }
                    wrap {}
                    prepend ( ($commands:tt) => )
                    wrap {} // { { (...) => { ... } } }
                    prepend {
                        $(#$expand_html_traits_attrs)*
                        macro_rules! $expand_html_traits
                    }
                }{
                    append(
                        $($item_macro (
                            $item_vis $item_type $item_name { $($item_body)* }
                        ))*
                    )
                    wrap {}
                    prepend( crate::define_item_and_traverse_traits! )
                })

            }
        }
    };
}

#[macro_export]
macro_rules! event_types {
    (expand_item $vis:vis $item_type:ident $item_name:ident {} $item_body_expanded:tt) => { $vis $item_type $item_name $item_body_expanded };
    (
        extends($($extends:ident)*)
        $(special_super_traits($($($special_super_traits:ident),+ $(,)?)?))?
        $(special_inter_traits $special_inter_traits:tt)?
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
        use super::behaviors::$trait_name;

        $(
            crate::event_type! {
                $fn_name $fn_args $fn_body_or_semi $trait_name
            }
        )*
    };
}

#[macro_export]
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

#[macro_export]
macro_rules! event_type_helpers {
    (expand_item $vis:vis $item_type:ident $item_name:ident {} $item_body_expanded:tt) => { $vis $item_type $item_name $item_body_expanded };
    (
        extends($($extends:ident)*)
        $(special_super_traits($($($special_super_traits:ident),+ $(,)?)?))?
        $(special_inter_traits $special_inter_traits:tt)?
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
            crate::event_type_helper! {
                $fn_name $fn_args $fn_body_or_semi $trait_name { super::super::behaviors }
            }
        )*
    };
}

#[macro_export]
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

#[macro_export]
macro_rules! props {
    (expand_item $vis:vis $item_type:ident $item_name:ident ; $item_body_expanded:tt) => { $vis $item_type $item_name $item_body_expanded };
    (
        extends $extends:tt
        $(special_super_traits $special_super_traits:tt)?
        $(special_inter_traits $special_inter_traits:tt)?
        vis($vis:vis)
        trait_name($trait_name:ident)
        $($rest:ident $rest_paren:tt)*
    ) => {
        $crate::define_props!(
            $vis mod $trait_name {
                $crate::define_props_builders! {
                    extends $extends
                    $(special_super_traits $special_super_traits)?
                    $(special_inter_traits $special_inter_traits)?
                    vis($vis)
                    trait_name($trait_name)
                    $($rest $rest_paren)*
                }
            }
        );
    };
}

#[macro_export]
macro_rules! props_without_builders {
    (expand_item $vis:vis $item_type:ident $item_name:ident ; $item_body_expanded:tt) => { $vis $item_type $item_name $item_body_expanded };
    (
        extends($($extends:ident)*)
        $(special_super_traits($($($special_super_traits:ident),+ $(,)?)?))?
        $(special_inter_traits($($special_inter_traits:ident),* $(,)?))?
        vis($vis:vis)
        trait_name($trait_name:ident)
        $($rest:ident $rest_:tt)*
    ) => {
        $crate::define_props!(
            $vis mod $trait_name;
        );
    };
}

#[macro_export]
macro_rules! define_props {
    ($vis:vis mod $trait_name:ident ;) => {
        $crate::define_props! { $vis mod $trait_name {} }
    };
    ($vis:vis mod $trait_name:ident {$($include:tt)*}) => {
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

            pub use super::super::attributes::$trait_name::attributes;

            $($include)*
        }

        #[allow(non_snake_case)]
        #[inline(always)]
        $vis fn $trait_name() -> $trait_name::Building {
            $trait_name::Building(Default::default())
        }
    };
}

#[macro_export]
macro_rules! props_builders {
    (expand_item $vis:vis $item_type:ident $item_name:ident ; $item_body_expanded:tt) => { $vis $item_type $item_name $item_body_expanded };
    (
        extends $extends:tt
        $(special_super_traits $special_super_traits:tt)?
        $(special_inter_traits $special_inter_traits:tt)?
        vis $vis:tt
        trait_name($trait_name:ident)
        $($rest:ident $rest_paren:tt)*
    ) => {
        mod $trait_name {
            use super::super::props::$trait_name::*;

            $crate::define_props_builders! {
                extends $extends
                $(special_super_traits $special_super_traits)?
                $(special_inter_traits $special_inter_traits)?
                vis $vis
                trait_name($trait_name)
                $($rest $rest_paren)*
            }
        }
    };
}

#[macro_export]
macro_rules! define_props_builders {
    (
        extends($($extends:ident)*)
        $(special_super_traits($($($special_super_traits:ident),+ $(,)?)?))?
        $(special_inter_traits($($special_inter_traits:ident),* $(,)?))?
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
        macro_rules! impl_props_builder_fns {
            // only children is overridable
            (impl_children) => {
                $crate::extract_only_children_or! {
                    {$(
                        {
                            $fn_name {
                                $(#$fn_attr)*
                                fn $fn_name $fn_args $fn_body_or_semi
                            }
                        }
                    )*}
                    {
                        // there is a children
                        wrap {}
                        prepend( $crate::define_fn_children! )
                    }
                    {
                        {$({$extends})*}
                        get_or_exit {0}
                        prepend( super::super:: )
                        append( ::impl_props_builder_fns! { impl_children } )
                    }
                }
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
                    $crate::parse_fn_args_as_bounds! {
                        $fn_args
                        do {
                            wrap()
                            prepend(
                                prepend(
                                    $(#$fn_attr)*
                                    pub fn
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
                            prepend( $crate::extract_attr_builder_fn_names! )
                        }
                    }
                )*
            };
            (impl_self_and_extended_attrs) => {
                super::super::$trait_name::impl_props_builder_fns! { impl_self_attrs }
                $crate::expand! {
                    {$({$extends})*}
                    get_or_exit {0}
                    prepend( super::super:: )
                    append( ::impl_props_builder_fns! { impl_self_and_extended_attrs } )
                }
            };
            (impl_all_attrs) => {
                super::super::$trait_name::impl_props_builder_fns! { impl_self_and_extended_attrs }

                $crate::expand! {
                    while (
                        $($($({$special_super_traits})+)?)?
                        $($({$special_inter_traits})*)?
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
                super::impl_props_builder_fns! { impl_children }
            }

            impl<Children, Attrs> super::Building<Children, Attrs> {
                super::impl_props_builder_fns! { impl_all_attrs }
            }
        }
    };
}

#[macro_export]
macro_rules! components {
    (expand_item $vis:vis $item_type:ident $item_name:ident ; $item_body_expanded:tt) => { $vis $item_type $item_name $item_body_expanded };
    (
        extends($($extends:ident)*)
        $(special_super_traits($($($special_super_traits:ident),+ $(,)?)?))?
        $(special_inter_traits($($special_inter_traits:ident),* $(,)?))?
        vis($vis:vis)
        trait_name($trait_name:ident)
        $(define(
            $(tags: ($($tags:ident),* $(,)?))?
            $(,)?
        ))?
        $(verbatim_trait_items $verbatim_trait_items:tt)?
        $(impl_for_web $impl_for_web:tt)?
        fns $fns:tt
    ) => {
        $crate::expand! {
            while ($($($({$tags})*)?)?) {
                prepend( $trait_name $vis )
                wrap {}
                prepend( $crate::define_component! )
            }
        }
    };
}

/// `children` is excluded
#[macro_export]
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

#[macro_export]
macro_rules! parse_fn_args_as_bounds {
    (($value:ident : event![
        $event_trait_name:ident,
        $event_type_name:literal,
        $event_type_ident:ident,
        $event_type_listener_ident:ident $(,)?
    ]) do $commands:tt) => {
        $crate::expand! {
            { $crate::impl_bounds::MaybeHandleEvent::Bounds::<dyn $crate::event::$event_trait_name> }
            do $commands
        }
    };
    (($value:ident : maybe![&$maybe_ty:ty]) do $commands:tt) => {
        $crate::expand! {
            { $crate::impl_bounds::MaybeValue::Bounds::<$maybe_ty> }
            do $commands
        }
    };
    (($value:ident : maybe![$maybe_ty:ty]) do $commands:tt) => {
        $crate::expand! {
            { $crate::impl_bounds::MaybeValue::Bounds::<$maybe_ty> }
            do $commands
        }
    };
    (($value:ident : children![impl $($bounds:tt)+]) do $commands:tt) => {
        $crate::expand! {
            { $($bounds)+ }
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
    ($fn_args:tt do $commands:tt) => {
        compile_error! { stringify!($fn_args) }
    };
}

#[macro_export]
macro_rules! extract_only_children_or {
    ($t:tt $do:tt $or:tt) => {
        $crate::extract_only_children_or! {
            @ $t
            []
            { $do $or }
        }
    };
    (@{ { children $children:tt } $($t:tt)* } [$($resolved_children:tt)*] $do_or:tt) => {
        $crate::extract_only_children_or! {
            @{$($t)*}
            [$($resolved_children)* $children]
            $do_or
        }
    };
    (@{ { $other_name:ident $other:tt } { children $children:tt } $($t:tt)* } [$($resolved_children:tt)*] $do_or:tt) => {
        $crate::extract_only_children_or! {
            @{$($t)*}
            [$($resolved_children)* $children]
            $do_or
        }
    };
    (@{ { $other_name:ident $other:tt } { $other_name1:ident $other1:tt } { children $children:tt } $($t:tt)* } [$($resolved_children:tt)*] $do_or:tt) => {
        $crate::extract_only_children_or! {
            @{$($t)*}
            [$($resolved_children)* $children]
            $do_or
        }
    };
    (@{ { $other_name:ident $other:tt } { $other_name1:ident $other1:tt } { $other_name2:ident $other2:tt } $($t:tt)* } $resolved_children:tt $do_or:tt) => {
        $crate::extract_only_children_or! {
            @{$($t)*}
            $resolved_children
            $do_or
        }
    };
    (@{ { $other_name:ident $other:tt } { $other_name1:ident $other1:tt } $($t:tt)* } $resolved_children:tt $do_or:tt) => {
        $crate::extract_only_children_or! {
            @{$($t)*}
            $resolved_children
            $do_or
        }
    };
    (@{ { $other_name:ident $other:tt } $($t:tt)* } $resolved_children:tt $do_or:tt) => {
        $crate::extract_only_children_or! {
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

#[macro_export]
macro_rules! define_fn_children {
    (
        $(#$fn_attr:tt)*
        fn $fn_name:ident($v:ident : children![impl $($bounds:tt)+] $(,)?);
    ) => {
        pub fn $fn_name<Children: $($bounds)+>(self, $v: Children) -> super::Building<Children, Attrs> {
            super::Building(super::Data {
                props: self.0.props.children($v),
            })
        }
    };
}

#[macro_export]
macro_rules! define_component {
    (
        $props_name:ident
        $vis:vis
        $component_name:ident
        // $component_options_or_semi:tt
    ) => {
        $vis mod $component_name {
            pub use super::super::props::$props_name as Props;

            pub type Data<Children, Props> = $crate::component::IntrinsicElement<
                ComponentType,
                super::super::props::$props_name::Data<Children, Props>,
            >;

            pub use Props::{
                prelude, Building,
            };

            pub use super::super::tags::$component_name as ComponentType;

            pub fn build<Children, Props>(
                building: Building<Children, Props>,
            ) -> Data<Children, Props> {
                $crate::component::IntrinsicElement(
                    self::ComponentType,
                    self::Props::build(building),
                )
            }

            pub use build as build_element;
            pub use build as valid;
        }

        $vis fn $component_name (
        ) -> super::props::$props_name::Building {
            super::props::$props_name ()
        }
    };
}
