use frender_dom::component::{HasIntrinsicComponentTag, IntoSpaceAndHtmlAttributesOrEmpty, SsrComponent};
use frender_dom::Empty;
use frender_ssr::html::tag::AssertTagName;

use crate::kinds::RenderInitNothing;
use crate::{html::markers as tags, CsrComponent, RenderHtml};

frender_common::impl_many!(
    impl<__> SsrComponent<Empty>
        for each_of![
            tags::area,
            tags::base,
            tags::br,
            tags::col,
            tags::embed,
            tags::hr,
            tags::img,
            // tags::input, // input is special
            tags::link,
            tags::meta,
            tags::source,
            tags::track,
            tags::wbr,
        ]
    {
        type OneElement<Attrs: IntoSpaceAndHtmlAttributesOrEmpty> = frender_ssr::html::element::VoidElement<AssertTagName<&'static str>, <Attrs as IntoSpaceAndHtmlAttributesOrEmpty>::SpaceAndHtmlAttributesOrEmpty>;

        fn ssr_component<Attrs: IntoSpaceAndHtmlAttributesOrEmpty>(self, attrs: Attrs, Empty: Empty) -> Self::OneElement<Attrs> {
            Self::OneElement::<Attrs>::new(Self::ASSERT_TAG_NAME, attrs.into_space_and_html_attributes_or_empty())
        }
    }
);

type Kind = crate::element_types::StateKindWithAnyParent<crate::kinds::KindOfNoState>;

frender_common::impl_many!(
    impl<__> CsrComponent<Empty>
        for each_of![
            tags::area,
            tags::base,
            tags::br,
            tags::col,
            tags::embed,
            tags::hr,
            tags::img,
            // tags::input, // input is special
            tags::link,
            tags::meta,
            tags::source,
            tags::track,
            tags::wbr,
        ]
    {
        type ChildrenRenderStateKind = Kind;
        type ChildrenPinnedRenderInit<R: RenderHtml + ?Sized> = RenderInitNothing;

        fn children_pinned_render_init<R: RenderHtml + ?Sized>(
            //
            self,
            Empty: Empty,
            _: &mut R,
            _: &mut Self::OfBehaviorType<R>,
        ) -> (
            //
            crate::element::PinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
            Self::ChildrenPinnedRenderInit<R>,
        ) {
            ((), RenderInitNothing)
        }

        fn children_pinned_render_init_by_reusing<R: RenderHtml + ?Sized>(
            //
            self,
            Empty: Empty,
            _: &mut R,
            _: &mut Self::OfBehaviorType<R>,
            _: std::pin::Pin<&mut crate::element::PinnedStateOfKind<R, Self::ChildrenRenderStateKind>>,
            (): crate::element::PinnedUnmountedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
        ) -> crate::element::PinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind> {
        }

        fn children_pinned_render_update<R: RenderHtml + ?Sized>(
            //
            self,
            Empty: Empty,
            _: &mut R,
            _: &mut Self::OfBehaviorType<R>,
            _: std::pin::Pin<&mut crate::element::PinnedStateOfKind<R, Self::ChildrenRenderStateKind>>,
            (): &mut crate::element::PinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
        ) {
        }

        fn children_unpinned_render_init<R: RenderHtml + ?Sized>(
            //
            self,
            Empty: Empty,
            _: &mut R,
            _: &mut Self::OfBehaviorType<R>,
        ) -> (
            //
            crate::element::UnpinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
            crate::element::UnpinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
        ) {
            ((), ())
        }

        fn children_unpinned_render_init_by_reusing<R: RenderHtml + ?Sized>(
            //
            self,
            Empty: Empty,
            _: &mut R,
            _: &mut Self::OfBehaviorType<R>,
            (): &mut crate::element::UnpinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
            (): crate::element::UnpinnedUnmountedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
        ) -> crate::element::UnpinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind> {
        }

        fn children_unpinned_render_update<R: RenderHtml + ?Sized>(
            //
            self,
            Empty: Empty,
            _: &mut R,
            _: &mut Self::OfBehaviorType<R>,
            (): &mut crate::element::UnpinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
            (): &mut crate::element::UnpinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
        ) {
        }
    }
);
