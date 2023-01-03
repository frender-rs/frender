use std::{borrow::Cow, pin::Pin};

use crate::{RenderState, UpdateRenderState};

pub trait RenderWithElement<E> {
    type State: RenderState;

    fn render_with_element(&mut self, state: Pin<&mut Self::State>, element: E);
}

macro_rules! impl_render_with_element {
    (
        $(
            $(@[$($generics:tt)+])?
            $for_ty:ty
        ),* $(,)?
    ) => {$(
        impl<$($($generics)+ ,)? Ctx: RenderWithElement<$for_ty>> UpdateRenderState<Ctx> for $for_ty {
            type State = Ctx::State;

            #[inline]
            fn update_render_state(self, ctx: &mut Ctx, state: ::core::pin::Pin<&mut Self::State>) {
                ctx.render_with_element(state, self)
            }
        }
    )*};
}

impl_render_with_element! {
    i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize,
    f32, f64,
    bool,
    char,
    @['a] &'a str,
    String,
    @['a] Cow<'a, str>,
    @[S: crate::StaticStr] crate::StaticText<S>,
}
