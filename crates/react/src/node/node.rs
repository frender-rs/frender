use wasm_bindgen::JsValue;

use crate::AnyNode;

/// Corresponding to `ReactNode` in typescript
///
/// ```typescript
/// type ReactNode = ReactElement | string | number | ReactNodeArray | boolean | null | undefined;
/// ```
pub trait Node {
    fn as_react_node_js(&self) -> JsValue;

    #[inline]
    fn as_react_children_js(&self) -> Option<js_sys::Array> {
        Some(js_sys::Array::of1(&self.as_react_node_js()))
    }

    #[inline]
    fn into_any_node(self) -> AnyNode
    where
        Self: Sized + 'static,
    {
        AnyNode::wrap(self)
    }
}

impl Node for () {
    #[inline]
    fn as_react_node_js(&self) -> JsValue {
        JsValue::UNDEFINED
    }

    #[inline]
    fn as_react_children_js(&self) -> Option<js_sys::Array> {
        None
    }
}

impl<T: Node> Node for Option<T> {
    #[inline]
    fn as_react_node_js(&self) -> JsValue {
        if let Some(node) = self {
            node.as_react_node_js()
        } else {
            JsValue::NULL
        }
    }

    #[inline]
    fn as_react_children_js(&self) -> Option<js_sys::Array> {
        self.as_ref().and_then(Node::as_react_children_js)
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
    js_sys::JsString
    js_sys::Number
    js_sys::BigInt
    js_sys::Boolean
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

macro_rules! impl_node_for_iter {
    ($($t:tt)+) => {
        $($t)+ {
            #[inline]
            fn as_react_node_js(&self) -> JsValue {
                js_sys::Array::from_iter(self.iter().map(|node| node.as_react_node_js())).into()
            }
        }
    };
}

impl_node_for_iter! { impl<T: Node> Node for Vec<T> }
impl_node_for_iter! { impl<T: Node> Node for &[T] }
impl_node_for_iter! { impl<N: Node, const S: usize> Node for [N; S] }

macro_rules! impl_node_for_tuple {
    (@impl ( $($t:ident),+ $(,)? )) => {
        impl<$($t: Node),+> Node for ($($t),+ ,) {
            #[inline]
            fn as_react_node_js(&self) -> JsValue {
                #![allow(non_snake_case)]
                let ($($t),+ ,) = self;
                js_sys::Array::from_iter([
                    $($t.as_react_node_js()),+
                ]).into()
            }
        }
    };
    ( $(( $($t:ident),+ $(,)? ))* ) => {
        $(
            impl_node_for_tuple! { @impl ($($t),+ ,) }
        )*
    };
}

impl_node_for_tuple! {
    (T0,)
    (T0,T1)
    (T0,T1,T2)
    (T0,T1,T2,T3)
    (T0,T1,T2,T3,T4)
    (T0,T1,T2,T3,T4,T5)
    (T0,T1,T2,T3,T4,T5,T6)
    (T0,T1,T2,T3,T4,T5,T6,T7)
    (T0,T1,T2,T3,T4,T5,T6,T7,T8)
    (T0,T1,T2,T3,T4,T5,T6,T7,T8,T9)
}

impl<N: Node> Node for Box<N> {
    #[inline]
    fn as_react_node_js(&self) -> JsValue {
        self.as_ref().as_react_node_js()
    }

    #[inline]
    fn into_any_node(self) -> AnyNode
    where
        N: 'static,
    {
        AnyNode(self)
    }
}
