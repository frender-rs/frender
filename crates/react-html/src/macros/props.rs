macro_rules! __impl_js_prop_name {
    ($k_js:literal $k:ident ) => {
        $k_js
    };
    ($k:ident) => {
        ::frender_macros::ident_snake_to_camel!($k)
    };
}

macro_rules! __impl_prop_default {
    (
        $k:ident { $k_js:expr } {$($attr:tt)*} [ $($fn_generics:tt)* ] : $v_ty:ty : { safe_into_js_runtime }
    ) => {
        $($attr)*
        fn $k $($fn_generics)* (mut self, v: $v_ty) -> Self {
            let ret = react::SafeIntoJsRuntime::safe_into_js_runtime(v);
            self.as_mut().set_static_prop_and_persist($k_js, ret);
            self
        }
    };
    (
        $k:ident { $k_js:expr } {$($attr:tt)*} [ $($fn_generics:tt)* ] : $v_ty:ty : { safe_into_js_runtime? }
    ) => {
        $($attr)*
        fn $k $($fn_generics)* (mut self, v: $v_ty) -> Self {
            if let Some(v) = v {
                let ret = react::SafeIntoJsRuntime::safe_into_js_runtime(v);
                self.as_mut().set_static_prop_and_persist($k_js, ret);
            } else {
                use react::any_js_props::AnyJsPropsBuilder;
                self.as_mut().remove_prop($k_js);
            }
            self
        }
    };
    (
        $k:ident { $k_js:expr } {$($attr:tt)*} [ $($fn_generics:tt)* ] : $v_ty:ty : { impl |$impl_this:ident, $impl_v:ident| $impl_expr:expr }
    ) => {
        $($attr)*
        fn $k $($fn_generics)* (self, $impl_v: $v_ty) -> Self {
            #[allow(unused_mut)]
            let mut $impl_this = self;
            $impl_expr
        }
    };
    (
        $k:ident { $k_js:expr } {$($attr:tt)*} [ $($fn_generics:tt)* ] : $v_ty:ty
    ) => {
        // convert value to JsValue with ::convert_js::ToJs::to_js
        $($attr)*
        fn $k $($fn_generics)* (mut self, v: $v_ty) -> Self {
            use react::any_js_props::AnyJsPropsBuilder;
            let js_value = ::convert_js::ToJs::to_js(&v);
            self.as_mut().set_prop(
                $k_js,
                &js_value,
            );
            self
        }
    };
    ($($t:tt)*) => {
        compile_error!(concat!("invalid __impl_prop_default", $(stringify!($t), )*));
    };
}

macro_rules! __impl_auto_impl_trait {
    (
        @
        {$($generics:tt)*} $struct_name:ident {$($type_params:tt)*} { $(where $($where:tt)+)? }
        $auto_impl_trait:ident $([$($auto_impl_trait_type_params:tt)*])?
    ) => {
        impl<$($generics)*> $auto_impl_trait $(<$($auto_impl_trait_type_params)*>)? for $struct_name $(<$($type_params)+>)? $(where $($where)+)? {}
    };
    ($generics:tt $struct_name:ident $type_params:tt $where:tt) => {};
    (
        $generics:tt $struct_name:ident $type_params:tt $where:tt
        $({$auto_impl_trait:ident $([$($auto_impl_trait_type_params:tt)*])?})+
    ) => {
        $(
            $crate::macros::__impl_auto_impl_trait! {
                @
                $generics $struct_name $type_params $where
                $auto_impl_trait $([$($auto_impl_trait_type_params)*])?
            }
        )+
    };
}

macro_rules! def_props_trait {
    (
        $([ $($generics:tt)+ ])? $struct_name:ident $([ $($type_params:tt)+ ])?
        : $trait_name:ident
        $(: $auto_impl_trait:ident $([$($auto_impl_trait_type_params:tt)*])?)*
        $(where { $($where:tt)+ })?
        {
            $($({$($attr:tt)*})? $k:ident $(@ $k_js:literal)? $([$($fn_generics:tt)*])? : $v_ty:ty $({$($impl_tt:tt)*})? ),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone)]
        pub struct $struct_name $(<$($generics)+>)? $(where $($where)+)? (
            react::any_js_props::AnyJsStaticProps,
            $(#[allow(unused_parens)] std::marker::PhantomData<($($type_params)+)>)?
        );

        impl<$($($generics)+)?> Default for $struct_name $(<$($type_params)+>)? $(where $($where)+)? {
            #[inline]
            fn default() -> Self {
                Self(
                    Default::default(),
                    $(std::marker::PhantomData::<($($type_params)+)>)?
                )
            }
        }

        impl<$($($generics)+)?> AsMut<react::any_js_props::AnyJsStaticProps> for $struct_name $(<$($type_params)+>)? $(where $($where)+)? {
            #[inline]
            fn as_mut(&mut self) -> &mut react::any_js_props::AnyJsStaticProps {
                &mut self.0
            }
        }

        impl<$($($generics)+)?> AsRef<react::any_js_props::AnyJsStaticProps> for $struct_name $(<$($type_params)+>)? $(where $($where)+)? {
            #[inline]
            fn as_ref(&self) -> &react::any_js_props::AnyJsStaticProps {
                &self.0
            }
        }

        impl<$($($generics)+)?> react::Props for $struct_name $(<$($type_params)+>)? $(where $($where)+)? {
            type InitialBuilder = Self;
            #[inline]
            fn init_builder() -> Self {
                Default::default()
            }
        }

        impl<$($($generics)+)?> react::PropsBuilder<Self> for $struct_name $(<$($type_params)+>)? $(where $($where)+)? {
            #[inline]
            fn build(self) -> Self {
                self
            }
        }

        impl<$($($generics)+)?> $trait_name $(<$($type_params)+>)? for $struct_name $(<$($type_params)+>)? $(where $($where)+)? {
        }

        pub trait $trait_name <$($($generics)+)?>: Sized + AsMut<react::any_js_props::AnyJsStaticProps> $(+ $auto_impl_trait $(<$($auto_impl_trait_type_params)*>)?)* $(where $($where)+)? {
            $(
                $crate::macros::__impl_prop_default! {
                    $k
                    { $crate::macros::__impl_js_prop_name! ($($k_js)? $k) }
                    {$($($attr)*)?}
                    [$(<$($fn_generics)*>)?]
                    :
                    $v_ty
                    $(: {$($impl_tt)*})?
                }
            )*
        }

        $crate::macros::__impl_auto_impl_trait! {
            { $( $($generics)+ )? }
            $struct_name
            { $($($type_params)+)? }
            { $(where $($where)+ )? }
            $({ $auto_impl_trait $([$($auto_impl_trait_type_params)*])? })*
        }
    };
}

pub(crate) use __impl_auto_impl_trait;
pub(crate) use __impl_js_prop_name;
pub(crate) use __impl_prop_default;
pub(crate) use def_props_trait;
