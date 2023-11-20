use crate::Element;

macro_rules! impl_render_scalar {
    (
        impl<__> each_of! {$(
            $for_ty:ty
        ),* $(,)?}
        $t:tt
    ) => {$(
        impl Element for $for_ty $t
    )*};
}

impl_render_scalar!(
    impl<__>
        each_of! {
            i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize,
            f32, f64,
            bool,
        }
    {
        type RenderState<R: crate::RenderHtml> = Option<super::str::State<Self, R::Text>>;

        fn render_update_maybe_reposition<Renderer: crate::RenderHtml>(self, renderer: &mut Renderer, render_state: std::pin::Pin<&mut Self::RenderState<Renderer>>, force_reposition: bool) {
            super::str::render_update_maybe_reposition::<Self, Self, Self, _>(self, renderer, render_state, force_reposition, PartialEq::ne, |vv, v| *vv = v, std::convert::identity)
        }
    }
);