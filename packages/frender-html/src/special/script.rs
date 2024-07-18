use frender_dom::{
    component::{IntoSpaceAndHtmlAttributesOrEmpty, SsrComponent},
    script::IntoScriptContent,
};
use frender_html_common::MaybeValue;

use crate::{element_types::RenderStateWithPehKind, elements::non_reactive::NonReactiveRenderState, CsrComponent};

impl<Attrs: IntoSpaceAndHtmlAttributesOrEmpty, Children: IntoScriptContent> SsrComponent<Attrs, Children> for crate::html::tags::script {
    type OneElement = frender_ssr::html::element::ScriptElement<<Attrs as IntoSpaceAndHtmlAttributesOrEmpty>::SpaceAndHtmlAttributesOrEmpty, Children::IntoScriptContent>;

    fn ssr_component(attrs: Attrs, children: Children) -> Self::OneElement {
        Self::OneElement::new(attrs.into_space_and_html_attributes_or_empty(), IntoScriptContent::into_script_content(children))
    }
}

enum Never {}
pub struct Kind<Cache: Default>(Never, std::marker::PhantomData<Cache>);

impl<Cache: Default> RenderStateWithPehKind<crate::html::tags::script> for Kind<Cache> {
    type RenderStateWithPeh<R: crate::RenderHtml + ?Sized> = NonReactiveRenderState<Cache>;
    type RenderStateWithPehUnpinned<R: crate::RenderHtml + ?Sized> = NonReactiveRenderState<Cache>;
}

impl<Children: IntoScriptContent> CsrComponent<Children> for crate::html::tags::script {
    type ChildrenRenderStateKind = Kind<<Children::IntoScriptInnerText as MaybeValue<str>>::UpdateWithState>;

    fn children_render_update<R: crate::RenderHtml + ?Sized>(
        children: Children,
        element: &mut Self::Element<R>,
        renderer: &mut R,
        children_state: std::pin::Pin<&mut <Self::ChildrenRenderStateKind as RenderStateWithPehKind<Self>>::RenderStateWithPeh<R>>,
    ) {
        Self::children_unpinned_render_update(children, element, renderer, children_state.get_mut())
    }

    fn children_unpinned_render_update<R: crate::RenderHtml + ?Sized>(
        children: Children,
        element: &mut Self::Element<R>,
        renderer: &mut R,
        children_state: &mut <Self::ChildrenRenderStateKind as RenderStateWithPehKind<Self>>::RenderStateWithPehUnpinned<R>,
    ) {
        MaybeValue::<str>::update_with_state(
            //
            Children::into_script_inner_text(children),
            &mut children_state.0,
            super::utils::UpdateInnerText(element, renderer),
        )
    }
}
