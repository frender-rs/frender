use crate::Element;

frender_common::impl_many!(
    impl<__> Element
        for each_of![
            i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, //
            f32, f64, //
            // TODO: Optimize char
            char,
        ]
    {
        type RenderState<R: crate::RenderHtml> = Option<super::str::State<Self, R::Text>>;

        fn render_update_maybe_reposition<Renderer: crate::RenderHtml>(self, renderer: &mut Renderer, render_state: std::pin::Pin<&mut Self::RenderState<Renderer>>, force_reposition: bool) {
            super::str::render_update_maybe_reposition::<Self, Self, Self, _>(self, renderer, render_state, force_reposition, PartialEq::ne, |vv, v| *vv = v, std::convert::identity)
        }

        crate::impl_unpinned_render_for_unpin! {}
    }
);
