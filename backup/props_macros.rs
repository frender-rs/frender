#[macro_export]
macro_rules! __impl_prop {
    ({ $v:ident } $prop_name:ident) => {
        let mut $v = $v.$prop_name($crate::IntoPropValue::into_prop_value(true));
    };
    ({ $v:ident } $prop_name:ident :) => {
        compile_error! (concat!("expect a value for prop " $prop_name));
    };
    ({ $v:ident } $prop_name:ident : None) => {
        let mut $v = $v.$prop_name(None);
    };
    ({ $v:ident } $prop_name:ident : $prop_value:expr) => {
        let mut $v = $v.$prop_name($crate::IntoPropValue::into_prop_value($prop_value));
    };
    ([-key] { $v:ident } key $(: $prop_value:expr)?) => {};
    ([-key] { $v:ident } $prop_name:ident) => {
        let mut $v = $v.$prop_name($crate::IntoPropValue::into_prop_value(true));
    };
    ([-key] { $v:ident } $prop_name:ident : None) => {
        let mut $v = $v.$prop_name(None);
    };
    ([-key] { $v:ident } $prop_name:ident : $prop_value:expr) => {
        let mut $v = $v.$prop_name($crate::IntoPropValue::into_prop_value($prop_value));
    };
}

#[macro_export]
macro_rules! props {
    (
        for $ty_component:ident {
            $($prop_name:ident $(: $prop_value:expr)?),*
            $(,)?
        }
        $(@ [$($options:tt)*])?
    ) => {{
        $crate::__impl_component_auto_import! {$ty_component}
        type FrenderTempComponentPropsTypeAlias = <$crate::component_type!($ty_component) as $crate::Component>::Props;

        $crate::props! {
            FrenderTempComponentPropsTypeAlias
            {
                $($prop_name $(: $prop_value)?),*
            }
            @ [$($($options)*)?]
            [|__frender_temp_value| $crate::__impl_component_props_pre_build!({__frender_temp_value} $ty_component)]
        }
    }};
    (
        for $ty_component:ty {
            $($prop_name:ident $(: $prop_value:expr)?),*
            $(,)?
        }
        $(@ [$($options:tt)*])?
    ) => {{
        type FrenderTempComponentPropsTypeAlias = <$ty_component as $crate::Component>::Props;
        $crate::props! {
            FrenderTempComponentPropsTypeAlias
            {
                $($prop_name $(: $prop_value)?),*
            }
            $(@ [$($options)*][|__frender_temp_val| __frender_temp_val])?
        }
    }};
    (
        $ty_props:ty {
            $($prop_name:ident $(: $prop_value:expr)?),*
            $(,)?
        }
    ) => {{
        let mut __frender_temp_val = <$ty_props as $crate::Props>::init_builder();
        $(
            $crate::__impl_prop! {
                {__frender_temp_val} $prop_name $(: $prop_value)?
            }
        )*
        $crate::PropsBuilder::<$ty_props>::build(__frender_temp_val)
    }};
    (
        $ty_props:ty {
            $($prop_name:ident $(: $prop_value:expr)?),*
            $(,)?
        }@[] [|$v_builder:ident| $expr_builder:expr]
    ) => {{
        let mut $v_builder = <$ty_props as $crate::Props>::init_builder();
        let mut __frender_temp_val = $expr_builder;
        $(
            $crate::__impl_prop! {
                {__frender_temp_val} $prop_name $(: $prop_value)?
            }
        )*
        $crate::PropsBuilder::<$ty_props>::build(__frender_temp_val)
    }};
    (
        $ty_props:ty {
            $($prop_name:ident $(: $prop_value:expr)?),*
            $(,)?
        }@[-key] [|$v_builder:ident| $expr_builder:expr]
    ) => {{
        let mut $v_builder = <$ty_props as $crate::Props>::init_builder();
        let mut __frender_temp_val = $expr_builder;
        $(
            $crate::__impl_prop! {
                [-key]
                {__frender_temp_val} $prop_name $(: $prop_value)?
            }
        )*
        $crate::PropsBuilder::<$ty_props>::build(__frender_temp_val)
    }};
    (
        $ty_props:ty {
            $($prop_name:ident $(: $prop_value:expr)?),*
            $(,)?
        }@[-key -build] [|$v_builder:ident| $expr_builder:expr]
    ) => {{
        let mut $v_builder = <$ty_props as $crate::Props>::init_builder();
        let mut __frender_temp_val = $expr_builder;
        $(
            $crate::__impl_prop! {
                [-key]
                {__frender_temp_val} $prop_name $(: $prop_value)?
            }
        )*
        __frender_temp_val
    }};
}
