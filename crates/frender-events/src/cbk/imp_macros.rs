macro_rules! impl_fn {
    ( [$($item:tt)*]$more:tt) => {
        crate::cbk::imp_macros::impl_fn! { ![$($item)*] }
        crate::cbk::imp_macros::impl_fn! { *[$($item)*] $more }
    };
    (*[$($item:tt)*][$next:tt $($more:tt)*]) => {
        crate::cbk::imp_macros::impl_fn! { [$($item)* $next][$($more)*] }
    };
    (*[$($item:tt)*][]) => {};
    (![$({$v:ident $tp:ident})*]) => {
        impl<$($tp,)*> Tuple for ($($tp,)*) {}

        impl<$($tp,)* Out, This: ?Sized> Fn<($($tp,)*)> for This
            where This: ::core::ops::Fn($($tp),*) -> Out {
            type Output = Out;

            fn call(&self, ($($v,)*): ($($tp,)*)) -> Self::Output {
                self($($v),*)
            }
        }
    };
}

pub(super) use impl_fn;

macro_rules! expand_if_else {
    ([]$if:tt[$($else:tt)*]) => {
        $($else)*
    };
    ([$($cond:tt)+][$($if:tt)*]$else:tt) => {
        $($if)*
    };
}

pub(super) use expand_if_else;

macro_rules! expand_if_ident_is_else {
    (value           value       [$($if:tt)*]   $else:tt   ) => {
        $($if)*
    };
    (r#ref           r#ref       [$($if:tt)*]   $else:tt   ) => {
        $($if)*
    };
    ($expected:ident $name:ident    $if:tt   [$($else:tt)*]) => {
        $($else)*
    };
}

pub(super) use expand_if_ident_is_else;

macro_rules! impl_one_resolved {
    (
        name!       { $fn_name:ident }
        fn_type!    { $fn_type:ty }
        fn_wrap!    { $($fn_wrap:tt)* }
        fn_wrapped! { $fn_wrapped:ty }
        fn_trait!   { $($should_impl:ident)? }
        all_tps!    { $({[$($all_t:tt)*] $all_tp:ident {$($($all_tp_bounds:tt)+)?}})* }
        before_tps! { $({[$($bef_t:tt)*] $bef_tp:ident {$($($bef_tp_bounds:tt)+)?}})* }
        cur_tp!     {   {[$($cur_t:tt)*] $cur_tp:ident {$($($cur_tp_bounds:tt)+)?}}   }
        more_tps!   { $more:tt }
    ) => {
        pub fn $fn_name<$($all_tp $(: $($all_tp_bounds)+)?,)* Out>(f: $fn_type) -> $fn_wrapped {
            $crate::cbk::imp_macros::expand_if_else![[$($fn_wrap)*][
                $($fn_wrap)* (f)
            ][
                f
            ]]
        }

        $crate::cbk::imp_macros::expand_if_else! { [$($should_impl)?][
        impl<$($all_tp $(: $($all_tp_bounds)+)?,)* Out> crate::cbk::Fn<($($($all_t)* $all_tp,)*)> for $fn_wrapped {
            type Output = Out;

            fn call(&self, args: ($($($all_t)* $all_tp,)*)) -> Self::Output {
                crate::cbk::Fn::<_>::call(&self.0, args)
            }
        }
        impl<$($all_tp $(: $($all_tp_bounds)+)?,)* Out> PartialEq for $fn_wrapped {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        ][] }

        impl<$($all_tp $(: $($all_tp_bounds)+)?,)* Out> crate::callback::IsCallback for $fn_wrapped {
        }

        pub mod $fn_name {
            crate::cbk::imp_macros::expand_if_ident_is_else! { r#ref $fn_name [
            mod provide_last_argument {
                use super::super::HkFn;
                pub fn provide_last_argument<$($bef_tp $(: $($bef_tp_bounds)+)?,)* $cur_tp, Out>(
                    f: $fn_type,
                    arg: $cur_tp,
                ) -> crate::cbk::LastArgumentProvided<
                    $fn_wrapped,
                    crate::cbk::argument::Refed<$cur_tp>,
                > {
                    use crate::cbk::HasLastArgument;
                    super::super::$fn_name(f).provide_last_argument_refed(arg)
                }
            }

            pub use provide_last_argument::provide_last_argument;
            ][] }

            crate::cbk::imp_macros::expand_if_ident_is_else! { value $fn_name [
            #[derive(Debug, Clone, Copy)]
            pub struct HkFn<F>(pub(super) F);
            ][
            crate::cbk::imp_macros::expand_if_else! {$more[
                use super::HkFn;
            ][]}
            ]}

            crate::cbk::imp_macros::expand_if_else! {$more[
            crate::cbk::imp_macros::impl_all! {
                ($({[$($all_t)*] $all_tp {$($($all_tp_bounds)+)?}})*) @$more
            }
            ][]}
        }

        impl<$($all_tp $(: $($all_tp_bounds)+)?,)* Out> crate::cbk::HasLastArgument for $fn_wrapped {
            type LastArgumentType = crate::cbk::argument::$fn_name <$cur_tp>;
        }

        impl<$($all_tp $(: $($all_tp_bounds)+)?,)* Out, Arg> crate::cbk::Fn<($($($bef_t)* $bef_tp,)*)> for
            crate::cbk::LastArgumentProvided<$fn_wrapped, Arg>
            where Arg: crate::cbk::argument::ProvideArgument<crate::cbk::argument::$fn_name <$cur_tp>>
        {
            type Output = Out;

            fn call(&self, ($($bef_tp,)*): ($($($bef_t)* $bef_tp,)*)) -> Self::Output {
                crate::cbk::Fn::<_>::call(&self.f, ($($bef_tp,)* self.last_argument.provide_argument(),))
            }
        }
    };
}

pub(super) use impl_one_resolved;

macro_rules! impl_one {
    // all arguments are by value
    ( @$fn_name:ident $(#$prefix_path:ident)? ($({[] $tp:ident {}})*) {[] $cur_tp:ident {}} @$more:tt) => {
        crate::cbk::imp_macros::impl_one_resolved! {
            name!       { $fn_name }
            fn_type!    { fn($($tp,)* $cur_tp) -> Out }
            fn_wrap!    {}
            fn_wrapped! { fn($($tp,)* $cur_tp) -> Out }
            fn_trait!   {}
            all_tps!    { $({[] $tp {}})* {[] $cur_tp {}} }
            before_tps! { $({[] $tp {}})*                 }
            cur_tp!     {                 {[] $cur_tp {}} }
            more_tps!   { $more }
        }
    };
    // some arguments are higher kinded
    ( @$fn_name:ident $(#$prefix_path:ident)? ($({[$($t:tt)*] $tp:ident $tp_bounds:tt})*) {[$($cur_t:tt)*] $cur_tp:ident $cur_tp_bounds:tt} @$more:tt) => {
        crate::cbk::imp_macros::impl_one_resolved! {
            name!       { $fn_name }
            fn_type!    {                          fn($( $($t)* $tp,)* $($cur_t)* $cur_tp) -> Out  }
            fn_wrap!    { $($prefix_path::)? HkFn                                                  }
            fn_wrapped! { $($prefix_path::)? HkFn <fn($( $($t)* $tp,)* $($cur_t)* $cur_tp) -> Out> }
            fn_trait!   { impl }
            all_tps!    { $({[$($t)*] $tp $tp_bounds})* {[$($cur_t)*] $cur_tp $cur_tp_bounds} }
            before_tps! { $({[$($t)*] $tp $tp_bounds})*                                       }
            cur_tp!     {                               {[$($cur_t)*] $cur_tp $cur_tp_bounds} }
            more_tps!   { $more }
        }
    };
}

pub(super) use impl_one;

macro_rules! impl_all {
    ( $before:tt @[$next:ident $($more:ident)*] ) => {
        crate::cbk::imp_macros::impl_one! { @value #value $before {[    ]$next{      }} @[$($more)*] }
        crate::cbk::imp_macros::impl_one! { @r#ref        $before {[&   ]$next{?Sized}} @[$($more)*] }
        crate::cbk::imp_macros::impl_one! { @r#mut        $before {[&mut]$next{?Sized}} @[$($more)*] }
    };
}

pub(super) use impl_all;

macro_rules! impl_with_macro_rules {
    ($($v:ident : $tp:ident),* $(,)?) => {
        use super::{Fn, Tuple};
        crate::cbk::imp_macros::impl_fn!([][$({$v $tp})*]);

        pub mod r#fn {
            #[derive(Debug, Clone, Copy)]
            pub struct HkFn<F>(F);

            crate::cbk::imp_macros::impl_all! {()@[$($tp)*]}
        }
    };
}

pub(super) use impl_with_macro_rules;
