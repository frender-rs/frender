use std::{marker::PhantomData, pin::Pin, task::Poll};

use frender_html::{
    experimental::{
        PinMutRenderInitStates, PinMutRenderInitStatesOfKind, PinnedMutRenderStatesOfKind,
        PinnedRenderStateKind, PinnedRenderStateKindPollRender, PinnedUiHandleOfKind, RenderStates,
        UnpinnedMutRenderStatesOfKind, UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender,
        UnpinnedRenderStatesOfKind,
    },
    kinds::UiHandleWithNonReactiveState,
    ui_handles::CursorPlaceholdersSurrounded,
    CsrElement, RenderStateKind, StateUnmount,
};
use hooks_core::{HookPollNextUpdate, HookUnmount};
use pin_project_lite::pin_project;

use super::{HookElement, UseHookData};

// region: ReactiveState

pin_project!(
    #[project = ReactiveStateProj]
    #[derive(Default)]
    pub struct ReactiveState<S, HookData> {
        #[pin]
        reactive_state: S,
        #[pin]
        hook_data: HookData,
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

impl<S: StateUnmount, HookData: HookUnmount> StateUnmount for ReactiveState<S, HookData> {
    fn state_unmount(self: Pin<&mut Self>) {
        let this = self.project();

        this.hook_data.unmount();
        this.reactive_state.state_unmount();
        *this.update_times = 0;
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
    type UnpinnedUiHandle<R: frender_html::RenderHtml + ?Sized> =
        CursorPlaceholdersSurrounded<R::CursorPlaceholder, EK::UnpinnedUiHandle<R>>;
    type UnpinnedNonReactiveState<R: frender_html::RenderHtml + ?Sized> =
        (EK::UnpinnedNonReactiveState<R>, F);
    type UnpinnedReactiveState = ReactiveState<EK::UnpinnedReactiveState, HookData>;
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
        RenderStates {
            ui_handle,
            non_reactive_state: (non_reactive_state, f),
            reactive_state:
                ReactiveState {
                    reactive_state,
                    hook_data,
                    update_times,
                },
        }: UnpinnedMutRenderStatesOfKind<Self, R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        match Pin::new(&mut *hook_data).poll_next_update(cx) {
            Poll::Ready(true) => {
                // HookData has a new value
                let new_element = f.use_hook_data(Pin::new(hook_data));

                // So we use the new value (CsrElement) to update the states
                ui_handle.use_surrounded_render_context(renderer, |ui_handle, render_context| {
                    new_element.unpinned_render_update(
                        render_context,
                        RenderStates {
                            ui_handle,
                            non_reactive_state,
                            reactive_state,
                        },
                    );
                });

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
                    Poll::Ready(false) => EK::unpinned_poll_render(
                        renderer,
                        RenderStates {
                            ui_handle: ui_handle.surrounded_mut(),
                            non_reactive_state,
                            reactive_state,
                        },
                        cx,
                    ),
                    // HookData is pending
                    Poll::Pending => {
                        _ = EK::unpinned_poll_render(
                            renderer,
                            RenderStates {
                                ui_handle: ui_handle.surrounded_mut(),
                                non_reactive_state,
                                reactive_state,
                            },
                            cx,
                        );
                        Poll::Pending
                    }
                }
            }
            // HookData is no longer reactive
            Poll::Ready(false) => EK::unpinned_poll_render(
                renderer,
                RenderStates {
                    ui_handle: ui_handle.surrounded_mut(),
                    non_reactive_state,
                    reactive_state,
                },
                cx,
            ),
            Poll::Pending => {
                _ = EK::unpinned_poll_render(
                    renderer,
                    RenderStates {
                        ui_handle: ui_handle.surrounded_mut(),
                        non_reactive_state,
                        reactive_state,
                    },
                    cx,
                );
                Poll::Pending
            }
        }
    }
}

impl<EK: PinnedRenderStateKind, HookData: Default + HookUnmount, F> PinnedRenderStateKind
    for Kind<EK, HookData, F>
{
    type PinnedUiHandle<R: frender_html::RenderHtml + ?Sized> = UiHandleWithNonReactiveState<
        CursorPlaceholdersSurrounded<R::CursorPlaceholder, EK::PinnedUiHandle<R>>,
        F, // F cannot be put into PinnedNonReactiveState because it doesn't implement Default
    >;
    type PinnedNonReactiveState<R: frender_html::RenderHtml + ?Sized> =
        EK::PinnedNonReactiveState<R>;
    type PinnedReactiveState = ReactiveState<EK::PinnedReactiveState, HookData>;
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
        RenderStates {
            ui_handle:
                UiHandleWithNonReactiveState {
                    ui_handle,
                    non_reactive_state: f,
                },
            mut non_reactive_state,
            reactive_state,
        }: PinnedMutRenderStatesOfKind<Self, R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        let ReactiveStateProj {
            mut reactive_state,
            mut hook_data,
            update_times,
        } = reactive_state.project();
        match hook_data.as_mut().poll_next_update(cx) {
            Poll::Ready(true) => {
                // HookData has a new value
                let new_element = f.use_hook_data(hook_data.as_mut());

                // So we use the new value (CsrElement) to update the states
                ui_handle.use_surrounded_render_context(renderer, |ui_handle, render_context| {
                    new_element.pinned_render_update(
                        render_context,
                        RenderStates {
                            ui_handle,
                            non_reactive_state: non_reactive_state.as_mut(),
                            reactive_state: reactive_state.as_mut(),
                        },
                    );
                });

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
                    Poll::Ready(false) => EK::pinned_poll_render(
                        renderer,
                        RenderStates {
                            ui_handle: ui_handle.surrounded_mut(),
                            non_reactive_state,
                            reactive_state,
                        },
                        cx,
                    ),
                    // HookData is pending
                    Poll::Pending => {
                        _ = EK::pinned_poll_render(
                            renderer,
                            RenderStates {
                                ui_handle: ui_handle.surrounded_mut(),
                                non_reactive_state,
                                reactive_state,
                            },
                            cx,
                        );
                        Poll::Pending
                    }
                }
            }
            // HookData is no longer reactive
            Poll::Ready(false) => EK::pinned_poll_render(
                renderer,
                RenderStates {
                    ui_handle: ui_handle.surrounded_mut(),
                    non_reactive_state,
                    reactive_state,
                },
                cx,
            ),
            Poll::Pending => {
                _ = EK::pinned_poll_render(
                    renderer,
                    RenderStates {
                        ui_handle: ui_handle.surrounded_mut(),
                        non_reactive_state,
                        reactive_state,
                    },
                    cx,
                );
                Poll::Pending
            }
        }
    }
}

// endregion

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

    fn pinned_render_init<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        PinMutRenderInitStates {
            non_reactive_state,
            reactive_state,
        }: PinMutRenderInitStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) -> PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        let Self(mut f) = self;
        let ui_handle = render_context.map_mut_render_context(|render_context| {
            CursorPlaceholdersSurrounded::surround(render_context, |render_context| {
                let ReactiveStateProj {
                    reactive_state,
                    hook_data,
                    update_times: _,
                } = reactive_state.project();
                let element = f.use_hook_data(hook_data);
                element.pinned_render_init(
                    render_context,
                    PinMutRenderInitStates {
                        non_reactive_state,
                        reactive_state,
                    },
                )
            })
        });
        UiHandleWithNonReactiveState {
            ui_handle,
            non_reactive_state: f,
        }
    }

    fn pinned_render_update<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        RenderStates {
            ui_handle:
                UiHandleWithNonReactiveState {
                    ui_handle,
                    non_reactive_state: state_f,
                },
            non_reactive_state,
            reactive_state,
        }: PinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) {
        let Self(f) = self;
        *state_f = f;
        // Immediately run f.use_hook_data(_)
        render_context.map_mut_render_context(|render_context| {
            ui_handle.map_mut_surrounded_with_render_context(
                render_context,
                |ui_handle, render_context| {
                    let ReactiveStateProj {
                        reactive_state,
                        hook_data,
                        update_times: _,
                    } = reactive_state.project();
                    let element = state_f.use_hook_data(hook_data);
                    element.pinned_render_update(
                        render_context,
                        RenderStates {
                            ui_handle,
                            non_reactive_state,
                            reactive_state,
                        },
                    )
                },
            )
        })
    }

    fn unpinned_render_init<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
    ) -> UnpinnedRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer> {
        let Self(mut f) = self;
        let mut hook_data = HookData::default();
        let (ui_handle, (non_reactive_state, reactive_state)) = render_context
            .map_mut_render_context(|render_context| {
                CursorPlaceholdersSurrounded::surround_and_output(
                    render_context,
                    |render_context| {
                        let element = f.use_hook_data(Pin::new(&mut hook_data));
                        let RenderStates {
                            ui_handle,
                            non_reactive_state,
                            reactive_state,
                        } = element.unpinned_render_init(render_context);
                        (ui_handle, (non_reactive_state, reactive_state))
                    },
                )
            });
        RenderStates {
            ui_handle,
            non_reactive_state: (non_reactive_state, f),
            reactive_state: ReactiveState {
                reactive_state,
                hook_data,
                update_times: Default::default(),
            },
        }
    }

    fn unpinned_render_update<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        RenderStates {
            ui_handle,
            non_reactive_state: (non_reactive_state, state_f),
            reactive_state:
                ReactiveState {
                    reactive_state,
                    hook_data,
                    update_times: _,
                },
        }: UnpinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) {
        let Self(f) = self;
        *state_f = f;
        // Immediately run f.use_hook_data(_)
        render_context.map_mut_render_context(|render_context| {
            ui_handle.map_mut_surrounded_with_render_context(
                render_context,
                |ui_handle, render_context| {
                    let element = state_f.use_hook_data(Pin::new(hook_data));
                    element.unpinned_render_update(
                        render_context,
                        RenderStates {
                            ui_handle,
                            non_reactive_state,
                            reactive_state,
                        },
                    );
                },
            )
        })
    }
}
