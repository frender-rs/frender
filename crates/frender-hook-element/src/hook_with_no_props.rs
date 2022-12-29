use std::{any::Any, pin::Pin};

use hooks::{Hook, HookPollNextUpdate, LazyPinnedHook};

use frender_core::RenderState;

use crate::ContextAndState;

#[derive(Clone, Copy, Debug)]
pub struct HookElementWithNoProps<HDom, HSsr> {
    pub with_dom: HDom,
    pub with_ssr: HSsr,
}

pin_project_lite::pin_project! {
    pub struct HookStateWithNoProps<H: HookPollNextUpdate, Ctx, S> {
        #[pin]
        pub hook: LazyPinnedHook<H>,
        #[pin]
        pub render_state: S,
        pub ctx: Option<Ctx>,
    }
}

impl<H, Ctx: crate::HookContext, S: RenderState + 'static> RenderState
    for HookStateWithNoProps<H, Ctx, S>
where
    H: for<'a> Hook<(ContextAndState<'a, Ctx, dyn Any>,), Value = ContextAndState<'a, Ctx, S>>,
{
    fn new_uninitialized() -> Self {
        Self {
            hook: Default::default(),
            render_state: S::new_uninitialized(),
            ctx: None,
        }
    }

    fn unmount(self: Pin<&mut Self>) {
        self.project().render_state.unmount()
    }

    fn poll_reactive(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<bool> {
        let mut this = self.project();

        let res = this.hook.as_mut().poll_next_update(cx);
        let r = this.render_state.as_mut().poll_reactive(cx);

        match (res, r) {
            (std::task::Poll::Ready(false), std::task::Poll::Ready(false)) => {
                std::task::Poll::Ready(false)
            }
            (
                std::task::Poll::Ready(false) | std::task::Poll::Pending,
                std::task::Poll::Ready(false) | std::task::Poll::Pending,
            ) => std::task::Poll::Pending,
            _ => {
                if let (Some(hook), Some(context)) =
                    (this.hook.pin_project_hook(), this.ctx.as_mut())
                {
                    Ctx::with_context(context, |context| {
                        hook.use_hook((ContextAndState::new(context, this.render_state),));
                    });
                    cx.waker().wake_by_ref();
                    std::task::Poll::Pending
                } else {
                    std::task::Poll::Ready(true)
                }
            }
        }
    }
}

pub trait FnOnceOutputElementHookWithNoProps<Ctx>: FnOnce() -> Self::Hook {
    type RenderState;
    type Hook: for<'a> Hook<
        (ContextAndState<'a, Ctx, dyn Any>,),
        Value = ContextAndState<'a, Ctx, Self::RenderState>,
    >;
}

impl<F, H, Ctx, S> FnOnceOutputElementHookWithNoProps<Ctx> for F
where
    F: FnOnce() -> H,
    H: for<'a> Hook<(ContextAndState<'a, Ctx, dyn Any>,), Value = ContextAndState<'a, Ctx, S>>,
{
    type RenderState = S;
    type Hook = H;
}
