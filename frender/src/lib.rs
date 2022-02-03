mod into_prop_value;
mod props_macros;
pub use into_prop_value::*;
pub use props_macros::*;

pub mod intrinsic;

pub use react;
pub use react::{AsKey, Children, Component, Node, Props, PropsBuilder};

pub use forgotten;

#[macro_export]
macro_rules! children {
    ($($e:expr)*) => {
        $crate::Children::new([
            $(
                $crate::Node::as_react_node_js(&$e)
            ),*
        ])
    };
}

#[macro_export]
macro_rules! component {
    ( $ty_comp:ident [ $($prop_name:ident $(= $prop_value:expr)?)* ] $({ $($child:expr)* })?  ) => {{
        type FrenderTempComponentTypeAlias = $crate::component_type!($ty_comp);
        <FrenderTempComponentTypeAlias as $crate::Component>::new_with_props(
            $crate::props! (
                for $ty_comp {
                    $($prop_name $(: $prop_value)?),*
                    $(, children: $crate::children!($($child)*) )?
                }
            )
        )
    }};
    ($(@)? $ty_comp:ty [ $($prop_name:ident $(= $prop_value:expr)?)* ] $({ $($child:expr)* })?) => {
        <$ty_comp as $crate::Component>::new_with_props(
            $crate::props! (
                for $ty_comp {
                    $($prop_name $(: $prop_value)?),*
                    $(, children: $crate::children!($($child)*) )?
                }
            )
        )
    };
}

#[macro_export]
macro_rules! __impl_component_excluding_key_prop {
    ( $ty_comp:ident [ $($prop_name:ident $(= $prop_value:expr)?)* ] $({ $($child:expr)* })?  ) => {{
        type FrenderTempComponentTypeAlias = $crate::component_type!($ty_comp);
        <FrenderTempComponentTypeAlias as $crate::Component>::new_with_props(
            $crate::props! (
                for $ty_comp {
                    $($prop_name $(: $prop_value)?),*
                    $(, children: $crate::children!($($child)*) )?
                }@[-key]
            )
        )
    }};
    ($(@)? $ty_comp:ty [ $($prop_name:ident $(= $prop_value:expr)?)* ] $({ $($child:expr)* })?) => {
        <$ty_comp as $crate::Component>::new_with_props(
            $crate::props! (
                for $ty_comp {
                    $($prop_name $(: $prop_value)?),*
                    $(, children: $crate::children!($($child)*) )?
                }@[-key]
            )
        )
    };
}

#[macro_export]
macro_rules! __impl_node_prop_filter_key {
    (@[$v:ident] key) => {
        compiler_error!{"value of prop \"key\" must be specified"}
    };
    (@[$v:ident] key $(= $prop_value:expr)? ) => {
        let $v = Some($crate::AsKey::as_key($prop_value))
    };
    (@[$v:ident] $prop_name:ident $(= $prop_value:expr)? ) => {};
    ($($prop_name:ident $(= $prop_value:expr)?)* ) => {{
        let __frender_temp_value_key = None;
        $($crate::__impl_node_prop_filter_key!(
            @[__frender_temp_value_key]
            $prop_name $(= $prop_value)?
        );)*

        __frender_temp_value_key
    }};
}

#[macro_export]
macro_rules! node {
    ( $ty_comp:ident [ $($prop_name:ident $(= $prop_value:expr)?)* ] $({ $($child:expr)* })?  ) => {{
        let __frender_temp_component_instance =
            $crate::__impl_component_excluding_key_prop! (
                $ty_comp
                [$($prop_name $(= $prop_value)?)*]
                $({ $($child)* })?
            );
        __frender_temp_component_instance.call_create_element(
            $crate::__impl_node_prop_filter_key! (
                $($prop_name $(= $prop_value)?)*
            )
        )
    }};
    ($(@)? $ty_comp:ty [ $($prop_name:ident $(= $prop_value:expr)?)* ] $({ $($child:expr)* })?) => {{
        let __frender_temp_component_instance =
            $crate::__impl_component_excluding_key_prop! (
                @ $ty_comp
                [$($prop_name $(= $prop_value)?)*]
                $({ $($child)* })?
            );

        __frender_temp_component_instance.call_create_element(
            $crate::__impl_node_prop_filter_key! (
                $($prop_name $(= $prop_value)?)*
            )
        )
    }};
}
