#[cfg(feature = "csr")]
mod dom {
    use frender_csr::wasm_bindgen::JsCast;
    use frender_html::props::IntrinsicComponent;
    use frender_html_simple::DomIntrinsicComponent;

    pub(super) fn expect_context_is_first_child_of<
        'a,
        C: IntrinsicComponent + DomIntrinsicComponent,
    >(
        ctx: &'a frender_csr::CsrContext,
    ) -> &'a C::Element {
        if let frender_csr::NextNodePosition::FirstChildOf(element) = &ctx.next_node_position {
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
    #[cfg(feature = "csr")]
    mod dom {
        use frender_csr::element::non_reactive::NonReactiveRenderState;
        use frender_html::props::MaybeUpdateValueWithState;
        use frender_html_simple::CsrWithChildren;

        use super::super::dom::expect_context_is_first_child_of;

        impl<S: MaybeUpdateValueWithState<str>> CsrWithChildren<S>
            for crate::html::simply_typed::script::ComponentType
        {
            type ChildrenState = NonReactiveRenderState<S::State>;

            fn children_into_csr_state(
                self,
                children: S,
                ctx: &mut frender_csr::CsrContext,
            ) -> Self::ChildrenState {
                let element = expect_context_is_first_child_of::<Self>(ctx);
                NonReactiveRenderState(S::initialize_state_and_update(
                    children,
                    |text| element.set_text(text).unwrap(),
                    || element.set_text_content(None),
                ))
            }

            fn children_update_csr_state(
                self,
                children: S,
                ctx: &mut frender_csr::CsrContext,
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

        use frender_html::props::MaybeUpdateValueWithState;
        use frender_html_simple::SsrWithChildren;
        use frender_ssr::{
            element::str::{EscapeStr, EscapedStrWritingState},
            AsyncWrite, Element,
        };

        #[derive(Clone, Copy)]
        pub struct EncodeScript;

        impl frender_ssr::EscapeSafe for EncodeScript {
            fn escape_safe<'a>(&mut self, input: &'a str) -> Cow<'a, str> {
                frender_ssr::html_escape::encode_script(input)
            }
        }

        impl<S: MaybeUpdateValueWithState<str>> SsrWithChildren<S>
            for crate::html::simply_typed::script::ComponentType
        {
            type ChildrenSsrState = Option<EscapedStrWritingState<S::ChildStr, EncodeScript>>;

            fn into_children_ssr_state(self, children: S) -> Self::ChildrenSsrState {
                S::maybe_into_child_str(children, ())
                    .map(|children| EscapedStrWritingState(children, EncodeScript))
            }
        }
    }
}
