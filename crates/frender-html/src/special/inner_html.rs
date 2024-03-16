mod csr {
    use frender_dom::{render_state::non_reactive::NonReactiveRenderState, special::DangerousInnerHtml};
    use frender_html_common::{maybe_str::MaybeStr, ValueUpdater};

    use crate::{CsrComponent, CsrComponentNormalElement};

    struct UpdateElementInnerHtml<'a, E: ?Sized, R: ?Sized> {
        element: &'a mut E,
        renderer: &'a mut R,
    }

    impl<'a, E: ?Sized + frender_dom::behaviors::Element<R>, R: ?Sized> ValueUpdater<str> for UpdateElementInnerHtml<'a, E, R> {
        fn update(self, value: &str) {
            self.element.set_inner_html(self.renderer, value)
        }

        fn remove(self) {
            self.element.set_inner_html(self.renderer, "")
        }
    }

    impl<C: CsrComponentNormalElement, S: MaybeStr> CsrComponent<DangerousInnerHtml<S>> for C {
        type ChildrenRenderState<R: crate::RenderHtml + ?Sized> = Self::ChildrenUnpinnedRenderState<R>;

        fn children_render_update<R: crate::RenderHtml + ?Sized>(children: DangerousInnerHtml<S>, element: &mut Self::Element<R>, renderer: &mut R, children_state: std::pin::Pin<&mut Self::ChildrenRenderState<R>>) {
            Self::children_unpinned_render_update(children, element, renderer, children_state.get_mut())
        }

        type ChildrenUnpinnedRenderState<R: crate::RenderHtml + ?Sized> = NonReactiveRenderState<S::UpdateWithState>;

        fn children_unpinned_render_update<R: crate::RenderHtml + ?Sized>(children: DangerousInnerHtml<S>, element: &mut Self::Element<R>, renderer: &mut R, children_state: &mut Self::ChildrenUnpinnedRenderState<R>) {
            S::update_with_state(children.0, &mut children_state.0, UpdateElementInnerHtml { element, renderer })
        }
    }
}
