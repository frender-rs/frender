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
    type ChildrenRenderState<R: crate::RenderHtml + ?Sized> = NonReactiveRenderState<<Children::MaybeStr as MaybeUpdateValueWithState<str>>::UpdateWithState>;

    fn children_render_update<R: crate::RenderHtml + ?Sized>(children: Children, element: &mut Self::Element<R>, renderer: &mut R, children_state: std::pin::Pin<&mut Self::ChildrenRenderState<R>>) {
        Self::children_unpinned_render_update(children, element, renderer, children_state.get_mut())
    }

    type ChildrenUnpinnedRenderState<R: crate::RenderHtml + ?Sized> = Self::ChildrenRenderState<R>;

    fn children_unpinned_render_update<R: crate::RenderHtml + ?Sized>(children: Children, element: &mut Self::Element<R>, renderer: &mut R, children_state: &mut Self::ChildrenUnpinnedRenderState<R>) {
        MaybeUpdateValueWithState::<str>::update_with_state(
            //
            Children::into_maybe_str(children),
            &mut children_state.0,
            super::utils::UpdateInnerText(element, renderer),
        )
    }
}
