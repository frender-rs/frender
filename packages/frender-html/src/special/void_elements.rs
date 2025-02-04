use frender_dom::component::{HasIntrinsicComponentTag, IntoSpaceAndHtmlAttributesOrEmpty, SsrComponent};
use frender_dom::Empty;
use frender_ssr::html::tag::AssertTagName;

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

        fn children_pinned_render_init<R: RenderHtml + ?Sized>(
            //
            self,
            Empty: Empty,
            _: &mut R,
            _: &mut Self::OfBehaviorType<R>,
            _: crate::element::PinMutRenderInitStatesOfKind<Self::ChildrenRenderStateKind, R>,
        ) -> crate::element::PinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind> {
        }

        fn children_pinned_render_update<R: RenderHtml + ?Sized>(
            //
            self,
            Empty: Empty,
            _: &mut R,
            _: &mut Self::OfBehaviorType<R>,
            _: crate::element::PinnedMutRenderStatesOfKind<Self::ChildrenRenderStateKind, R>,
        ) {
        }

        fn children_unpinned_render_init<R: RenderHtml + ?Sized>(
            //
            self,
            Empty: Empty,
            _: &mut R,
            _: &mut Self::OfBehaviorType<R>,
        ) -> crate::element::UnpinnedRenderStatesOfKind<Self::ChildrenRenderStateKind, R> {
            crate::element::RenderStates {
                ui_handle: (),
                non_reactive_state: (),
                reactive_state: (),
            }
        }

        fn children_unpinned_render_update<R: RenderHtml + ?Sized>(
            //
            self,
            Empty: Empty,
            _: &mut R,
            _: &mut Self::OfBehaviorType<R>,
            _: crate::element::UnpinnedMutRenderStatesOfKind<Self::ChildrenRenderStateKind, R>,
        ) {
        }
    }
);
