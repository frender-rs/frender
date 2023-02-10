use frender_core::UpdateRenderState;
use wasm_bindgen::JsValue;

use crate::Dom;

fn js_value_from_deref<V: Copy>(v: &V) -> JsValue
where
    JsValue: From<V>,
{
    JsValue::from(*v)
}

macro_rules! impl_render_scalar {
    ($(
        $for_ty:ty
    ),* $(,)?) => {$(
        impl UpdateRenderState<Dom> for $for_ty {
            type State = super::str::State<Self>;

        #[inline]
        fn initialize_render_state(self, ctx: &mut Dom) -> Self::State {
                Self::State::initialize_with_js_value(self, ctx, js_value_from_deref)
            }

        #[inline]
        fn update_render_state(self, ctx: &mut Dom, state: std::pin::Pin<&mut Self::State>) {
                let this = state.get_mut();
                this.update_with_js_value(self, ctx, js_value_from_deref);
            }
        }
    )*};
}

impl_render_scalar! {
    i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize,
    f32, f64,
    bool,
}

fn char_to_js_string(v: &char) -> js_sys::JsString {
    (*v).into()
}

impl UpdateRenderState<Dom> for char {
    type State = super::str::State<Self>;

    #[inline]
    fn initialize_render_state(self, ctx: &mut Dom) -> Self::State {
        Self::State::initialize_with_js_string(self, ctx, char_to_js_string)
    }

    #[inline]
    fn update_render_state(self, ctx: &mut Dom, state: std::pin::Pin<&mut Self::State>) {
        let this = state.get_mut();
        this.update_with_js_string(self, ctx, char_to_js_string);
    }
}
