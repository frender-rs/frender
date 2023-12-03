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
    use frender_dom::component::{IntoSpaceAndHtmlAttributesOrEmpty, SsrComponent};
    use frender_ssr::html::script::SsrElementScriptContent;

    impl<Attrs: IntoSpaceAndHtmlAttributesOrEmpty, Children: SsrElementScriptContent> SsrComponent<Attrs, Children> for crate::html::tags::script {
        type OneElement = frender_ssr::html::element::ScriptElement<<Attrs as IntoSpaceAndHtmlAttributesOrEmpty>::SpaceAndHtmlAttributesOrEmpty, Children::ScriptContent>;

        fn ssr_component(attrs: Attrs, children: Children) -> Self::OneElement {
            Self::OneElement::new(attrs.into_space_and_html_attributes_or_empty(), SsrElementScriptContent::into_script_content(children))
        }
    }
}
