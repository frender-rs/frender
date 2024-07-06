use std::pin::Pin;

use crate::{Element, HtmlRenderContext, RenderStateOfContext};

frender_common::impl_many!(
    impl<__> Element
        for each_of![
            i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, //
            f32, f64, //
            // TODO: Optimize char
            char,
        ]
    {
        type RenderStateKind = super::str::Kind<Self>;

        fn render_update_maybe_reposition<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>, force_reposition: bool) {
            super::str::render_update_maybe_reposition::<Self, Self, Self, Ctx>(self, render_context, render_state, force_reposition, PartialEq::ne, |vv, v| *vv = v, std::convert::identity)
        }

        crate::impl_unpinned_render_for_unpin! {}
    }
);
