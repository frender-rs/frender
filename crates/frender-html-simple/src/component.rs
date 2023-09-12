use frender_html_common::IntrinsicComponent;

use crate::IntoElementProps;

#[cfg(feature = "csr")]
pub trait DomIntrinsicComponent {
    type Element: AsRef<frender_csr::web_sys::Element> + frender_csr::wasm_bindgen::JsCast;
}

#[cfg(feature = "csr")]
pub trait CsrWithChildren<Children> {
    type ChildrenState: frender_csr::RenderState;

    fn children_into_csr_state(
        self,
        children: Children,
        ctx: &mut frender_csr::CsrContext,
    ) -> Self::ChildrenState;
    fn children_update_csr_state(
        self,
        children: Children,
        ctx: &mut frender_csr::CsrContext,
        children_state: std::pin::Pin<&mut Self::ChildrenState>,
    );
}

#[cfg(feature = "ssr")]
pub trait SsrWithChildren<Children> {
    type ChildrenSsrState: frender_ssr::RenderState;

    fn into_children_ssr_state(self, children: Children) -> Self::ChildrenSsrState;
}

pub struct IntrinsicChildrenAsElement<C, Children> {
    pub component_type: C,
    pub children: Children,
}

#[cfg(feature = "ssr")]
impl<C, Children> frender_ssr::Element for IntrinsicChildrenAsElement<C, Children>
where
    C: SsrWithChildren<Children>,
{
    type SsrState = C::ChildrenSsrState;

    fn into_ssr_state(self) -> Self::SsrState {
        self.component_type.into_children_ssr_state(self.children)
    }
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
    type ChildrenRenderState<R: frender_html::RenderHtml>: frender_html::RenderState<R> + Default;

    fn children_render_update<Renderer: frender_html::RenderHtml>(
        self,
        children: Children,
        renderer: &mut Renderer,
        render_state: std::pin::Pin<&mut Self::ChildrenRenderState<Renderer>>,
    );
}

mod imp {
    use frender_csr::element::intrinsic::ElementAndMounted;
    use frender_html::{
        renderer::{node_behaviors, CreateElement},
        Element, ElementOfType, IntrinsicComponent, RenderHtml, RenderState,
        UpdateElementNonReactive,
    };

    use crate::IntoElementProps;

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

    impl<E: node_behaviors::Element<R>, S: RenderState<R>, R> RenderState<R>
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
        C::ElementTagType: frender_html::ElementSupportChildren<P::Children>,
        P::Attrs: UpdateElementNonReactive<C::ElementType>,
    {
        type RenderState<R: frender_html::RenderHtml> = IntrinsicElementRenderState<
            ElementOfType<C::ElementType, R>,
            ElementPropsState<
                <C::ElementTagType as frender_html::ElementSupportChildren<P::Children>>::ChildrenRenderState<
                    R,
                >,
                <P::Attrs as UpdateElementNonReactive<C::ElementType>>::State,
            >,
        >;

        fn render_update_maybe_reposition<Renderer: frender_html::RenderHtml>(
            self,
            renderer: &mut Renderer,
            render_state: std::pin::Pin<&mut Self::RenderState<Renderer>>,
            force_reposition: bool,
        ) {
            let render_state = render_state.project();

            let props_state = render_state.props_state.project();

            let crate::ElementProps {
                children,
                attributes,
            } = P::into_element_props(self.1);

            let element_and_mounted =
                render_state
                    .element_and_mounted
                    .get_or_insert_with(|| ElementAndMounted {
                        element: <C::ElementType as CreateElement>::create_element(renderer),
                        mounted: false,
                    });

            update_element_maybe_reposition(
                element_and_mounted,
                renderer,
                |element, renderer| {
                    <P::Attrs>::update_element_non_reactive(
                        attributes,
                        renderer,
                        element,
                        props_state.attrs_state,
                    );
                    <C::ElementTagType as frender_html::ElementSupportChildren<P::Children>>::children_render_update(
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
        E: frender_html::renderer::node_behaviors::Element<R>,
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

#[cfg(feature = "csr")]
mod dom {
    use frender_csr::Element;
    use frender_csr::{props::UpdateElementNonReactive, CsrContext};
    use frender_html_common::IntrinsicComponent;

    use crate::{states::ElementPropsStates, ElementProps};

    use super::{DomIntrinsicComponent, IntoElementProps};

    impl<C: IntrinsicComponent, P: IntoElementProps> Element for super::IntrinsicElement<C, P>
    where
        C: DomIntrinsicComponent,
        C: crate::CsrWithChildren<P::Children>,
        P::Attrs: UpdateElementNonReactive<C::Element>,
    {
        type CsrState = ::frender_csr::element::intrinsic::IntrinsicComponentRenderState<
            C::Element,
            ElementPropsStates<
                C::ChildrenState,
                <P::Attrs as UpdateElementNonReactive<C::Element>>::State,
            >,
        >;

        fn into_csr_state(self, ctx: &mut CsrContext) -> Self::CsrState {
            let ElementProps {
                children,
                attributes,
            } = P::into_element_props(self.1);
            Self::CsrState::initialize_with_tag(
                move |element, children_ctx| ElementPropsStates {
                    children: C::children_into_csr_state(self.0, children, children_ctx),
                    other_state:
                        UpdateElementNonReactive::<C::Element>::initialize_state_non_reactive(
                            attributes,
                            element,
                            children_ctx,
                        ),
                },
                ctx,
                C::INTRINSIC_TAG,
            )
        }

        fn update_csr_state_maybe_reposition(
            self,
            ctx: &mut frender_csr::CsrContext,
            state: std::pin::Pin<&mut Self::CsrState>,
            force_reposition: bool,
        ) where
            Self: Sized,
        {
            #[cfg(debug_assertions)]
            frender_csr::web_sys::console::log_2(
                &"IntrinsicElement force_reposition".into(),
                &force_reposition.into(),
            );
            let (node_and_mounted, state) = state.pin_project();
            let (children_state, attrs_state) = state.pin_project();
            node_and_mounted.update_maybe_reposition(
                ctx,
                |element, children_ctx| {
                    let ElementProps {
                        children,
                        attributes,
                    } = P::into_element_props(self.1);
                    UpdateElementNonReactive::<C::Element>::update_element_non_reactive(
                        attributes,
                        element,
                        children_ctx,
                        attrs_state,
                    );
                    C::children_update_csr_state(self.0, children, children_ctx, children_state);
                },
                force_reposition,
            )
        }
    }
}

#[cfg(feature = "ssr")]
mod ssr {
    use std::borrow::Cow;

    use frender_common::write::attrs::IntoAsyncWritableAttrs;
    use frender_html_common::IntrinsicComponent;
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
