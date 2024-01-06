use frender_dom::component::{HasIntrinsicComponentTag, IntoSpaceAndHtmlAttributesOrEmpty, SsrComponent};
use frender_ssr::html::tag::AssertTagName;

use crate::{html::tags, CsrComponent, RenderHtml};

frender_common::impl_many!(
    impl<__>
        (
            Generics![Attrs: IntoSpaceAndHtmlAttributesOrEmpty],
            Trait![SsrComponent<Attrs, ()>],
            each_of![
                tags::area,
                tags::base,
                tags::br,
                tags::col,
                tags::embed,
                tags::hr,
                tags::img,
                tags::input,
                tags::link,
                tags::meta,
                tags::source,
                tags::track,
                tags::wbr,
            ],
        )
    {
        type OneElement = frender_ssr::html::element::VoidElement<AssertTagName<&'static str>, <Attrs as IntoSpaceAndHtmlAttributesOrEmpty>::SpaceAndHtmlAttributesOrEmpty>;

        fn ssr_component(attrs: Attrs, (): ()) -> Self::OneElement {
            Self::OneElement::new(Self::ASSERT_TAG_NAME, attrs.into_space_and_html_attributes_or_empty())
        }
    }
);

frender_common::impl_many!(
    impl<__>
        (
            Trait![CsrComponent<()>],
            each_of![
                tags::area,
                tags::base,
                tags::br,
                tags::col,
                tags::embed,
                tags::hr,
                tags::img,
                tags::input,
                tags::link,
                tags::meta,
                tags::source,
                tags::track,
                tags::wbr,
            ],
        )
    {
        type ChildrenRenderState<R: RenderHtml + ?Sized> = ();
        fn children_render_update<R: RenderHtml + ?Sized>(_: (), _: &mut Self::Element<R>, _: &mut R, _: std::pin::Pin<&mut Self::ChildrenRenderState<R>>) {}
        type ChildrenUnpinnedRenderState<R: RenderHtml + ?Sized> = ();
        fn children_unpinned_render_update<R: RenderHtml + ?Sized>(_: (), _: &mut Self::Element<R>, _: &mut R, _: &mut Self::ChildrenUnpinnedRenderState<R>) {}
    }
);
