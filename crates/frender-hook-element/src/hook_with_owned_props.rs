use std::any::Any;

use hooks::Hook;

use crate::ContextAndState;

#[derive(Clone, Copy, Debug)]
pub struct HookElementWithOwnedProps<HDom, HSsr, Props> {
    pub with_dom: HDom,
    pub with_ssr: HSsr,
    pub props: Props,
}

pub trait FnOnceOutputElementHookWithOwnedProps<Ctx, Props>: FnOnce() -> Self::Hook {
    type RenderState;
    type Hook: for<'a> Hook<
        (ContextAndState<'a, Ctx, dyn Any>, Props),
        Value = ContextAndState<'a, Ctx, Self::RenderState>,
    >;
}

impl<F, H, Ctx, S, Props> FnOnceOutputElementHookWithOwnedProps<Ctx, Props> for F
where
    F: FnOnce() -> H,
    H: for<'a> Hook<
        (ContextAndState<'a, Ctx, dyn Any>, Props),
        Value = ContextAndState<'a, Ctx, S>,
    >,
{
    type RenderState = S;
    type Hook = H;
}
