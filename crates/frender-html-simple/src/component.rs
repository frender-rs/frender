pub trait IntrinsicComponentWithElementType {
    type Element;
}

pub struct IntrinsicComponent<
    C: frender_html_common::IntrinsicComponent + IntrinsicComponentWithElementType,
    P,
>(pub C, pub P);

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
