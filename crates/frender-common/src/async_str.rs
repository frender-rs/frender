use std::{pin::Pin, task::Poll};

pub mod any_str;
pub mod chain;
pub mod empty;
pub mod never;
pub mod option;

pub trait AsyncStrIterator {
    fn poll_next_str(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<&str>>;
}

pub trait IntoAsyncStrIterator {
    type IntoAsyncStrIterator: AsyncStrIterator;
    fn into_async_str_iterator(self) -> Self::IntoAsyncStrIterator;
}

impl<T: AsyncStrIterator> IntoAsyncStrIterator for T {
    type IntoAsyncStrIterator = T;

    fn into_async_str_iterator(self) -> Self::IntoAsyncStrIterator {
        self
    }
}

impl AsyncStrIterator for &str {
    fn poll_next_str(self: Pin<&mut Self>, _: &mut std::task::Context<'_>) -> Poll<Option<&str>> {
        let s = std::mem::take(self.get_mut());
        Poll::Ready(if s.is_empty() { None } else { Some(s) })
    }
}

impl<P: std::ops::DerefMut> AsyncStrIterator for Pin<P>
where
    P::Target: AsyncStrIterator,
{
    fn poll_next_str(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<&str>> {
        P::Target::poll_next_str(crate::utils::pin_as_deref_mut(self), cx)
    }
}

crate::impl_many!(
    impl<__> IntoAsyncStrIterator
        for each_of![
            String,
            std::borrow::Cow<'_, str>,
            std::rc::Rc<str>,
            std::sync::Arc<str>,
        ]
    {
        type IntoAsyncStrIterator = any_str::IterAnyStr<Self>;
        fn into_async_str_iterator(self) -> Self::IntoAsyncStrIterator {
            any_str::AnyStr(self).into_async_str_iterator()
        }
    }
);

#[macro_export]
macro_rules! Strings {
    (
        $vis_state:vis enum $state_name:ident {}
        $vis:vis struct $name:ident
        $(<
            $($tp0:ident $($tp1:ident)? $(: $bounds:path)? $(= $tp_default:ty)?),*
            $(,)?
        >)? ($(
            $field:ident $field_bang:tt $field_info:tt
        ),* $(,)?)
        $($rest:tt)+
    ) => {
        #[allow(non_camel_case_types)]
        $vis_state enum $state_name {
            $($field,)*
            __AllDone
        }

        $crate::expand! {
            {$({$field})* {__AllDone}} get {0}
            prepend( $state_name:: )
            wrap {}
            prepend(
                #[allow(non_snake_case)]
                $vis_state fn $state_name() -> $state_name
            )
        }

        $crate::__private::pin_project! {
        $vis struct $name
        $(<
            $($tp0 $($tp1)? $(: $bounds)? $(= $tp_default)?),*
        >)?
        {
            _state: $state_name,
            $(
                #[pin]
                $field: $crate::__field_ty! $field_info,
            )*
        }
        }

        impl
        $(<
            $($tp0 $($tp1)? $(: $bounds)? $(= $tp_default)?),*
        >)?
        $crate::AsyncStrIterator for $name $(< $($crate::expand![ { $($tp1)? } or ( $tp0 ) ] ),*>)?
        {
            fn poll_next_str(
                self: ::core::pin::Pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
            ) -> ::core::task::Poll<::core::option::Option<&str>> {
                let this = self.project();

                $crate::__fields_macros! { $($field)* };

                $(
                    if let $state_name::$field = this._state {
                        $crate::__field_value!{ $field_info, this.$field.poll_next_str(cx), {
                            *this._state = $field $field_bang ({ prepend( $state_name:: ) } or __AllDone);
                        }}
                    }
                )*

                ::core::task::Poll::Ready(::core::option::Option::None)
            }
        }
    };
}

#[macro_export]
macro_rules! __field_ty {
    ($lit:literal) => {
        ()
    };
    ($ty:ty) => {
        $ty
    };
}

#[macro_export]
macro_rules! __field_value {
    (($lit:literal), $v:expr, {$($mut_state:tt)*}) => {
        $($mut_state)*
        return ::core::task::Poll::Ready(Some($lit))
    };
    (($ty:ty      ), $v:expr, {$($mut_state:tt)*}) => {
        let () = $crate::ready_none!($v);
        $($mut_state)*
    };
}

#[macro_export]
macro_rules! __fields_macros {
    ($field0:ident $field1:ident $($fields:ident)*) => {
        macro_rules! $field0 {
            ($commands:tt or $or:ident) => {
                $crate::expand! { {$field1} do $commands }
            }
        }
        $crate::__fields_macros! { $field1 $($fields)* }
    };
    ($field0:ident) => {
        macro_rules! $field0 {
            ($commands:tt or $or:ident) => {
                $crate::expand! { {$or} do $commands }
            }
        }
    };
}
