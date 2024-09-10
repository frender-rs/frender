mod csr {
    use frender_attr_value::csr::UpdateAttrValue;
    use frender_common::strings::{csr::update_with_option_cache, CsrStr};
    use frender_dom::{behaviors::Element as _, render_state::non_reactive::NonReactiveRenderState, special::DangerousInnerHtml};

    use crate::{kinds::KindOfNonReactive, CsrComponent, CsrComponentNormalElement};

    struct UpdateElementInnerHtml<'a, E: ?Sized, R: ?Sized> {
        element: &'a mut E,
        renderer: &'a mut R,
    }

    impl<'a, E: ?Sized + frender_dom::behaviors::Element<R>, R: ?Sized> UpdateAttrValue for UpdateElementInnerHtml<'a, E, R> {
        type Kind = str;

        fn set(self, value: &str) {
            self.element.set_inner_html(self.renderer, value)
        }

        fn remove(self) {
            self.element.set_inner_html(self.renderer, "")
        }
    }

    impl<C: CsrComponentNormalElement, S: CsrStr> CsrComponent<DangerousInnerHtml<S>> for C {
        type ChildrenRenderStateKind = KindOfNonReactive<Option<S::StaticStrCache>>;

        fn children_render_update<R: crate::RenderHtml + ?Sized>(
            children: DangerousInnerHtml<S>,
            element: &mut Self::Element<R>,
            renderer: &mut R,
            children_state: std::pin::Pin<&mut NonReactiveRenderState<Option<S::StaticStrCache>>>,
        ) {
            Self::children_unpinned_render_update(children, element, renderer, children_state.get_mut())
        }

        fn children_unpinned_render_update<R: crate::RenderHtml + ?Sized>(
            DangerousInnerHtml(inner_html): DangerousInnerHtml<S>,
            element: &mut Self::Element<R>,
            renderer: &mut R,
            NonReactiveRenderState(cache): &mut NonReactiveRenderState<Option<S::StaticStrCache>>,
        ) {
            _ = update_with_option_cache(inner_html, cache, |value| element.set_inner_html(renderer, value))
        }
    }
}
