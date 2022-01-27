use wasm_bindgen::JsValue;

pub trait AsKey {
    fn as_key(&self) -> JsValue;
}

impl AsKey for String {
    #[inline]
    fn as_key(&self) -> JsValue {
        JsValue::from_str(self)
    }
}

impl<'a> AsKey for &'a str {
    #[inline]
    fn as_key(&self) -> JsValue {
        JsValue::from_str(self)
    }
}

macro_rules! impl_as_key {
    ($($t:ty)*) => {
        $(
            impl AsKey for $t {
                #[inline]
                fn as_key(&self) -> JsValue {
                    JsValue::from(*self)
                }
            }
        )*
    };
}

impl_as_key! {
    // numbers https://docs.rs/wasm-bindgen/0.2.78/src/wasm_bindgen/lib.rs.html#849
    i8 u8 i16 u16 i32 u32 f32 f64
    // big_numbers https://docs.rs/wasm-bindgen/0.2.78/src/wasm_bindgen/lib.rs.html#869
    i64 u64 i128 u128 isize usize
}
