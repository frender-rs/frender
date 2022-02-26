use wasm_bindgen::JsValue;

use crate::{AnyNode, AnyNodeValue, Children, IntoElement, Keyed};

/// Corresponding to `ReactNode` in typescript
///
/// ```typescript
/// type ReactNode = ReactElement | string | number | ReactNodeArray | boolean | null | undefined;
/// ```
pub trait Node {
    fn to_node(&self) -> AnyNode;

    #[inline]
    fn to_children(&self) -> Option<Children> {
        Some(Children::from_single(self.to_node()))
    }

    #[inline]
    fn into_node(self) -> AnyNode
    where
        Self: Sized,
    {
        self.to_node()
    }

    #[inline]
    fn into_children(self) -> Option<Children>
    where
        Self: Sized,
    {
        self.to_children()
    }
}

impl Node for () {
    #[inline]
    fn to_node(&self) -> AnyNode {
        AnyNode::Null
    }

    #[inline]
    fn to_children(&self) -> Option<Children> {
        None
    }
}

impl<T: Node> Node for Option<T> {
    #[inline]
    fn to_node(&self) -> AnyNode {
        if let Some(node) = self {
            node.to_node()
        } else {
            AnyNode::Null
        }
    }

    #[inline]
    fn to_children(&self) -> Option<Children> {
        self.as_ref().and_then(Node::to_children)
    }

    #[inline]
    fn into_node(self) -> AnyNode {
        self.map(Node::into_node).unwrap_or(AnyNode::Null)
    }

    #[inline]
    fn into_children(self) -> Option<Children> {
        self.and_then(Node::into_children)
    }
}

macro_rules! into_js_node {
    (deref: $($n:ty)*) => ($(
        impl Node for $n {
            #[inline]
            fn to_node(&self) -> AnyNode {
                AnyNode::Value(AnyNodeValue::unsafe_from_js_react_node(JsValue::from(*self)))
            }
        }
    )*);
    ($($n:ty)*) => ($(
        impl Node for $n {
            #[inline]
            fn to_node(&self) -> AnyNode {
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
    (impl into_children) => {
        #[inline]
        fn into_children(self) -> Option<Children> {
            Some(Children::from_single(self.into_node()))
        }
    };
    (impl sized) => {
        #[inline]
        fn into_node(self) -> AnyNode {
            AnyNode::Multiple(std::rc::Rc::new(self.into_iter().map(into_keyed_element).collect()))
        }

        impl_node_for_iter! { impl into_children }
    };
    (impl sized opt) => {
        #[inline]
        fn into_node(self) -> AnyNode {
            AnyNode::Multiple(std::rc::Rc::new(
                self.into_iter()
                    .filter_map(option_into_keyed_element)
                    .collect()
            ))
        }

        impl_node_for_iter! { impl into_children }

    };
    ( $($sized:ident)? {$($t:tt)+} {$($t_opt:tt)+}) => {
        $($t)+ {
            #[inline]
            fn to_node(&self) -> AnyNode {
                self.to_vec().into_node()
            }

            $( impl_node_for_iter! { impl $sized } )?
        }
        $($t_opt)+ {
            #[inline]
            fn to_node(&self) -> AnyNode {
                self.iter().filter_map(Clone::clone).collect::<Vec<_>>().into_node()
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
            fn to_node(&self) -> AnyNode {
                #![allow(non_snake_case)]
                let ($($t),+ ,) = self;
                let v = Children::from_static_nodes([
                    $($t.to_node()),+
                ]);
                v.to_node()
            }
            #[inline]
            fn to_children(&self) -> Option<Children> {
                #![allow(non_snake_case)]
                let ($($t),+ ,) = self;
                let v = Children::from_static_nodes([
                    $($t.to_node()),+
                ]);
                Some(v)
            }
            #[inline]
            fn into_node(self) -> AnyNode {
                #![allow(non_snake_case)]
                let ($($t),+ ,) = self;
                let v = Children::from_static_nodes([
                    $($t.into_node()),+
                ]);
                v.into_node()
            }
            #[inline]
            fn into_children(self) -> Option<Children> {
                #![allow(non_snake_case)]
                let ($($t),+ ,) = self;
                Some(Children::from_static_nodes([
                    $($t.into_node()),+
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
    fn to_node(&self) -> AnyNode {
        self.as_ref().to_node()
    }
    #[inline]
    fn to_children(&self) -> Option<Children> {
        self.as_ref().to_children()
    }
    #[inline]
    fn into_node(self) -> AnyNode {
        (*self).into_node()
    }
    #[inline]
    fn into_children(self) -> Option<Children> {
        (*self).into_children()
    }
}

impl<N: ?Sized + Node> Node for &N {
    #[inline]
    fn to_node(&self) -> AnyNode {
        (*self).to_node()
    }

    #[inline]
    fn to_children(&self) -> Option<Children> {
        (*self).to_children()
    }
}
