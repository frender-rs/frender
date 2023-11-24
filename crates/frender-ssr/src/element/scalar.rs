macro_rules! impl_render_scalar {
    ($(
        $for_ty:ty
    ),* $(,)?) => {$(
        impl $crate::Element for $for_ty {
            type SsrState = super::bytes::State<$crate::bytes::SlicedBytes>;

            #[inline]
            fn into_ssr_state(self) -> Self::SsrState {
                self.to_string().into_ssr_state()
            }

            type IntoIterHtmlChunk = super::str::AnyStrIter<String>;

            fn into_iter_html_chunk(self) -> Self::IntoIterHtmlChunk {
                use crate::IntoAsyncStrIterator;
                super::str::AnyStr(self.to_string()).into_async_str_iterator()
            }
        }
    )*};
}

impl_render_scalar! {
    i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize,
    f32, f64,
    // TODO: optimize for bool and char
    bool,
    char,
}
