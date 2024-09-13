mod ssr {
    use async_str_iter::any_str::IterAnyStr;

    use frender_common::{strings::SsrStr, IntoStaticStr};

    use frender_dom::component::{IntoSpaceAndHtmlAttributesOrEmpty, SsrComponent};

    impl<Attrs: IntoSpaceAndHtmlAttributesOrEmpty, Children: SsrStr> SsrComponent<Attrs, Children> for crate::html::tags::style {
        type OneElement = frender_ssr::html::element::StyleElement<<Attrs as IntoSpaceAndHtmlAttributesOrEmpty>::SpaceAndHtmlAttributesOrEmpty, IterAnyStr<Children::StaticStr>>;

        fn ssr_component(attrs: Attrs, children: Children) -> Self::OneElement {
            Self::OneElement::new(attrs.into_space_and_html_attributes_or_empty(), IterAnyStr::new(children.into_into_static_str().into_static_str()))
        }
    }
}

mod csr {
    use crate::element_types::RenderStateWithPehKind;

    use crate::kinds::KindOfNonReactive;
    use crate::CsrComponent;

    use frender_common::strings::csr::update_with_option_cache;
    use frender_common::strings::CsrStr;
    
    use frender_dom::behaviors::HtmlElement;

    impl<Children: CsrStr> CsrComponent<Children> for crate::html::tags::style {
        type ChildrenRenderStateKind = KindOfNonReactive<Option<<Children as CsrStr>::StaticStrCache>>;

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
            _ = update_with_option_cache(children, &mut children_state.0, |value| element.set_inner_text(renderer, value))
        }
    }
}
