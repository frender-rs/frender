use frender_core::{RenderContext, RenderState, UpdateRenderState};
use frender_html_common::IntrinsicComponent;

use crate::IntoElementProps;

#[cfg(feature = "dom")]
pub trait DomIntrinsicComponent {
    type Element: AsRef<frender_dom::web_sys::Element> + frender_dom::wasm_bindgen::JsCast;
}

pub trait IntrinsicComponentWithChildren<Ctx: for<'ctx> RenderContext<'ctx>, Children> {
    type ChildrenState: RenderState<Ctx>;

    fn initialize_children_state(
        self,
        children: Children,
        ctx: &mut <Ctx as RenderContext<'_>>::ContextData,
    ) -> Self::ChildrenState;
    fn update_children_state(
        self,
        children: Children,
        ctx: &mut <Ctx as RenderContext<'_>>::ContextData,
        children_state: std::pin::Pin<&mut Self::ChildrenState>,
    );
}

pub struct IntrinsicChildrenAsElement<C, Children> {
    pub component_type: C,
    pub children: Children,
}

impl<C, Children, Ctx: for<'ctx> RenderContext<'ctx>> UpdateRenderState<Ctx>
    for IntrinsicChildrenAsElement<C, Children>
where
    C: IntrinsicComponentWithChildren<Ctx, Children>,
{
    type State = C::ChildrenState;

    fn initialize_render_state(
        self,
        ctx: &mut <Ctx as RenderContext<'_>>::ContextData,
    ) -> Self::State {
        C::initialize_children_state(self.component_type, self.children, ctx)
    }

    fn update_render_state(
        self,
        ctx: &mut <Ctx as RenderContext<'_>>::ContextData,
        children_state: std::pin::Pin<&mut Self::State>,
    ) {
        C::update_children_state(self.component_type, self.children, ctx, children_state)
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

#[cfg(feature = "dom")]
mod dom {
    use frender_core::UpdateRenderState;
    use frender_dom::{props::UpdateElementNonReactive, Dom};
    use frender_html_common::IntrinsicComponent;

    use crate::{states::ElementPropsStates, ElementProps};

    use super::{DomIntrinsicComponent, IntoElementProps};

    impl<C: IntrinsicComponent, P: IntoElementProps> UpdateRenderState<Dom>
        for super::IntrinsicElement<C, P>
    where
        C: DomIntrinsicComponent,
        C: crate::IntrinsicComponentWithChildren<Dom, P::Children>,
        P::Attrs: UpdateElementNonReactive<C::Element>,
    {
        type State = ::frender_dom::element::intrinsic::IntrinsicComponentRenderState<
            C::Element,
            ElementPropsStates<
                C::ChildrenState,
                <P::Attrs as UpdateElementNonReactive<C::Element>>::State,
            >,
        >;

        fn initialize_render_state(self, ctx: &mut Dom) -> Self::State {
            let ElementProps {
                children,
                attributes,
            } = P::into_element_props(self.1);
            Self::State::initialize_with_tag(
                move |element, children_ctx| ElementPropsStates {
                    children: C::initialize_children_state(self.0, children, children_ctx),
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

        fn update_render_state(self, ctx: &mut Dom, state: std::pin::Pin<&mut Self::State>) {
            let (node_and_mounted, state) = state.pin_project();
            let (children_state, attrs_state) = state.pin_project();
            node_and_mounted.update(ctx, |element, children_ctx| {
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
                C::update_children_state(self.0, children, children_ctx, children_state);
            })
        }
    }
}

#[cfg(feature = "ssr")]
mod ssr {
    use std::borrow::Cow;

    use frender_core::UpdateRenderState;
    use frender_html_common::IntrinsicComponent;
    use frender_ssr::{attrs::IntoIteratorAttrs, AsyncWrite, SsrContext};

    use crate::{ElementProps, IntoElementProps, IntrinsicChildrenAsElement};

    use super::{IntrinsicComponentWithChildren, SsrIntrinsicComponent};

    fn ssr_html_element<C: IntrinsicComponent + SsrIntrinsicComponent, P: IntoElementProps>(
        element: super::IntrinsicElement<C, P>,
    ) -> ::frender_ssr::element::html::HtmlElement<
        'static,
        IntrinsicChildrenAsElement<C, P::Children>,
        P::Attrs,
    > {
        let ElementProps {
            children,
            attributes,
        } = P::into_element_props(element.1);
        let children = IntrinsicChildrenAsElement {
            component_type: element.0,
            children,
        };
        ::frender_ssr::element::html::HtmlElement {
            tag: Cow::Borrowed(C::INTRINSIC_TAG),
            attributes,
            children: C::wrap_children(children),
        }
    }

    impl<W: AsyncWrite + Unpin, C: IntrinsicComponent, P: IntoElementProps>
        UpdateRenderState<SsrContext<W>> for super::IntrinsicElement<C, P>
    where
        C: SsrIntrinsicComponent,
        C: IntrinsicComponentWithChildren<SsrContext<W>, P::Children>,
        P::Attrs: IntoIteratorAttrs<'static>,
    {
        type State = ::frender_ssr::element::html::HtmlElementRenderState<
            'static,
            C::ChildrenState,
            <P::Attrs as IntoIteratorAttrs<'static>>::IntoIterAttrs,
        >;

        fn initialize_render_state(self, ctx: &mut SsrContext<W>) -> Self::State {
            ssr_html_element(self).initialize_render_state(ctx)
        }

        fn update_render_state(
            self,
            ctx: &mut SsrContext<W>,
            state: std::pin::Pin<&mut Self::State>,
        ) {
            ssr_html_element(self).update_render_state(ctx, state)
        }
    }
}
