mod props_builder {
    use frender_dom::special::textarea::TextAreaValue;

    use crate::html::components::{textarea, HtmlTextAreaElement};

    impl<Attrs, EL> HtmlTextAreaElement::Props<crate::Empty, Attrs, EL> {
        /// Alias for [`Self::children`]
        pub fn value<V: TextAreaValue>(self, value: V) -> HtmlTextAreaElement::Props<V, Attrs, EL> {
            self.children(value)
        }
    }

    impl<Attrs, EL> textarea::Element<crate::Empty, Attrs, EL> {
        /// Alias for [`Self::children`]
        pub fn value<V: TextAreaValue>(self, value: V) -> textarea::Element<V, Attrs, EL> {
            self.children(value)
        }
    }
}

pub mod ssr {
    use frender_dom::component::{IntoSpaceAndHtmlAttributesOrEmpty, SsrComponent};
    use frender_dom::special::textarea::SsrTextAreaValue;
    use frender_ssr::html::tag::AssertTagName;

    use crate::cs::textarea;

    type Element<Attrs, Children> = frender_ssr::html::element::NormalElement<
        //
        AssertTagName<&'static str>,
        Attrs,
        Children,
    >;

    impl<Attrs: IntoSpaceAndHtmlAttributesOrEmpty, Children> SsrComponent<Attrs, Children> for textarea::Marker
    where
        Children: SsrTextAreaValue,
    {
        type OneElement = Element<
            //
            Attrs::SpaceAndHtmlAttributesOrEmpty,
            Children::IntoSsrTextAreaValue,
        >;

        fn ssr_component(attrs: Attrs, children: Children) -> Self::OneElement {
            use frender_dom::component::HasIntrinsicComponentTag;
            frender_ssr::html::element::NormalElement::new(Self::ASSERT_TAG_NAME, attrs.into_space_and_html_attributes_or_empty(), Children::into_ssr_text_area_value(children))
        }
    }
}

pub mod csr {
    use crate::cs::textarea;
    use crate::element_types::RenderStateWithPehKind;
    use crate::form_control::value::FormControlValue;
    use crate::{CsrComponent, RenderHtml};

    enum Never {}
    pub struct Kind<V>(Never, std::marker::PhantomData<V>);

    impl<V: FormControlValue<str>> RenderStateWithPehKind<textarea::Marker> for Kind<V> {
        type RenderStateWithPeh<R: RenderHtml + ?Sized> = V::State<R::textarea, R>;
        type RenderStateWithPehUnpinned<R: RenderHtml + ?Sized> = V::State<R::textarea, R>;
    }

    impl<Children> CsrComponent<Children> for textarea::Marker
    where
        Children: FormControlValue<str>,
    {
        type ChildrenRenderStateKind = Kind<Children>;

        fn children_render_update<R: RenderHtml + ?Sized>(
            children: Children,
            element: &mut Self::Element<R>,
            renderer: &mut R,
            children_state: std::pin::Pin<&mut <Self::ChildrenRenderStateKind as RenderStateWithPehKind<Self>>::RenderStateWithPeh<R>>,
        ) {
            Self::children_unpinned_render_update(children, element, renderer, children_state.get_mut())
        }

        fn children_unpinned_render_update<R: RenderHtml + ?Sized>(
            children: Children,
            element: &mut Self::Element<R>,
            renderer: &mut R,
            children_state: &mut <Self::ChildrenRenderStateKind as RenderStateWithPehKind<Self>>::RenderStateWithPehUnpinned<R>,
        ) {
            let element: &mut R::textarea = element;
            Children::update_with_state(children, children_state, element, renderer)
        }
    }
}
