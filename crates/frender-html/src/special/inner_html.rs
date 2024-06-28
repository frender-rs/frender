mod csr {
    use async_str_iter::IntoAsyncStrIterator;
    use frender_dom::{render_state::non_reactive::NonReactiveRenderState, special::DangerousInnerHtml};
    use frender_html_common::{MaybeValue, ValueUpdater};

    use crate::{element_types::RenderStateWithPehKind, CsrComponent, CsrComponentNormalElement};

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

    enum Never {}
    pub struct Kind<S: Default>(Never, std::marker::PhantomData<S>);

    impl<S: Default, C: CsrComponentNormalElement> RenderStateWithPehKind<C> for Kind<S> {
        type RenderStateWithPeh<R: crate::RenderHtml + ?Sized> = NonReactiveRenderState<S>;
        type RenderStateWithPehUnpinned<R: crate::RenderHtml + ?Sized> = NonReactiveRenderState<S>;
    }

    impl<C: CsrComponentNormalElement, S: MaybeValue<str> + IntoAsyncStrIterator> CsrComponent<DangerousInnerHtml<S>> for C {
        type ChildrenRenderStateKind = Kind<S::UpdateWithState>;

        fn children_render_update<R: crate::RenderHtml + ?Sized>(
            children: DangerousInnerHtml<S>,
            element: &mut Self::Element<R>,
            renderer: &mut R,
            children_state: std::pin::Pin<&mut NonReactiveRenderState<S::UpdateWithState>>,
        ) {
            Self::children_unpinned_render_update(children, element, renderer, children_state.get_mut())
        }

        fn children_unpinned_render_update<R: crate::RenderHtml + ?Sized>(
            children: DangerousInnerHtml<S>,
            element: &mut Self::Element<R>,
            renderer: &mut R,
            children_state: &mut NonReactiveRenderState<S::UpdateWithState>,
        ) {
            S::update_with_state(children.0, &mut children_state.0, UpdateElementInnerHtml { element, renderer })
        }
    }
}
