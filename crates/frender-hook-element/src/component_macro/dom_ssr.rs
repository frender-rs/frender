#[macro_export]
macro_rules! def_component {
    (
        $(#$attr:tt)*
        $vis:vis fn $name:ident () $body:tt
    ) => {
        $crate::def_component! {
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
            pub fn build_element<TypesDef: ?Sized + $crate::bg::Empty::ValidTypes>(
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
            pub fn $name<TypesDef: ?Sized + $($props_name)? $(:: $props_p)* ::ValidTypes>(
                $ctx_arg: $crate::ContextAndState<'render_ctx, $crate::AnySsrContext, dyn ::core::any::Any>,
                $props_arg: &$($props_name)? $(:: $props_p)* ::Data<TypesDef>,
            ) -> $crate::ContextAndState<'render_ctx, $crate::AnySsrContext, impl $crate::RenderState + 'static>
            $impl_code
        }

        impl self:: $name ::ImplDom {
            #[$crate::component_macro::hook(args_generics = "'render_ctx", hooks_core_path($crate::component_macro::hooks_core))]
            #[allow(non_snake_case)]
            pub fn $name<TypesDef: ?Sized + $($props_name)? $(:: $props_p)* ::ValidTypes>(
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
                pub fn build_element<TypesDef: 'static + ?Sized + $($props_name)? $(:: $props_p)* ::ValidTypes>(
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
            pub fn $name<TypesDef: ?Sized + $($props_name)? $(:: $props_p)* ::ValidTypes>(
                $ctx_arg: $crate::ContextAndState<'render_ctx, $crate::AnySsrContext, dyn ::core::any::Any>,
                $props_arg: $($props_name)? $(:: $props_p)* ::Data<TypesDef>,
            ) -> $crate::ContextAndState<'render_ctx, $crate::AnySsrContext, impl $crate::RenderState + 'static>
            $impl_code
        }

        impl self:: $name :: ImplDom {
            #[$crate::component_macro::hook(args_generics = "'render_ctx", hooks_core_path($crate::component_macro::hooks_core))]
            #[allow(non_snake_case)]
            pub fn $name<TypesDef: ?Sized + $($props_name)? $(:: $props_p)* ::ValidTypes>(
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
                pub fn build_element<TypesDef: 'static + ?Sized + $($props_name)? $(:: $props_p)* ::ValidTypes>(
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
