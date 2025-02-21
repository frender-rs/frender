use std::{marker::PhantomData, pin::Pin, task::Poll};

use frender_html::{
    csr::experimental::{
        self, PinnedRenderStateKind, PinnedRenderStateKindPollRender, PinnedUiHandleOfKind,
        UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender,
    },
    CsrElement, HtmlRenderContext, RenderHtml, RenderStateKind, StateUnmount,
};
use hooks_core::{HookPollNextUpdate, HookUnmount};
use pin_project_lite::pin_project;

use super::{HookElement, UseHookData};

// region: ReactiveState

pin_project!(
    #[project = ReactiveStateProj]
    #[derive(Default)]
    pub struct ReactiveState<S, HookData, F> {
        #[pin]
        reactive_state: S,
        #[pin]
        hook_data: HookData,
        f: F,
        update_times: UpdateTimes,
    }
);

#[cfg(not(debug_assertions))]
type UpdateTimes = ();
#[cfg(debug_assertions)]
type UpdateTimes = u8;
#[cfg(debug_assertions)]
fn increment_update_times(
    update_times: &mut UpdateTimes,
    renderer: &mut (impl ?Sized + frender_html::RenderHtml),
    f: &'static str,
) {
    const MAX_MINUS_1: UpdateTimes = UpdateTimes::MAX - 1;
    match *update_times {
        MAX_MINUS_1 => {
            *update_times = UpdateTimes::MAX;
            // TODO: warn instead of log
            renderer.log(&format!(
            r##"WARNING: HookElementUpdateStateTooManyTimes.
The hook used by `{}` has been emitting next update too many times in one time of calling poll_render(),
which means, repeatedly, calling `poll_next_update()` returns `Poll::Ready(true)` even after the hook has been updated.
As a result, the returned element has been updating its state that many times.
This might be caused by bugs of hooks and frender-hook-element."##,
            f,
        ));
        }
        UpdateTimes::MAX => {
            // already warned
        }
        _ => *update_times += 1,
    }
}

impl<S: StateUnmount, HookData: HookUnmount, F> StateUnmount for ReactiveState<S, HookData, F> {
    fn state_unmount(self: Pin<&mut Self>) {
        let this = self.project();

        this.hook_data.unmount();
        this.reactive_state.state_unmount();
        #[cfg(debug_assertions)]
        {
            *this.update_times = 0;
        }
    }
}

// endregion
// region: kind

enum Never {}
pub struct Kind<EK, HookData: Default + HookUnmount, F>(Never, PhantomData<(EK, HookData, F)>);

impl<EK: UnpinnedRenderStateKind, HookData: Default + HookUnmount, F> UnpinnedRenderStateKind
    for Kind<EK, HookData, F>
where
    // Note
    HookData: Unpin,
{
    type UnpinnedUiHandle<R: frender_html::RenderHtml + ?Sized> = EK::UnpinnedUiHandle<R>;
    type UnpinnedState<R: frender_html::RenderHtml + ?Sized> =
        ReactiveState<EK::UnpinnedState<R>, HookData, F>;
}

impl<EK: UnpinnedRenderStateKindPollRender, HookData: Default + HookUnmount, F>
    UnpinnedRenderStateKindPollRender for Kind<EK, HookData, F>
where
    HookData: HookPollNextUpdate,
    // Note
    HookData: Unpin,
    // Note: F must output CsrElement of same kind.
    F: for<'hook> UseHookData<HookData = HookData, Value<'hook>: CsrElement<RenderStateKind = EK>>,
{
    fn unpinned_poll_render<R: frender_html::RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        ReactiveState {
            reactive_state,
            hook_data,
            f,
            update_times,
        }: &mut Self::UnpinnedState<R>,
        ui_handle: &mut Self::UnpinnedUiHandle<R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        match Pin::new(&mut *hook_data).poll_next_update(cx) {
            Poll::Ready(true) => {
                // HookData has a new value
                let new_element = f.use_hook_data(Pin::new(hook_data));

                // So we use the new value (CsrElement) to update the states
                new_element.unpinned_render_update(renderer, reactive_state, ui_handle);

                // Then, we re-check if HookData still emits new value
                match Pin::new(&mut *hook_data).poll_next_update(cx) {
                    // HookData still has a new value!
                    Poll::Ready(true) => {
                        // Increment update times
                        #[cfg(debug_assertions)]
                        increment_update_times(update_times, renderer, std::any::type_name::<F>());

                        // Let next poll decide what to do
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    }
                    // HookData is no longer reactive
                    Poll::Ready(false) => {
                        EK::unpinned_poll_render(renderer, reactive_state, ui_handle, cx)
                    }
                    // HookData is pending
                    Poll::Pending => {
                        _ = EK::unpinned_poll_render(renderer, reactive_state, ui_handle, cx);
                        Poll::Pending
                    }
                }
            }
            // HookData is no longer reactive
            Poll::Ready(false) => EK::unpinned_poll_render(renderer, reactive_state, ui_handle, cx),
            Poll::Pending => {
                _ = EK::unpinned_poll_render(renderer, reactive_state, ui_handle, cx);
                Poll::Pending
            }
        }
    }
}

impl<EK: PinnedRenderStateKind, HookData: Default + HookUnmount, F> PinnedRenderStateKind
    for Kind<EK, HookData, F>
{
    type PinnedUiHandle<R: frender_html::RenderHtml + ?Sized> = EK::PinnedUiHandle<R>;
    type PinnedState<R: frender_html::RenderHtml + ?Sized> =
        ReactiveState<Option<EK::PinnedState<R>>, HookData, F>;
}

impl<EK: PinnedRenderStateKindPollRender, HookData: Default + HookUnmount, F>
    PinnedRenderStateKindPollRender for Kind<EK, HookData, F>
where
    HookData: HookPollNextUpdate,
    // Note: F must output CsrElement of same kind.
    F: for<'hook> UseHookData<HookData = HookData, Value<'hook>: CsrElement<RenderStateKind = EK>>,
{
    fn pinned_poll_render<R: frender_html::RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        state: Pin<&mut Self::PinnedState<R>>,
        ui_handle: &mut Self::PinnedUiHandle<R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        let ReactiveStateProj {
            reactive_state,
            mut hook_data,
            f,
            update_times,
        } = state.project();
        let mut reactive_state = reactive_state.as_pin_mut().unwrap();

        match hook_data.as_mut().poll_next_update(cx) {
            Poll::Ready(true) => {
                // HookData has a new value
                let new_element = f.use_hook_data(hook_data.as_mut());

                // So we use the new value (CsrElement) to update the states
                new_element.pinned_render_update(renderer, reactive_state.as_mut(), ui_handle);

                // Then, we re-check if HookData still emits new value
                match hook_data.poll_next_update(cx) {
                    // HookData still has a new value!
                    Poll::Ready(true) => {
                        // Increment update times
                        #[cfg(debug_assertions)]
                        increment_update_times(update_times, renderer, std::any::type_name::<F>());

                        // Let next poll decide what to do
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    }
                    // HookData is no longer reactive
                    Poll::Ready(false) => {
                        EK::pinned_poll_render(renderer, reactive_state, ui_handle, cx)
                    }
                    // HookData is pending
                    Poll::Pending => {
                        _ = EK::pinned_poll_render(renderer, reactive_state, ui_handle, cx);
                        Poll::Pending
                    }
                }
            }
            // HookData is no longer reactive
            Poll::Ready(false) => EK::pinned_poll_render(renderer, reactive_state, ui_handle, cx),
            Poll::Pending => {
                _ = EK::pinned_poll_render(renderer, reactive_state, ui_handle, cx);
                Poll::Pending
            }
        }
    }
}

// endregion

pub struct RenderInit;

impl<
        EK: RenderStateKind,
        HookData: Default + HookUnmount + HookPollNextUpdate + Unpin,
        F: for<'hook> UseHookData<HookData = HookData, Value<'hook>: CsrElement<RenderStateKind = EK>>,
        Ctx: ?Sized + HtmlRenderContext,
    >
    experimental::RenderInitPinned<
        //
        &mut Ctx,
        ReactiveState<Option<EK::PinnedState<Ctx::Renderer>>, HookData, F>,
    > for RenderInit
{
    type Output = EK::PinnedUiHandle<Ctx::Renderer>;
    fn render_init_pinned(
        self,
        render_context: &mut Ctx,
        state: Pin<&mut ReactiveState<Option<EK::PinnedState<Ctx::Renderer>>, HookData, F>>,
    ) -> Self::Output {
        let ReactiveStateProj {
            mut reactive_state,
            hook_data,
            f,
            update_times: _,
        } = state.project();
        let element = F::use_hook_data(f, hook_data);
        let (state, init) = element.pinned_render_init(render_context.renderer_mut());
        reactive_state.set(Some(state));
        let reactive_state = reactive_state.as_pin_mut().unwrap();

        render_context.map_mut_render_context(|render_context| {
            init.render_init_pinned(render_context, reactive_state)
        })
    }
}

// Current implementation calls f.use_hook_data(_) when render_init() and render_update().
// This might be postponed in poll_render().
impl<EK, HookData, F> CsrElement for HookElement<F>
where
    EK: RenderStateKind,
    HookData: Default + HookUnmount + HookPollNextUpdate,
    // Note: HookData must be Unpin so that it works in the unpinned version.
    HookData: Unpin,
    // Note: F must output CsrElement of same kind.
    F: for<'hook> UseHookData<HookData = HookData, Value<'hook>: CsrElement<RenderStateKind = EK>>,
{
    type RenderStateKind = Kind<EK, HookData, F>;
    type PinnedRenderInit<R: ?Sized + frender_html::RenderHtml> = RenderInit;

    fn pinned_render_init<Renderer: ?Sized + RenderHtml>(
        //
        self,
        _: &mut Renderer,
    ) -> (
        //
        experimental::PinnedStateOfKind<Renderer, Self::RenderStateKind>,
        Self::PinnedRenderInit<Renderer>,
    ) {
        let Self(f) = self;

        (
            ReactiveState {
                reactive_state: None,
                hook_data: HookData::default(),
                f,
                update_times: UpdateTimes::default(),
            },
            RenderInit,
        )
    }

    fn pinned_render_init_by_reusing<Ctx: ?Sized + HtmlRenderContext>(
        self,
        render_context: &mut Ctx,
        reused_state: Pin<
            &mut experimental::PinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        >,
        unmounted_ui_handle: experimental::PinnedUnmountedUiHandleOfKind<
            Ctx::Renderer,
            Self::RenderStateKind,
        >,
    ) -> PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        let ReactiveStateProj {
            reactive_state,
            hook_data,
            f,
            update_times: _, // already reset when state_unmount
        } = reused_state.project();

        let reactive_state = reactive_state.as_pin_mut().unwrap();
        *f = self.0;

        let element = f.use_hook_data(hook_data);
        element.pinned_render_init_by_reusing(render_context, reactive_state, unmounted_ui_handle)
    }

    fn pinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        state: Pin<&mut experimental::PinnedStateOfKind<Renderer, Self::RenderStateKind>>,
        ui_handle: &mut PinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
        let ReactiveStateProj {
            reactive_state,
            hook_data,
            f,
            #[cfg(not(debug_assertions))]
                update_times: (),
            #[cfg(debug_assertions)]
            update_times,
        } = state.project();
        let reactive_state = reactive_state.as_pin_mut().unwrap();

        #[cfg(debug_assertions)]
        {
            *update_times = 0;
        }

        *f = self.0;

        // Immediately run f.use_hook_data(_)
        let element = f.use_hook_data(hook_data);
        element.pinned_render_update(renderer, reactive_state, ui_handle)
    }

    fn unpinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
    ) -> (
        //
        experimental::UnpinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        experimental::UnpinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
    ) {
        let Self(mut f) = self;
        let mut hook_data = HookData::default();
        let element = f.use_hook_data(Pin::new(&mut hook_data));
        let (reactive_state, ui_handle) = element.unpinned_render_init(render_context);

        (
            ReactiveState {
                reactive_state,
                hook_data,
                f,
                update_times: Default::default(),
            },
            ui_handle,
        )
    }

    fn unpinned_render_init_by_reusing<Ctx: ?Sized + HtmlRenderContext>(
        self,
        render_context: &mut Ctx,
        ReactiveState {
            reactive_state,
            hook_data,
            f,
            update_times: _, // already reset when state_unmount
        }: &mut experimental::UnpinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        unmounted_ui_handle: experimental::UnpinnedUnmountedUiHandleOfKind<
            Ctx::Renderer,
            Self::RenderStateKind,
        >,
    ) -> experimental::UnpinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        *f = self.0;
        let element = f.use_hook_data(Pin::new(hook_data));
        element.unpinned_render_init_by_reusing(render_context, reactive_state, unmounted_ui_handle)
    }

    fn unpinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        ReactiveState {
            reactive_state,
            hook_data,
            f,
            #[cfg(not(debug_assertions))]
                update_times: (),
            #[cfg(debug_assertions)]
            update_times,
        }: &mut experimental::UnpinnedStateOfKind<Renderer, Self::RenderStateKind>,
        ui_handle: &mut experimental::UnpinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
        #[cfg(debug_assertions)]
        {
            *update_times = 0;
        }
        *f = self.0;
        // Immediately run f.use_hook_data(_)
        let element = f.use_hook_data(Pin::new(hook_data));
        element.unpinned_render_update(renderer, reactive_state, ui_handle)
    }
}
