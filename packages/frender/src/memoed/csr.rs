use std::marker::PhantomData;

use frender_html::dom::ui_handle::UiHandle as _;
use frender_html::RenderStateKind;
use frender_html::{
    experimental::{
        self, PinnedRenderStateKind, PinnedRenderStateKindPollRender, RenderStates,
        UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender,
    },
    kinds::UiHandleWithNonReactiveState,
    CsrElement, RenderHtml,
};

use crate::fn_traits::{FnOnce1, FnOnce2};

use super::{Memo, MemoAndProvideFirstArgument};

mod render_update;

enum Never {}
pub struct Kind<K, Dep>(Never, PhantomData<(K, Dep)>);

impl<K: UnpinnedRenderStateKind, Dep> UnpinnedRenderStateKind for Kind<K, Dep> {
    type UnpinnedUiHandle<R: RenderHtml + ?Sized> =
        UiHandleWithNonReactiveState<K::UnpinnedUiHandle<R>, Dep>;
    type UnpinnedNonReactiveState<R: RenderHtml + ?Sized> = K::UnpinnedNonReactiveState<R>;
    type UnpinnedReactiveState = K::UnpinnedReactiveState;
}

impl<K: UnpinnedRenderStateKindPollRender, Dep> UnpinnedRenderStateKindPollRender for Kind<K, Dep> {
    fn unpinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        RenderStates {
            ui_handle:
                UiHandleWithNonReactiveState {
                    ui_handle,
                    non_reactive_state: _,
                },
            non_reactive_state,
            reactive_state,
        }: experimental::UnpinnedMutRenderStatesOfKind<Self, R>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        K::unpinned_poll_render(
            renderer,
            RenderStates {
                ui_handle,
                non_reactive_state,
                reactive_state,
            },
            cx,
        )
    }
}

impl<K: PinnedRenderStateKind, Dep> PinnedRenderStateKind for Kind<K, Dep> {
    type PinnedUiHandle<R: RenderHtml + ?Sized> =
        UiHandleWithNonReactiveState<K::PinnedUiHandle<R>, Dep>;
    type PinnedNonReactiveState<R: RenderHtml + ?Sized> = K::PinnedNonReactiveState<R>;
    type PinnedReactiveState = K::PinnedReactiveState;
}

impl<K: PinnedRenderStateKindPollRender, Dep> PinnedRenderStateKindPollRender for Kind<K, Dep> {
    fn pinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        RenderStates {
            ui_handle:
                UiHandleWithNonReactiveState {
                    ui_handle,
                    non_reactive_state: _,
                },
            non_reactive_state,
            reactive_state,
        }: experimental::PinnedMutRenderStatesOfKind<Self, R>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        K::pinned_poll_render(
            renderer,
            RenderStates {
                ui_handle,
                non_reactive_state,
                reactive_state,
            },
            cx,
        )
    }
}

impl<
        F: for<'a> FnOnce1<&'a Dep, Output: CsrElement<RenderStateKind = K>>,
        Dep: PartialEq,
        K: RenderStateKind,
    > CsrElement for Memo<F, Dep>
{
    type RenderStateKind = Kind<K, Dep>;

    fn pinned_render_init<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        states: experimental::PinMutRenderInitStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) -> experimental::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        let Self(f, dep) = self;
        let ui_handle = f(&dep).pinned_render_init(render_context, states);
        UiHandleWithNonReactiveState {
            ui_handle,
            non_reactive_state: dep,
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
                    non_reactive_state: memoed_dep,
                },
            non_reactive_state,
            reactive_state,
        }: experimental::PinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) {
        self.map_memoed(memoed_dep, PartialEq::eq, |f, dep, unchanged| {
            if unchanged {
                render_context.map_mut_render_context(|render_context| {
                    ui_handle.check_and_move_cursor(render_context)
                });
            } else {
                f(dep).pinned_render_update(
                    render_context,
                    RenderStates {
                        ui_handle,
                        non_reactive_state,
                        reactive_state,
                    },
                )
            }
        })
    }

    fn unpinned_render_init<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
    ) -> experimental::UnpinnedRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer> {
        let Self(f, dep) = self;

        let RenderStates {
            ui_handle,
            non_reactive_state,
            reactive_state,
        } = f(&dep).unpinned_render_init(render_context);

        RenderStates {
            ui_handle: UiHandleWithNonReactiveState {
                ui_handle,
                non_reactive_state: dep,
            },
            non_reactive_state,
            reactive_state,
        }
    }

    fn unpinned_render_update<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        RenderStates {
            ui_handle:
                UiHandleWithNonReactiveState {
                    ui_handle,
                    non_reactive_state: memoed_dep,
                },
            non_reactive_state,
            reactive_state,
        }: experimental::UnpinnedMutRenderStatesOfKind<
            Self::RenderStateKind,
            Ctx::Renderer,
        >,
    ) {
        self.map_memoed(memoed_dep, PartialEq::eq, |f, dep, unchanged| {
            if unchanged {
                render_context.map_mut_render_context(|render_context| {
                    ui_handle.check_and_move_cursor(render_context)
                });
            } else {
                f(dep).unpinned_render_update(
                    render_context,
                    RenderStates {
                        ui_handle,
                        non_reactive_state,
                        reactive_state,
                    },
                )
            }
        })
    }
}

impl<
        F: for<'a> FnOnce2<A, &'a Dep, Output: CsrElement<RenderStateKind = K>>,
        A,
        Dep: PartialEq,
        K: RenderStateKind,
    > CsrElement for MemoAndProvideFirstArgument<F, A, Dep>
{
    type RenderStateKind = Kind<K, Dep>;
    frender_html::proxy_csr_element!(|this| this.into_memo());
}
