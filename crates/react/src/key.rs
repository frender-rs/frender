use wasm_bindgen::JsValue;

#[derive(Debug, Clone)]
pub struct Key {
    inner: JsValue,
}

impl Into<JsValue> for Key {
    #[inline]
    fn into(self) -> JsValue {
        self.inner
    }
}

impl AsRef<JsValue> for Key {
    #[inline]
    fn as_ref(&self) -> &JsValue {
        &self.inner
    }
}

pub trait AsKey {
    fn as_key(&self) -> Key;
}

impl AsKey for String {
    #[inline]
    fn as_key(&self) -> Key {
        Key {
            inner: JsValue::from_str(self),
        }
    }
}

impl AsKey for str {
    #[inline]
    fn as_key(&self) -> Key {
        Key {
            inner: JsValue::from_str(self),
        }
    }
}

impl<T: AsKey> AsKey for &T {
    #[inline]
    fn as_key(&self) -> Key {
        (*self).as_key()
    }
}

macro_rules! impl_as_key {
    ($($t:ty)*) => {
        $(
            impl AsKey for $t {
                #[inline]
                fn as_key(&self) -> Key {
                    Key { inner: JsValue::from(*self) }
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
