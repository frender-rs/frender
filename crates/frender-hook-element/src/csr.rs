use std::{marker::PhantomData, pin::Pin};

use frender_csr::{CsrContext, Element, RenderState};
use lazy_pinned::LazyPinned;

use crate::fn_wrapper;

pub struct Rendered<S>(PhantomData<S>);

pub struct CsrRenderContext<'a, 'ctx, State> {
    context: &'a mut CsrContext<'ctx>,
    state: Pin<&'a mut LazyPinned<State>>,
    pub(crate) force_reposition: bool,
}

impl<'a, 'ctx, State> CsrRenderContext<'a, 'ctx, State> {
    pub fn new(context: &'a mut CsrContext<'ctx>, state: Pin<&'a mut LazyPinned<State>>) -> Self {
        Self::_new(context, state, false)
    }

    pub(crate) fn _new(
        context: &'a mut CsrContext<'ctx>,
        state: Pin<&'a mut LazyPinned<State>>,
        force_reposition: bool,
    ) -> Self {
        Self {
            context,
            state,
            force_reposition,
        }
    }
}

impl<'a, 'ctx, S: RenderState> CsrRenderContext<'a, 'ctx, S> {
    pub fn render<E: Element<CsrState = S>>(self, element: E) -> Rendered<S> {
        self.state.use_pin_or_insert_with_data(
            (element, self.context),
            |(element, context), state| {
                element.update_csr_state_maybe_reposition(context, state, self.force_reposition)
            },
            |(element, context)| element.into_csr_state(context),
        );

        Rendered(PhantomData)
    }
}

pub trait UseCsr<InnerHook> {
    type CsrState: RenderState;

    fn use_render<'a, 'ctx>(
        &mut self,
        inner_hook: Pin<&'a mut InnerHook>,
        cs: CsrRenderContext<'a, 'ctx, Self::CsrState>,
    ) -> Rendered<Self::CsrState>;
}

impl<InnerHook, U, S> UseCsr<InnerHook> for fn_wrapper::FnMutWithRenderContext<U, S>
where
    S: RenderState,
    U: for<'a, 'ctx> FnMut(Pin<&'a mut InnerHook>, CsrRenderContext<'a, 'ctx, S>) -> Rendered<S>,
{
    type CsrState = S;

    #[inline]
    fn use_render<'a, 'ctx>(
        &mut self,
        inner_hook: Pin<&'a mut InnerHook>,
        cs: CsrRenderContext<'a, 'ctx, S>,
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
    fn use_render<'a, 'ctx>(
        &mut self,
        inner_hook: Pin<&'a mut InnerHook>,
        cs: CsrRenderContext<'a, 'ctx, <E as Element>::CsrState>,
    ) -> Rendered<<E as Element>::CsrState> {
        let element = (self.0)(inner_hook);
        cs.render(element)
    }
}
