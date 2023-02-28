#[macro_export]
macro_rules! def_props {
    (
        $($name:ident),* $(,)?
    ) => {$(
        #[derive(Debug, Clone, Copy)]
        pub struct $name<V>(pub V);
        impl<V> ::core::marker::Unpin for $name<V> {}
    )*};
}

#[macro_export]
macro_rules! inherit_props_from {
    (
        $($p:tt)+
    ) => {
        mod __inherited_props {
            use super::super::super::*;

            pub use $($p)+::props;
        }

        pub use __inherited_props::props::*;
    };
}

#[macro_export]
macro_rules! impl_dom {
    () => {
        mod impl_update_element {
            #[allow(unused_imports)]
            use super::super::*;
            impl<E, Children, Props> $crate::frender_dom::props::UpdateElement<E>
                for super::Data<Children, Props>
            where
                $crate::ElementProps<Children, Props>: $crate::frender_dom::props::UpdateElement<E>,
            {
                type State = <$crate::ElementProps<Children, Props> as $crate::frender_dom::props::UpdateElement<E>>::State;

                #[inline(always)]
                fn initialize_state(
                    this: Self,
                    element: &E,
                    children_ctx: &mut ::frender_dom::Dom,
                ) -> Self::State {
                    $crate::ElementProps::<Children, Props>::initialize_state(
                        this.props,
                        element,
                        children_ctx,
                    )
                }

                #[inline(always)]
                fn update_element(
                    this: Self,
                    element: &E,
                    children_ctx: &mut ::frender_dom::Dom,
                    state: ::core::pin::Pin<&mut Self::State>,
                ) {
                    $crate::ElementProps::<Children, Props>::update_element(
                        this.props,
                        element,
                        children_ctx,
                        state,
                    )
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_ssr {
    () => {
        mod impl_ssr {
            #[allow(unused_imports)]
            use super::super::*;
            impl<W: ::frender_ssr::AsyncWrite + ::core::marker::Unpin, Children, Props>
                $crate::frender_ssr::IntoSsrData<W> for super::Data<Children, Props>
            where
                $crate::ElementProps<Children, Props>: $crate::frender_ssr::IntoSsrData<W>,
            {
                type Children =
                    <$crate::ElementProps<Children, Props> as $crate::frender_ssr::IntoSsrData<
                        W,
                    >>::Children;

                type ChildrenRenderState =
                    <$crate::ElementProps<Children, Props> as $crate::frender_ssr::IntoSsrData<
                        W,
                    >>::ChildrenRenderState;

                type Attrs =
                    <$crate::ElementProps<Children, Props> as $crate::frender_ssr::IntoSsrData<
                        W,
                    >>::Attrs;

                fn into_ssr_data(this: Self) -> (Self::Children, Self::Attrs) {
                    $crate::frender_ssr::IntoSsrData::<W>::into_ssr_data(this.props)
                }
            }
        }
    };
}
