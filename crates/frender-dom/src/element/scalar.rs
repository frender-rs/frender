use frender_core::UpdateRenderState;
use wasm_bindgen::JsValue;

use crate::Dom;

macro_rules! impl_render_scalar {
    ($(
        $for_ty:ty
    ),* $(,)?) => {$(
        impl UpdateRenderState<Dom> for $for_ty {
            type State = super::str::State<Self>;

            #[inline]
            fn update_render_state(self, ctx: &mut Dom, state: std::pin::Pin<&mut Self::State>) {
                let this = state.get_mut();
                this.update_with_js_value(self, ctx, |v| JsValue::from(*v));
            }
        }
    )*};
}

impl_render_scalar! {
    i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize,
    f32, f64,
    bool,
}

impl UpdateRenderState<Dom> for char {
    type State = super::str::State<Self>;

    #[inline]
    fn update_render_state(self, ctx: &mut Dom, state: std::pin::Pin<&mut Self::State>) {
        let this = state.get_mut();
        this.update_with_js_string(self, ctx, |v| (*v).into());
    }
}
