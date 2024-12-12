use std::pin::Pin;

use frender_dom::{ui_handle::UiHandle, StateUnmount};
use pin_project_lite::pin_project;

use crate::{
    element::{PinMutRenderInitStates, PinnedRenderStateKind, PinnedRenderStateKindPollRender, RenderStates, UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender},
    kinds::UiHandleWithNonReactiveState,
    ui_handles::EitherUiHandle,
    CsrElement, HtmlRenderContext, RenderHtml,
};

pub use frender_common::either::EitherElement;

// region: EitherState

pin_project!(
    #[project = EitherReactiveStateProj]
    pub enum EitherState<A, B> {
        A {
            #[pin]
            inner: A,
        },
        B {
            #[pin]
            inner: B,
        },
    }
);

impl<A, B> EitherState<A, B> {
    fn assert_a_pin_mut(self: Pin<&mut Self>) -> Pin<&mut A> {
        match self.project() {
            EitherReactiveStateProj::A { inner } => inner,
            EitherReactiveStateProj::B { inner: _ } => unreachable!(),
        }
    }
    fn assert_b_pin_mut(self: Pin<&mut Self>) -> Pin<&mut B> {
        match self.project() {
            EitherReactiveStateProj::B { inner } => inner,
            EitherReactiveStateProj::A { inner: _ } => unreachable!(),
        }
    }
    fn assert_a_mut(&mut self) -> &mut A {
        match self {
            Self::A { inner } => inner,
            Self::B { inner: _ } => unreachable!(),
        }
    }
    fn assert_b_mut(&mut self) -> &mut B {
        match self {
            Self::B { inner } => inner,
            Self::A { inner: _ } => unreachable!(),
        }
    }
}

/// Prefer `A::default()`
impl<A: Default, B: Default> Default for EitherState<A, B> {
    fn default() -> Self {
        Self::A { inner: Default::default() }
    }
}

impl<A: StateUnmount, B: StateUnmount> StateUnmount for EitherState<A, B> {
    fn state_unmount(self: Pin<&mut Self>) {
        match self.project() {
            EitherReactiveStateProj::A { inner } => inner.state_unmount(),
            EitherReactiveStateProj::B { inner } => inner.state_unmount(),
        }
    }
}

// endregion

// region: kind

pub struct Kind<KA, KB>(super::Kind<(KA, KB)>);

impl<KA: UnpinnedRenderStateKind, KB: UnpinnedRenderStateKind> UnpinnedRenderStateKind for Kind<KA, KB> {
    type UnpinnedUiHandle<R: RenderHtml + ?Sized> = EitherUiHandle<
        //
        UiHandleWithNonReactiveState<KA::UnpinnedUiHandle<R>, KA::UnpinnedNonReactiveState<R>>,
        UiHandleWithNonReactiveState<KB::UnpinnedUiHandle<R>, KB::UnpinnedNonReactiveState<R>>,
    >;
    type UnpinnedNonReactiveState<R: RenderHtml + ?Sized> = ();
    type UnpinnedReactiveState = EitherState<KA::UnpinnedReactiveState, KB::UnpinnedReactiveState>;
}

impl<KA: UnpinnedRenderStateKindPollRender, KB: UnpinnedRenderStateKindPollRender> UnpinnedRenderStateKindPollRender for Kind<KA, KB> {
    fn unpinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        states: crate::element::UnpinnedMutRenderStatesOfKind<Self, R>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        let RenderStates {
            ui_handle,
            non_reactive_state: (),
            reactive_state,
        } = states;

        match (ui_handle, reactive_state) {
            (EitherUiHandle::A(UiHandleWithNonReactiveState { ui_handle, non_reactive_state }), EitherState::A { inner: reactive_state }) => KA::unpinned_poll_render(
                renderer,
                RenderStates {
                    ui_handle,
                    non_reactive_state,
                    reactive_state,
                },
                cx,
            ),
            (EitherUiHandle::B(UiHandleWithNonReactiveState { ui_handle, non_reactive_state }), EitherState::B { inner: reactive_state }) => KB::unpinned_poll_render(
                renderer,
                RenderStates {
                    ui_handle,
                    non_reactive_state,
                    reactive_state,
                },
                cx,
            ),
            _ => unreachable!(),
        }
    }
}

impl<KA: PinnedRenderStateKind, KB: PinnedRenderStateKind> PinnedRenderStateKind for Kind<KA, KB> {
    type PinnedUiHandle<R: RenderHtml + ?Sized> = EitherUiHandle<KA::PinnedUiHandle<R>, KB::PinnedUiHandle<R>>;
    type PinnedNonReactiveState<R: RenderHtml + ?Sized> = EitherState<KA::PinnedNonReactiveState<R>, KB::PinnedNonReactiveState<R>>;
    type PinnedReactiveState = EitherState<KA::PinnedReactiveState, KB::PinnedReactiveState>;
}

impl<KA: PinnedRenderStateKindPollRender, KB: PinnedRenderStateKindPollRender> PinnedRenderStateKindPollRender for Kind<KA, KB> {
    fn pinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        states: crate::element::PinnedMutRenderStatesOfKind<Self, R>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        let RenderStates {
            ui_handle,
            non_reactive_state,
            reactive_state,
        } = states;

        let non_reactive_state = non_reactive_state.project();
        let reactive_state = reactive_state.project();
        match (ui_handle, non_reactive_state, reactive_state) {
            (EitherUiHandle::A(ui_handle), EitherReactiveStateProj::A { inner: non_reactive_state }, EitherReactiveStateProj::A { inner: reactive_state }) => KA::pinned_poll_render(
                renderer,
                RenderStates {
                    ui_handle,
                    non_reactive_state,
                    reactive_state,
                },
                cx,
            ),
            (EitherUiHandle::B(ui_handle), EitherReactiveStateProj::B { inner: non_reactive_state }, EitherReactiveStateProj::B { inner: reactive_state }) => KB::pinned_poll_render(
                renderer,
                RenderStates {
                    ui_handle,
                    non_reactive_state,
                    reactive_state,
                },
                cx,
            ),
            _ => unreachable!(),
        }
    }
}

// endregion

impl<A: CsrElement, B: CsrElement> CsrElement for EitherElement<A, B> {
    type RenderStateKind = Kind<A::RenderStateKind, B::RenderStateKind>;

    fn pinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        PinMutRenderInitStates {
            mut non_reactive_state,
            mut reactive_state,
        }: crate::element::PinMutRenderInitStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) -> crate::element::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        match self {
            EitherElement::A(this) => EitherUiHandle::A(this.pinned_render_init(
                render_context,
                PinMutRenderInitStates {
                    // pinned_render_init is assumed to be called with default states
                    non_reactive_state: non_reactive_state.assert_a_pin_mut(),
                    reactive_state: reactive_state.assert_a_pin_mut(),
                },
            )),
            EitherElement::B(this) => EitherUiHandle::B(this.pinned_render_init(
                render_context,
                PinMutRenderInitStates {
                    // pinned_render_init is assumed to be called with default states
                    // So EitherState::A doesn't need to be state_unmounted
                    non_reactive_state: {
                        non_reactive_state.set(EitherState::B { inner: Default::default() });
                        non_reactive_state.assert_b_pin_mut()
                    },
                    reactive_state: {
                        reactive_state.set(EitherState::B { inner: Default::default() });
                        reactive_state.assert_b_pin_mut()
                    },
                },
            )),
        }
    }

    fn pinned_render_update<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        states: crate::element::PinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) {
        let RenderStates {
            ui_handle,
            mut non_reactive_state,
            mut reactive_state,
        } = states;
        match self {
            EitherElement::A(this) => match ui_handle {
                EitherUiHandle::A(ui_handle) => this.pinned_render_update(
                    render_context,
                    RenderStates {
                        ui_handle,
                        non_reactive_state: non_reactive_state.assert_a_pin_mut(),
                        reactive_state: reactive_state.assert_a_pin_mut(),
                    },
                ),
                EitherUiHandle::B(_) => {
                    // drop B::NonReactiveState
                    non_reactive_state.set(EitherState::A { inner: Default::default() });
                    let non_reactive_state = non_reactive_state.assert_a_pin_mut();

                    // state_unmount B::ReactiveState
                    reactive_state.as_mut().assert_b_pin_mut().state_unmount();
                    reactive_state.set(EitherState::A { inner: Default::default() });
                    let reactive_state = reactive_state.assert_a_pin_mut();

                    let ui_handle_a = this.pinned_render_init(render_context, PinMutRenderInitStates { non_reactive_state, reactive_state });
                    let ui_handle_b = std::mem::replace(ui_handle, EitherUiHandle::A(ui_handle_a));
                    let EitherUiHandle::B(ui_handle_b) = ui_handle_b else { unreachable!() };

                    // unmount B::UiHandle
                    _ = ui_handle_b.unmount(render_context.renderer_mut());
                }
            },
            EitherElement::B(this) => match ui_handle {
                EitherUiHandle::B(ui_handle) => this.pinned_render_update(
                    render_context,
                    RenderStates {
                        ui_handle,
                        non_reactive_state: non_reactive_state.assert_b_pin_mut(),
                        reactive_state: reactive_state.assert_b_pin_mut(),
                    },
                ),
                EitherUiHandle::A(_) => {
                    // drop A::NonReactiveState
                    non_reactive_state.set(EitherState::B { inner: Default::default() });
                    let non_reactive_state = non_reactive_state.assert_b_pin_mut();

                    // state_unmount A::ReactiveState
                    reactive_state.as_mut().assert_a_pin_mut().state_unmount();
                    reactive_state.set(EitherState::B { inner: Default::default() });
                    let reactive_state = reactive_state.assert_b_pin_mut();

                    let ui_handle_b = this.pinned_render_init(render_context, PinMutRenderInitStates { non_reactive_state, reactive_state });
                    let ui_handle_a = std::mem::replace(ui_handle, EitherUiHandle::B(ui_handle_b));
                    let EitherUiHandle::A(ui_handle_a) = ui_handle_a else { unreachable!() };

                    // unmount A::UiHandle
                    _ = ui_handle_a.unmount(render_context.renderer_mut());
                }
            },
        }
    }

    fn unpinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
    ) -> crate::element::UnpinnedRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer> {
        match self {
            EitherElement::A(this) => {
                let RenderStates {
                    ui_handle,
                    non_reactive_state,
                    reactive_state,
                } = this.unpinned_render_init(render_context);

                RenderStates {
                    ui_handle: EitherUiHandle::A(UiHandleWithNonReactiveState { ui_handle, non_reactive_state }),
                    non_reactive_state: (),
                    reactive_state: EitherState::A { inner: reactive_state },
                }
            }
            EitherElement::B(this) => {
                let RenderStates {
                    ui_handle,
                    non_reactive_state,
                    reactive_state,
                } = this.unpinned_render_init(render_context);

                RenderStates {
                    ui_handle: EitherUiHandle::B(UiHandleWithNonReactiveState { ui_handle, non_reactive_state }),
                    non_reactive_state: (),
                    reactive_state: EitherState::B { inner: reactive_state },
                }
            }
        }
    }

    fn unpinned_render_update<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        RenderStates {
            ui_handle,
            non_reactive_state: (),
            reactive_state,
        }: crate::element::UnpinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) {
        match self {
            EitherElement::A(this) => match ui_handle {
                EitherUiHandle::A(UiHandleWithNonReactiveState { ui_handle, non_reactive_state }) => {
                    this.unpinned_render_update(
                        render_context,
                        RenderStates {
                            ui_handle,
                            non_reactive_state,
                            reactive_state: reactive_state.assert_a_mut(),
                        },
                    );
                }
                wrong_ui_handle @ EitherUiHandle::B(_) => {
                    let RenderStates {
                        ui_handle,
                        non_reactive_state,
                        reactive_state: correct_reactive_state,
                    } = this.unpinned_render_init(render_context);

                    let wrong_ui_handle = std::mem::replace(wrong_ui_handle, EitherUiHandle::A(UiHandleWithNonReactiveState { ui_handle, non_reactive_state }));
                    let EitherUiHandle::B(wrong_ui_handle) = wrong_ui_handle else { unreachable!() };

                    drop(wrong_ui_handle.non_reactive_state);
                    Pin::new(reactive_state.assert_b_mut()).state_unmount();
                    _ = wrong_ui_handle.ui_handle.unmount(render_context.renderer_mut());

                    *reactive_state = EitherState::A { inner: correct_reactive_state };
                }
            },
            EitherElement::B(this) => match ui_handle {
                EitherUiHandle::B(UiHandleWithNonReactiveState { ui_handle, non_reactive_state }) => {
                    this.unpinned_render_update(
                        render_context,
                        RenderStates {
                            ui_handle,
                            non_reactive_state,
                            reactive_state: reactive_state.assert_b_mut(),
                        },
                    );
                }
                wrong_ui_handle @ EitherUiHandle::A(_) => {
                    let RenderStates {
                        ui_handle,
                        non_reactive_state,
                        reactive_state: correct_reactive_state,
                    } = this.unpinned_render_init(render_context);

                    let wrong_ui_handle = std::mem::replace(wrong_ui_handle, EitherUiHandle::B(UiHandleWithNonReactiveState { ui_handle, non_reactive_state }));
                    let EitherUiHandle::B(wrong_ui_handle) = wrong_ui_handle else { unreachable!() };

                    drop(wrong_ui_handle.non_reactive_state);
                    Pin::new(reactive_state.assert_a_mut()).state_unmount();
                    _ = wrong_ui_handle.ui_handle.unmount(render_context.renderer_mut());

                    *reactive_state = EitherState::B { inner: correct_reactive_state };
                }
            },
        }
    }
}

#[cfg(feature = "either")]
mod extern_either;
