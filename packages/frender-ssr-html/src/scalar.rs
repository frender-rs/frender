use async_str_iter::{
    any_str::{AnyStr, IterAnyStr},
    AsyncStrIterator, IntoAsyncStrIterator,
};

/// A stringified scalar value which implements [`HtmlChildren`](crate::assert::HtmlChildren).
pub struct Scalar(IterAnyStr<String>);

impl AsyncStrIterator for Scalar {
    fn poll_next_str(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<&str>> {
        std::pin::Pin::new(&mut self.get_mut().0).poll_next_str(cx)
    }
}

macro_rules! impl_from {
    (
        impl $(<__>)? $From:ident<$($from_ty:ty),+ $(,)?> for $for_ty:ty {
            fn $from:ident($value:ident : _) -> $Self:ident
            $body:tt
        }
    ) => {
        $(
            impl $From<$from_ty> for $for_ty {
                fn $from($value: $from_ty) -> $Self
                $body
            }
        )+
    };
}

impl_from!(
    impl<__> From<i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64> for Scalar {
        fn from(value: _) -> Self {
            Scalar(AnyStr(value.to_string()).into_async_str_iterator())
        }
    }
);
