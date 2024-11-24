mod csr {
    use frender_common::strings::{csr::update_with_option_cache, CsrStr};
    use frender_dom::{behaviors::Element as _, render_state::non_reactive::NonReactiveRenderState, special::DangerousInnerHtml};

    use crate::{kinds::KindOfNonReactive, CsrComponent, CsrComponentNormalElement};

    impl<C: CsrComponentNormalElement, S: CsrStr> CsrComponent<DangerousInnerHtml<S>> for C {
        type ChildrenRenderStateKind = KindOfNonReactive<Option<S::StaticStrCache>>;

        fn children_render_update<R: crate::RenderHtml + ?Sized>(
            self,
            children: DangerousInnerHtml<S>,
            element: &mut Self::Element<R>,
            renderer: &mut R,
            children_state: std::pin::Pin<&mut NonReactiveRenderState<Option<S::StaticStrCache>>>,
        ) {
            self.children_unpinned_render_update(children, element, renderer, children_state.get_mut())
        }

        fn children_unpinned_render_update<R: crate::RenderHtml + ?Sized>(
            self,
            DangerousInnerHtml(inner_html): DangerousInnerHtml<S>,
            element: &mut Self::Element<R>,
            renderer: &mut R,
            NonReactiveRenderState(cache): &mut NonReactiveRenderState<Option<S::StaticStrCache>>,
        ) {
            _ = update_with_option_cache(inner_html, cache, |value| element.set_inner_html(renderer, value))
        }
    }
}
