use std::{pin::Pin, task::Poll};

use frender_common::reactive_value::RenderInitPinned;
use frender_dom::csr::{
    behaviors::{NodeRenderSelf, NodeWithRenderContextAfterSelf as _},
    render::{Render, RenderContext, RenderWithContext},
    StateUnmount as _, UiHandle as _, UnmountedUiHandle as _,
};

use crate::{
    element::{PinnedRenderStateKind, PinnedRenderStateKindPollRender, UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender},
    CsrElement, HtmlRenderContext, RenderHtml,
};

pub type UiHandle<R, T> = (<R as Render>::CursorPlaceholder, Option<T>);

// region: kind

pub struct Kind<K>(super::Kind<K>);

impl<K: UnpinnedRenderStateKind> UnpinnedRenderStateKind for Kind<K> {
    type UnpinnedUiHandle<R: RenderHtml + ?Sized> = UiHandle<R, K::UnpinnedUiHandle<R>>;
    type UnpinnedState<R: RenderHtml + ?Sized> = Option<K::UnpinnedState<R>>;
}

impl<K: UnpinnedRenderStateKindPollRender> UnpinnedRenderStateKindPollRender for Kind<K> {
    fn unpinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        state: &mut Self::UnpinnedState<R>,
        (_, ui_handle): &mut Self::UnpinnedUiHandle<R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        match (state, ui_handle) {
            (None, None) => Poll::Ready(()),
            (Some(state), Some(ui_handle)) => K::unpinned_poll_render(renderer, state, ui_handle, cx),
            _ => super::unreachable_debug!("unpinned state and ui handle of Option<impl CsrElement> are invalid"),
        }
    }
}

// region: RenderInit
pub struct OptionRenderInit<T>(pub Option<T>);

impl<T: RenderInitPinned<R, S>, R, S> RenderInitPinned<R, Option<S>> for OptionRenderInit<T> {
    type Output = Option<T::Output>;

    fn render_init_pinned(self, renderer: R, state: Pin<&mut Option<S>>) -> Self::Output {
        match (self.0, state.as_pin_mut()) {
            (Some(init), Some(state)) => Some(init.render_init_pinned(renderer, state)),
            (None, None) => None,
            _ => super::unreachable_debug!("state of option::RenderInit is invalid"),
        }
    }
}
// endregion

impl<K: PinnedRenderStateKind> PinnedRenderStateKind for Kind<K> {
    type PinnedUiHandle<R: RenderHtml + ?Sized> = UiHandle<R, K::PinnedUiHandle<R>>;
    type PinnedState<R: RenderHtml + ?Sized> = Option<K::PinnedState<R>>;
}

impl<K: PinnedRenderStateKindPollRender> PinnedRenderStateKindPollRender for Kind<K> {
    fn pinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        state: Pin<&mut Self::PinnedState<R>>,
        (_, ui_handle): &mut Self::PinnedUiHandle<R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        match (state.as_pin_mut(), ui_handle) {
            (None, None) => Poll::Ready(()),
            (Some(state), Some(ui_handle)) => K::pinned_poll_render(renderer, state, ui_handle, cx),
            _ => super::unreachable_debug!("pinned state and ui handle of Option<impl CsrElement> are invalid"),
        }
    }
}

// endregion

pub type RenderInit<T> = super::prefix_cursor_placeholder::RenderInit<OptionRenderInit<T>>;

impl<E: CsrElement> CsrElement for Option<E> {
    type RenderStateKind = Kind<E::RenderStateKind>;
    type PinnedRenderInit<R: ?Sized + RenderHtml> = RenderInit<E::PinnedRenderInit<R>>;

    fn pinned_render_init<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
    ) -> (
        //
        crate::element::PinnedStateOfKind<Renderer, Self::RenderStateKind>,
        Self::PinnedRenderInit<Renderer>,
    ) {
        if let Some(this) = self {
            let (state, render_init) = this.pinned_render_init(renderer);
            (Some(state), super::prefix_cursor_placeholder::RenderInit(OptionRenderInit(Some(render_init))))
        } else {
            (None, super::prefix_cursor_placeholder::RenderInit(OptionRenderInit(None)))
        }
    }

    fn pinned_render_init_by_reusing<Ctx: ?Sized + HtmlRenderContext>(
        self,
        render_context: &mut Ctx,
        mut reused_state: Pin<&mut crate::element::PinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>>,
        (unmounted_cp, unmounted_ui_handle): crate::element::PinnedUnmountedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
    ) -> crate::element::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        render_context.map_mut_render_context(|render_context| {
            let cp = unmounted_cp.mount(render_context);
            let ui_handle = match (reused_state.as_mut().as_pin_mut(), unmounted_ui_handle) {
                (None, None) => {
                    if let Some(this) = self {
                        Some({
                            let (state, render_init) = this.pinned_render_init(render_context.renderer_mut());
                            reused_state.set(Some(state));
                            let state = reused_state.as_pin_mut().unwrap();

                            render_init.render_init_pinned(render_context, state)
                        })
                    } else {
                        None
                    }
                }
                (Some(reused_state_some), Some(unmounted_ui_handle)) => {
                    if let Some(this) = self {
                        Some(this.pinned_render_init_by_reusing(render_context, reused_state_some, unmounted_ui_handle))
                    } else {
                        reused_state.set(None);
                        drop(unmounted_ui_handle);
                        None
                    }
                }
                _ => super::unreachable_debug!("pinned_render_init_by_reusing state is invalid"),
            };
            (cp, ui_handle)
        })
    }

    fn pinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        mut state_full: Pin<&mut crate::element::PinnedStateOfKind<Renderer, Self::RenderStateKind>>,
        (cp, ui_handle_full): &mut crate::element::PinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
        match (state_full.as_mut().as_pin_mut(), &mut *ui_handle_full) {
            (None, None) => {
                if let Some(this) = self {
                    *ui_handle_full = Some({
                        cp.with_render_context_after_self(renderer, |render_context| {
                            let (state, render_init) = this.pinned_render_init(render_context.renderer_mut());
                            state_full.set(Some(state));
                            let state = state_full.as_pin_mut().unwrap();
                            render_init.render_init_pinned(render_context, state)
                        })
                    })
                } else {
                    // does nothing
                }
            }
            (Some(state), Some(ui_handle)) => {
                if let Some(this) = self {
                    this.pinned_render_update(renderer, state, ui_handle)
                } else {
                    state.state_unmount();
                    state_full.set(None);
                    drop(ui_handle_full.take().unwrap().unmount(renderer));
                }
            }
            _ => super::unreachable_debug!("pinned_render_update state invalid"),
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
        let cp = render_context.map_mut_render_context(|render_context| NodeRenderSelf::render_self(render_context));

        if let Some(this) = self {
            let (state, ui_handle) = this.unpinned_render_init(render_context);
            (Some(state), (cp, Some(ui_handle)))
        } else {
            (None, (cp, None))
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
            (None, None) => {
                if let Some(this) = self {
                    let (state, ui_handle) = this.unpinned_render_init(render_context);
                    *reused_state_full = Some(state);
                    Some(ui_handle)
                } else {
                    None
                }
            }
            (Some(reused_state), Some(unmounted_ui_handle)) => {
                if let Some(this) = self {
                    Some(this.unpinned_render_init_by_reusing(render_context, reused_state, unmounted_ui_handle))
                } else {
                    *reused_state_full = None;
                    drop(unmounted_ui_handle);
                    None
                }
            }
            _ => super::unreachable_debug!("unpinned_render_init_by_reusing state invalid"),
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
            (None, None) => {
                if let Some(this) = self {
                    let (state, ui_handle) = cp.with_render_context_after_self(renderer, |render_context| this.unpinned_render_init(render_context));
                    *state_full = Some(state);
                    *ui_handle_full = Some(ui_handle);
                } else {
                    // does nothing
                }
            }
            (Some(state), Some(ui_handle)) => {
                if let Some(this) = self {
                    this.unpinned_render_update(renderer, state, ui_handle)
                } else {
                    Pin::new(state).state_unmount();
                    *state_full = None;
                    ui_handle_full.take().unwrap().unmount(renderer);
                }
            }
            _ => super::unreachable_debug!("unpinned_render_update state invalid"),
        }
    }
}
