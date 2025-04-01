pub mod input;

#[macro_export]
macro_rules! impl_HasConstValue_for {
    (
        const _: () = $paths:tt;
        impl <$(__)?> $($rest:tt)*
    ) => {
        $crate::impl_HasConstValue_for! {
            const _: () = $paths;
            impl $($rest)*
        }
    };
    (
        const _: () = $paths:tt;
        impl $for_ty:ty {
            const $NAME:tt: _ = $const_expr:expr;
        }
    ) => {
        $crate::impl_HasConstValue_for! {
            const _: () = $paths;
            impl $for_ty {
                const $NAME: &'static $crate::__private::str = $const_expr;
            }
        }
    };
    (
        const _: () = {
            trait __ = $HasConstValue:path;
            const $VALUE:ident: Self::$Value:ident;
            type For = $For:ty;
        };
        impl $for_ty:ty {
            const $NAME:tt: $input_ty:ty = $const_expr:expr;
        }
    ) => {
        const _: () = {
            $crate::__expand_if_underscore_else! { $NAME {} {
                impl $for_ty {
                    const $NAME: $input_ty = $const_expr;
                }
            }}

            enum __FrenderConstValueMarkerOfInput {}
            enum __FrenderConstValueMarkerOfTemp {}

            impl $crate::input::MarkerOfConstValue<$input_ty> for __FrenderConstValueMarkerOfInput {
                const VALUE: $input_ty = $crate::__expand_if_underscore_else!($NAME {
                    $const_expr
                } {
                    <$for_ty>::$NAME
                });
            }

            impl $crate::input::MarkerOfConstValue<<$input_ty as $crate::input::Input<$For>>::Temp> for __FrenderConstValueMarkerOfTemp {
                const VALUE: <$input_ty as $crate::input::Input<$For>>::Temp =
                    $crate::input::const_into::<$input_ty, _, __FrenderConstValueMarkerOfInput>();
            }

            const __FRENDER_CONST_INFO: $crate::input::Info =
                $crate::input::const_into::<
                    <$input_ty as $crate::input::Input<$For>>::Temp,
                    _,
                    __FrenderConstValueMarkerOfTemp,
                >();

            impl $HasConstValue for $for_ty {
                type $Value =
                    $crate::as_InputWithInfo![
                        <$input_ty as InputWithInfo<$For, { __FRENDER_CONST_INFO }>>::Output
                    ];
                const $VALUE: Self::$Value =
                    $crate::input::const_into::<
                        <$input_ty as $crate::input::Input<$For>>::Temp,
                        _,
                        __FrenderConstValueMarkerOfTemp,
                    >();
            }
        };
    };
}

#[doc(hidden)]
pub mod __private {
    #[doc(hidden)]
    pub use str;

    #[doc(hidden)]
    #[macro_export]
    macro_rules! __expand_if_underscore_else {
        (_     {$($then:tt)*}   $else:tt   ) => {$($then)*};
        ($t:tt    $then:tt   {$($else:tt)*}) => {$($else)*};
    }
}
