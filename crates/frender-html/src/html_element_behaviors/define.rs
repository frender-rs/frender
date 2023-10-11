macro_rules! define_one {
    (
        $(@super_traits!($($super_traits:ident),* $(,)?);)?
        $(@special_extends!($($special_extends:ident),* $(,)?);)?
        $vis:vis trait $trait_name:ident {
            $(super_traits!($($additional_super_traits:ident),* $(,)?);)?
            $(extends!($($extends:ident),* $(,)?);)?
            $(special_extends!($($self_special_extends:ident),* $(,)?);)?
            $(special_impl_for!($($special_impl_for:ty),* $(,)?);)?

            $(
                $(#$fn_attrs:tt)*
                fn $fn_name:ident($($fn_value_args:tt)*)
                $fn_body_or_semi:tt
            )*

            $(sub_traits!($($sub_traits:tt)*);)?
        }
    ) => {
        $vis trait $trait_name<Renderer: ?Sized>:
            $($($super_traits<Renderer> +)*)?
            $($($additional_super_traits<Renderer> +)*)?
            $($($extends<Renderer> +)*)?
            $($($self_special_extends<Renderer> +)*)?
        {
            $(
                $(#$fn_attrs)*
                fn $fn_name(&mut self, renderer: &mut Renderer, $($fn_value_args)*);
            )*
        }

        crate::expand! {{

        #[cfg(feature = "web")]
        impl<
            Renderer: ?Sized + crate::csr::web::Renderer,
            E: AsRef<web_sys::$trait_name>
                $($(+ AsRef<web_sys::$super_traits>)*)?
                $($(+ AsRef<web_sys::$additional_super_traits>)*)?
            ,
        > $trait_name<Renderer> for crate::csr::web::Node<E>
        where $(Self: $($self_special_extends<Renderer> +)*,)?
        {$(
            fn $fn_name(&mut self, _: &mut Renderer, $($fn_value_args)*) {
                crate::html_element_behaviors::define::check_fn_body_or_semi! {
                    $fn_body_or_semi $fn_name {
                        wrap {}
                        append( AsRef::<web_sys::$trait_name>::as_ref(&self.0) )
                        wrap {}
                        prepend( crate::html_element_behaviors::define::resolve_fn_body! )
                    } {
                        prepend( AsRef::<web_sys::$trait_name>::as_ref(&self.0). )
                        wrap () // (el.)
                        prepend(wrap() prepend) // wrap() prepend(el.)
                        wrap {} // { wrap() prepend(el.) }
                        prepend( {$($fn_value_args)*} )
                        wrap ()
                        prepend( crate::html_element_behaviors::define::extract_args! )
                    }
                }
            }
        )*}

        } unless ( $($($special_impl_for)*)? ) }

        crate::expand! { while ($($({$special_impl_for})*)?) {
        prepend(
            #[cfg(feature = "web")]
            impl<
                Renderer: ?Sized + crate::csr::web::Renderer,
            > $trait_name<Renderer> for crate::csr::web::Node<
        ) append(> {$(
            fn $fn_name(&mut self, _: &mut Renderer, $($fn_value_args)*) {
                crate::html_element_behaviors::define::check_fn_body_or_semi! {
                    $fn_body_or_semi $fn_name {
                        wrap {}
                        append( self.0 )
                        wrap {}
                        prepend( crate::html_element_behaviors::define::resolve_fn_body! )
                    } {
                        prepend( self.0. )
                        wrap () // (el.)
                        prepend(wrap() prepend) // wrap() prepend(el.)
                        wrap {} // { wrap() prepend(el.) }
                        prepend( {$($fn_value_args)*} )
                        wrap ()
                        prepend( crate::html_element_behaviors::define::extract_args! )
                    }
                }
            }
        )*}

        )}}

        crate::for_each_trait! {{$($($sub_traits)*)?} {
            prepend(
                @super_traits!($trait_name $($(, $super_traits)*)? $($(, $additional_super_traits)*)?);
                @special_extends!($($($special_extends,)*)? $($($self_special_extends,)*)?);
            )
            wrap {}
            prepend(crate::html_element_behaviors::define::define_one!)
        }}
    };
}

macro_rules! define_one_prelude {
    (
        $(@super_traits!($($super_traits:ident),* $(,)?);)?
        $(@special_extends!($($special_extends:ident),* $(,)?);)?
        $vis:vis trait $trait_name:ident {
            $(super_traits!($($additional_super_traits:ident),* $(,)?);)?
            $(extends!($($extends:ident),* $(,)?);)?
            $(special_extends!($($self_special_extends:ident),* $(,)?);)?
            $(special_impl_for!($($special_impl_for:ty),* $(,)?);)?

            $(
                $(#$fn_attrs:tt)*
                fn $fn_name:ident($($fn_value_args:tt)*)
                $fn_body_or_semi:tt
            )*

            $(sub_traits!($($sub_traits:tt)*);)?
        }
    ) => {
        $vis mod $trait_name {
            crate::expand! { while (
                {$trait_name}
                $($({$super_traits})*)?
                $($({$additional_super_traits})*)?
                $($({$extends})*)?
                $($({$self_special_extends})*)?
            ) {
                prepend( $vis use crate::renderer::node_behaviors:: )
                append( as _; )
            } }
        }

        crate::for_each_trait! {{$($($sub_traits)*)?} {
            prepend(
                @super_traits!($trait_name $($(, $super_traits)*)? $($(, $additional_super_traits)*)?);
                @special_extends!($($($special_extends,)*)? $($($self_special_extends,)*)?);
            )
            wrap {}
            prepend(crate::html_element_behaviors::define::define_one_prelude!)
        }}
    };
}

macro_rules! define {
    (
        $vis:vis mod $prelude:ident { $($prelude_body:tt)* }
        $($t:tt)*
    ) => {
        $vis mod $prelude {
            $($prelude_body)*

            crate::for_each_trait! {{$($t)*} {
                wrap {}
                prepend(crate::html_element_behaviors::define::define_one_prelude!)
            }}
        }

        crate::for_each_trait! {{$($t)*} {
            wrap {}
            prepend(crate::html_element_behaviors::define::define_one!)
        }}
    };
}

macro_rules! check_fn_body_or_semi {
    ( ;       $fn_name:ident $do_if_body:tt $do_if_simple:tt) => {
        crate::expand! { {$fn_name} do $do_if_simple }
    };
    (
        {#![web_sys_name($web_sys_name:ident)]}
              $fn_name:ident $do_if_body:tt $do_if_simple:tt
    ) => {
        crate::expand! { {$web_sys_name} do $do_if_simple }
    };
    ($body:tt $fn_name:ident $do_if_body:tt $do_if_simple:tt) => {
        crate::expand! { $body do $do_if_body }
    };
}

macro_rules! resolve_fn_body {
    ({ |$element:pat_param| $e:expr } $element_expr:expr) => {
        let $element = $element_expr;
        $e
    };
}

macro_rules! extract_args {
    ({$($arg:ident : $ty:ty)*} $commands:tt) => {
        crate::expand! { {$($arg,)*} do $commands }
    };
}

pub(super) use check_fn_body_or_semi;
pub(super) use define;
pub(super) use define_one;
pub(super) use define_one_prelude;
pub(super) use extract_args;
pub(super) use resolve_fn_body;
