use hooks_core::{HookPollNextUpdate, HookUnmount};
use lazy_pinned::LazyPinned;

pin_project_lite::pin_project!(
    pub struct FnMutHookElementState<HookData, U, S, C = ()>
    where
        HookData: HookPollNextUpdate,
        HookData: HookUnmount,
    {
        #[pin]
        hook_data: HookData,
        #[pin]
        state: LazyPinned<S>,
        use_hook: U,
        render_iteration_count: C,
    }
);

impl<InnerHook: HookPollNextUpdate + HookUnmount, U, S, C> FnMutHookElementState<InnerHook, U, S, C>
where
    InnerHook: Default,
    C: Default,
{
    pub(crate) fn _new(use_hook: U) -> Self {
        Self {
            hook_data: Default::default(),
            state: LazyPinned(None),
            use_hook,
            render_iteration_count: Default::default(),
        }
    }
}

mod prelude_names {
    pub(super) use std::pin::Pin;

    pub(super) use hooks_core::{HookPollNextUpdate, HookUnmount};
    pub(super) use lazy_pinned::LazyPinned;

    pub(super) use super::FnMutHookElementState;
}

#[cfg(feature = "ssr")]
mod impl_ssr {
    use super::prelude_names::*;
    use crate::ssr;

    #[cfg(feature = "ssr")]
    impl<HookData: HookPollNextUpdate + HookUnmount + Default, U> frender_ssr::RenderState
        for FnMutHookElementState<HookData, U, U::SsrState>
    where
        U: ssr::UseSsr<HookData>,
    {
        fn poll_render<W: frender_ssr::AsyncWrite + ?Sized>(
            self: Pin<&mut Self>,
            writer: Pin<&mut W>,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<std::io::Result<()>> {
            let this = self.project();
            let cs = ssr::SsrRenderContext::new(this.state);
            let rendered: ssr::Rendered<U::SsrState> = this.use_hook.use_ssr(this.hook_data, cs);
            let state = rendered.into_state();

            state.poll_render(writer, cx)
        }
    }
}

#[cfg(feature = "csr")]
mod impl_csr {
    use std::task::Poll;

    use super::prelude_names::*;
    use crate::csr;

    use frender_csr::{CsrContext, RenderState};

    impl<InnerHook: HookPollNextUpdate + HookUnmount, U, S> FnMutHookElementState<InnerHook, U, S, u8>
    where
        InnerHook: Default,
    {
        pub(crate) fn csr_update(
            self: Pin<&mut Self>,
            mut use_hook: U,
            ctx: &mut frender_csr::CsrContext,
            force_reposition: bool,
        ) where
            U: csr::UseCsr<InnerHook, CsrState = S>,
        {
            let state = self.project();

            let mut ctx = ctx.as_borrowed();
            let cs = csr::CsrRenderContext::_new(&mut ctx, state.state, force_reposition);

            let _: csr::Rendered<U::CsrState> = use_hook.use_render(state.hook_data, cs);
            *state.render_iteration_count = 0;
            *state.use_hook = use_hook;
        }
    }

    impl<HookData: HookPollNextUpdate + HookUnmount, U> RenderState
        for FnMutHookElementState<HookData, U, U::CsrState, u8>
    where
        U: csr::UseCsr<HookData>,
    {
        #[inline]
        fn unmount(self: Pin<&mut Self>) {
            let this = self.project();
            this.hook_data.unmount();
            if let Some(state) = this.state.as_pin_mut() {
                state.unmount();
            }
        }

        #[inline]
        fn state_unmount(self: std::pin::Pin<&mut Self>) {
            let this = self.project();
            this.hook_data.unmount();
            if let Some(state) = this.state.as_pin_mut() {
                state.state_unmount();
            }
        }

        fn poll_csr(
            self: Pin<&mut Self>,
            ctx: &mut CsrContext,
            cx: &mut std::task::Context<'_>,
        ) -> Poll<()> {
            let mut this = self.project();

            loop {
                let a = this.hook_data.as_mut().poll_next_update(cx);
                let b = this.state.as_mut().as_pin_mut().map(|state| {
                    <U::CsrState as RenderState>::poll_csr(state, &mut ctx.as_borrowed(), cx)
                });

                match (a, b) {
                    (Poll::Ready(false), Some(Poll::Ready(()))) => return Poll::Ready(()),
                    (Poll::Ready(true), _) | (_, None) => {
                        let _: csr::Rendered<U::CsrState> = this.use_hook.use_render(
                            this.hook_data.as_mut(),
                            csr::CsrRenderContext::new(&mut ctx.as_borrowed(), this.state.as_mut()),
                        );

                        if *this.render_iteration_count == u8::MAX {
                            *this.render_iteration_count = 0;
                            cx.waker().wake_by_ref();
                            return Poll::Pending;
                        } else {
                            *this.render_iteration_count += 1;
                        }
                    }
                    _ => return Poll::Pending,
                }
            }
        }
    }
}
