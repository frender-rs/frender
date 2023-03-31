use std::{pin::Pin, task::Poll};

use frender_common::utils::pin_as_deref_mut;

pub trait RenderState {
    fn unmount(self: Pin<&mut Self>);

    fn poll_csr(
        self: Pin<&mut Self>,
        ctx: &mut crate::CsrContext,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()>;
}

impl<P> RenderState for Pin<P>
where
    P: std::ops::DerefMut,
    P::Target: RenderState,
{
    #[inline]
    fn unmount(self: Pin<&mut Self>) {
        P::Target::unmount(pin_as_deref_mut(self))
    }

    #[inline]
    fn poll_csr(
        self: Pin<&mut Self>,
        ctx: &mut crate::CsrContext,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        P::Target::poll_csr(pin_as_deref_mut(self), ctx, cx)
    }
}

pub trait AnyRenderState<Ctx>: std::any::Any + RenderState {
    fn pin_as_mut_any(self: Pin<&mut Self>) -> Pin<&mut dyn std::any::Any>;
}

impl<E, Ctx> AnyRenderState<Ctx> for E
where
    E: std::any::Any + RenderState,
{
    fn pin_as_mut_any(self: Pin<&mut Self>) -> Pin<&mut dyn std::any::Any> {
        self
    }
}

// TODO: remove
pub fn join_poll_reactive_results(a: Poll<()>, b: Poll<()>) -> Poll<()> {
    match (a, b) {
        (Poll::Ready(()), Poll::Ready(())) => Poll::Ready(()),
        _ => Poll::Pending,
    }
}
