mod ssr {
    use async_str_iter::any_str::IterAnyStr;

    use frender_common::{strings::SsrStr, IntoStaticStr};

    use frender_dom::component::{IntoSpaceAndHtmlAttributesOrEmpty, SsrComponent};

    use crate::html::components::style;

    impl<Children: SsrStr> SsrComponent<Children> for style::Marker {
        type OneElement<Attrs: IntoSpaceAndHtmlAttributesOrEmpty> = frender_ssr::html::element::StyleElement<<Attrs as IntoSpaceAndHtmlAttributesOrEmpty>::SpaceAndHtmlAttributesOrEmpty, IterAnyStr<Children::StaticStr>>;

        fn ssr_component<Attrs: IntoSpaceAndHtmlAttributesOrEmpty>(self, attrs: Attrs, children: Children) -> Self::OneElement<Attrs> {
            Self::OneElement::<Attrs>::new(attrs.into_space_and_html_attributes_or_empty(), IterAnyStr::new(children.into_into_static_str().into_static_str()))
        }
    }
}

pub mod csr {
    use crate::html::behavior_type_traits;
    use crate::html::components::style;

    use crate::CsrComponent;

    use frender_common::reactive_value::{ReactiveValue, RenderValueMut, RenderValueWithFnMutAndData};

    fn into_renderer<'a, ET: ?Sized + behavior_type_traits::HtmlElement, R: crate::RenderHtml + ?Sized>(
        //
        renderer: &'a mut R,
        parent: &'a mut <ET as crate::BehaviorType>::OfBehaviorType<R>,
    ) -> impl 'a + RenderValueMut<str, RenderOutput = ()> {
        fn fn_mut_into_renderer<F: FnMut(&str)>(f: F) -> impl RenderValueMut<str, RenderOutput = ()> {
            RenderValueWithFnMutAndData {
                update: |f: &mut F, value: &str| f(value),
                remove: |f: &mut F| f(""),
                data: f,
            }
        }

        fn_mut_into_renderer(|value| {
            use frender_common::convert::FromMut as _;
            use frender_dom::behaviors::HtmlElement as _;
            ET::HtmlElement::from_mut(parent).set_inner_text(renderer, value)
        })
    }

    super::super::define_Kind_with_ReactiveValue!(
        pub struct Kind;
        type ReactiveValueKind = str;
        type ET: behavior_type_traits::HtmlStyleElement;

        const into_mut_renderer: _ = for<R> |renderer, parent| &mut into_renderer::<ET, R>(renderer, parent);
    );

    impl<Children: ReactiveValue<str>> CsrComponent<Children> for style::Marker {
        super::super::impl_CsrComponent_with_ReactiveValue!(
            type Kind = Kind;
            type This = Children;
            const into_reactive_value: Children = |children| children;
            const into_renderer: _ = for<R> |renderer, parent| into_renderer::<style::Marker, R>(renderer, parent);
        );
    }
}
