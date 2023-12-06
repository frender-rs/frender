pub use frender_common::expand;
pub use frender_dom as dom;
pub use frender_html_common::{dom_token::DomTokenList, DomTokens};

pub use create_node::CreateNode;
pub use html::RenderHtml;
pub use update_element::UpdateElementNonReactive;

pub use element::{Element, RenderState};
pub use element_types::{CsrComponent, CsrComponentNormalElement};

pub mod html;
pub mod impl_bounds;

mod create_node;
mod update_element;

mod element;

mod element_types;

pub mod elements;

mod macros;
#[cfg(feature = "web")]
mod shims;

// TODO(refactor)
pub mod __private {
    pub use crate::RenderHtml;
}

mod special {
    use frender_dom::{
        component::{IntoSpaceAndHtmlAttributesOrEmpty, SsrComponent},
        script::SsrElementScriptContent,
    };
    use frender_html_common::MaybeUpdateValueWithState;

    use crate::{elements::non_reactive::NonReactiveRenderState, CsrComponent};

    impl<Attrs: IntoSpaceAndHtmlAttributesOrEmpty, Children: SsrElementScriptContent> SsrComponent<Attrs, Children> for crate::html::tags::script {
        type OneElement = frender_ssr::html::element::ScriptElement<<Attrs as IntoSpaceAndHtmlAttributesOrEmpty>::SpaceAndHtmlAttributesOrEmpty, Children::ScriptContent>;

        fn ssr_component(attrs: Attrs, children: Children) -> Self::OneElement {
            Self::OneElement::new(attrs.into_space_and_html_attributes_or_empty(), SsrElementScriptContent::into_script_content(children))
        }
    }

    impl<Children: SsrElementScriptContent> CsrComponent<Children> for crate::html::tags::script {
        type ChildrenRenderState<R: crate::RenderHtml> = NonReactiveRenderState<<Children::MaybeStr as MaybeUpdateValueWithState<str>>::UpdateWithState>;

        fn children_render_update<R: crate::RenderHtml>(children: Children, element: &mut Self::Element<R>, renderer: &mut R, children_state: std::pin::Pin<&mut Self::ChildrenRenderState<R>>) {
            Self::children_unpinned_render_update(children, element, renderer, children_state.get_mut())
        }

        type ChildrenUnpinnedRenderState<R: crate::RenderHtml> = Self::ChildrenRenderState<R>;

        fn children_unpinned_render_update<R: crate::RenderHtml>(children: Children, element: &mut Self::Element<R>, renderer: &mut R, children_state: &mut Self::ChildrenUnpinnedRenderState<R>) {
            struct UpdateInnerText<'a, E, R>(&'a mut E, &'a mut R);

            impl<'a, E: crate::html::behaviors::HtmlElement<R>, R> frender_html_common::ValueUpdater<str> for UpdateInnerText<'a, E, R> {
                fn update(self, value: &str) {
                    self.0.set_inner_text(self.1, value)
                }

                fn remove(self) {
                    self.0.set_inner_text(self.1, "")
                }
            }

            MaybeUpdateValueWithState::<str>::update_with_state(Children::into_maybe_str(children), &mut children_state.0, UpdateInnerText(element, renderer))
        }
    }
}
