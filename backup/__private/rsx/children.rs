use super::ReceiveNode;

pub struct NoChildrenValue;

pub struct PropsBuilderWillAcceptNoChildren<P>(P);

impl<P> PropsBuilderWillAcceptNoChildren<P> {
    pub fn children(self, c: NoChildrenValue) -> P {
        self.0
    }
}

pub trait UnwrapChildrenValue<TBuilder> {
    type ChildrenValue;
    type PropsBuilderWrapper;
    fn unwrap_children_value(self) -> Self::ChildrenValue;
    fn wrap_props_builder(props_builder: TBuilder) -> Self::PropsBuilderWrapper;
}

pub struct RsxChildren<T>(pub T);

impl<V> ReceiveNode<V> for RsxChildren<()> {
    type Output = RsxChildren<(V,)>;

    fn receive_node(self, value: V) -> Self::Output {
        RsxChildren((value,))
    }
}

impl<TBuilder> UnwrapChildrenValue<TBuilder> for RsxChildren<()> {
    type ChildrenValue = NoChildrenValue;
    type PropsBuilderWrapper = PropsBuilderWillAcceptNoChildren<TBuilder>;

    #[inline]
    fn unwrap_children_value(self) -> Self::ChildrenValue {
        NoChildrenValue
    }

    #[inline]
    fn wrap_props_builder(props_builder: TBuilder) -> Self::PropsBuilderWrapper {
        PropsBuilderWillAcceptNoChildren(props_builder)
    }
}

macro_rules! impl_for_children {
    (@impl unwrap_children_value ( $t:ident $(,)? ) )=>{
        impl<TBuilder, $t> UnwrapChildrenValue<TBuilder> for RsxChildren<($t ,)> {
            type ChildrenValue = $t;
            type PropsBuilderWrapper = TBuilder;

            #[inline]
            fn unwrap_children_value(self) -> Self::ChildrenValue {
                self.0.0
            }

            #[inline]
            fn wrap_props_builder(props_builder: TBuilder) -> Self::PropsBuilderWrapper {
                props_builder
            }
        }
    };
    (@impl unwrap_children_value ( $($t:ident),+ $(,)? ) )=>{
        impl<TBuilder, $($t),+> UnwrapChildrenValue<TBuilder> for RsxChildren<($($t),+ ,)> {
            type ChildrenValue = ($($t),+);
            type PropsBuilderWrapper = TBuilder;

            #[inline]
            fn unwrap_children_value(self) -> Self::ChildrenValue {
                self.0
            }

            #[inline]
            fn wrap_props_builder(props_builder: TBuilder) -> Self::PropsBuilderWrapper {
                props_builder
            }
        }
    };
    (impl ( $($t:ident),+ $(,)? )) => {
        impl<V, $($t),+> ReceiveNode<V> for RsxChildren<($($t),+ ,)> {
            type Output = RsxChildren<($($t),+ , V)>;

            #[inline]
            fn receive_node(self, other: V) -> Self::Output {
                #![allow(non_snake_case)]
                let ($($t),+ ,) = self.0;
                RsxChildren(($($t),+ , other))
            }
        }

        impl_for_children! { @impl unwrap_children_value ( $($t),+ ) }
    };
    ( $(( $($t:ident),+ $(,)? ))* ) => {
        $(
            impl_for_children! { impl ($($t),+ ,) }
        )*
    };
}

impl_for_children! {
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
    // (T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,T15,T16)
}
