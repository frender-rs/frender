use wasm_bindgen::JsValue;

/// Corresponding to `ReactNode` in typescript
///
/// ```typescript
/// type ReactNode = ReactElement | string | number | ReactNodeArray | boolean | null | undefined;
/// ```
pub trait Node {
    fn as_react_node_js(&self) -> JsValue;
}

impl Node for () {
    fn as_react_node_js(&self) -> JsValue {
        JsValue::UNDEFINED
    }
}

impl<T: Node> Node for Option<T> {
    fn as_react_node_js(&self) -> JsValue {
        if let Some(node) = self {
            node.as_react_node_js()
        } else {
            JsValue::NULL
        }
    }
}

impl<T: crate::Nodes> Node for T {
    fn as_react_node_js(&self) -> JsValue {
        self.as_react_node_array_js().into()
    }
}

macro_rules! into_js_node {
    (deref: $($n:ty)*) => ($(
        impl Node for $n {
            #[inline]
            fn as_react_node_js(&self) -> JsValue {
                JsValue::from(*self)
            }
        }
    )*);
    ($($n:ty)*) => ($(
        impl Node for $n {
            #[inline]
            fn as_react_node_js(&self) -> JsValue {
                JsValue::from(self)
            }
        }
    )*);
}

into_js_node! {
    react_sys::Element
    String
}

into_js_node! {
    deref:
    &str
    // numbers https://docs.rs/wasm-bindgen/0.2.78/src/wasm_bindgen/lib.rs.html#849
    i8 u8 i16 u16 i32 u32 f32 f64
    // big_numbers https://docs.rs/wasm-bindgen/0.2.78/src/wasm_bindgen/lib.rs.html#869
    i64 u64 i128 u128 isize usize
    bool
}
