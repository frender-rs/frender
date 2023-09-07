use std::{marker::PhantomData, pin::Pin};

use frender_html::{RenderHtml, RenderState};
use lazy_pinned::LazyPinned;

use crate::fn_wrapper;

pub struct Rendered<S>(pub(crate) PhantomData<S>);

pub struct RenderContext<'a, R, State> {
    renderer: &'a mut R,
    state: Pin<&'a mut LazyPinned<State>>,
}

pub trait UseRender<HookData> {
    type RenderState: RenderState;

    fn use_render<R: RenderHtml>(
        &mut self,
        hook_data: Pin<&mut HookData>,
        cs: RenderContext<'_, R, Self::RenderState>,
    ) -> Rendered<Self::RenderState>;
}

impl<HookData, U, S> UseRender<HookData> for fn_wrapper::FnMutWithRenderContext<U, S>
where
    S: RenderState,
    U: for<'a> FnMut(Pin<&'a mut HookData>, RenderContext<'a, S>) -> Rendered<S>,
{
    type RenderState = S;

    #[inline]
    fn use_render<'a, 'ctx>(
        &mut self,
        inner_hook: Pin<&'a mut HookData>,
        cs: CsrRenderContext<'a, 'ctx, S>,
    ) -> Rendered<S> {
        (self.0)(inner_hook, cs)
    }
}

impl<InnerHook, U, E: Element> UseRender<InnerHook> for fn_wrapper::FnMutOutputElement<U>
where
    U: FnMut(Pin<&mut InnerHook>) -> E,
{
    type RenderState = <E as Element>::CsrState;

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
