use crate::Element;
use wasm_bindgen::JsValue;

use crate::CsrContext;

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
        impl Element for $for_ty {
            type CsrState = super::str::State<Self>;

            #[inline]
            fn into_csr_state(self, ctx: &mut CsrContext) -> Self::CsrState {
                Self::CsrState::initialize_with_js_value(self, ctx, js_value_from_deref)
            }

            #[inline]
            fn update_csr_state(self, ctx: &mut CsrContext, state: std::pin::Pin<&mut Self::CsrState>) {
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

impl Element for char {
    type CsrState = super::str::State<Self>;

    #[inline]
    fn into_csr_state(self, ctx: &mut CsrContext) -> Self::CsrState {
        Self::CsrState::initialize_with_js_string(self, ctx, char_to_js_string)
    }

    #[inline]
    fn update_csr_state(self, ctx: &mut CsrContext, state: std::pin::Pin<&mut Self::CsrState>) {
        let this = state.get_mut();
        this.update_with_js_string(self, ctx, char_to_js_string);
    }
}
