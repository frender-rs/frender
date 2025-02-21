use frender_dom::Empty;
use frender_dom::{
    ssr::{IntoSpaceAndHtmlAttributesOrEmpty, SsrComponent},
    HasIntrinsicComponentTag as _,
};

use crate::kinds::RenderInitNothing;
use crate::{
    csr::element,
    html::{markers as tags, RenderHtml},
    CsrComponent,
};

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
            element::PinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
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
            _: std::pin::Pin<&mut element::PinnedStateOfKind<R, Self::ChildrenRenderStateKind>>,
            (): element::PinnedUnmountedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
        ) -> element::PinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind> {
        }

        fn children_pinned_render_update<R: RenderHtml + ?Sized>(
            //
            self,
            Empty: Empty,
            _: &mut R,
            _: &mut Self::OfBehaviorType<R>,
            _: std::pin::Pin<&mut element::PinnedStateOfKind<R, Self::ChildrenRenderStateKind>>,
            (): &mut element::PinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
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
            element::UnpinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
            element::UnpinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
        ) {
            ((), ())
        }

        fn children_unpinned_render_init_by_reusing<R: RenderHtml + ?Sized>(
            //
            self,
            Empty: Empty,
            _: &mut R,
            _: &mut Self::OfBehaviorType<R>,
            (): &mut element::UnpinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
            (): element::UnpinnedUnmountedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
        ) -> element::UnpinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind> {
        }

        fn children_unpinned_render_update<R: RenderHtml + ?Sized>(
            //
            self,
            Empty: Empty,
            _: &mut R,
            _: &mut Self::OfBehaviorType<R>,
            (): &mut element::UnpinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
            (): &mut element::UnpinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
        ) {
        }
    }
);
