#[cfg(feature = "dom")]
pub trait IntrinsicComponentWithElementType {
    type Element;
}

#[cfg(feature = "ssr")]
pub trait IntrinsicComponentSupportChildren<Children> {
    #[inline]
    fn wrap_children(
        children: Children,
    ) -> frender_ssr::element::html::HtmlElementChildren<Children> {
        frender_ssr::element::html::HtmlElementChildren::Children(children)
    }
}

pub struct IntrinsicComponent<C: frender_html_common::IntrinsicComponent, P>(pub C, pub P);

#[cfg(feature = "dom")]
mod dom {
    use frender_core::UpdateRenderState;
    use frender_dom::{props::UpdateElement, Dom};
    use frender_html_common::IntrinsicComponent;

    use super::IntrinsicComponentWithElementType;

    impl<
            C: IntrinsicComponent + IntrinsicComponentWithElementType,
            P: UpdateElement<C::Element>,
        > UpdateRenderState<Dom> for super::IntrinsicComponent<C, P>
    where
        C::Element: AsRef<frender_dom::web_sys::Element> + frender_dom::wasm_bindgen::JsCast,
    {
        type State =
            ::frender_dom::element::intrinsic::IntrinsicComponentRenderState<C::Element, P::State>;

        fn initialize_render_state(self, ctx: &mut Dom) -> Self::State {
            Self::State::initialize_with_tag(self.1, ctx, C::INTRINSIC_TAG)
        }

        fn update_render_state(self, ctx: &mut Dom, state: std::pin::Pin<&mut Self::State>) {
            let (node_and_mounted, state) = state.pin_project();
            node_and_mounted.update(ctx, |element, children_ctx| {
                P::update_element(self.1, element, children_ctx, state)
            })
        }
    }
}

#[cfg(feature = "ssr")]
mod ssr {
    use std::borrow::Cow;

    use frender_core::UpdateRenderState;
    use frender_html_common::IntrinsicComponent;
    use frender_ssr::{AsyncWrite, IntoSsrData, SsrContext};

    use super::IntrinsicComponentSupportChildren;

    impl<W: AsyncWrite + Unpin, C: IntrinsicComponent, P> UpdateRenderState<SsrContext<W>>
        for super::IntrinsicComponent<C, P>
    where
        C: IntrinsicComponentSupportChildren<P::Children>,
        P: IntoSsrData<W>,
    {
        type State = ::frender_ssr::element::html::HtmlElementRenderState<
            'static,
            P::ChildrenRenderState,
            P::Attrs,
            W,
        >;

        fn initialize_render_state(self, ctx: &mut SsrContext<W>) -> Self::State {
            let (children, attributes) = IntoSsrData::into_ssr_data(self.1);
            ::frender_ssr::element::html::HtmlElement {
                tag: Cow::Borrowed(C::INTRINSIC_TAG),
                attributes,
                children: C::wrap_children(children),
            }
            .initialize_render_state(ctx)
        }

        fn update_render_state(
            self,
            ctx: &mut SsrContext<W>,
            state: std::pin::Pin<&mut Self::State>,
        ) {
            let (children, attributes) = IntoSsrData::into_ssr_data(self.1);
            ::frender_ssr::element::html::HtmlElement {
                tag: Cow::Borrowed(C::INTRINSIC_TAG),
                attributes,
                children: C::wrap_children(children),
            }
            .update_render_state(ctx, state)
        }
    }
}
