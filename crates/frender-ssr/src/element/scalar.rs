use frender_core::UpdateRenderState;

macro_rules! impl_render_scalar {
    ($(
        $for_ty:ty
    ),* $(,)?) => {$(
        impl<W: $crate::AsyncWrite + Unpin> UpdateRenderState<$crate::SsrContext<W>> for $for_ty {
            type State = super::bytes::State<W, $crate::bytes::SlicedBytes>;

            #[inline]
            fn initialize_render_state(self, ctx: &mut crate::SsrContext<W>) -> Self::State {
                self.to_string().initialize_render_state(ctx)
            }

            #[inline]
            fn update_render_state(self, ctx: &mut $crate::SsrContext<W>, state: std::pin::Pin<&mut Self::State>) {
                self.to_string().update_render_state(ctx, state)
            }
        }
    )*};
}

impl_render_scalar! {
    i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize,
    f32, f64,
    bool,
    char,
}
