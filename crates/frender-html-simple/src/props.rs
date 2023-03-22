use frender_core::UpdateRenderState;

use super::ElementPropsStates;

#[derive(Debug, Clone, Copy, Default)]
pub struct ElementProps<Children, Props> {
    pub children: Children,
    pub other_props: Props,
}

impl<Props> ElementProps<(), Props> {
    #[inline(always)]
    pub fn children<C>(self, children: C) -> ElementProps<C, Props> {
        ElementProps {
            children,
            other_props: self.other_props,
        }
    }
}

impl<Children, Props> ElementProps<Children, Props> {
    #[inline(always)]
    pub fn chain_prop<P>(self, prop: P) -> ElementProps<Children, (Props, P)> {
        ElementProps {
            children: self.children,
            other_props: (self.other_props, prop),
        }
    }
}

// pub type ElementPropsEmpty = ElementProps<(), EmptyProps>;

#[cfg(feature = "dom")]
mod dom {
    use super::*;
    use frender_dom::{
        props::{UpdateElement, UpdateElementNonReactive},
        Dom,
    };

    impl<E, Children, Props> UpdateElement<E> for ElementProps<Children, Props>
    where
        Children: UpdateRenderState<Dom>,
        Props: UpdateElementNonReactive<E>,
    {
        type State = ElementPropsStates<Children::State, Props::State>;

        fn initialize_state(this: Self, element: &E, children_ctx: &mut Dom) -> Self::State {
            ElementPropsStates {
                children: this.children.initialize_render_state(children_ctx),
                other_state: Props::initialize_state_non_reactive(
                    this.other_props,
                    element,
                    children_ctx,
                ),
            }
        }

        fn update_element(
            this: Self,
            element: &E,
            children_ctx: &mut Dom,
            state: std::pin::Pin<&mut Self::State>,
        ) {
            let (children_state, other_state) = state.pin_project();
            this.children
                .update_render_state(children_ctx, children_state);

            Props::update_element_non_reactive(
                this.other_props,
                element,
                children_ctx,
                other_state,
            );
        }
    }
}

#[cfg(feature = "ssr")]
mod ssr {
    use frender_core::UpdateRenderState;
    use frender_ssr::{attrs::IntoIteratorAttrs, AsyncWrite, IntoSsrData, SsrContext};

    impl<W: AsyncWrite + Unpin, Children, Props> IntoSsrData<W> for super::ElementProps<Children, Props>
    where
        Children: UpdateRenderState<SsrContext<W>>,
        Props: IntoIteratorAttrs<'static>,
    {
        type Children = Children;

        type ChildrenRenderState = Children::State;

        type Attrs = Props::IntoIterAttrs;

        fn into_ssr_data(this: Self) -> (Self::Children, Self::Attrs) {
            (this.children, Props::into_iter_attrs(this.other_props))
        }
    }
}
