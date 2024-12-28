macro_rules! allow_children {
    (
        for_marker! { $marker:ty }
        $(#$fn_attr:tt)*
        fn $fn_name:ident($v:ident : children![$(impl $($bounds:tt)+)?] $(,)?);
    ) => {
        impl<C: $($($bounds)+)?> AllowChildren<C> for $marker {}
    };
}

macro_rules! allow_prop_ignoring_children {
    // children
    (
        for_marker! { $marker:ty }
        $(#$fn_attr:tt)*
        fn children $fn_args:tt $fn_body_or_semi:tt
    ) => {};
    // attributes or attributes_with_pinned_state
    (
        for_marker! { $marker:ty }
        $(#$fn_attr:tt)*
        fn $fn_name:ident $fn_args:tt $fn_body_or_semi:tt
    ) => {
        super::prop_markers::expand_if_conflicted_name_or_else! { $fn_name {
            // $fn_name is a conflicted name
            // allow attribute name
            impl AllowAttributeName<conflicted_names::$fn_name> for $marker {
                type AttributeMarker = prop_markers::$fn_name;
            }
        } {}}

        crate::macros::parse_fn_args_as_whether_pinned_state! { $fn_args {
            AllowAttributeWithPinnedState
        } {
            AllowAttribute
        } {
            prepend {
                impl
            }
            append {
                <prop_markers::$fn_name> for $marker {}
            }
        }}
    };
}

macro_rules! allow_props_of_ancestor_ignoring_children_if_not_ancestor_of {
    (
        for_marker! $for_marker:tt // wrapped in {}
        $ancestor:ident
        if_not_ancestor_of $if_not_ancestor_of:tt // wrapped in {}
    ) => {
        crate::macros::define_props_macro::check_is_ancestor_of_any! {
            $if_not_ancestor_of
            $ancestor
            {{}}
            {{
                props_macros::$ancestor! { for_each_prop_of_self_without_fn_attrs {
                    prepend { for_marker! $for_marker }
                    wrap {}
                    prepend { crate::macros::props_builders::allow_prop_ignoring_children! }
                }}
            }}
        }
    };
}

macro_rules! props_builders {
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
        // allow children
        crate::macros::extract_only_children_or! {
            {$(
                {
                    $fn_name {
                        $(#$fn_attr)*
                        fn $fn_name $fn_args $fn_body_or_semi
                    }
                }
            )*}
            // If self defines fn children:
            {
                prepend( for_marker! { super::markers::$trait_name } )
                wrap {} prepend( crate::macros::props_builders::allow_children! )
            }
            // Else:
            {{
                // inherit fn children from main ancestor
                $(
                    impl<C> AllowChildren<C> for super::markers::$trait_name
                        where super::markers::$extends: AllowChildren<C>
                    {}
                )*
            }}
        }

        // allow attributes from main ancestors ($extends)
        $(
            impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::$trait_name
                where super::markers::$extends: AllowAttribute<AttrMarker>
            {}

            impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::$trait_name
                where super::markers::$extends: AllowAttributeWithPinnedState<AttrMarker>
            {}

            impl<AttrName> AllowAttributeName<AttrName> for super::markers::$trait_name
                where super::markers::$extends: AllowAttributeName<AttrName>
            {
                type AttributeMarker = <super::markers::$extends as AllowAttributeName<AttrName>>::AttributeMarker;
            }
        )*

        const _: () = {
            #[allow(unused_imports)]
            use super::{props::$trait_name as props, prop_markers::$trait_name as prop_markers};

            // allow attributes from self ($trait_name)
            crate::macros::props_builders::allow_props_of_ancestor_ignoring_children_if_not_ancestor_of! {
                for_marker! { super::markers::$trait_name }
                $trait_name
                if_not_ancestor_of {}
            }

            // allow attributes from other ancestors ($special_super_traits) excluding those are ancestors of main ancestors ($extends)
            super::props_macros::$trait_name! { for_each_other_ancestor {
                prepend {
                    for_marker! { super::markers::$trait_name }
                }
                append {
                    if_not_ancestor_of {$($extends)*}
                }
                wrap {} prepend { crate::macros::props_builders::allow_props_of_ancestor_ignoring_children_if_not_ancestor_of! }
            }}

            // builder methods of this trait
            $(
                super::prop_markers::expand_if_conflicted_name_or_else! { $fn_name {} {
                    // $fn_name is not a conflicted name
                    crate::macros::parse_fn_args_as_whether_pinned_state! { $fn_args {
                        AllowAttributeWithPinnedState
                    } {
                        AllowAttribute
                    } {
                        prepend {
                            impl<M:
                        }
                        append {
                            <prop_markers::$fn_name>, C, A, P> Intrinsic<M, C, A, P> {
                                crate::macros::parse_fn_args_as_bounds! {
                                    $fn_args
                                    do {
                                        prepend( <V: )
                                        append(
                                            >(self, value: V) -> $crate::macros::parse_fn_args_as_whether_pinned_state![ $fn_args {
                                                Intrinsic<M, C, A, (P, props::$fn_name<V>)>
                                            } {
                                                Intrinsic<M, C, (A, props::$fn_name<V>), P>
                                            } {} ] {
                                                crate::macros::parse_fn_args_as_whether_pinned_state!( $fn_args {
                                                    Self::with_attribute_with_pinned_state_appended
                                                } {
                                                    Self::with_attribute_appended
                                                } {})(
                                                    self,
                                                    props::$fn_name(value),
                                                )
                                            }
                                        )
                                        wrap()
                                        prepend(
                                            prepend(
                                                $(#$fn_attr)*
                                                $vis fn
                                            )
                                            append
                                        )
                                        // $crate::extract_attr_builder_fn_names! { {$fn_name $fn_body_or_semi} do { for_each {...} } }
                                        wrap {}
                                        prepend( for_each )
                                        // $crate::extract_attr_builder_fn_names! { {$fn_name $fn_body_or_semi} do {...} }
                                        wrap {}
                                        prepend( {$fn_name $fn_body_or_semi} do )
                                        // $crate::extract_attr_builder_fn_names! { ... }
                                        wrap {} prepend( crate::macros::extract_attr_builder_fn_names! )
                                    }
                                }
                            }
                        }
                    }}
                }}
            )*
        };
    };
}

pub(crate) use {allow_children, allow_prop_ignoring_children, allow_props_of_ancestor_ignoring_children_if_not_ancestor_of, props_builders};
