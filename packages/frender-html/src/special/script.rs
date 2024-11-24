use frender_attr_value::csr::CsrAttrValue;
use frender_dom::{
    component::{IntoSpaceAndHtmlAttributesOrEmpty, SsrComponent},
    script::IntoScriptContent,
};

use crate::{element_types::RenderStateWithPehKind, elements::non_reactive::NonReactiveRenderState, html::components::script, kinds::KindOfNonReactive, CsrComponent};

impl<Children: IntoScriptContent> SsrComponent<Children> for script::Marker {
    type OneElement<Attrs: IntoSpaceAndHtmlAttributesOrEmpty> = frender_ssr::html::element::ScriptElement<<Attrs as IntoSpaceAndHtmlAttributesOrEmpty>::SpaceAndHtmlAttributesOrEmpty, Children::IntoScriptContent>;

    fn ssr_component<Attrs: IntoSpaceAndHtmlAttributesOrEmpty>(self, attrs: Attrs, children: Children) -> Self::OneElement<Attrs> {
        Self::OneElement::<Attrs>::new(attrs.into_space_and_html_attributes_or_empty(), IntoScriptContent::into_script_content(children))
    }
}

impl<Children: IntoScriptContent> CsrComponent<Children> for script::Marker {
    type ChildrenRenderStateKind = KindOfNonReactive<Option<<Children::IntoScriptInnerText as CsrAttrValue<str>>::State>>;

    fn children_render_update<R: crate::RenderHtml + ?Sized>(
        self,
        children: Children,
        element: &mut Self::Element<R>,
        renderer: &mut R,
        children_state: std::pin::Pin<&mut <Self::ChildrenRenderStateKind as RenderStateWithPehKind<Self>>::RenderStateWithPeh<R>>,
    ) {
        self.children_unpinned_render_update(children, element, renderer, children_state.get_mut())
    }

    fn children_unpinned_render_update<R: crate::RenderHtml + ?Sized>(
        self,
        children: Children,
        element: &mut Self::Element<R>,
        renderer: &mut R,
        NonReactiveRenderState(state): &mut <Self::ChildrenRenderStateKind as RenderStateWithPehKind<Self>>::RenderStateWithPehUnpinned<R>,
    ) {
        let children = Children::into_script_inner_text(children);
        let updater = super::utils::UpdateInnerText(element, renderer);
        if let Some(state) = state {
            <_>::update_attribute_value_with_state(children, updater, state)
        } else {
            *state = Some(<_>::update_absent_attribute_value_into_state(children, updater))
        }
    }
}
