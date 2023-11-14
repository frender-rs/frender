use frender_html_common::IntrinsicComponent;

use super::IntoElementProps;

#[derive(Default)]
pub struct ElementAndMounted<E> {
    pub element: E,
    pub mounted: bool,
}

pub struct IntrinsicChildrenAsElement<C, Children> {
    pub component_type: C,
    pub children: Children,
}

#[cfg(feature = "ssr")]
pub trait SsrIntrinsicComponent {
    #[inline]
    fn wrap_children<Children>(
        children: Children,
    ) -> frender_ssr::element::html::HtmlElementChildren<Children> {
        frender_ssr::element::html::HtmlElementChildren::Children(children)
    }
}

pub struct IntrinsicElement<C: IntrinsicComponent, P: IntoElementProps>(pub C, pub P);

pub trait ElementWithChildren<Children> {
    type ChildrenRenderState<R: crate::RenderHtml>: crate::RenderState<R> + Default;

    fn children_render_update<Renderer: crate::RenderHtml>(
        self,
        children: Children,
        renderer: &mut Renderer,
        render_state: std::pin::Pin<&mut Self::ChildrenRenderState<Renderer>>,
    );
}

mod imp {
    use crate::{
        component::IntoElementProps, renderer::CreateNode, Element, ElementOfType,
        IntrinsicComponent, RenderHtml, RenderState, UpdateElementNonReactive,
    };

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

    impl<E: crate::html::behaviors::Element<R>, S: RenderState<R>, R> RenderState<R>
        for IntrinsicElementRenderState<E, S>
    {
        fn unmount(self: std::pin::Pin<&mut Self>, renderer: &mut R) {
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
            if this
                .element_and_mounted
                .as_ref()
                .map_or(false, |v| v.mounted)
            {
                this.props_state.state_unmount();
            }
        }

        fn poll_render(
            self: std::pin::Pin<&mut Self>,
            renderer: &mut R,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<()> {
            let this = self.project();
            let element = if let Some(v) = this.element_and_mounted {
                &mut v.element
            } else {
                return std::task::Poll::Ready(());
            };

            element.move_cursor_at_the_first_child_of_self(renderer);
            // renderer.mark_position_at_first_child(element);
            let result = S::poll_render(this.props_state, renderer, cx);

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

    impl<R, C: RenderState<R>, A> RenderState<R> for ElementPropsState<C, A> {
        fn unmount(self: std::pin::Pin<&mut Self>, renderer: &mut R) {
            self.project().children_render_state.unmount(renderer)
        }

        fn state_unmount(self: std::pin::Pin<&mut Self>) {
            self.project().children_render_state.state_unmount()
        }

        fn poll_render(
            self: std::pin::Pin<&mut Self>,
            renderer: &mut R,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<()> {
            self.project()
                .children_render_state
                .poll_render(renderer, cx)
        }
    }

    impl<C: IntrinsicComponent + frender_html_common::IntrinsicComponent, P: IntoElementProps>
        Element for super::IntrinsicElement<C, P>
    where
        C::ElementTagType: crate::ElementSupportChildren<P::Children>,
        P::Attrs: UpdateElementNonReactive<C::ElementType>,
    {
        type RenderState<R: crate::RenderHtml> = IntrinsicElementRenderState<
            ElementOfType<C::ElementType, R>,
            ElementPropsState<
                <C::ElementTagType as crate::ElementSupportChildren<P::Children>>::ChildrenRenderState<
                    R,
                >,
                <P::Attrs as UpdateElementNonReactive<C::ElementType>>::State<R>,
            >,
        >;

        fn render_update_maybe_reposition<Renderer: crate::RenderHtml>(
            self,
            renderer: &mut Renderer,
            render_state: std::pin::Pin<&mut Self::RenderState<Renderer>>,
            force_reposition: bool,
        ) {
            let render_state = render_state.project();

            let props_state = render_state.props_state.project();

            let super::super::ElementProps {
                children,
                attributes,
            } = P::into_element_props(self.1);

            let element_and_mounted =
                render_state
                    .element_and_mounted
                    .get_or_insert_with(|| ElementAndMounted {
                        element: <C::ElementType as CreateNode>::create_node(renderer).into(),
                        mounted: false,
                    });

            update_element_maybe_reposition(
                element_and_mounted,
                renderer,
                |element, renderer| {
                    <P::Attrs>::update_element_non_reactive(
                        attributes,
                        renderer,
                        crate::convert::IntoMut::into_mut(element),
                        props_state.attrs_state,
                    );
                    <C::ElementTagType as crate::ElementSupportChildren<P::Children>>::children_render_update(
                            children,
                            renderer,
                            props_state.children_render_state,
                        )
                },
                force_reposition,
            )
        }
    }

    pub fn update_element_maybe_reposition<
        E: crate::renderer::node_behaviors::Element<R>,
        R: ?Sized + RenderHtml,
    >(
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

#[cfg(aaa)]
#[cfg(feature = "ssr")]
mod ssr {
    use std::borrow::Cow;

    use crate_common::IntrinsicComponent;
    use frender_common::write::attrs::IntoAsyncWritableAttrs;
    use frender_ssr::{AsyncWrite, Element};

    use crate::{ElementProps, IntoElementProps, IntrinsicChildrenAsElement, SsrWithChildren};

    use super::{CsrWithChildren, SsrIntrinsicComponent};

    impl<C: IntrinsicComponent, P: IntoElementProps> Element for super::IntrinsicElement<C, P>
    where
        C: SsrIntrinsicComponent,
        C: SsrWithChildren<P::Children>,
        P::Attrs: IntoAsyncWritableAttrs,
    {
        type SsrState = ::frender_ssr::element::html::HtmlElementRenderState<
            'static,
            C::ChildrenSsrState,
            <P::Attrs as IntoAsyncWritableAttrs>::AsyncWritableAttrs,
        >;

        fn into_ssr_state(self) -> Self::SsrState {
            let ElementProps {
                children,
                attributes,
            } = P::into_element_props(self.1);
            let children = IntrinsicChildrenAsElement {
                component_type: self.0,
                children,
            };
            ::frender_ssr::element::html::HtmlElement {
                tag: Cow::Borrowed(C::INTRINSIC_TAG),
                attributes,
                children: C::wrap_children(children),
            }
            .into_ssr_state()
        }
    }
}