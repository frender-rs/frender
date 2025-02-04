use std::pin::Pin;

use frender_dom::{
    behaviors::{NodeRenderSelf, NodeWithRenderContextAfterSelf},
    render::{Render, RenderContext},
    ui_handle::{UiHandle as _, UnmountedUiHandle as _},
    StateUnmount,
};
use pin_project_lite::pin_project;

use crate::{
    element::{CsrElementRenderInitPinned, PinnedRenderStateKind, PinnedRenderStateKindPollRender, PinnedRenderInitKind, UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender},
    ui_handles::EitherUiHandle,
    CsrElement, HtmlRenderContext, RenderHtml,
};

pub use frender_common::either::EitherElement;

// region: UiHandle
pub type UiHandle<R, A, B> = (
    //
    <R as Render>::CursorPlaceholder,
    EitherUiHandle<A, B>,
);
// endregion
// region: EitherState

pin_project!(
    #[project = EitherStateProj]
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
            EitherStateProj::A { inner } => inner,
            EitherStateProj::B { inner: _ } => unreachable!(),
        }
    }
    fn assert_b_pin_mut(self: Pin<&mut Self>) -> Pin<&mut B> {
        match self.project() {
            EitherStateProj::B { inner } => inner,
            EitherStateProj::A { inner: _ } => unreachable!(),
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

impl<A: StateUnmount, B: StateUnmount> StateUnmount for EitherState<A, B> {
    fn state_unmount(self: Pin<&mut Self>) {
        match self.project() {
            EitherStateProj::A { inner } => inner.state_unmount(),
            EitherStateProj::B { inner } => inner.state_unmount(),
        }
    }
}

// endregion
// region: kind

pub struct Kind<KA, KB>(super::Kind<(KA, KB)>);

impl<KA: UnpinnedRenderStateKind, KB: UnpinnedRenderStateKind> UnpinnedRenderStateKind for Kind<KA, KB> {
    type UnpinnedUiHandle<R: RenderHtml + ?Sized> = UiHandle<
        //
        R,
        KA::UnpinnedUiHandle<R>,
        KB::UnpinnedUiHandle<R>,
    >;
    type UnpinnedState<R: RenderHtml + ?Sized> = EitherState<
        //
        KA::UnpinnedState<R>,
        KB::UnpinnedState<R>,
    >;
}

impl<KA: UnpinnedRenderStateKindPollRender, KB: UnpinnedRenderStateKindPollRender> UnpinnedRenderStateKindPollRender for Kind<KA, KB> {
    fn unpinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        state: &mut Self::UnpinnedState<R>,
        ui_handle: &mut Self::UnpinnedUiHandle<R>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        match (state, ui_handle) {
            (EitherState::A { inner: state }, (_, EitherUiHandle::A(ui_handle))) => KA::unpinned_poll_render(renderer, state, ui_handle, cx),
            (EitherState::B { inner: state }, (_, EitherUiHandle::B(ui_handle))) => KB::unpinned_poll_render(renderer, state, ui_handle, cx),
            _ => {
                if cfg!(debug_assertions) {
                    unreachable!("unpinned state and ui handle of EitherElement is invalid")
                } else {
                    unreachable!()
                }
            }
        }
    }
}

pub enum EitherPinnedRenderInit<A, B> {
    A(A),
    B(B),
}

impl<A: CsrElementRenderInitPinned<R>, B: CsrElementRenderInitPinned<R>, R: ?Sized + Render> CsrElementRenderInitPinned<R> for EitherPinnedRenderInit<A, B> {
    type UiHandle = UiHandle<R, A::UiHandle, B::UiHandle>;
    type State = EitherState<A::State, B::State>;

    fn render_init_pinned(self, render_context: &mut <R as frender_dom::render::RenderWithContext>::RenderContext<'_>, state: Pin<&mut Self::State>) -> Self::UiHandle {
        match (self, state.project()) {
            (EitherPinnedRenderInit::A(this), EitherStateProj::A { inner }) => {
                //

                (NodeRenderSelf::render_self(render_context), EitherUiHandle::A(A::render_init_pinned(this, render_context, inner)))
            }
            (EitherPinnedRenderInit::B(this), EitherStateProj::B { inner }) => {
                //
                (NodeRenderSelf::render_self(render_context), EitherUiHandle::B(B::render_init_pinned(this, render_context, inner)))
            }
            _ => {
                if cfg!(debug_assertions) {
                    unreachable!("pinned state and RenderInit of EitherElement is invalid")
                } else {
                    unreachable!()
                }
            }
        }
    }
}

impl<KA: PinnedRenderStateKind, KB: PinnedRenderStateKind> PinnedRenderStateKind for Kind<KA, KB> {
    type PinnedUiHandle<R: RenderHtml + ?Sized> = UiHandle<R, KA::PinnedUiHandle<R>, KB::PinnedUiHandle<R>>;
    type PinnedState<R: RenderHtml + ?Sized> = EitherState<KA::PinnedState<R>, KB::PinnedState<R>>;
}

impl<KA: PinnedRenderInitKind, KB: PinnedRenderInitKind> PinnedRenderInitKind for Kind<KA, KB> {
    type PinnedRenderInit<R: RenderHtml + ?Sized> = EitherPinnedRenderInit<KA::PinnedRenderInit<R>, KB::PinnedRenderInit<R>>;
}

impl<KA: PinnedRenderStateKindPollRender, KB: PinnedRenderStateKindPollRender> PinnedRenderStateKindPollRender for Kind<KA, KB> {
    fn pinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        state: Pin<&mut Self::PinnedState<R>>,
        ui_handle: &mut Self::PinnedUiHandle<R>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        match (state.project(), ui_handle) {
            (EitherStateProj::A { inner }, (_, EitherUiHandle::A(ui_handle))) => {
                //
                KA::pinned_poll_render(renderer, inner, ui_handle, cx)
            }
            (EitherStateProj::B { inner }, (_, EitherUiHandle::B(ui_handle))) => {
                //
                KB::pinned_poll_render(renderer, inner, ui_handle, cx)
            }
            _ => {
                if cfg!(debug_assertions) {
                    unreachable!("pinned state and ui handle of EitherElement is invalid")
                } else {
                    unreachable!()
                }
            }
        }
    }
}

// endregion

impl<A: CsrElement, B: CsrElement> CsrElement for EitherElement<A, B> {
    type RenderStateKind = Kind<A::RenderStateKind, B::RenderStateKind>;

    fn pinned_render_init<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
    ) -> (
        //
        crate::element::PinnedStateOfKind<Renderer, Self::RenderStateKind>,
        crate::element::PinnedRenderInitOfKind<Renderer, Self::RenderStateKind>,
    ) {
        match self {
            EitherElement::A(this) => {
                let (state, render_init) = this.pinned_render_init(renderer);
                (EitherState::A { inner: state }, EitherPinnedRenderInit::A(render_init))
            }
            EitherElement::B(this) => {
                let (state, render_init) = this.pinned_render_init(renderer);
                (EitherState::B { inner: state }, EitherPinnedRenderInit::B(render_init))
            }
        }
    }

    fn pinned_render_init_by_reusing<Ctx: ?Sized + HtmlRenderContext>(
        self,
        render_context: &mut Ctx,
        mut reused_state_full: Pin<&mut crate::element::PinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>>,
        (unmounted_cp, unmounted_ui_handle): crate::element::PinnedUnmountedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
    ) -> crate::element::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        let cp = render_context.map_mut_render_context(|render_context| unmounted_cp.mount(render_context));

        let ui_handle = match (reused_state_full.as_mut().project(), unmounted_ui_handle) {
            (EitherStateProj::A { inner: reused_state }, EitherUiHandle::A(unmounted_ui_handle)) => match self {
                EitherElement::A(this) => EitherUiHandle::A(this.pinned_render_init_by_reusing(render_context, reused_state, unmounted_ui_handle)),
                EitherElement::B(this) => {
                    drop(unmounted_ui_handle);
                    let (state, render_init) = this.pinned_render_init(render_context.renderer_mut());
                    reused_state_full.set(EitherState::B { inner: state });

                    let reused_state = reused_state_full.assert_b_pin_mut();
                    let ui_handle = render_context.map_mut_render_context(|render_context| render_init.render_init_pinned(render_context, reused_state));
                    EitherUiHandle::B(ui_handle)
                }
            },
            (EitherStateProj::B { inner: reused_state }, EitherUiHandle::B(unmounted_ui_handle)) => match self {
                EitherElement::B(this) => EitherUiHandle::B(this.pinned_render_init_by_reusing(render_context, reused_state, unmounted_ui_handle)),
                EitherElement::A(this) => {
                    drop(unmounted_ui_handle);
                    let (state, render_init) = this.pinned_render_init(render_context.renderer_mut());
                    reused_state_full.set(EitherState::A { inner: state });

                    let reused_state = reused_state_full.assert_a_pin_mut();
                    let ui_handle = render_context.map_mut_render_context(|render_context| render_init.render_init_pinned(render_context, reused_state));
                    EitherUiHandle::A(ui_handle)
                }
            },
            _ => {
                if cfg!(debug_assertions) {
                    unreachable!("pinned reused state and unmounted ui handle of EitherElement is invalid")
                } else {
                    unreachable!()
                }
            }
        };

        (cp, ui_handle)
    }

    fn pinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        mut state_full: Pin<&mut crate::element::PinnedStateOfKind<Renderer, Self::RenderStateKind>>,
        ui_handle_full: &mut crate::element::PinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
        match (state_full.as_mut().project(), &mut *ui_handle_full) {
            (EitherStateProj::A { inner: state }, (cp, EitherUiHandle::A(ui_handle))) => match self {
                EitherElement::A(this) => this.pinned_render_update(renderer, state, ui_handle),
                EitherElement::B(this) => {
                    state.state_unmount();

                    let (state, render_init) = this.pinned_render_init(renderer);

                    state_full.set(EitherState::B { inner: state });
                    let state = state_full.assert_b_pin_mut();

                    let ui_handle = cp.with_render_context_after_self(renderer, |render_context| render_init.render_init_pinned(render_context, state));
                    let old_ui_handle = std::mem::replace(&mut ui_handle_full.1, EitherUiHandle::B(ui_handle));

                    let EitherUiHandle::A(old_ui_handle) = old_ui_handle else { unreachable!() };
                    old_ui_handle.unmount(renderer);
                }
            },
            (EitherStateProj::B { inner: state }, (cp, EitherUiHandle::B(ui_handle))) => match self {
                EitherElement::B(this) => this.pinned_render_update(renderer, state, ui_handle),
                EitherElement::A(this) => {
                    state.state_unmount();

                    let (state, render_init) = this.pinned_render_init(renderer);

                    state_full.set(EitherState::A { inner: state });
                    let state = state_full.assert_a_pin_mut();

                    let ui_handle = cp.with_render_context_after_self(renderer, |render_context| render_init.render_init_pinned(render_context, state));
                    let old_ui_handle = std::mem::replace(&mut ui_handle_full.1, EitherUiHandle::A(ui_handle));

                    let EitherUiHandle::B(old_ui_handle) = old_ui_handle else { unreachable!() };
                    old_ui_handle.unmount(renderer);
                }
            },
            _ => {
                if cfg!(debug_assertions) {
                    unreachable!("pinned reused state and unmounted ui handle of EitherElement is invalid")
                } else {
                    unreachable!()
                }
            }
        }
    }

    fn unpinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
    ) -> (
        //
        crate::element::UnpinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        crate::element::UnpinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
    ) {
        match self {
            EitherElement::A(this) => {
                let cp = render_context.map_mut_render_context(|render_context| NodeRenderSelf::render_self(render_context));
                let (state, ui_handle) = this.unpinned_render_init(render_context);
                (EitherState::A { inner: state }, (cp, EitherUiHandle::A(ui_handle)))
            }
            EitherElement::B(this) => {
                let cp = render_context.map_mut_render_context(|render_context| NodeRenderSelf::render_self(render_context));
                let (state, ui_handle) = this.unpinned_render_init(render_context);
                (EitherState::B { inner: state }, (cp, EitherUiHandle::B(ui_handle)))
            }
        }
    }

    fn unpinned_render_init_by_reusing<Ctx: ?Sized + HtmlRenderContext>(
        self,
        render_context: &mut Ctx,
        reused_state_full: &mut crate::element::UnpinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        (unmounted_cp, unmounted_ui_handle): crate::element::UnpinnedUnmountedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
    ) -> crate::element::UnpinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        let cp = render_context.map_mut_render_context(|render_context| unmounted_cp.mount(render_context));
        let ui_handle = match (&mut *reused_state_full, unmounted_ui_handle) {
            (EitherState::A { inner: reused_state }, EitherUiHandle::A(unmounted_ui_handle)) => match self {
                EitherElement::A(this) => EitherUiHandle::A(this.unpinned_render_init_by_reusing(render_context, reused_state, unmounted_ui_handle)),
                EitherElement::B(this) => {
                    drop(unmounted_ui_handle);

                    let (state, ui_handle) = this.unpinned_render_init(render_context);
                    *reused_state_full = EitherState::B { inner: state };
                    EitherUiHandle::B(ui_handle)
                }
            },
            (EitherState::B { inner: reused_state }, EitherUiHandle::B(unmounted_ui_handle)) => match self {
                EitherElement::B(this) => EitherUiHandle::B(this.unpinned_render_init_by_reusing(render_context, reused_state, unmounted_ui_handle)),
                EitherElement::A(this) => {
                    drop(unmounted_ui_handle);

                    let (state, ui_handle) = this.unpinned_render_init(render_context);
                    *reused_state_full = EitherState::A { inner: state };
                    EitherUiHandle::A(ui_handle)
                }
            },
            _ => {
                if cfg!(debug_assertions) {
                    unreachable!("unpinned reused state and unmounted ui handle of EitherElement is invalid")
                } else {
                    unreachable!()
                }
            }
        };

        (cp, ui_handle)
    }

    fn unpinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        state_full: &mut crate::element::UnpinnedStateOfKind<Renderer, Self::RenderStateKind>,
        (cp, ui_handle_full): &mut crate::element::UnpinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
        match (&mut *state_full, &mut *ui_handle_full) {
            (EitherState::A { inner: state }, EitherUiHandle::A(ui_handle)) => match self {
                EitherElement::A(this) => this.unpinned_render_update(renderer, state, ui_handle),
                EitherElement::B(this) => {
                    Pin::new(state).state_unmount();
                    let (state, ui_handle) = cp.with_render_context_after_self(renderer, |render_context| this.unpinned_render_init(render_context));

                    *state_full = EitherState::B { inner: state };
                    let old_ui_handle = std::mem::replace(ui_handle_full, EitherUiHandle::B(ui_handle));
                    let EitherUiHandle::A(old_ui_handle) = old_ui_handle else { unreachable!() };
                    old_ui_handle.unmount(renderer);
                }
            },
            (EitherState::B { inner: state }, EitherUiHandle::B(ui_handle)) => match self {
                EitherElement::B(this) => this.unpinned_render_update(renderer, state, ui_handle),
                EitherElement::A(this) => {
                    Pin::new(state).state_unmount();
                    let (state, ui_handle) = cp.with_render_context_after_self(renderer, |render_context| this.unpinned_render_init(render_context));

                    *state_full = EitherState::A { inner: state };
                    let old_ui_handle = std::mem::replace(ui_handle_full, EitherUiHandle::A(ui_handle));
                    let EitherUiHandle::B(old_ui_handle) = old_ui_handle else { unreachable!() };
                    old_ui_handle.unmount(renderer);
                }
            },
            _ => {
                if cfg!(debug_assertions) {
                    unreachable!("unpinned state and ui handle of EitherElement for render update is invalid")
                } else {
                    unreachable!()
                }
            }
        }
    }
}

#[cfg(feature = "either")]
mod extern_either;
