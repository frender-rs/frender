#[cfg(all(feature = "ssr", feature = "dom"))]
#[macro_export]
macro_rules! component_ssr_dom {
    (
        $(#$attr:tt)*
        $vis:vis fn $name:ident () $body:tt
    ) => {
        $crate::component_ssr_dom! {
            $(#$attr)*
            $vis fn $name (_ctx: _) $body
        }
    };
    (
        $(#$attr:tt)*
        $vis:vis fn $name:ident ($ctx_arg:tt : _ $(,)?)
        $impl_code:tt
    ) => {
        impl self:: $name ::ImplSsr {
            #[$crate::component_macro::hook(args_generics = "'render_ctx", hooks_core_path($crate::component_macro::hooks_core))]
            #[allow(non_snake_case)]
            fn $name(
                $ctx_arg: $crate::ContextAndState<'render_ctx, $crate::AnySsrContext, dyn ::core::any::Any>,
            ) -> $crate::ContextAndState<'render_ctx, $crate::AnySsrContext, impl $crate::RenderState + 'static>
            $impl_code
        }

        impl self:: $name ::ImplDom {
            #[$crate::component_macro::hook(args_generics = "'render_ctx", hooks_core_path($crate::component_macro::hooks_core))]
            #[allow(non_snake_case)]
            fn $name(
                $ctx_arg: $crate::ContextAndState<'render_ctx, $crate::Dom, dyn ::core::any::Any>,
            ) -> $crate::ContextAndState<'render_ctx, $crate::Dom, impl $crate::RenderState + 'static>
            $impl_code
        }

        $crate::builder! {
            $(#$attr)*
            $vis struct $name($crate::bg::Empty);

            pub(super) enum ImplSsr {}
            pub(super) enum ImplDom {}

            #[inline]
            pub fn build_element<TypesDef: ?::core::marker::Sized + $crate::bg::Empty::ValidTypes>(
                _: Building<TypesDef>,
            ) -> $crate::HookElementWithNoProps<
                impl $crate::FnOnceOutputElementHookWithNoProps<
                    $crate::Dom,
                    RenderState = impl $crate::RenderState + 'static,
                > + ::core::marker::Copy + 'static,
                impl $crate::FnOnceOutputElementHookWithNoProps<
                    $crate::AnySsrContext,
                    RenderState = impl $crate::RenderState + 'static,
                > + ::core::marker::Copy + 'static,
            > {
                $crate::HookElementWithNoProps {
                    with_dom: self::ImplDom::$name,
                    with_ssr: self::ImplSsr::$name,
                }
            }
        }
    };
    (
        $(#$attr:tt)*
        $vis:vis fn $name:ident ($ctx_arg:ident : _, $props_arg:ident : & $($props_name:ident)? $(:: $props_p:ident)* $(,)?)
        $impl_code:tt
    ) => {
        impl self:: $name ::ImplSsr {
            #[$crate::component_macro::hook(args_generics = "'render_ctx", hooks_core_path($crate::component_macro::hooks_core))]
            #[allow(non_snake_case)]
            pub fn $name<TypesDef: ?::core::marker::Sized + $($props_name)? $(:: $props_p)* ::ValidTypes>(
                $ctx_arg: $crate::ContextAndState<'render_ctx, $crate::AnySsrContext, dyn ::core::any::Any>,
                $props_arg: &$($props_name)? $(:: $props_p)* ::Data<TypesDef>,
            ) -> $crate::ContextAndState<'render_ctx, $crate::AnySsrContext, impl $crate::RenderState + 'static>
            $impl_code
        }

        impl self:: $name ::ImplDom {
            #[$crate::component_macro::hook(args_generics = "'render_ctx", hooks_core_path($crate::component_macro::hooks_core))]
            #[allow(non_snake_case)]
            pub fn $name<TypesDef: ?::core::marker::Sized + $($props_name)? $(:: $props_p)* ::ValidTypes>(
                $ctx_arg: $crate::ContextAndState<'render_ctx, $crate::Dom, dyn ::core::any::Any>,
                $props_arg: &$($props_name)? $(:: $props_p)* ::Data<TypesDef>,
            ) -> $crate::ContextAndState<'render_ctx, $crate::Dom, impl $crate::RenderState + 'static>
            $impl_code
        }

        $crate::builder! {
            $(#$attr)*
            $vis struct $name($($props_name)? $(:: $props_p)*);

            pub(super) enum ImplDom {}
            pub(super) enum ImplSsr {}

            mod build_element {
                use super::super::*;

                #[inline]
                pub fn build_element<TypesDef: 'static + ?::core::marker::Sized + $($props_name)? $(:: $props_p)* ::ValidTypes>(
                    super::Building(props): super::Building<TypesDef>,
                ) -> $crate::HookElementWithRefProps<
                    impl $crate::FnOnceOutputElementHookWithRefProps<
                            $crate::Dom,
                            $($props_name)? $(:: $props_p)* ::Data<TypesDef>,
                            RenderState = impl $crate::RenderState + 'static,
                        > + Copy
                        + 'static,
                    impl $crate::FnOnceOutputElementHookWithRefProps<
                            $crate::AnySsrContext,
                            $($props_name)? $(:: $props_p)* ::Data<TypesDef>,
                            RenderState = impl $crate::RenderState + 'static,
                        > + Copy
                        + 'static,
                    $($props_name)? $(:: $props_p)* ::Data<TypesDef>,
                > {
                    $crate::HookElementWithRefProps {
                        with_dom: super::ImplDom::$name,
                        with_ssr: super::ImplSsr::$name,
                        props,
                    }
                }
            }

            pub use build_element::build_element;
        }
    };
    (
        $(#$attr:tt)*
        $vis:vis fn $name:ident ($ctx_arg:ident : _, $props_arg:ident : $($props_name:ident)? $(:: $props_p:ident)* $(,)?)
        $impl_code:tt
    ) => {
        impl self:: $name :: ImplSsr {
            #[$crate::component_macro::hook(args_generics = "'render_ctx", hooks_core_path($crate::component_macro::hooks_core))]
            #[allow(non_snake_case)]
            pub fn $name<TypesDef: ?::core::marker::Sized + $($props_name)? $(:: $props_p)* ::ValidTypes>(
                $ctx_arg: $crate::ContextAndState<'render_ctx, $crate::AnySsrContext, dyn ::core::any::Any>,
                $props_arg: $($props_name)? $(:: $props_p)* ::Data<TypesDef>,
            ) -> $crate::ContextAndState<'render_ctx, $crate::AnySsrContext, impl $crate::RenderState + 'static>
            $impl_code
        }

        impl self:: $name :: ImplDom {
            #[$crate::component_macro::hook(args_generics = "'render_ctx", hooks_core_path($crate::component_macro::hooks_core))]
            #[allow(non_snake_case)]
            pub fn $name<TypesDef: ?::core::marker::Sized + $($props_name)? $(:: $props_p)* ::ValidTypes>(
                $ctx_arg: $crate::ContextAndState<'render_ctx, $crate::Dom, dyn ::core::any::Any>,
                $props_arg: $($props_name)? $(:: $props_p)* ::Data<TypesDef>,
            ) -> $crate::ContextAndState<'render_ctx, Dom, impl $crate::RenderState + 'static>
            $impl_code
        }

        $crate::builder! {
            $(#$attr)*
            $vis struct $name($($props_name)? $(:: $props_p)*);

            pub(super) enum ImplDom {}
            pub(super) enum ImplSsr {}

            mod build_element {
                use super::super::*;
                #[inline]
                pub fn build_element<TypesDef: 'static + ?::core::marker::Sized + $($props_name)? $(:: $props_p)* ::ValidTypes>(
                    super::Building(props): super::Building<TypesDef>,
                ) -> $crate::HookElementWithOwnedProps<
                    impl $crate::FnOnceOutputElementHookWithOwnedProps<
                            $crate::Dom,
                            $($props_name)? $(:: $props_p)* ::Data<TypesDef>,
                            RenderState = impl $crate::RenderState + 'static,
                        > + Copy
                        + 'static,
                    impl $crate::FnOnceOutputElementHookWithOwnedProps<
                            $crate::AnySsrContext,
                            $($props_name)? $(:: $props_p)* ::Data<TypesDef>,
                            RenderState = impl $crate::RenderState + 'static,
                        > + Copy
                        + 'static,
                    $($props_name)? $(:: $props_p)* ::Data<TypesDef>,
                > {
                    $crate::HookElementWithOwnedProps {
                        with_dom: super::ImplDom:: $name,
                        with_ssr: super::ImplSsr:: $name,
                        props,
                    }
                }
            }

            pub use build_element::build_element;
        }
    };
}

// #[cfg(feature = "ssr")]
// pub mod only_ssr;

#[cfg(feature = "dom")]
#[macro_export]
macro_rules! component_only_dom {
    // fn MyComp () {}
    (
        $(#$attr:tt)*
        $vis:vis fn $name:ident () $body:tt
    ) => {
        $crate::component_only_dom! {
            $(#$attr)*
            $vis fn $name (_ctx: _) $body
        }
    };
    // fn MyComp (ctx: _) {}
    (
        $(#$attr:tt)*
        $vis:vis fn $name:ident ($ctx_arg:tt : _ $(,)?)
        $impl_code:tt
    ) => {
        impl self:: $name ::ImplDom {
            #[allow(non_snake_case)]
            fn $name() -> impl $crate::frender_core::UpdateRenderState<
                $crate::frender_dom::Dom,
                State = impl $crate::frender_core::RenderState + 'static
            > + 'static {
                $crate::component_macro::detect_hooks! {
                    @[$crate::component_macro::hooks_core]
                    macro($crate::def_hook_fn_component_with_ref_props)
                    no_data($crate::NoData)
                    prepend([$ctx_arg : $crate::ContextAndState<$crate::frender_dom::Dom, _>, _: &()][()][][()])
                    $impl_code
                }
            }
        }

        $crate::bg::builder! {
            $(#$attr)*
            $vis struct $name($crate::bg::Empty);

            pub(super) enum ImplDom {}

            #[inline]
            pub fn build_element<TypesDef: ?::core::marker::Sized + $crate::bg::Empty::ValidTypes>(
                _: Building<TypesDef>,
            ) -> impl $crate::frender_core::UpdateRenderState<
                $crate::frender_dom::Dom,
                State = impl $crate::frender_core::RenderState + 'static
            > + 'static {
                self::ImplDom::$name()
            }
        }
    };
    // fn MyComp (ctx: _, props: &MyCompProps) {}
    (
        $(#$attr:tt)*
        $vis:vis fn $name:ident ($ctx_arg:ident : _, $props_arg:ident : & $($props_name:ident)? $(:: $props_p:ident)* $(,)?)
        $impl_code:tt
    ) => {
        impl self:: $name ::ImplDom {
            #[allow(non_snake_case)]
            pub fn $name<TypesDef: ?::core::marker::Sized + $($props_name)? $(:: $props_p)* ::ValidTypes>(
                props: $($props_name)? $(:: $props_p)* ::Data<TypesDef>,
            ) -> impl $crate::frender_core::UpdateRenderState<
                $crate::frender_dom::Dom,
                State = impl $crate::frender_core::RenderState
            > {
                $crate::component_macro::detect_hooks! {
                    @[$crate::component_macro::hooks_core]
                    macro($crate::def_hook_fn_component_with_ref_props)
                    no_data($crate::NoData)
                    prepend(
                        [$ctx_arg : $crate::ContextAndState<$crate::frender_dom::Dom, _>, $props_arg: &$($props_name)? $(:: $props_p)* ::Data<TypesDef>]
                        [props][][()]
                    )
                    $impl_code
                }
            }
        }

        $crate::bg::builder! {
            $(#$attr)*
            $vis struct $name($($props_name)? $(:: $props_p)*);

            pub(super) enum ImplDom {}
            pub(super) enum ImplSsr {}

            mod build_element {
                use super::super::*;

                #[inline]
                pub fn build_element<TypesDef: 'static + ?::core::marker::Sized + $($props_name)? $(:: $props_p)* ::ValidTypes>(
                    super::Building(props): super::Building<TypesDef>,
                ) -> impl $crate::frender_core::UpdateRenderState<
                    $crate::frender_dom::Dom,
                    State = impl $crate::frender_core::RenderState
                > {
                    super::ImplDom::$name (props)
                }
            }

            pub use build_element::build_element;
        }
    };
    // fn MyComp (ctx: _, props: MyCompProps) {}
    (
        $(#$attr:tt)*
        $vis:vis fn $name:ident ($ctx_arg:ident : _, $props_arg:ident : $($props_name:ident)? $(:: $props_p:ident)* $(,)?)
        $impl_code:tt
    ) => {
        impl self:: $name :: ImplDom {
            #[allow(non_snake_case)]
            pub fn $name<TypesDef: ?::core::marker::Sized + $($props_name)? $(:: $props_p)* ::ValidTypes>(
                $props_arg: $($props_name)? $(:: $props_p)* ::Data<TypesDef>,
            ) -> impl $crate::frender_core::UpdateRenderState<
                $crate::frender_dom::Dom,
                State = impl $crate::frender_core::RenderState + 'static
            > {
                $crate::component_macro::detect_hooks! {
                    @[$crate::component_macro::hooks_core]
                    macro($crate::def_hook_fn_component_with_owned_props)
                    no_data($crate::NoData)
                    prepend([$ctx_arg : $crate::ContextAndState<$crate::frender_dom::Dom, _>, _: ()][()])
                    $impl_code
                }
            }
        }

        $crate::bg::builder! {
            $(#$attr)*
            $vis struct $name($($props_name)? $(:: $props_p)*);

            pub(super) enum ImplDom {}

            mod build_element {
                use super::super::*;
                #[inline]
                pub fn build_element<TypesDef: 'static + ?::core::marker::Sized + $($props_name)? $(:: $props_p)* ::ValidTypes>(
                    super::Building(props): super::Building<TypesDef>,
                ) -> impl $crate::frender_core::UpdateRenderState<
                    $crate::frender_dom::Dom,
                    State = impl $crate::frender_core::RenderState + 'static
                > {
                    super::ImplDom:: $name (props)
                }
            }

            pub use build_element::build_element;
        }
    };
}

pub use frender_macros::detect_hooks;
pub use hooks::core as hooks_core;
pub use hooks::hook;

#[macro_export]
macro_rules! def_hook_fn_component_without_props {
    (
        [$($ctx_arg:tt)*]
        data_expr($($data_expr:tt)+)
        fn_arg_data_pat($($fn_arg_data_pat:tt)+)
        fn_stmts_extract_data($($fn_stmts_extract_data:tt)*)
        fn_stmts($($stmts:tt)*)
    ) => {
        $crate::fn_hook_component_with_no_props(
            $($data_expr)+ ,
            |$($fn_arg_data_pat)+ , $($ctx_arg)*| {
                $($fn_stmts_extract_data)*
                $($stmts)*
            }
        )
    };
}

#[macro_export]
macro_rules! def_hook_fn_component_with_owned_props {
    (
        [$($args:tt)*][$($props:tt)*]
        data_expr($($data_expr:tt)+)
        fn_arg_data_pat($($fn_arg_data_pat:tt)+)
        fn_stmts_extract_data($($fn_stmts_extract_data:tt)*)
        fn_stmts($($stmts:tt)*)
    ) => {
        $crate::fn_dom_hook_element_with_owned_props(
            $($data_expr)+ ,
            move |$($fn_arg_data_pat)+ , $($args)*| {
                $($fn_stmts_extract_data)*
                $($stmts)*
            },
            $($props)*
        )
    };
}

#[macro_export]
macro_rules! def_hook_fn_component_with_ref_props {
    (
        [$($args:tt)*][$($props:tt)*][$($dom:tt)*][$($ssr:tt)*]
        data_expr($($data_expr:tt)+)
        fn_arg_data_pat($($fn_arg_data_pat:tt)+)
        fn_stmts_extract_data($($fn_stmts_extract_data:tt)*)
        fn_stmts($($stmts:tt)*)
    ) => {
        $crate::FnHookElementWithRefProps::new_dom(
            $($data_expr)+ ,
            $($props)* ,
            $($dom)*
            move |$($fn_arg_data_pat)+ , $($args)*| {
                $($fn_stmts_extract_data)*
                $($stmts)*
            },
            $($ssr)*
        )
    };
}
