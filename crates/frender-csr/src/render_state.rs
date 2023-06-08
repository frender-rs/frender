use std::{pin::Pin, task::Poll};

use frender_common::utils::pin_as_deref_mut;

pub use CsrRenderState as RenderState;

pub trait CsrRenderState {
    fn unmount(self: Pin<&mut Self>);

    fn state_unmount(self: Pin<&mut Self>);

    fn poll_csr(
        self: Pin<&mut Self>,
        ctx: &mut crate::CsrContext,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()>;
}

impl<S: ?Sized + RenderState + Unpin> RenderState for &mut S {
    fn unmount(self: Pin<&mut Self>) {
        S::unmount(Pin::new(self.get_mut()))
    }

    fn state_unmount(self: Pin<&mut Self>) {
        S::state_unmount(Pin::new(self.get_mut()))
    }

    fn poll_csr(
        self: Pin<&mut Self>,
        ctx: &mut crate::CsrContext,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        S::poll_csr(Pin::new(self.get_mut()), ctx, cx)
    }
}

impl<S: ?Sized + RenderState + Unpin> RenderState for Box<S> {
    fn unmount(self: Pin<&mut Self>) {
        S::unmount(Pin::new(self.get_mut()))
    }

    fn state_unmount(self: Pin<&mut Self>) {
        S::state_unmount(Pin::new(self.get_mut()))
    }

    fn poll_csr(
        self: Pin<&mut Self>,
        ctx: &mut crate::CsrContext,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        S::poll_csr(Pin::new(self.get_mut()), ctx, cx)
    }
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
    fn state_unmount(self: Pin<&mut Self>) {
        P::Target::state_unmount(pin_as_deref_mut(self))
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
