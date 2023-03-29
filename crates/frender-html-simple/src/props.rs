use super::ElementPropsStates;

pub trait IntoElementProps {
    type Children;
    type Attrs;

    fn into_element_props(this: Self) -> ElementProps<Self::Children, Self::Attrs>;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ElementProps<Children, Attrs> {
    pub children: Children,
    pub attributes: Attrs,
}

impl<Attrs> ElementProps<(), Attrs> {
    #[inline(always)]
    pub fn children<C>(self, children: C) -> ElementProps<C, Attrs> {
        ElementProps {
            children,
            attributes: self.attributes,
        }
    }
}

impl<Children, Attrs> ElementProps<Children, Attrs> {
    #[inline(always)]
    pub fn chain_prop<P>(self, prop: P) -> ElementProps<Children, (Attrs, P)> {
        ElementProps {
            children: self.children,
            attributes: (self.attributes, prop),
        }
    }
}

// pub type ElementPropsEmpty = ElementProps<(), EmptyProps>;

// TODO: remove
#[cfg(feature = "csr")]
mod dom {
    use super::*;
    use frender_csr::Element;
    use frender_csr::{
        props::{UpdateElement, UpdateElementNonReactive},
        CsrContext,
    };

    impl<E, Children, Attrs> UpdateElement<E> for ElementProps<Children, Attrs>
    where
        Children: Element,
        Attrs: UpdateElementNonReactive<E>,
    {
        type State = ElementPropsStates<Children::CsrState, Attrs::State>;

        fn initialize_state(this: Self, element: &E, children_ctx: &mut CsrContext) -> Self::State {
            ElementPropsStates {
                children: this.children.into_csr_state(children_ctx),
                other_state: Attrs::initialize_state_non_reactive(
                    this.attributes,
                    element,
                    children_ctx,
                ),
            }
        }

        fn update_element(
            this: Self,
            element: &E,
            children_ctx: &mut CsrContext,
            state: std::pin::Pin<&mut Self::State>,
        ) {
            let (children_state, other_state) = state.pin_project();
            this.children
                .update_csr_state(children_ctx, children_state);

            Attrs::update_element_non_reactive(this.attributes, element, children_ctx, other_state);
        }
    }
}
