#[cfg(feature = "dom")]
mod dom {
    use frender_dom::wasm_bindgen::JsCast;
    use frender_html::props::IntrinsicComponent;
    use frender_html_simple::DomIntrinsicComponent;

    pub(super) fn expect_context_is_first_child_of<
        C: IntrinsicComponent + DomIntrinsicComponent,
    >(
        ctx: &frender_dom::Dom,
    ) -> &C::Element {
        if let frender_dom::NextNodePosition::FirstChildOf(element) = &ctx.next_node_position {
            if let Some(element) = element.dyn_ref() {
                element
            } else {
                let tag = C::INTRINSIC_TAG;
                panic!("current parent element should be <{tag}>")
            }
        } else {
            let tag = C::INTRINSIC_TAG;
            panic!("current csr context should be first child of <{tag}>");
        }
    }
}

mod script {
    #[cfg(feature = "dom")]
    mod dom {
        use frender_core::NonReactiveRenderState;
        use frender_html::props::MaybeUpdateValueWithState;
        use frender_html_simple::IntrinsicComponentWithChildren;

        use super::super::dom::expect_context_is_first_child_of;

        impl<S: MaybeUpdateValueWithState<str>> IntrinsicComponentWithChildren<frender_dom::Dom, S>
            for crate::html::simply_typed::script::ComponentType
        {
            type ChildrenState = NonReactiveRenderState<S::State>;

            fn initialize_children_state(
                self,
                children: S,
                ctx: &mut frender_dom::Dom,
            ) -> Self::ChildrenState {
                let element = expect_context_is_first_child_of::<Self>(ctx);
                NonReactiveRenderState(S::initialize_state_and_update(
                    children,
                    |text| element.set_text(text).unwrap(),
                    || element.set_text_content(None),
                ))
            }

            fn update_children_state(
                self,
                children: S,
                ctx: &mut frender_dom::Dom,
                children_state: std::pin::Pin<&mut Self::ChildrenState>,
            ) {
                let element = expect_context_is_first_child_of::<Self>(ctx);
                S::maybe_update_value_with_state(
                    children,
                    &mut children_state.get_mut().0,
                    |text| element.set_text(text).unwrap(),
                    || element.set_text_content(None),
                )
            }
        }
    }

    #[cfg(feature = "ssr")]
    mod ssr {
        use std::borrow::Cow;

        use frender_core::UpdateRenderState;
        use frender_html::props::MaybeUpdateValueWithState;
        use frender_html_simple::IntrinsicComponentWithChildren;
        use frender_ssr::{element::str::EscapeStr, AsyncWrite, SsrContext};

        fn maybe_str<S: MaybeUpdateValueWithState<str>>(s: S) -> Option<Cow<'static, str>> {
            S::maybe_into_html_attribute_value(s).map(Option::unwrap)
        }

        impl<W: AsyncWrite + Unpin, S: MaybeUpdateValueWithState<str>>
            IntrinsicComponentWithChildren<SsrContext<W>, S>
            for crate::html::simply_typed::script::ComponentType
        {
            type ChildrenState = Option<
                frender_ssr::element::bytes::State<frender_ssr::bytes::CowSlicedBytes<'static>>,
            >;

            fn initialize_children_state(
                self,
                children: S,
                ctx: &mut SsrContext<W>,
            ) -> Self::ChildrenState {
                maybe_str(children).map(|children| {
                    EscapeStr(children, frender_ssr::html_escape::encode_script)
                        .initialize_render_state(ctx)
                })
            }

            fn update_children_state(
                self,
                children: S,
                ctx: &mut SsrContext<W>,
                children_state: std::pin::Pin<&mut Self::ChildrenState>,
            ) {
                maybe_str(children)
                    .map(|children| EscapeStr(children, frender_ssr::html_escape::encode_script))
                    .update_render_state(ctx, children_state)
            }
        }
    }
}
