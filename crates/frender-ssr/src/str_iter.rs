use std::{pin::Pin, task::Poll};

pub trait AsyncStrIterator {
    fn poll_next_str(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<&str>>;
}

pin_project_lite::pin_project!(
    pub struct Vectored<T> {
        #[pin]
        v: T,
        current: usize,
    }
);

impl<Item> AsyncStrIterator for Vectored<Vec<Item>>
where
    Item: AsyncStrIterator,
{
    fn poll_next_str(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<&str>> {
        let this = self.project();

        while *this.current < this.v.len() {
            let item = frender_common::utils::pin_project_index_vec(this.v, *this.current);
            let () = crate::ready_none!(item.poll_next_str(cx), {
                *this.current += 1;
            });
        }

        Poll::Ready(None)
    }
}

pub trait IntoAsyncStrIterator {
    type IntoAsyncStrIterator: AsyncStrIterator;
    fn into_async_str_iterator(self) -> Self::IntoAsyncStrIterator;
}

pin_project_lite::pin_project!(
    #[project = ChainProj]
    pub struct Chain<A, B> {
        a_ready: bool,
        #[pin]
        a: A,
        #[pin]
        b: B,
    }
);

impl<A: AsyncStrIterator, B: AsyncStrIterator> AsyncStrIterator for Chain<A, B> {
    fn poll_next_str(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<&str>> {
        let ChainProj { a_ready, a, b } = self.project();

        if !*a_ready {
            crate::ready_none!(a.poll_next_str(cx), {
                *a_ready = true;
            })
        }

        b.poll_next_str(cx)
    }
}

impl<T: AsyncStrIterator> IntoAsyncStrIterator for T {
    type IntoAsyncStrIterator = T;

    fn into_async_str_iterator(self) -> Self::IntoAsyncStrIterator {
        self
    }
}

impl IntoAsyncStrIterator for &str {
    type IntoAsyncStrIterator = Option<Self>;

    fn into_async_str_iterator(self) -> Self::IntoAsyncStrIterator {
        Some(self)
    }
}

impl AsyncStrIterator for Option<&str> {
    fn poll_next_str(self: Pin<&mut Self>, _: &mut std::task::Context<'_>) -> Poll<Option<&str>> {
        Poll::Ready(self.get_mut().take())
    }
}

#[macro_export]
macro_rules! Strings {
    (
        enum $state_name:ident {}
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
        enum $state_name {
            $($field,)*
            __AllDone
        }

        $crate::__private::expand! {
            {$({$field})* {__AllDone}} get {0}
            prepend( $state_name:: )
            wrap {}
            prepend(
                #[allow(non_snake_case)]
                fn $state_name() -> $state_name
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
        $crate::AsyncStrIterator for $name $(< $($crate::__private::expand![ { $($tp1)? } or ( $tp0 ) ] ),*>)?
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
    (($lit:literal), $v:expr , {$($mut_state:tt)*}) => {
        $($mut_state)*
        return ::core::task::Poll::Ready(Some($lit))
    };
    (($ty:ty      ), $v:expr, $mut_state:tt) => {
        let () = $crate::ready_none!($v, $mut_state);
    };
}

#[macro_export]
macro_rules! __fields_macros {
    ($field0:ident $field1:ident $($fields:ident)*) => {
        macro_rules! $field0 {
            ($commands:tt or $or:ident) => {
                $crate::__private::expand! { {$field1} do $commands }
            }
        }
        $crate::__fields_macros! { $field1 $($fields)* }
    };
    ($field0:ident) => {
        macro_rules! $field0 {
            ($commands:tt or $or:ident) => {
                $crate::__private::expand! { {$or} do $commands }
            }
        }
    };
}

pin_project_lite::pin_project!(
    pub struct Encode<E: crate::EscapeSafe, S: AsyncStrIterator> {
        escape: E,
        #[pin]
        s: S,
        encoded: String,
    }
);

impl<E: crate::EscapeSafe, S: AsyncStrIterator> Encode<E, S> {
    pub fn new(escape: E, s: S) -> Self {
        Self {
            escape,
            s,
            encoded: String::new(),
        }
    }
}

impl<E: crate::EscapeSafe, S: AsyncStrIterator> AsyncStrIterator for Encode<E, S> {
    fn poll_next_str(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<&str>> {
        let this = self.project();
        this.s.poll_next_str(cx).map(|v| {
            v.map(|v| match this.escape.escape_safe(v) {
                std::borrow::Cow::Borrowed(v) => v,
                std::borrow::Cow::Owned(v) => {
                    *this.encoded = v;
                    &*this.encoded
                }
            })
        })
    }
}

#[cfg(test)]
mod test {
    // use super::AsyncStrIterator;
    Strings![
        enum MyDivState {}
        pub struct MyElement<Attrs: super::AsyncStrIterator>(
            //
            lt!("<"),
            tag!(Option<&'static str>),
            attrs!(Attrs),
            gt!(">"),
        );
    ];

    #[allow(non_snake_case)]
    pub fn MyElement<Attrs: super::AsyncStrIterator>(
        tag: &'static str,
        attrs: Attrs,
    ) -> MyElement<Attrs> {
        MyElement {
            _state: MyDivState(),
            lt: (),
            tag: Some(tag),
            attrs,
            gt: (),
        }
    }

    #[test]
    fn test() {
        let el = MyElement("div", None::<&str>);
        let mut el = std::pin::pin!(el);

        futures_lite::future::block_on(async {
            let mut s = String::new();
            while let Some(()) = std::future::poll_fn(|cx| {
                use super::AsyncStrIterator;
                el.as_mut()
                    .poll_next_str(cx)
                    .map(|v| v.map(|v| s.push_str(v)))
            })
            .await
            {}

            assert_eq!(s, "<div>")
        })
    }
}
