use std::{pin::Pin, task::Poll};

use frender_csr::RenderState;

pub trait RenderStateWithParentElementsHandle<PEH: ?Sized, R: ?Sized> {
    fn unmount_with_peh(self: Pin<&mut Self>, peh: &mut PEH, renderer: &mut R);

    fn state_unmount_with_peh(self: Pin<&mut Self>, peh: &mut PEH);

    /// Implementation shouldn't change cursor
    fn poll_render_with_peh(
        self: Pin<&mut Self>,
        peh: &mut PEH,
        renderer: &mut R,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()>;
}

#[macro_export]
macro_rules! proxy_render_state_with_peh {
    (|$_self:ident| -> RenderState<$PEH:ty, $R:ty> { $this:expr }) => {
        fn unmount_with_peh($_self: ::core::pin::Pin<&mut Self>, _: &mut $PEH, renderer: &mut $R) {
            $crate::RenderState::unmount($this, renderer)
        }

        fn state_unmount_with_peh($_self: ::core::pin::Pin<&mut Self>, _: &mut $PEH) {
            $crate::RenderState::state_unmount($this)
        }

        fn poll_render_with_peh(
            $_self: ::core::pin::Pin<&mut Self>,
            _: &mut $PEH,
            renderer: &mut $R,
            cx: &mut ::core::task::Context<'_>,
        ) -> ::core::task::Poll<()> {
            $crate::RenderState::poll_render($this, renderer, cx)
        }
    };
    (|$_self:ident| -> ($PEH:ty, $R:ty) { $this:expr }) => {
        fn unmount_with_peh(
            $_self: ::core::pin::Pin<&mut Self>,
            peh: &mut $PEH,
            renderer: &mut $R,
        ) {
            $crate::RenderStateWithParentElementsHandle::unmount_with_peh($this, peh, renderer)
        }

        fn state_unmount_with_peh($_self: ::core::pin::Pin<&mut Self>, peh: &mut $PEH) {
            $crate::RenderStateWithParentElementsHandle::state_unmount_with_peh($this, peh)
        }

        fn poll_render_with_peh(
            $_self: ::core::pin::Pin<&mut Self>,
            peh: &mut $PEH,
            renderer: &mut $R,
            cx: &mut ::core::task::Context<'_>,
        ) -> ::core::task::Poll<()> {
            $crate::RenderStateWithParentElementsHandle::poll_render_with_peh(
                $this, peh, renderer, cx,
            )
        }
    };
}

pin_project_lite::pin_project!(
    #[derive(Default)]
    pub struct RenderStateWithAnyParent<T> {
        #[pin]
        pub render_state: T,
    }
);

impl<S> RenderStateWithAnyParent<S> {
    pub fn as_pin_mut(self: Pin<&mut Self>) -> Pin<&mut S> {
        self.project().render_state
    }
}

impl<PEH: ?Sized, R: ?Sized, S: RenderState<R>> RenderStateWithParentElementsHandle<PEH, R>
    for RenderStateWithAnyParent<S>
{
    proxy_render_state_with_peh!(|self| -> RenderState<PEH, R> { self.as_pin_mut() });
}

impl<PEH: ?Sized, R: ?Sized> RenderStateWithParentElementsHandle<PEH, R> for () {
    fn unmount_with_peh(self: Pin<&mut Self>, _: &mut PEH, _: &mut R) {}

    fn state_unmount_with_peh(self: Pin<&mut Self>, _: &mut PEH) {}

    fn poll_render_with_peh(
        self: Pin<&mut Self>,
        _: &mut PEH,
        _: &mut R,
        _: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        Poll::Ready(())
    }
}

impl<PEH: ?Sized, R: ?Sized, S> RenderStateWithParentElementsHandle<PEH, R>
    for frender_csr::render_state::non_reactive::NonReactiveRenderState<S>
{
    fn unmount_with_peh(self: Pin<&mut Self>, _: &mut PEH, _: &mut R) {}

    fn state_unmount_with_peh(self: Pin<&mut Self>, _: &mut PEH) {}

    fn poll_render_with_peh(
        self: Pin<&mut Self>,
        _: &mut PEH,
        _: &mut R,
        _: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        Poll::Ready(())
    }
}

impl<PEH: ?Sized, R: ?Sized, S: RenderStateWithParentElementsHandle<PEH, R>, T>
    RenderStateWithParentElementsHandle<PEH, R>
    for frender_csr::render_state::compound::CompoundState<S, T>
{
    proxy_render_state_with_peh!(|self| -> (PEH, R) { self.pin_project().reactive });
}

#[cfg(feature = "either")]
impl<
        PEH: ?Sized,
        R: ?Sized,
        A: RenderStateWithParentElementsHandle<PEH, R>,
        B: RenderStateWithParentElementsHandle<PEH, R>,
    > RenderStateWithParentElementsHandle<PEH, R>
    for frender_csr::render_state::either::EitherRenderState<A, B>
{
    fn unmount_with_peh(self: Pin<&mut Self>, peh: &mut PEH, renderer: &mut R) {
        either::for_both!(self.project_inner().as_pin_mut(), state => state.unmount_with_peh(peh, renderer))
    }

    fn state_unmount_with_peh(self: Pin<&mut Self>, peh: &mut PEH) {
        either::for_both!(self.project_inner().as_pin_mut(), state => state.state_unmount_with_peh(peh))
    }

    fn poll_render_with_peh(
        self: Pin<&mut Self>,
        peh: &mut PEH,
        renderer: &mut R,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        either::for_both!(self.project_inner().as_pin_mut(), state => state.poll_render_with_peh(peh, renderer, cx))
    }
}

mod tuple {
    use super::*;
    impl<
            PEH: ?Sized,
            R: ?Sized,
            S0: RenderStateWithParentElementsHandle<PEH, R>,
            S1: RenderStateWithParentElementsHandle<PEH, R>,
        > RenderStateWithParentElementsHandle<PEH, R> for (S0, S1)
    {
        fn unmount_with_peh(self: Pin<&mut Self>, peh: &mut PEH, renderer: &mut R) {
            let (s0, s1) = frender_common::utils::pin_project::tuple_2(self);
            s0.unmount_with_peh(peh, renderer);
            s1.unmount_with_peh(peh, renderer);
        }

        fn state_unmount_with_peh(self: Pin<&mut Self>, peh: &mut PEH) {
            let (s0, s1) = frender_common::utils::pin_project::tuple_2(self);
            s0.state_unmount_with_peh(peh);
            s1.state_unmount_with_peh(peh);
        }

        fn poll_render_with_peh(
            self: Pin<&mut Self>,
            peh: &mut PEH,
            renderer: &mut R,
            cx: &mut std::task::Context<'_>,
        ) -> Poll<()> {
            let (s0, s1) = frender_common::utils::pin_project::tuple_2(self);

            match (
                s0.poll_render_with_peh(peh, renderer, cx),
                s1.poll_render_with_peh(peh, renderer, cx),
            ) {
                (Poll::Ready(()), Poll::Ready(())) => Poll::Ready(()),
                _ => Poll::Pending,
            }
        }
    }
}

mod option {
    use super::*;

    impl<PEH: ?Sized, R: ?Sized, S: RenderStateWithParentElementsHandle<PEH, R>>
        RenderStateWithParentElementsHandle<PEH, R> for Option<S>
    {
        fn unmount_with_peh(mut self: Pin<&mut Self>, peh: &mut PEH, renderer: &mut R) {
            if let Some(this) = self.as_mut().as_pin_mut() {
                this.unmount_with_peh(peh, renderer)
            } else {
                return;
            }
            // TODO: is this needed?
            self.set(None)
        }

        fn state_unmount_with_peh(self: Pin<&mut Self>, peh: &mut PEH) {
            if let Some(this) = self.as_pin_mut() {
                this.state_unmount_with_peh(peh)
            }
        }

        fn poll_render_with_peh(
            self: Pin<&mut Self>,
            peh: &mut PEH,
            renderer: &mut R,
            cx: &mut std::task::Context<'_>,
        ) -> Poll<()> {
            if let Some(this) = self.as_pin_mut() {
                this.poll_render_with_peh(peh, renderer, cx)
            } else {
                Poll::Ready(())
            }
        }
    }
}
