use frender_dom::{
    component::{IntoSpaceAndHtmlAttributesOrEmpty, SsrComponent},
    script::IntoScriptContent,
};

use crate::html::components::script;

impl<Children: IntoScriptContent> SsrComponent<Children> for script::Marker {
    type OneElement<Attrs: IntoSpaceAndHtmlAttributesOrEmpty> = frender_ssr::html::element::ScriptElement<<Attrs as IntoSpaceAndHtmlAttributesOrEmpty>::SpaceAndHtmlAttributesOrEmpty, Children::IntoScriptContent>;

    fn ssr_component<Attrs: IntoSpaceAndHtmlAttributesOrEmpty>(self, attrs: Attrs, children: Children) -> Self::OneElement<Attrs> {
        Self::OneElement::<Attrs>::new(attrs.into_space_and_html_attributes_or_empty(), IntoScriptContent::into_script_content(children))
    }
}
