mod props_builder {
    use frender_html_common::IntoOneStringOrEmpty;

    use crate::form_control::value::FormControlValue;

    use crate::html::props::HtmlTextAreaElement;
    use crate::props_builder::PropsBuilderWithValue;

    impl<V: FormControlValue<str> + IntoOneStringOrEmpty, Attrs, EL> PropsBuilderWithValue<V> for HtmlTextAreaElement<(), Attrs, EL> {
        type WithValue = HtmlTextAreaElement<V, Attrs, EL>;

        /// Alias for [`Self::children`]
        fn value(self, value: V) -> HtmlTextAreaElement<V, Attrs, EL> {
            use crate::props_builder::PropsBuilderWithChildren;
            self.children(value)
        }
    }
}

pub mod ssr {
    use frender_dom::component::{IntoSpaceAndHtmlAttributesOrEmpty, SsrComponent};
    use frender_html_common::IntoOneStringOrEmpty;
    use frender_ssr::html::{encode::Encode, escape_safe, tag::AssertTagName};

    use crate::form_control::value::FormControlValue;
    use crate::html::tags;

    type Element<Attrs, Children> = frender_ssr::html::element::NormalElement<
        //
        AssertTagName<&'static str>,
        Attrs,
        Encode<escape_safe::Safe, Children>,
    >;

    impl<Attrs: IntoSpaceAndHtmlAttributesOrEmpty, Children> SsrComponent<Attrs, Children> for tags::textarea
    where
        Children: FormControlValue<str> + IntoOneStringOrEmpty,
    {
        type OneElement = Element<
            //
            Attrs::SpaceAndHtmlAttributesOrEmpty,
            Children::OneStringOrEmpty,
        >;

        fn ssr_component(attrs: Attrs, children: Children) -> Self::OneElement {
            use frender_dom::component::HasIntrinsicComponentTag;
            frender_ssr::html::element::NormalElement::new(
                Self::ASSERT_TAG_NAME,
                attrs.into_space_and_html_attributes_or_empty(),
                Encode::new(escape_safe::Safe, Children::into_one_string_or_empty(children)),
            )
        }
    }
}

pub mod csr {
    use frender_html_common::IntoOneStringOrEmpty;

    use crate::element_types::RenderStateWithPehKind;
    use crate::form_control::value::FormControlValue;
    use crate::{html::tags, CsrComponent, RenderHtml};

    enum Never {}
    pub struct Kind<V>(Never, std::marker::PhantomData<V>);

    impl<V: FormControlValue<str>> RenderStateWithPehKind<tags::textarea> for Kind<V> {
        type RenderStateWithPeh<R: RenderHtml + ?Sized> = V::State<R::textarea, R>;
        type RenderStateWithPehUnpinned<R: RenderHtml + ?Sized> = V::State<R::textarea, R>;
    }

    impl<Children> CsrComponent<Children> for tags::textarea
    where
        Children: FormControlValue<str> + IntoOneStringOrEmpty,
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
