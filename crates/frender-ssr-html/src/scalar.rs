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

impl Scalar {
    pub fn new<V: IntoScalar>(value: V) -> Self {
        V::into_scalar(value)
    }
}

pub trait IntoScalar {
    fn into_scalar(this: Self) -> Scalar;
}

frender_common::impl_many!(
    impl<__> IntoScalar
        for each_of![
            i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64, char,
        ]
    {
        fn into_scalar(this: Self) -> Scalar {
            Scalar(AnyStr(this.to_string()).into_async_str_iterator())
        }
    }
);
