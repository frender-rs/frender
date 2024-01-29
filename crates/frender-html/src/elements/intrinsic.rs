#[derive(Default)]
pub struct ElementAndMounted<E> {
    pub element: E,
    pub mounted: bool,
}

mod imp {
    use crate::dom::component::{HasIntrinsicComponentTag, IntoElementProps};

    use crate::{CreateNode, RenderHtml, UpdateNodeNonReactive};

    use crate::{Element, RenderState};

    use super::ElementAndMounted;

    pin_project_lite::pin_project!(
        pub struct IntrinsicElementRenderState<E, S> {
            element_and_mounted: Option<ElementAndMounted<E>>,
            #[pin]
            props_state: S,
        }
    );

    impl<E, S: Default> Default for IntrinsicElementRenderState<E, S> {
        fn default() -> Self {
            Self {
                element_and_mounted: None,
                props_state: Default::default(),
            }
        }
    }

    impl<
            //
            PEH: ?Sized,
            R: ?Sized,
            E: crate::html::behaviors::Element<R>,
            S: RenderState<E, R>,
        > RenderState<PEH, R> for IntrinsicElementRenderState<E, S>
    {
        fn unmount(self: std::pin::Pin<&mut Self>, _: &mut PEH, renderer: &mut R) {
            let this = self.project();
            if let Some(ElementAndMounted { element, mounted }) = this.element_and_mounted {
                if *mounted {
                    *mounted = false;
                    // renderer.remove_node(element);
                    element.remove_self(renderer);
                    this.props_state.state_unmount();
                }
            }
        }

        fn state_unmount(self: std::pin::Pin<&mut Self>) {
            let this = self.project();
            if this.element_and_mounted.as_ref().map_or(false, |v| v.mounted) {
                this.props_state.state_unmount();
            }
        }

        fn poll_render(self: std::pin::Pin<&mut Self>, peh: &mut PEH, renderer: &mut R, cx: &mut std::task::Context<'_>) -> std::task::Poll<()> {
            let this = self.project();
            let element = if let Some(v) = this.element_and_mounted {
                &mut v.element
            } else {
                return std::task::Poll::Ready(());
            };

            element.move_cursor_at_the_first_child_of_self(renderer);
            // renderer.mark_position_at_first_child(element);
            let result = S::poll_render(this.props_state, element, renderer, cx);

            element.move_cursor_after_self(renderer);
            // renderer.mark_position_after(element);

            result
        }
    }

    pin_project_lite::pin_project! {
        #[derive(Default)]
        pub struct ElementPropsState<C, A> {
            #[pin]
            children_render_state: C,
            attrs_state: A,
        }
    }

    impl<
            //
            PEH: ?Sized,
            R: ?Sized,
            C: RenderState<PEH, R>,
            A,
        > RenderState<PEH, R> for ElementPropsState<C, A>
    {
        fn unmount(self: std::pin::Pin<&mut Self>, peh: &mut PEH, renderer: &mut R) {
            self.project().children_render_state.unmount(peh, renderer)
        }

        fn state_unmount(self: std::pin::Pin<&mut Self>) {
            self.project().children_render_state.state_unmount()
        }

        fn poll_render(self: std::pin::Pin<&mut Self>, peh: &mut PEH, renderer: &mut R, cx: &mut std::task::Context<'_>) -> std::task::Poll<()> {
            self.project().children_render_state.poll_render(peh, renderer, cx)
        }
    }

    impl<C: HasIntrinsicComponentTag + crate::html::behavior_type_traits::Element + CreateNode, P: IntoElementProps> Element for crate::dom::component::IntrinsicElement<C, P>
    where
        C: crate::CsrComponent<P::Children>,
        P::Attrs: UpdateNodeNonReactive<C>,
        // ssr bounds
        P::Attrs: crate::dom::component::IntoSpaceAndHtmlAttributesOrEmpty,
        C: crate::dom::component::SsrComponent<P::Attrs, P::Children>,
    {
        type RenderState<PEH: ?Sized, R: RenderHtml + ?Sized> =
            IntrinsicElementRenderState<C::Element<R>, ElementPropsState<<C as crate::CsrComponent<P::Children>>::ChildrenRenderState<R>, <P::Attrs as UpdateNodeNonReactive<C>>::State<R>>>;

        fn render_update_maybe_reposition<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(
            self,
            peh: &mut PEH,
            renderer: &mut Renderer,
            render_state: std::pin::Pin<&mut Self::RenderState<PEH, Renderer>>,
            force_reposition: bool,
        ) {
            let render_state = render_state.project();

            let props_state = render_state.props_state.project();

            let crate::dom::component::ElementProps { children, attributes } = P::into_element_props(self.1);

            let element_and_mounted = render_state.element_and_mounted.get_or_insert_with(|| ElementAndMounted {
                element: <C::Element<Renderer>>::from(<C as CreateNode>::create_node(renderer)),
                mounted: false,
            });

            update_element_maybe_reposition(
                element_and_mounted,
                renderer,
                |element, renderer| {
                    <P::Attrs>::update_node_non_reactive(attributes, renderer, frender_common::convert::IntoMut::into_mut(element), props_state.attrs_state);
                    <C as crate::CsrComponent<P::Children>>::children_render_update(children, element, renderer, props_state.children_render_state)
                },
                force_reposition,
            )
        }

        type UnpinnedRenderState<PEH: ?Sized, R: RenderHtml + ?Sized> =
            IntrinsicElementRenderState<C::Element<R>, ElementPropsState<<C as crate::CsrComponent<P::Children>>::ChildrenUnpinnedRenderState<R>, <P::Attrs as UpdateNodeNonReactive<C>>::State<R>>>;

        fn unpinned_render_update_maybe_reposition<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(
            self,
            peh: &mut PEH,
            renderer: &mut Renderer,
            render_state: &mut Self::UnpinnedRenderState<PEH, Renderer>,
            force_reposition: bool,
        ) {
            let props_state = &mut render_state.props_state;

            let crate::dom::component::ElementProps { children, attributes } = P::into_element_props(self.1);

            let element_and_mounted = render_state.element_and_mounted.get_or_insert_with(|| ElementAndMounted {
                element: <C as CreateNode>::create_node(renderer).into(),
                mounted: false,
            });

            update_element_maybe_reposition(
                element_and_mounted,
                renderer,
                |element, renderer| {
                    <P::Attrs>::update_node_non_reactive(attributes, renderer, frender_common::convert::IntoMut::into_mut(element), &mut props_state.attrs_state);
                    <C as crate::CsrComponent<P::Children>>::children_unpinned_render_update(children, element, renderer, &mut props_state.children_render_state)
                },
                force_reposition,
            )
        }
    }

    pub fn update_element_maybe_reposition<E: crate::html::behaviors::Element<R>, R: ?Sized + RenderHtml>(
        element_and_mounted: &mut ElementAndMounted<E>,
        renderer: &mut R,
        update: impl FnOnce(&mut E, &mut R),
        force_reposition: bool,
    ) {
        let ElementAndMounted { element, mounted } = element_and_mounted;

        if *mounted && !force_reposition {
            // element.move_cursor_after_self(renderer);
        } else {
            // if *mounted && element.cursor_is_at_self(renderer) {
            //     element.move_cursor_after_self(renderer);
            //     return;
            // }

            element.readd_self(renderer, true);
            *mounted = true;
        }

        {
            element.move_cursor_at_the_first_child_of_self(renderer);
            update(element, renderer);
        };

        element.move_cursor_after_self(renderer);
    }
}
