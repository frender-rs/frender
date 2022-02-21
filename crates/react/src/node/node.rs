use wasm_bindgen::JsValue;

use crate::{AnyNode, AnyNodeValue, Children, IntoElement, Keyed, KeyedElement};

/// Corresponding to `ReactNode` in typescript
///
/// ```typescript
/// type ReactNode = ReactElement | string | number | ReactNodeArray | boolean | null | undefined;
/// ```
pub trait Node {
    fn as_react_node_js(&self) -> AnyNode;

    #[inline]
    fn as_react_children_js(&self) -> Option<Children> {
        Some(Children::from_single(self.as_react_node_js()))
    }

    #[inline]
    fn into_react_node_js(self) -> AnyNode
    where
        Self: Sized,
    {
        self.as_react_node_js()
    }

    #[inline]
    fn into_react_children_js(self) -> Option<Children>
    where
        Self: Sized,
    {
        self.as_react_children_js()
    }
}

impl Node for () {
    #[inline]
    fn as_react_node_js(&self) -> AnyNode {
        AnyNode::Null
    }

    #[inline]
    fn as_react_children_js(&self) -> Option<Children> {
        None
    }
}

impl<T: Node> Node for Option<T> {
    #[inline]
    fn as_react_node_js(&self) -> AnyNode {
        if let Some(node) = self {
            node.as_react_node_js()
        } else {
            AnyNode::Null
        }
    }

    #[inline]
    fn as_react_children_js(&self) -> Option<Children> {
        self.as_ref().and_then(Node::as_react_children_js)
    }

    #[inline]
    fn into_react_node_js(self) -> AnyNode {
        self.map(Node::into_react_node_js).unwrap_or(AnyNode::Null)
    }

    #[inline]
    fn into_react_children_js(self) -> Option<Children> {
        self.and_then(Node::into_react_children_js)
    }
}

macro_rules! into_js_node {
    (deref: $($n:ty)*) => ($(
        impl Node for $n {
            #[inline]
            fn as_react_node_js(&self) -> AnyNode {
                AnyNode::Value(AnyNodeValue::unsafe_from_js_react_node(JsValue::from(*self)))
            }
        }
    )*);
    ($($n:ty)*) => ($(
        impl Node for $n {
            #[inline]
            fn as_react_node_js(&self) -> AnyNode {
                AnyNode::Value(AnyNodeValue::unsafe_from_js_react_node(JsValue::from(self)))
            }
        }
    )*);
}

into_js_node! {
    str
    react_sys::Element
    String
    js_sys::JsString
    js_sys::Number
    js_sys::BigInt
    js_sys::Boolean
}

into_js_node! {
    deref:
    // numbers https://docs.rs/wasm-bindgen/0.2.78/src/wasm_bindgen/lib.rs.html#849
    i8 u8 i16 u16 i32 u32 f32 f64
    // big_numbers https://docs.rs/wasm-bindgen/0.2.78/src/wasm_bindgen/lib.rs.html#869
    i64 u64 i128 u128 isize usize
    bool
}

#[inline]
fn into_keyed_element<E: IntoElement>(el: Keyed<E>) -> Keyed<crate::Element> {
    Keyed(el.0.into_element())
}

#[inline]
fn option_into_keyed_element<E: IntoElement>(
    el: Option<Keyed<E>>,
) -> Option<Keyed<crate::Element>> {
    el.map(into_keyed_element)
}

macro_rules! impl_node_for_iter {
    (impl into_react_children_js) => {
        #[inline]
        fn into_react_children_js(self) -> Option<Children> {
            Some(Children::from_single(self.into_react_node_js()))
        }
    };
    (impl sized) => {
        #[inline]
        fn into_react_node_js(self) -> AnyNode {
            AnyNode::Multiple(std::rc::Rc::new(self.into_iter().map(into_keyed_element).collect()))
        }

        impl_node_for_iter! { impl into_react_children_js }
    };
    (impl sized opt) => {
        #[inline]
        fn into_react_node_js(self) -> AnyNode {
            AnyNode::Multiple(std::rc::Rc::new(
                self.into_iter()
                    .filter_map(option_into_keyed_element)
                    .collect()
            ))
        }

        impl_node_for_iter! { impl into_react_children_js }

    };
    ( $($sized:ident)? {$($t:tt)+} {$($t_opt:tt)+}) => {
        $($t)+ {
            #[inline]
            fn as_react_node_js(&self) -> AnyNode {
                self.to_vec().into_react_node_js()
            }

            $( impl_node_for_iter! { impl $sized } )?
        }
        $($t_opt)+ {
            #[inline]
            fn as_react_node_js(&self) -> AnyNode {
                self.iter().filter_map(Clone::clone).collect::<Vec<_>>().into_react_node_js()
            }

            $( impl_node_for_iter! { impl $sized opt } )?
        }
    };
}

impl_node_for_iter! {
    sized
    { impl<E: IntoElement + Clone> Node for Vec<Keyed<E>> }
    { impl<E: IntoElement + Clone> Node for Vec<Option<Keyed<E>>> }
}
impl_node_for_iter! {
    { impl<E: IntoElement + Clone> Node for [Keyed<E>] }
    { impl<E: IntoElement + Clone> Node for [Option<Keyed<E>>] }
}
impl_node_for_iter! {
    sized
    { impl<E: IntoElement + Clone, const S: usize> Node for [Keyed<E>; S] }
    { impl<E: IntoElement + Clone, const S: usize> Node for [Option<Keyed<E>>; S] }
}

macro_rules! impl_node_for_tuple {
    (@impl ( $($t:ident),+ $(,)? )) => {
        impl<$($t: Node),+> Node for ($($t),+ ,) {
            #[inline]
            fn as_react_node_js(&self) -> AnyNode {
                #![allow(non_snake_case)]
                let ($($t),+ ,) = self;
                let v = Children::from_static_nodes([
                    $($t.as_react_node_js()),+
                ]);
                v.as_react_node_js()
            }
            #[inline]
            fn as_react_children_js(&self) -> Option<Children> {
                #![allow(non_snake_case)]
                let ($($t),+ ,) = self;
                let v = Children::from_static_nodes([
                    $($t.as_react_node_js()),+
                ]);
                Some(v)
            }
            #[inline]
            fn into_react_node_js(self) -> AnyNode {
                #![allow(non_snake_case)]
                let ($($t),+ ,) = self;
                let v = Children::from_static_nodes([
                    $($t.into_react_node_js()),+
                ]);
                v.into_react_node_js()
            }
            #[inline]
            fn into_react_children_js(self) -> Option<Children> {
                #![allow(non_snake_case)]
                let ($($t),+ ,) = self;
                Some(Children::from_static_nodes([
                    $($t.into_react_node_js()),+
                ]))
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
    (T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10)
    (T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11)
    (T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12)
    (T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13)
    (T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14)
    (T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,T15)
    (T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,T15,T16)
}

impl<N: Node> Node for Box<N> {
    #[inline]
    fn as_react_node_js(&self) -> AnyNode {
        self.as_ref().as_react_node_js()
    }
    #[inline]
    fn as_react_children_js(&self) -> Option<Children> {
        self.as_ref().as_react_children_js()
    }
    #[inline]
    fn into_react_node_js(self) -> AnyNode {
        (*self).into_react_node_js()
    }
    #[inline]
    fn into_react_children_js(self) -> Option<Children> {
        (*self).into_react_children_js()
    }
}

impl<N: ?Sized + Node> Node for &N {
    #[inline]
    fn as_react_node_js(&self) -> AnyNode {
        (*self).as_react_node_js()
    }

    #[inline]
    fn as_react_children_js(&self) -> Option<Children> {
        (*self).as_react_children_js()
    }
}
