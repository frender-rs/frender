use wasm_bindgen::JsValue;

use crate::Node;

/// Corresponding to [`ReactNodeArray`] in typescript
///
/// ```typescript
/// interface ReactNodeArray extends Array<ReactNode> {}
/// ```
///
/// [`ReactNodeArray`]: https://github.com/DefinitelyTyped/DefinitelyTyped/blob/54d540ab4deb2588c0eff39dadf370cbf0a2dee4/types/react/v16/index.d.ts#L233
pub trait Nodes {
    fn as_react_node_array_js(&self) -> js_sys::Array;
}

impl<N: Node> Nodes for Vec<N> {
    fn as_react_node_array_js(&self) -> js_sys::Array {
        js_sys::Array::from_iter(
            self.iter()
                .map(|node| JsValue::from(node.as_react_node_js())),
        )
    }
}

impl<N: Node> Nodes for &[N] {
    fn as_react_node_array_js(&self) -> js_sys::Array {
        js_sys::Array::from_iter(
            self.iter()
                .map(|node| JsValue::from(node.as_react_node_js())),
        )
    }
}

impl<N: Node, const S: usize> Nodes for [N; S] {
    fn as_react_node_array_js(&self) -> js_sys::Array {
        js_sys::Array::from_iter(
            self.iter()
                .map(|node| JsValue::from(node.as_react_node_js())),
        )
    }
}

macro_rules! impl_nodes_for_tuple {
    (@impl ( $($t:ident),+ $(,)? )) => {
        impl<$($t: Node),+> Nodes for ($($t),+ ,) {
            fn as_react_node_array_js(&self) -> js_sys::Array {
                #![allow(non_snake_case)]
                let ($($t),+ ,) = self;
                js_sys::Array::from_iter([
                    $($t.as_react_node_js()),+
                ])
            }
        }
    };
    ( $(( $($t:ident),+ $(,)? ))* ) => {
        $(
            impl_nodes_for_tuple! { @impl ($($t),+ ,) }
        )*
    };
}

impl_nodes_for_tuple! {
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
