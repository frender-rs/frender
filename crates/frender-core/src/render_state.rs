use std::{pin::Pin, task::Poll};

use crate::utils::pin_as_deref_mut;

pub trait RenderState {
    fn unmount(self: Pin<&mut Self>);

    #[inline]
    fn poll_reactive(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<bool> {
        let _ = cx;
        Poll::Ready(false)
    }
}

impl<S: RenderState + ?Sized> RenderState for Pin<Box<S>> {
    #[inline]
    fn unmount(self: Pin<&mut Self>) {
        S::unmount(pin_as_deref_mut(self))
    }

    #[inline]
    fn poll_reactive(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<bool> {
        S::poll_reactive(pin_as_deref_mut(self), cx)
    }
}

pub trait AnyRenderState: std::any::Any + RenderState {
    fn pin_as_mut_any(self: Pin<&mut Self>) -> Pin<&mut dyn std::any::Any>;
}

impl<E> AnyRenderState for E
where
    E: std::any::Any + RenderState,
{
    fn pin_as_mut_any(self: Pin<&mut Self>) -> Pin<&mut dyn std::any::Any> {
        self
    }
}

pub fn join_poll_reactive_results(a: Poll<bool>, b: Poll<bool>) -> Poll<bool> {
    match (a, b) {
        (Poll::Ready(false), Poll::Ready(false)) => Poll::Ready(false),
        (Poll::Ready(false) | Poll::Pending, Poll::Ready(false) | Poll::Pending) => Poll::Pending,
        _ => Poll::Ready(true),
    }
}
