use std::pin::Pin;

use frender_ssr::{Element, RenderState};
use lazy_pinned::LazyPinned;

use crate::fn_wrapper;

pub struct Rendered<'a, State> {
    state: Pin<&'a mut State>,
}

impl<'a, State> Rendered<'a, State> {
    pub fn into_state(self) -> Pin<&'a mut State> {
        self.state
    }
}

pub struct SsrRenderContext<'a, State> {
    state: Pin<&'a mut LazyPinned<State>>,
}

impl<'a, State> SsrRenderContext<'a, State> {
    pub fn new(state: Pin<&'a mut LazyPinned<State>>) -> Self {
        Self { state }
    }

    fn render<E: Element<SsrState = State>>(self, element: E) -> Rendered<'a, State> {
        Rendered {
            state: self
                .state
                .pin_project_or_insert_with(|| element.into_ssr_state()),
        }
    }
}

pub trait UseSsr<InnerHook> {
    type SsrState: RenderState;

    fn use_ssr<'cs>(
        &mut self,
        inner_hook: Pin<&mut InnerHook>,
        cs: SsrRenderContext<'cs, Self::SsrState>,
    ) -> Rendered<'cs, Self::SsrState>;
}

impl<InnerHook, U, S> UseSsr<InnerHook> for fn_wrapper::FnMutWithRenderContext<U, S>
where
    S: RenderState,
    U: for<'cs> FnMut(Pin<&mut InnerHook>, SsrRenderContext<'cs, S>) -> Rendered<'cs, S>,
{
    type SsrState = S;

    fn use_ssr<'cs>(
        &mut self,
        inner_hook: Pin<&mut InnerHook>,
        cs: SsrRenderContext<'cs, Self::SsrState>,
    ) -> Rendered<'cs, Self::SsrState> {
        (self.0)(inner_hook, cs)
    }
}

impl<InnerHook, U, E: Element> UseSsr<InnerHook> for fn_wrapper::FnMutOutputElement<U>
where
    U: FnMut(Pin<&mut InnerHook>) -> E,
{
    type SsrState = E::SsrState;

    fn use_ssr<'cs>(
        &mut self,
        inner_hook: Pin<&mut InnerHook>,
        cs: SsrRenderContext<'cs, Self::SsrState>,
    ) -> Rendered<'cs, Self::SsrState> {
        let element = (self.0)(inner_hook);
        cs.render(element)
    }
}
