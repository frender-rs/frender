use frender_html_common::IntrinsicComponent;

use crate::IntoElementProps;

#[cfg(feature = "csr")]
pub trait DomIntrinsicComponent {
    type Element: AsRef<frender_csr::web_sys::Element> + frender_csr::wasm_bindgen::JsCast;
}

#[cfg(feature = "csr")]
pub trait CsrWithChildren<Children> {
    type ChildrenState: frender_csr::RenderState;

    fn children_into_csr_state(
        self,
        children: Children,
        ctx: &mut frender_csr::CsrContext,
    ) -> Self::ChildrenState;
    fn children_update_csr_state(
        self,
        children: Children,
        ctx: &mut frender_csr::CsrContext,
        children_state: std::pin::Pin<&mut Self::ChildrenState>,
    );
}

#[cfg(feature = "ssr")]
pub trait SsrWithChildren<Children> {
    type ChildrenSsrState: frender_ssr::RenderState;

    fn into_children_ssr_state(self, children: Children) -> Self::ChildrenSsrState;
}

pub struct IntrinsicChildrenAsElement<C, Children> {
    pub component_type: C,
    pub children: Children,
}

#[cfg(feature = "ssr")]
impl<C, Children> frender_ssr::Element for IntrinsicChildrenAsElement<C, Children>
where
    C: SsrWithChildren<Children>,
{
    type SsrState = C::ChildrenSsrState;

    fn into_ssr_state(self) -> Self::SsrState {
        self.component_type.into_children_ssr_state(self.children)
    }
}

#[cfg(feature = "ssr")]
pub trait SsrIntrinsicComponent {
    #[inline]
    fn wrap_children<Children>(
        children: Children,
    ) -> frender_ssr::element::html::HtmlElementChildren<Children> {
        frender_ssr::element::html::HtmlElementChildren::Children(children)
    }
}

pub struct IntrinsicElement<C: IntrinsicComponent, P: IntoElementProps>(pub C, pub P);

#[cfg(feature = "csr")]
mod dom {
    use frender_csr::Element;
    use frender_csr::{props::UpdateElementNonReactive, CsrContext};
    use frender_html_common::IntrinsicComponent;

    use crate::{states::ElementPropsStates, ElementProps};

    use super::{DomIntrinsicComponent, IntoElementProps};

    impl<C: IntrinsicComponent, P: IntoElementProps> Element for super::IntrinsicElement<C, P>
    where
        C: DomIntrinsicComponent,
        C: crate::CsrWithChildren<P::Children>,
        P::Attrs: UpdateElementNonReactive<C::Element>,
    {
        type CsrState = ::frender_csr::element::intrinsic::IntrinsicComponentRenderState<
            C::Element,
            ElementPropsStates<
                C::ChildrenState,
                <P::Attrs as UpdateElementNonReactive<C::Element>>::State,
            >,
        >;

        fn into_csr_state(self, ctx: &mut CsrContext) -> Self::CsrState {
            let ElementProps {
                children,
                attributes,
            } = P::into_element_props(self.1);
            Self::CsrState::initialize_with_tag(
                move |element, children_ctx| ElementPropsStates {
                    children: C::children_into_csr_state(self.0, children, children_ctx),
                    other_state:
                        UpdateElementNonReactive::<C::Element>::initialize_state_non_reactive(
                            attributes,
                            element,
                            children_ctx,
                        ),
                },
                ctx,
                C::INTRINSIC_TAG,
            )
        }

        fn update_csr_state_maybe_reposition(
            self,
            ctx: &mut frender_csr::CsrContext,
            state: std::pin::Pin<&mut Self::CsrState>,
            force_reposition: bool,
        ) where
            Self: Sized,
        {
            #[cfg(debug_assertions)]
            frender_csr::web_sys::console::log_2(
                &"IntrinsicElement force_reposition".into(),
                &force_reposition.into(),
            );
            let (node_and_mounted, state) = state.pin_project();
            let (children_state, attrs_state) = state.pin_project();
            node_and_mounted.update_maybe_reposition(
                ctx,
                |element, children_ctx| {
                    let ElementProps {
                        children,
                        attributes,
                    } = P::into_element_props(self.1);
                    UpdateElementNonReactive::<C::Element>::update_element_non_reactive(
                        attributes,
                        element,
                        children_ctx,
                        attrs_state,
                    );
                    C::children_update_csr_state(self.0, children, children_ctx, children_state);
                },
                force_reposition,
            )
        }
    }
}

#[cfg(feature = "ssr")]
mod ssr {
    use std::borrow::Cow;

    use frender_common::write::attrs::IntoAsyncWritableAttrs;
    use frender_html_common::IntrinsicComponent;
    use frender_ssr::{AsyncWrite, Element};

    use crate::{ElementProps, IntoElementProps, IntrinsicChildrenAsElement, SsrWithChildren};

    use super::{CsrWithChildren, SsrIntrinsicComponent};

    impl<C: IntrinsicComponent, P: IntoElementProps> Element for super::IntrinsicElement<C, P>
    where
        C: SsrIntrinsicComponent,
        C: SsrWithChildren<P::Children>,
        P::Attrs: IntoAsyncWritableAttrs,
    {
        type SsrState = ::frender_ssr::element::html::HtmlElementRenderState<
            'static,
            C::ChildrenSsrState,
            <P::Attrs as IntoAsyncWritableAttrs>::AsyncWritableAttrs,
        >;

        fn into_ssr_state(self) -> Self::SsrState {
            let ElementProps {
                children,
                attributes,
            } = P::into_element_props(self.1);
            let children = IntrinsicChildrenAsElement {
                component_type: self.0,
                children,
            };
            ::frender_ssr::element::html::HtmlElement {
                tag: Cow::Borrowed(C::INTRINSIC_TAG),
                attributes,
                children: C::wrap_children(children),
            }
            .into_ssr_state()
        }
    }
}
