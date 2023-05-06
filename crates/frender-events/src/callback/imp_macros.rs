macro_rules! expand_first_or {
    ([                         ]$($or:tt)*) => { $($or)* };
    ([{$($expand:tt)*}$($t:tt)*]$($or:tt)*) => { $($expand)* };
}

pub(super) use expand_first_or;

macro_rules! expand_last_or {
    ([                         ]$($or:tt)*) => { $($or)* };
    ([{$($expand:tt)*}]$($or:tt)*) => { $($expand)* };
    ([$t:tt $($more:tt)+]$($or:tt)*) => {
        crate::callback::imp_macros::expand_last_or! {
            [$($more)+]
        }
    };
}

pub(super) use expand_last_or;

macro_rules! expand_first_trimmed {
    ((,) $({$($first:tt)*}$({$($expand:tt)*})*)?) => { ($($($($expand)*,)*)?) };
}

pub(super) use expand_first_trimmed;

macro_rules! argument_type {
    (&mut $t:ty) => { crate::callback::argument::ByMut<$t> };
    (&    $t:ty) => { crate::callback::argument::ByRef<$t> };
    (     $t:ty) => { crate::callback::argument::Value<$t> };
}

pub(super) use argument_type;

macro_rules! expand_last_trimmed {
    ($(@impl[$($prepend:tt)*])? (,) $last:tt) => {
        ($($($prepend)*)?)
    };
    ($(@impl[$($prepend:tt)*])? (,) {$($not_last:tt)*}  $($more:tt)+) => {
        crate::callback::imp_macros::expand_last_trimmed! {
            @impl[$($($prepend)*)? $($not_last)*,]
            (,)
            $($more)+
        }
    };
}

pub(super) use expand_last_trimmed;

macro_rules! impl_fn {
    ( [$($item:tt)*]$more:tt) => {
        crate::callback::imp_macros::impl_fn! { ![$($item)*] $more }
        crate::callback::imp_macros::impl_fn! { *[$($item)*] $more }
    };
    (*[$($item:tt)*][$next:tt $($more:tt)*]) => {
        crate::callback::imp_macros::impl_fn! { [$($item)* $next][$($more)*] }
    };
    (*[$($item:tt)*][]) => {};
    (![$({$v:ident $tp:ident})*] $more:tt) => {
        impl<$($tp,)*> crate::callback::sealed::Tuple for ($($tp,)*) {}
        impl<'arg, $($tp: crate::callback::argument::ArgumentType,)*> crate::callback::argument::Arguments<'arg> for ($($tp,)*) {
            type Arguments = ($(crate::callback::argument::ArgumentOfType<'arg, $tp>,)*);
        }
        impl<      $($tp: crate::callback::argument::ArgumentType,)*> crate::callback::argument::ArgumentTypes   for ($($tp,)*) {
            type First = crate::callback::imp_macros::expand_first_or![
                [$({$tp})*]
                crate::callback::argument::Invalid
            ];
            type FirstTrimmed = crate::callback::imp_macros::expand_if_else![ [$($tp)*][
                crate::callback::imp_macros::expand_first_trimmed![
                    (,)
                    $({$tp})*
                ]
            ][
                crate::callback::argument::InvalidTuple
            ]];
            type Last = crate::callback::imp_macros::expand_last_or![
                [$({$tp})*]
                crate::callback::argument::Invalid
            ];
            type LastTrimmed = crate::callback::imp_macros::expand_if_else![[$($tp)*][
                crate::callback::imp_macros::expand_last_trimmed![ (,) $({$tp})* ]
            ][
                crate::callback::argument::InvalidTuple
            ]];

            fn re_borrow<'a: 'b, 'b>(($($v,)*): crate::callback::argument::ArgumentsOfTypes<'a, Self>) -> crate::callback::argument::ArgumentsOfTypes<'b, Self> {
                ($($tp::re_borrow($v),)*)
            }
        }

        $crate::callback::imp_macros::expand_if_else! { $more [
        impl<
            Arg: crate::callback::argument::ArgumentType,
            $($tp: crate::callback::argument::ArgumentType,)*
        > crate::callback::argument::PrependArgument<Arg> for ($($tp,)*) {
            type Prepended = (Arg, $($tp,)*);

            fn prepend_argument<'a>(
                first: crate::callback::argument::ArgumentOfType<'a, Arg>,
                ($($v,)*): crate::callback::argument::ArgumentsOfTypes<'a, Self>,
            ) -> crate::callback::argument::ArgumentsOfTypes<'a, Self::Prepended> {
                (first, $($v,)*)
            }
        }
        impl<
            Arg: crate::callback::argument::ArgumentType,
            $($tp: crate::callback::argument::ArgumentType,)*
        > crate::callback::argument::AppendArgument<Arg> for ($($tp,)*) {
            type Appended = ($($tp,)* Arg,);

            fn append_argument<'a>(
                ($($v,)*): crate::callback::argument::ArgumentsOfTypes<'a, Self>,
                last: crate::callback::argument::ArgumentOfType<'a, Arg>,
            ) -> crate::callback::argument::ArgumentsOfTypes<'a, Self::Appended> {
                ($($v,)* last,)
            }
        }
        ][] }

        impl<$($tp,)* Out> crate::callback::Callable<($($tp,)*)> for fn($($tp),*) -> Out {
            type Output = Out;

            fn call_fn(&self, ($($v,)*): ($($tp,)*)) -> Self::Output {
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
        fn_expr!    { |$self_ident:ident| $fn_expr:expr }
    ) => {
        pub fn $fn_name<$($all_tp $(: $($all_tp_bounds)+)?,)* Out>(f: $fn_type) -> $fn_wrapped {
            $crate::callback::imp_macros::expand_if_else![[$($fn_wrap)*][
                $($fn_wrap)* (f)
            ][
                f
            ]]
        }

        $crate::callback::imp_macros::expand_if_else! { [$($should_impl)?][
        impl<$($all_tp $(: $($all_tp_bounds)+)?,)* Out> crate::callback::Callable<($($($all_t)* $all_tp,)*)> for $fn_wrapped {
            type Output = Out;

            fn call_fn(&self, ($($all_tp,)*): ($($($all_t)* $all_tp,)*)) -> Self::Output {
                self.0($($all_tp),*)
            }
        }
        impl<$($all_tp $(: $($all_tp_bounds)+)?,)* Out> PartialEq for $fn_wrapped {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        ][] }

        impl<$($all_tp $(: $($all_tp_bounds)+)?,)* Out> crate::callback::IsCallable for $fn_wrapped {
        }

        pub mod $fn_name {
            crate::callback::imp_macros::expand_if_ident_is_else! { r#ref $fn_name [
            mod provide_last_argument {
                use super::super::HkFn;
                pub fn provide_last_argument<$($bef_tp : 'static $(+ $($bef_tp_bounds)+)?,)* $cur_tp: 'static, Out>(
                    f: $fn_type,
                    arg: $cur_tp,
                ) -> crate::callback::argument::LastArgumentProvided<
                    $fn_wrapped,
                    crate::callback::argument::Refed<$cur_tp>,
                > {
                    use crate::callback::IsCallable;
                    super::super::$fn_name(f).provide_last_argument_refed(arg)
                }
            }

            pub use provide_last_argument::provide_last_argument;
            ][] }

            crate::callback::imp_macros::expand_if_ident_is_else! { value $fn_name [
            #[derive(Debug, Clone, Copy)]
            pub struct HkFn<F>(pub(super) F);
            ][
            crate::callback::imp_macros::expand_if_else! {$more[
                use super::HkFn;
            ][]}
            ]}

            crate::callback::imp_macros::expand_if_else! {$more[
            crate::callback::imp_macros::impl_all! {
                ($({[$($all_t)*] $all_tp {$($($all_tp_bounds)+)?}})*) @$more
            }
            ][]}
        }

        impl<$($all_tp :'static $(+ $($all_tp_bounds)+)?,)* Out> crate::callback::CallableWithFixedArguments for $fn_wrapped {
            type FixedArgumentTypes   = ($(crate::callback::imp_macros::argument_type![$($all_t)* $all_tp],)*);
        }
    };
}

pub(super) use impl_one_resolved;

macro_rules! impl_one {
    // all arguments are by value
    ( @$fn_name:ident $(#$prefix_path:ident)? ($({[] $tp:ident {}})*) {[] $cur_tp:ident {}} @$more:tt) => {
        crate::callback::imp_macros::impl_one_resolved! {
            name!       { $fn_name }
            fn_type!    { fn($($tp,)* $cur_tp) -> Out }
            fn_wrap!    {}
            fn_wrapped! { fn($($tp,)* $cur_tp) -> Out }
            fn_trait!   {}
            all_tps!    { $({[] $tp {}})* {[] $cur_tp {}} }
            before_tps! { $({[] $tp {}})*                 }
            cur_tp!     {                 {[] $cur_tp {}} }
            more_tps!   { $more }
            fn_expr!    { |self| self }
        }
    };
    // some arguments are higher kinded
    ( @$fn_name:ident $(#$prefix_path:ident)? ($({[$($t:tt)*] $tp:ident $tp_bounds:tt})*) {[$($cur_t:tt)*] $cur_tp:ident $cur_tp_bounds:tt} @$more:tt) => {
        crate::callback::imp_macros::impl_one_resolved! {
            name!       { $fn_name }
            fn_type!    {                          fn($( $($t)* $tp,)* $($cur_t)* $cur_tp) -> Out  }
            fn_wrap!    { $($prefix_path::)? HkFn                                                  }
            fn_wrapped! { $($prefix_path::)? HkFn <fn($( $($t)* $tp,)* $($cur_t)* $cur_tp) -> Out> }
            fn_trait!   { impl }
            all_tps!    { $({[$($t)*] $tp $tp_bounds})* {[$($cur_t)*] $cur_tp $cur_tp_bounds} }
            before_tps! { $({[$($t)*] $tp $tp_bounds})*                                       }
            cur_tp!     {                               {[$($cur_t)*] $cur_tp $cur_tp_bounds} }
            more_tps!   { $more }
            fn_expr!    { |self| self.0 }
        }
    };
}

pub(super) use impl_one;

macro_rules! impl_all {
    ( $before:tt @[$next:ident $($more:ident)*] ) => {
        crate::callback::imp_macros::impl_one! { @value #value $before {[    ]$next{      }} @[$($more)*] }
        crate::callback::imp_macros::impl_one! { @r#ref        $before {[&   ]$next{?Sized}} @[$($more)*] }
        crate::callback::imp_macros::impl_one! { @r#mut        $before {[&mut]$next{?Sized}} @[$($more)*] }
    };
}

pub(super) use impl_all;

macro_rules! impl_with_macro_rules {
    ($($v:ident : $tp:ident),* $(,)?) => {
        use super::HkFn;
        crate::callback::imp_macros::impl_fn!([][$({$v $tp})*]);
        crate::callback::imp_macros::impl_all! {()@[$($tp)*]}
    };
}

pub(super) use impl_with_macro_rules;
