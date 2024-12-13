// region: filter

macro_rules! process_one {
    (
        {$fn_name:ident ($value:ident : event! $event:tt);}
        {
            rest $rest:tt
            recorded { $($recorded:tt)* }
            do $commands:tt
        }
    ) => {
        crate::macros::event_names::continue_or_return! {
            rest $rest
            recorded { $($recorded)* $fn_name }
            do $commands
        }
    };
    (
        {$fn_name:ident $($fn_rest:tt)*}
        $options:tt
    ) => {
        crate::macros::event_names::continue_or_return! $options
    };
}

macro_rules! continue_or_return {
    (
        rest {}
        recorded $recorded:tt
        do $commands:tt
    ) => {
        ::frender_common::expand! {
            $recorded
            do $commands
        }
    };
    (
        rest {$one:tt $($rest:tt)*}
        recorded $recorded:tt
        do $commands:tt
    ) => {
        crate::macros::event_names::process_one! {
            $one
            {
                rest {$($rest)*}
                recorded $recorded
                do $commands
            }
        }
    };
}

macro_rules! filter {
    (
        $fns:tt // wrapped with {}. each item is wrapped with {}
        do $commands:tt
    ) => {
        crate::macros::event_names::continue_or_return! {
            rest $fns
            recorded {}
            do $commands
        }
    };
}

// endregion

macro_rules! expand_impl {
    (
        {$($fn_name:tt)*}
        {$name:tt}
        do $commands:tt
    ) => {
        ::frender_common::expand! {
            {$(
                $name<super::event_types::$fn_name> +
            )*} do $commands
        }
    };
    (
        {$($fn_name:tt)*}
        {$name:tt $generic:tt}
        do $commands:tt
    ) => {
        ::frender_common::expand! {
            {$(
                $name<$generic, super::event_types::$fn_name> +
            )*} do $commands
        }
    };
}

macro_rules! expand {
    (
        $fns:tt
        args $args:tt
        do $commands:tt
    ) => {
        crate::macros::event_names::filter! {
            $fns do {
                wrap {}
                append { $args do $commands }
                wrap {} prepend { crate::macros::event_names::expand_impl! }
            }
        }
    };
}

macro_rules! expand_macro_impl {
    (
        $fn_names:tt
        $macro_name:ident
    ) => {
        macro_rules! $macro_name {
            ($commands:tt) => {
                ::frender_common::expand! {
                    $fn_names
                    do $commands
                }
            };
            (args $args:tt do $commands:tt) => {
                crate::macros::event_names::expand_impl! {
                    $fn_names
                    $args
                    do $commands
                }
            };
        }
    };
}

macro_rules! expand_macro {
    ($fns:tt $macro_name:ident) => {
        crate::macros::event_names::filter! {
            $fns
            do {
                wrap {}
                append { $macro_name }
                wrap {} prepend { crate::macros::event_names::expand_macro_impl! }
            }
        }
    };
}

macro_rules! impl_OnEventType {
    (
        {$($fn_name:ident)*}
        $for_ty:ty
    ) => {$(
        impl OnEventType<super::event_types::$fn_name> for $for_ty {
            type OnEvent<R: ?Sized + RenderHtml> = Self::OfBehaviorType<R>;
        }
    )*};
}

pub(crate) use {continue_or_return, expand, expand_impl, expand_macro, expand_macro_impl, filter, impl_OnEventType, process_one};
