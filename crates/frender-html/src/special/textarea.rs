mod props_builder {
    use frender_html_common::maybe_str::IntoOneStringOrEmpty;

    use crate::form_control::value::FormControlValue;

    use crate::html::props::HtmlTextAreaElement;

    impl<Attrs> HtmlTextAreaElement::Building<(), Attrs> {
        /// Alias for [`Self::children`]
        pub fn value<Children: FormControlValue<str> + IntoOneStringOrEmpty>(self, value: Children) -> HtmlTextAreaElement::Building<Children, Attrs> {
            use HtmlTextAreaElement::prelude::*;

            self.children(value)
        }
    }
}

pub mod ssr {
    use frender_dom::component::{IntoSpaceAndHtmlAttributesOrEmpty, SsrComponent};
    use frender_html_common::maybe_str::IntoOneStringOrEmpty;
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
    use frender_html_common::maybe_str::IntoOneStringOrEmpty;

    use crate::form_control::value::FormControlValue;
    use crate::{html::tags, CsrComponent, RenderHtml};

    impl<Children> CsrComponent<Children> for tags::textarea
    where
        Children: FormControlValue<str> + IntoOneStringOrEmpty,
    {
        type ChildrenRenderState<R: RenderHtml + ?Sized> = Self::ChildrenUnpinnedRenderState<R>;

        fn children_render_update<R: RenderHtml + ?Sized>(children: Children, element: &mut Self::Element<R>, renderer: &mut R, children_state: std::pin::Pin<&mut Self::ChildrenRenderState<R>>) {
            Self::children_unpinned_render_update(children, element, renderer, children_state.get_mut())
        }

        type ChildrenUnpinnedRenderState<R: RenderHtml + ?Sized> = Children::State<
            //
            R::textarea,
            R,
        >;

        fn children_unpinned_render_update<R: RenderHtml + ?Sized>(children: Children, element: &mut Self::Element<R>, renderer: &mut R, children_state: &mut Self::ChildrenUnpinnedRenderState<R>) {
            let element: &mut R::textarea = element;
            Children::update_with_state(children, children_state, element, renderer)
        }
    }
}
