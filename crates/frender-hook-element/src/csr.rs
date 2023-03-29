use std::{marker::PhantomData, pin::Pin};

use frender_csr::{CsrContext, Element, RenderState};
use lazy_pinned::LazyPinned;

use crate::fn_wrapper;

pub struct Rendered<S>(PhantomData<S>);

pub struct CsrRenderContext<'a, State> {
    context: &'a mut CsrContext,
    state: Pin<&'a mut LazyPinned<State>>,
}

impl<'a, State> CsrRenderContext<'a, State> {
    pub fn new(context: &'a mut CsrContext, state: Pin<&'a mut LazyPinned<State>>) -> Self {
        Self { context, state }
    }
}

impl<'a, S: RenderState> CsrRenderContext<'a, S> {
    pub fn render<E: Element<CsrState = S>>(self, element: E) -> Rendered<S> {
        self.state.use_pin_or_insert_with_data(
            (element, self.context),
            |(element, context), state| element.update_csr_state(context, state),
            |(element, context)| element.into_csr_state(context),
        );

        Rendered(PhantomData)
    }
}

pub trait UseCsr<InnerHook> {
    type CsrState: RenderState;

    fn use_render<'a>(
        &'a mut self,
        inner_hook: Pin<&'a mut InnerHook>,
        cs: CsrRenderContext<'a, Self::CsrState>,
    ) -> Rendered<Self::CsrState>;
}

impl<InnerHook, U, S> UseCsr<InnerHook> for fn_wrapper::FnMutWithRenderContext<U, S>
where
    S: RenderState,
    U: for<'a> FnMut(Pin<&'a mut InnerHook>, CsrRenderContext<'a, S>) -> Rendered<S>,
{
    type CsrState = S;

    #[inline]
    fn use_render<'a>(
        &'a mut self,
        inner_hook: Pin<&'a mut InnerHook>,
        cs: CsrRenderContext<'a, S>,
    ) -> Rendered<S> {
        (self.0)(inner_hook, cs)
    }
}

impl<InnerHook, U, E: Element> UseCsr<InnerHook> for fn_wrapper::FnMutOutputElement<U>
where
    U: FnMut(Pin<&mut InnerHook>) -> E,
{
    type CsrState = <E as Element>::CsrState;

    #[inline]
    fn use_render<'a>(
        &'a mut self,
        inner_hook: Pin<&'a mut InnerHook>,
        cs: CsrRenderContext<'a, <E as Element>::CsrState>,
    ) -> Rendered<<E as Element>::CsrState> {
        let element = (self.0)(inner_hook);
        cs.render(element)
    }
}
