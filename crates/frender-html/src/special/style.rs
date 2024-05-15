use frender_dom::component::{IntoSpaceAndHtmlAttributesOrEmpty, SsrComponent};
use frender_html_common::{IntoOneStringOrEmpty, MaybeValue};

use crate::{elements::non_reactive::NonReactiveRenderState, CsrComponent};

impl<Attrs: IntoSpaceAndHtmlAttributesOrEmpty, Children: MaybeValue<str> + IntoOneStringOrEmpty> SsrComponent<Attrs, Children> for crate::html::tags::style {
    type OneElement = frender_ssr::html::element::StyleElement<<Attrs as IntoSpaceAndHtmlAttributesOrEmpty>::SpaceAndHtmlAttributesOrEmpty, Children::OneStringOrEmpty>;

    fn ssr_component(attrs: Attrs, children: Children) -> Self::OneElement {
        Self::OneElement::new(attrs.into_space_and_html_attributes_or_empty(), Children::into_one_string_or_empty(children))
    }
}

impl<Children: MaybeValue<str> + IntoOneStringOrEmpty> CsrComponent<Children> for crate::html::tags::style {
    type ChildrenRenderState<R: crate::RenderHtml + ?Sized> = NonReactiveRenderState<<Children as MaybeValue<str>>::UpdateWithState>;

    fn children_render_update<R: crate::RenderHtml + ?Sized>(children: Children, element: &mut Self::Element<R>, renderer: &mut R, children_state: std::pin::Pin<&mut Self::ChildrenRenderState<R>>) {
        Self::children_unpinned_render_update(children, element, renderer, children_state.get_mut())
    }

    type ChildrenUnpinnedRenderState<R: crate::RenderHtml + ?Sized> = Self::ChildrenRenderState<R>;

    fn children_unpinned_render_update<R: crate::RenderHtml + ?Sized>(children: Children, element: &mut Self::Element<R>, renderer: &mut R, children_state: &mut Self::ChildrenUnpinnedRenderState<R>) {
        MaybeValue::<str>::update_with_state(
            //
            children,
            &mut children_state.0,
            super::utils::UpdateInnerText(element, renderer),
        )
    }
}
