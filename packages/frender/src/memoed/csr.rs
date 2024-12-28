use std::marker::PhantomData;

use frender_csr::{render_state::compound::CompoundState, RenderState};
use frender_html::dom::ui_handle::UiHandle as _;
use frender_html::RenderStateKind;
use frender_html::{
    experimental::{
        self, PinMutRenderInitStates, PinnedRenderStateKind, PinnedRenderStateKindPollRender,
        RenderStates, UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender,
    },
    kinds::UiHandleWithNonReactiveState,
    CsrElement, RenderHtml,
};

use crate::fn_traits::{FnOnce1, FnOnce2, FnOnce2OutputCsrElement};

use super::{Memo, MemoAndProvideFirstArgument, MemoPhantom, MemoPhantomAndProvideFirstArgument};

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

fn skip_or_panic<const SKIP: bool>() {
    struct ConstBool<const V: bool>;

    trait ConstNot {
        const NOT: bool;
    }

    impl<const V: bool> ConstNot for ConstBool<V> {
        const NOT: bool = !V;
    }

    if ConstBool::<SKIP>::NOT {
        panic!("memoed dependency has not been initialized by Memo* but accessed by MemoPhantom*")
    }
}

#[cfg(todo)]
impl<F, Dep, K: RenderStateKind, const SKIP_IF_DEP_IS_NONE: bool> CsrElement
    for MemoPhantom<F, Dep, SKIP_IF_DEP_IS_NONE>
where
    F: for<'a> FnOnce1<&'a Dep, Output: CsrElement<RenderStateKind = K>>,
{
    type RenderStateKind = Kind<K, Dep>;

    fn pinned_render_init<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        states: experimental::PinMutRenderInitStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) -> experimental::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        (self.f)(dep).unpinned_render_update(render_context, render_state)
    }

    fn pinned_render_update<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        states: experimental::PinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) {
        todo!()
    }

    fn unpinned_render_init<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
    ) -> experimental::UnpinnedRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer> {
        todo!()
    }

    fn unpinned_render_update<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        render_state: &mut frender_html::UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>,
    ) where
        Self: Sized,
    {
        let CompoundState {
            reactive: render_state,
            non_reactive: dep,
        } = render_state;

        if let Some(dep) = dep.as_ref() {
            (self.f)(dep).unpinned_render_update(render_context, render_state)
        } else {
            skip_or_panic::<SKIP_IF_DEP_IS_NONE>()
        }
    }

    fn render_update<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        render_state: std::pin::Pin<
            &mut frender_html::RenderStateOfContext<Self::RenderStateKind, Ctx>,
        >,
    ) where
        Self: Sized,
    {
        let CompoundState {
            reactive: render_state,
            non_reactive: dep,
        } = render_state.pin_project();
        if let Some(dep) = dep.as_ref() {
            (self.f)(dep).render_update(render_context, render_state)
        } else {
            skip_or_panic::<SKIP_IF_DEP_IS_NONE>()
        }
    }

    fn render_update_force_reposition<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        render_state: std::pin::Pin<
            &mut frender_html::RenderStateOfContext<Self::RenderStateKind, Ctx>,
        >,
    ) where
        Self: Sized,
    {
        let CompoundState {
            reactive: render_state,
            non_reactive: dep,
        } = render_state.pin_project();
        if let Some(dep) = dep.as_ref() {
            (self.f)(dep).render_update_force_reposition(render_context, render_state)
        } else {
            skip_or_panic::<SKIP_IF_DEP_IS_NONE>()
        }
    }

    fn render_update_maybe_reposition<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        render_state: std::pin::Pin<
            &mut frender_html::RenderStateOfContext<Self::RenderStateKind, Ctx>,
        >,
        force_reposition: bool,
    ) {
        let CompoundState {
            reactive: render_state,
            non_reactive: dep,
        } = render_state.pin_project();
        if let Some(dep) = dep.as_ref() {
            (self.f)(dep).render_update_maybe_reposition(
                render_context,
                render_state,
                force_reposition,
            )
        } else {
            skip_or_panic::<SKIP_IF_DEP_IS_NONE>()
        }
    }

    fn unpinned_render_update_force_reposition<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        render_state: &mut frender_html::UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>,
    ) where
        Self: Sized,
    {
        let CompoundState {
            reactive: render_state,
            non_reactive: dep,
        } = render_state;

        if let Some(dep) = dep.as_ref() {
            (self.f)(dep).unpinned_render_update_force_reposition(render_context, render_state)
        } else {
            skip_or_panic::<SKIP_IF_DEP_IS_NONE>()
        }
    }

    fn unpinned_render_update_maybe_reposition<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        render_state: &mut frender_html::UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>,
        force_reposition: bool,
    ) {
        let CompoundState {
            reactive: render_state,
            non_reactive: dep,
        } = render_state;

        if let Some(dep) = dep.as_ref() {
            (self.f)(dep).unpinned_render_update_maybe_reposition(
                render_context,
                render_state,
                force_reposition,
            )
        } else {
            skip_or_panic::<SKIP_IF_DEP_IS_NONE>()
        }
    }
}

#[cfg(todo)]
impl<F, V, Dep, K: RenderStateKind, const SKIP_IF_DEP_IS_NONE: bool> CsrElement
    for MemoPhantomAndProvideFirstArgument<F, V, Dep, SKIP_IF_DEP_IS_NONE>
where
    F: for<'a> FnOnce2OutputCsrElement<V, &'a Dep, OutputElementRenderStateKind = K>,
{
    type RenderStateKind = crate::memoed::Kind<K, Dep>;

    frender_html::proxy_csr_element!(|this| this.into_memo_phantom());
}
