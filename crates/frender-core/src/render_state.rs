use std::{pin::Pin, task::Poll};

use crate::utils::pin_as_deref_mut;

pub trait RenderState<Ctx> {
    fn unmount(self: Pin<&mut Self>);

    fn poll_reactive(
        self: Pin<&mut Self>,
        ctx: &mut Ctx,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()>;
}

impl<Ctx, S: RenderState<Ctx> + ?Sized> RenderState<Ctx> for Pin<Box<S>> {
    #[inline]
    fn unmount(self: Pin<&mut Self>) {
        S::unmount(pin_as_deref_mut(self))
    }

    #[inline]
    fn poll_reactive(
        self: Pin<&mut Self>,
        ctx: &mut Ctx,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        S::poll_reactive(pin_as_deref_mut(self), ctx, cx)
    }
}

pub trait AnyRenderState<Ctx>: std::any::Any + RenderState<Ctx> {
    fn pin_as_mut_any(self: Pin<&mut Self>) -> Pin<&mut dyn std::any::Any>;
}

impl<E, Ctx> AnyRenderState<Ctx> for E
where
    E: std::any::Any + RenderState<Ctx>,
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
