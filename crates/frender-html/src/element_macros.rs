#[macro_export]
macro_rules! expand_a_or_b {
    ($({$($prefix:tt)*})?[][$($b:tt)*]$({$($suffix:tt)*})?) => {
        $($($prefix)*)?
        $($b)*
        $($($suffix)*)?
    };
    ($({$($prefix:tt)*})?[$($a:tt)+][$($b:tt)*]$({$($suffix:tt)*})?) => {
        $($($prefix)*)?
        $($a)+
        $($($suffix)*)?
    };
}

#[macro_export]
macro_rules! __impl_mod_overwrite {
    (+ $fields:tt) => {
        $crate::__impl_mod_overwrite! {
            @ $fields $fields
        }
    };
    (- $field:ident [$($fields:ident)*]) => {
        dyn super::Types<
            $(
                $fields = $field![$fields TypeDefs Value],
            )*
        >
    };
    (@ [$($field:ident)*] $fields:tt) => {
        $(
            macro_rules! $field {
                ($field $base:ident $new_ty:ident) => {
                    $new_ty
                };
                ($other_field:ident $base:ident $new_ty:ident) => {
                    <$base as super::Types>::$other_field
                };
            }

            pub type $field<TypeDefs, Value> = $crate::__impl_mod_overwrite![
                - $field $fields
            ];
        )*
    };
}

#[macro_export]
macro_rules! __impl_builder_fn_body {
    (
        $name:ident
        $this:ident
        $field:ident = $field_value:expr;
        {$($fields:ident)*}
    ) => {
        let __tmp_value = $field_value;

        let $name::Data {
            $($fields),*
        } = $this.0;

        let _ = $field;

        let $field = __tmp_value;

        $name::Building(
            $name::Data {
                $($fields),*
            }
        )
    };
}

#[macro_export]
macro_rules! __impl_replace_fn_body {
    (
        $name:ident
        $this:ident
        $field:ident = $field_value:expr;
        {$($fields:ident)*}
    ) => {
        let __tmp_value = $field_value;

        let $name::Data {
            $($fields),*
        } = $this.0;

        let $field = __tmp_value($field);

        $name::Replacing(
            $name::Data {
                $($fields),*
            }
        )
    };
}

#[macro_export]
macro_rules! __impl_builder_fns {
    (
        $name:ident {$(
            $(#$field_attr:tt)*
            $field:ident $([$($bounds:tt)+])?
        )*}
        $fields:tt
    ) => {
        impl<TypeDefs: $name::Types + ?::core::marker::Sized>
        $name::Building<TypeDefs> {$(
            $(#$field_attr)*
            pub fn $field<V $(: $($bounds)+)?>(
                self,
                $field: V,
            ) -> $name::Building<
                $name::overwrite::$field<TypeDefs, V>,
            > {
                $crate::__impl_builder_fn_body! {
                    $name self
                    $field = $field;
                    $fields
                }
            }
        )*}
    };
}

#[macro_export]
macro_rules! __impl_replace_fns {
    (
        $name:ident {$(
            $(#$field_attr:tt)*
            $field:ident $([$($bounds:tt)+])?
        )*}
        $fields:tt
    ) => {
        impl<TypeDefs: $name::Types>
        $name::Replacing<TypeDefs> {$(
            $(#$field_attr)*
            pub fn $field<V $(: $($bounds)+)?>(
                self,
                $field: impl FnOnce(TypeDefs::$field) -> V,
            ) -> $name::Replacing<
                $name::overwrite::$field<TypeDefs, V>,
            > {
                $crate::__impl_replace_fn_body! {
                    $name self
                    $field = $field;
                    $fields
                }
            }
        )*}
    };
}

#[macro_export]
macro_rules! __impl_def_intrinsic_component_props_one {
    (
        $vis:vis struct $name:ident (
            $dom_element_ty:ty
            $(: $($component_name:ident),+ $(,)?)?
        ) {$(
            $(#$field_attr:tt)*
            $field:ident
            $(
                ? $maybe_ty:ty $({
                    $($prop_name:literal)?
                    $([update $maybe_update_element:ident $($maybe_update:tt)+ ])?
                    $([remove $maybe_remove_element:ident $($maybe_remove:tt)+ ])?
                    $($element_method:ident $($deref_star:tt)?)?
                })?
            )?
            $(
                $([$($bounds:tt)+])?
                : $initial_ty:ty
                = $initial_value:expr
                => $defs:tt
            )?
        ,)*}
    ) => {
        $crate::__impl_def_intrinsic_component_props! {
            $vis struct $name ($dom_element_ty) {$(
                $(#$field_attr)*
                $field
                $(
                    [::frender_dom::props::MaybeUpdateValue<$maybe_ty>]: () = () => {
                        dom {
                            state
                                < <TypeDefs::$field as ::frender_dom::props::MaybeUpdateValue<$maybe_ty>>::State >
                            impl { data, dom_element, state, element, .. } {
                                #[allow(unused)]
                                const ATTR_NAME: &::core::primitive::str = $crate::expand_a_or_b!(
                                    [$($($prop_name)?)?]
                                    [::core::stringify!($field)]
                                );
                                <TypeDefs::$field as ::frender_dom::props::MaybeUpdateValue<$maybe_ty>>::maybe_update_value(
                                    data,
                                    state,
                                    $crate::expand_a_or_b!(
                                        [$($(match element { $maybe_update_element => $($maybe_update)+ })?)?]
                                        [|v| $crate::expand_a_or_b!(
                                            [$($(element.$element_method($($deref_star)? v))?)?]
                                            [$crate::props::UpdateElementAttribute::update_element_attribute(v, dom_element, ATTR_NAME)]
                                        )]
                                    ),
                                    $crate::expand_a_or_b!(
                                        [$($(match element { $maybe_remove_element => $($maybe_remove)+ })?)?]
                                        [|| dom_element.remove_attribute(ATTR_NAME).unwrap()]
                                    )
                                )
                            }
                        }
                    }
                )?

                $(
                    $([$($bounds)+])?
                    : $initial_ty
                    = $initial_value => $defs
                )?
            ),*}
        }

        $($(
            $crate::__impl_def_intrinsic_component! {
                $component_name
                $name
                $dom_element_ty
            }
        )+)?
    };
}

#[macro_export]
macro_rules! __impl_def_intrinsic_component_props_inherit {
    (
        {$($prepend_fields:tt)*}
        [
            $vis:vis struct $name:ident $dom_element_ty:tt
            {$($fields:tt)*}
            $($inherit:tt)*
        ]
    ) => {
        $crate::def_intrinsic_component_props! {
            $vis struct $name $dom_element_ty
            {
                $($prepend_fields)*
                $($fields)*
            }
            $($inherit)*
        }
    };
}

#[macro_export]
macro_rules! def_intrinsic_component_props {
    (
        $vis:vis struct $name:ident $dom_element_ty:tt
        $fields:tt
        $($inherit:tt)*
    ) => {
        $crate::__impl_def_intrinsic_component_props_one! {
            $vis struct $name $dom_element_ty $fields
        }

        $(
            $crate::__impl_def_intrinsic_component_props_inherit! {
                $fields
                $inherit
            }
        )*
    };
}

#[macro_export]
macro_rules! __impl_def_intrinsic_component_props {
    (
        $vis:vis struct $name:ident ($dom_element_ty:ty) {$(
            $(#$field_attr:tt)*
            $field:ident $([$($bounds:tt)+])?
            : $initial_ty:ty
            = $initial_value:expr => {
                dom {
                    $(bounds [$($dom_bounds:tt)+])?
                    $(
                        state $($pin:ident)?
                        <$dom_state_ty:ty>
                        $(:[$($dom_state_bounds:tt)+]=($dom_state_init:expr))?
                    )?
                    impl $dom_pat:tt $dom_impl:tt
                }
            }
        ),* $(,)?}
    ) => {
        #[allow(non_snake_case)]
        $vis mod $name {
            pub mod prelude {}

            pub mod overwrite {
                #![allow(non_camel_case_types)]

                $crate::__impl_mod_overwrite! { +[$($field)*] }
            }

            mod trait_types {
                #[allow(unused_imports)]
                use super::super::*;
                #[allow(non_camel_case_types)]
                pub trait Types {$(
                    type $field $(: $($bounds)+)? ;
                )*}
            }

            pub use trait_types::Types;
            pub use trait_types::Types as ValidTypes;

            pub mod data_struct {
                #[non_exhaustive]
                pub struct $name<TypeDefs: super::Types + ?::core::marker::Sized> {
                    $(
                        pub $field : TypeDefs::$field,
                    )*
                }
            }

            pub use data_struct::$name as Data;

            pub struct Building<TypeDefs: ?::core::marker::Sized + Types>(
                pub Data<TypeDefs>
            );

            pub struct Replacing<TypeDefs: ?::core::marker::Sized + Types>(
                pub Data<TypeDefs>
            );

            mod types_initial {
                #[allow(unused_imports)]
                use super::super::*;
                pub type TypesInitial = dyn super::Types<$(
                    $field = $initial_ty,
                )*>;
            }
            pub use types_initial::TypesInitial;

            pub type DataInitial = Data<TypesInitial>;

            pub mod render_state {
                #[allow(non_camel_case_types)]
                pub trait RenderStateTypes {$(
                    $crate::expand_a_or_b! {
                        { type $field : }
                        [$($($($dom_state_bounds)+)?)?]
                        [::core::default::Default]
                        {;}
                    }
                )*}
                ::pin_project_lite::pin_project! {
                    #[project = RenderStateProj]
                    pub struct RenderState<TypeDefs: RenderStateTypes>
                    where TypeDefs: ?::core::marker::Sized {
                        $(
                            $($(#[$pin])?)?
                            pub $field: TypeDefs::$field,
                        )*
                    }
                }

                impl<
                    TypeDefs: ?::core::marker::Sized + RenderStateTypes,
                > RenderState<TypeDefs> {
                    #[inline]
                    pub(crate) fn pin_project(self: ::core::pin::Pin<&mut Self>) -> RenderStateProj<'_, TypeDefs> {
                        self.project()
                    }
                }

                impl<
                    TypeDefs: ?::core::marker::Sized + RenderStateTypes,
                > ::core::default::Default for RenderState<TypeDefs> {
                    fn default() -> Self {
                        Self {
                            $(
                                $field: $crate::expand_a_or_b![
                                    [$($($dom_state_init)?)?]
                                    [::core::default::Default::default()]
                                ],
                            )*
                        }
                    }
                }
            }

            #[inline]
            pub fn build<TypeDefs: ?::core::marker::Sized + Types>(building: Building<TypeDefs>) -> Data::<TypeDefs> {
                building.0
            }
        }

        $crate::__impl_builder_fns! {
            $name {$(
                $(#$field_attr)*
                $field $([$($bounds)+])?
            )*}
            {$($field)*}
        }

        $crate::__impl_replace_fns! {
            $name {$(
                $(#$field_attr)*
                $field $([$($bounds)+])?
            )*}
            {$($field)*}
        }

        impl<
            TypeDefs: ?::core::marker::Sized + $name::Types,
        > $crate::props::UpdateElement<$dom_element_ty> for $name::Data<TypeDefs>
        where
            $(
                $(TypeDefs::$field : $($dom_bounds)+)?
            )*
        {
            type State = $name::render_state::RenderState<
                dyn $name::render_state::RenderStateTypes<$(
                    $field = $crate::expand_a_or_b![[$($dom_state_ty)?][()]],
                )*>
            >;

            fn update_element(this: Self, element: &$dom_element_ty, children_ctx: &mut ::frender_dom::Dom, state: ::core::pin::Pin<&mut Self::State>) {
                let state = state.pin_project();

                let dom_element: &::web_sys::Element = element.as_ref();

                $(
                    #[allow(unused_variables)]
                    match ($crate::props::FieldData {
                        data: this.$field,
                        state: state.$field,
                        element,
                        dom_element,
                        children_ctx: &mut *children_ctx,
                    }) {
                        $crate::props::FieldData $dom_pat => $dom_impl
                    }
                )*
            }
        }

        #[allow(non_snake_case)]
        $vis fn $name () -> $name::Building<$name::TypesInitial> {
            $name::Building(
                $name::Data {
                    $(
                        $field : $initial_value,
                    )*
                }
            )
        }
    };
}

#[macro_export]
macro_rules! __impl_def_intrinsic_component {
    (
        $component_name:ident
        $props_name:ident
        $dom_element_ty:ty
    ) => {
        ::bg::builder! {
            pub struct $component_name($props_name);

            pub use build as build_element;
        }

        impl<TypeDefs: ?::core::marker::Sized + $component_name::Types>
            ::frender_core::UpdateRenderState<::frender_dom::Dom>
            for $component_name::Data<TypeDefs>
        where
            $props_name::Data<TypeDefs>: $crate::props::UpdateElement<$dom_element_ty>,
        {
            type State =
                $crate::props::IntrinsicComponentRenderState<
                    $dom_element_ty,
                    <$props_name::Data<TypeDefs> as $crate::props::UpdateElement<
                        $dom_element_ty,
                    >>::State,
                >;

            fn update_render_state(
                self,
                ctx: &mut ::frender_dom::Dom,
                state: ::core::pin::Pin<&mut Self::State>,
            ) {
                let (node_and_mounted, state) = state.pin_project();
                $crate::utils::dom::insert_element_and_update_with_tag(
                    node_and_mounted,
                    ctx,
                    ::core::stringify!($component_name),
                    |element, children_ctx| {
                        <$props_name::Data<TypeDefs> as $crate::props::UpdateElement<
                            $dom_element_ty,
                        >>::update_element(self.0, element, children_ctx, state)
                    },
                )
            }
        }
    };
}
