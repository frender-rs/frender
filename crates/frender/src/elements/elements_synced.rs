use std::{pin::Pin, task::Poll};

use frender_csr::RenderState;
use frender_ssr::SsrElement;

use crate::{into_element::ToElement, Element, IntoElement, IntoElements};

pub struct ElementsSynced<ES> {
    updates: (),
    elements: ES,
}

pub struct ElementsSyncedToElement<'a, E: ToElement> {
    elements: &'a Vec<E>,
}

impl<'a, E: ToElement> SsrElement for ElementsSyncedToElement<'a, E> {
    type HtmlChildren = async_str_iter::empty::Empty;

    fn into_html_children(self) -> Self::HtmlChildren {
        todo!()
    }
}

pub struct States<S> {
    states: Vec<S>,
}

impl<S> Default for States<S> {
    fn default() -> Self {
        Self {
            states: Default::default(),
        }
    }
}

impl<S: RenderState<R> + Unpin, R: ?Sized> RenderState<R> for States<S> {
    fn unmount(self: std::pin::Pin<&mut Self>, renderer: &mut R) {
        self.get_mut()
            .states
            .iter_mut()
            .for_each(|state| S::unmount(Pin::new(state), renderer))
    }

    fn state_unmount(self: std::pin::Pin<&mut Self>) {
        self.get_mut()
            .states
            .iter_mut()
            .for_each(|state| S::state_unmount(Pin::new(state)))
    }

    fn poll_render(
        self: std::pin::Pin<&mut Self>,
        renderer: &mut R,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        self.get_mut()
            .states
            .iter_mut()
            .fold(Poll::Ready(()), |res, state| {
                match S::poll_render(Pin::new(state), renderer, cx) {
                    Poll::Ready(()) => res,
                    Poll::Pending => Poll::Pending,
                }
            })
    }
}

impl<'a, E: ToElement> Element for ElementsSyncedToElement<'a, E> {
    type RenderState<R: frender_html::RenderHtml + ?Sized> = Self::UnpinnedRenderState<R>;

    type UnpinnedRenderState<R: frender_html::RenderHtml + ?Sized> =
        States<<E::ToElement<'a> as Element>::UnpinnedRenderState<R>>;

    fn render_update_maybe_reposition<Renderer: frender_html::RenderHtml + ?Sized>(
        //
        self,
        render_context: &mut Renderer::RenderContext<'_>,
        render_state: Pin<&mut Self::RenderState<Renderer>>,
        force_reposition: bool,
    ) {
        todo!()
    }

    fn unpinned_render_update_maybe_reposition<Renderer: frender_html::RenderHtml + ?Sized>(
        //
        self,
        render_context: &mut Renderer::RenderContext<'_>,
        render_state: &mut Self::UnpinnedRenderState<Renderer>,
        force_reposition: bool,
    ) {
        let mut render_states = render_state.states.iter_mut();
        self.elements.iter().for_each(|el| {
            el.to_element().unpinned_render_update_maybe_reposition(
                render_context,
                render_states.next().unwrap(),
                force_reposition,
            )
        })
    }
}

impl<E: ToElement> ToElement for ElementsSynced<Vec<E>> {
    type ToElement<'a> = ElementsSyncedToElement<'a, E>
    where
        Self: 'a;

    fn to_element(&self) -> Self::ToElement<'_> {
        ElementsSyncedToElement {
            elements: &self.elements,
        }
    }
}

pub fn ElementsSynced<ES: IntoElements>() {}
