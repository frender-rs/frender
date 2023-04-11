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
        type CsrState = super::str::State<Self>;

        #[inline]
        fn into_csr_state(self, ctx: &mut CsrContext) -> Self::CsrState {
            Self::CsrState::initialize_with_js_value(self, ctx, js_value_from_deref)
        }

        #[inline]
        fn update_csr_state_maybe_reposition(
            self,
            ctx: &mut crate::CsrContext,
            state: std::pin::Pin<&mut Self::CsrState>,
            force_reposition: bool,
        ) {
            state.get_mut().update_with_js_value_maybe_reposition(
                self,
                ctx,
                js_value_from_deref,
                force_reposition,
            );
        }
    }
);

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
    fn update_csr_state_maybe_reposition(
        self,
        ctx: &mut crate::CsrContext,
        state: std::pin::Pin<&mut Self::CsrState>,
        force_reposition: bool,
    ) {
        state.get_mut().update_with_js_string_maybe_reposition(
            self,
            ctx,
            char_to_js_string,
            force_reposition,
        );
    }
}
