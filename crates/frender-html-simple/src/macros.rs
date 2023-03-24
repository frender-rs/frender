#[macro_export]
macro_rules! __impl_dom {
    () => {
        mod impl_update_element {
            #[allow(unused_imports)]
            use super::super::*;
            impl<E, Children, Attrs> $crate::frender_dom::props::UpdateElement<E>
                for super::Data<Children, Attrs>
            where
                $crate::ElementProps<Children, Attrs>: $crate::frender_dom::props::UpdateElement<E>,
            {
                type State = <$crate::ElementProps<Children, Attrs> as $crate::frender_dom::props::UpdateElement<E>>::State;

                #[inline(always)]
                fn initialize_state(
                    this: Self,
                    element: &E,
                    children_ctx: &mut ::frender_dom::Dom,
                ) -> Self::State {
                    $crate::ElementProps::<Children, Attrs>::initialize_state(
                        this.props,
                        element,
                        children_ctx,
                    )
                }

                #[inline(always)]
                fn update_element(
                    this: Self,
                    element: &E,
                    children_ctx: &mut ::frender_dom::Dom,
                    state: ::core::pin::Pin<&mut Self::State>,
                ) {
                    $crate::ElementProps::<Children, Attrs>::update_element(
                        this.props,
                        element,
                        children_ctx,
                        state,
                    )
                }
            }
        }
    };
}

#[macro_export]
macro_rules! __impl_ssr {
    () => {
        mod impl_ssr {
            #[allow(unused_imports)]
            use super::super::*;
            impl<W: ::frender_ssr::AsyncWrite + ::core::marker::Unpin, Children, Attrs>
                $crate::frender_ssr::IntoSsrData<W> for super::Data<Children, Attrs>
            where
                $crate::ElementProps<Children, Attrs>: $crate::frender_ssr::IntoSsrData<W>,
            {
                type Children =
                    <$crate::ElementProps<Children, Attrs> as $crate::frender_ssr::IntoSsrData<
                        W,
                    >>::Children;

                type ChildrenRenderState =
                    <$crate::ElementProps<Children, Attrs> as $crate::frender_ssr::IntoSsrData<
                        W,
                    >>::ChildrenRenderState;

                type Attrs =
                    <$crate::ElementProps<Children, Attrs> as $crate::frender_ssr::IntoSsrData<
                        W,
                    >>::Attrs;

                fn into_ssr_data(this: Self) -> (Self::Children, Self::Attrs) {
                    $crate::frender_ssr::IntoSsrData::<W>::into_ssr_data(this.props)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! __impl_children_fn {
    (
        [children]
        [$($attrs:tt)*]
        $method_name:ident $($t:tt)*) => {
        impl<Attrs> Building<(), Attrs> {
            $($attrs)*
            #[inline(always)]
            pub fn $method_name<Children>(self, children: Children) -> Building<Children, Attrs> {
                Building(Data {
                    props: self.0.props.children(children),
                })
            }
        }
    };
    ($($t:tt)+) => {};
}

#[macro_export]
macro_rules! __impl_children_fns {
    (
        $(
            ..
            $inherit_from:ident
            $inherit_fields:tt
            ,
        )*
        $(
            $(#$attr:tt)*
            $name:ident
            $(:)?
            $( $field_macro:ident ! $field_macro_tt:tt $(+)? )*
            ,
        )*
    ) => {
        $(
            use super::$inherit_from::props as _;
            $crate::__impl_children_fns! $inherit_fields ;
        )*

        $(
            $crate::__impl_children_fn! {
                [$name]
                [$(#$attr)*]
                $name : $( $field_macro ! $field_macro_tt )*
            }
        )*
    };
}

#[macro_export]
macro_rules! __impl_builder_fn {
    (
        $(#$attr:tt)*
        children
        $($t:tt)*
    ) => {};
    (
        $(#$attr:tt)*
        $name:ident
        $(:)?
        $( alias![ $($alias:ident),+ $(,)? ] $(+)? )?
        $(bounds! $bounds:tt)?
    ) => {
        $crate::__impl_builder_fn_with_prop_name! {
            [$(#$attr)*][$($bounds)?]
            { $name }
            $name $($($alias)+)?
        }
    };
}

#[macro_export]
macro_rules! __impl_builder_fn_with_prop_name {
    (
        @[$($attrs:tt)*][$([ $($bounds:tt)+ ])?]
        $name:ident($prop_name:ident)
    ) => {
        $($attrs)*
        #[inline(always)]
        pub fn $name<V $(: $($bounds)+)? >(
            self,
            $name: V,
        ) -> super::Building<Children, (Attrs, super::props::$prop_name<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::$prop_name($name)),
            })
        }
    };
    (
        $attrs:tt $bounds:tt
        { $prop_name:ident }
        $($method_name:ident)+
    ) => {
        $(
            $crate::__impl_builder_fn_with_prop_name! {
                @ $attrs $bounds
                $method_name($prop_name)
            }
        )+
    };
}

#[macro_export]
macro_rules! __impl_builder_fns {
    (
        $(
            ..
            $inherit_from:ident
            $inherit_fields:tt
            ,
        )*
        $(
            $(#$attr:tt)*
            $name:ident
            $(:)?
            $( $field_macro:ident ! $field_macro_tt:tt $(+)? )*
            ,
        )*
    ) => {
        $(
            $crate::__impl_builder_fns! $inherit_fields ;
        )*

        $(
            $crate::__impl_builder_fn! {
                $(#$attr)*
                $name : $( $field_macro ! $field_macro_tt )*
            }
        )*
    };
}

#[macro_export]
macro_rules! __impl_prop_struct {
    (children) => {};
    ($name:ident) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name<V>(pub V);
        impl<V> Unpin for $name<V> {}
    };
}

#[macro_export]
macro_rules! __impl_mod_props {
    (
        $(
            ..
            $inherit_from:ident
            $inherit_fields:tt
            ,
        )*
        $(
            $(#$attr:tt)* // not appended to prop structs
            $name:ident
            $(:)?
            $( $field_macro:ident ! $field_macro_tt:tt $(+)? )*
            ,
        )*
    ) => {
        pub mod props {
            $(
                pub use super::super::$inherit_from::props::*;
            )*

            $(
                $crate::__impl_prop_struct!{ $name }
            )*
        }
    };
}

#[macro_export]
macro_rules! __impl_props_type {
    (
        $(#$struct_attr:tt)*
        $name:ident
    ) => {
        pub mod data_struct {
            #[allow(unused_imports)]
            use super::super::*;

            $(#$struct_attr)*
            #[repr(transparent)]
            pub struct $name<Children = (), Attrs = ()> {
                pub props: $crate::ElementProps<Children, Attrs>,
            }
        }

        pub mod building_struct {
            #[allow(unused_imports)]
            use super::super::*;
            #[repr(transparent)]
            pub struct $name<Children = (), Attrs = ()>(pub super::Data<Children, Attrs>);
        }

        pub use building_struct::$name as Building;
        pub use data_struct::$name as Data;
        pub type DataInitial = data_struct::$name;
        pub mod prelude {}

        #[inline(always)]
        pub fn build<Children, Attrs>(
            building: Building<Children, Attrs>,
        ) -> Data<Children, Attrs> {
            building.0
        }
        pub use build as valid;

        #[cfg(feature = "dom")]
        $crate::__impl_dom! {}

        #[cfg(feature = "ssr")]
        $crate::__impl_ssr! {}
    };
}

#[macro_export]
macro_rules! def_props_type {
    (
        $(#!$fn_attr:tt)*
        $(#$struct_attr:tt)*
        $name:ident $fields:tt
    ) => {
        $(#$fn_attr)*
        #[allow(non_snake_case)]
        #[inline(always)]
        pub fn $name() -> Building {
            Building(Default::default())
        }

        $crate::__impl_mod_props! $fields ;

        $crate::__impl_children_fns! $fields ;

        mod props_builder {
            #[allow(unused_imports)]
            use super::super::*;

            impl<Children, Attrs> super::Building<Children, Attrs> {
                $crate::__impl_builder_fns! $fields ;
            }
        }

        $crate::__impl_props_type! {
            $(#$struct_attr)*
            $name
        }
    };
}
