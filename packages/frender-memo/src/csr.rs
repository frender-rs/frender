use std::marker::PhantomData;
use std::pin::Pin;
use std::task::Poll;

use frender_csr::{
    CsrElement, RenderStateKind, StateUnmount, UnmountedUiHandle as _,
    experimental::{
        self, HtmlRenderContext, PinnedRenderStateKind, PinnedRenderStateKindPollRender,
        PinnedStateOfKind, PinnedUiHandleOfKind, RenderHtml, RenderInitPinned,
        UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender,
    },
    proxy_csr_element_without_pinned_render_init,
};

use frender_fn_traits::{FnOnce1, FnOnce2};

use super::{Memo, MemoAndProvideFirstArgument};

impl<F, Dep> Memo<F, Dep> {
    fn map_changed_memo<Out>(
        self,
        memoed_dep: &mut Dep,
        ne: impl FnOnce(&Dep, &Dep) -> bool,
        run: impl FnOnce(F, &mut Dep) -> Out,
    ) -> Option<Out> {
        let Self(f, dep) = self;

        let changed = ne(memoed_dep, &dep);

        *memoed_dep = dep; // TODO: should we update memoed dep even if the new dep eq the memoed dep?

        changed.then(|| run(f, memoed_dep))
    }
    // The third argument of `run` is `new_dep == memoed_dep`.
    // `true` means new dep equals to the memoed dep.
    fn map_memoed<R>(
        self,
        memoed_dep: &mut Dep,
        eq: impl FnOnce(&Dep, &Dep) -> bool,
        run: impl FnOnce(F, &mut Dep, bool) -> R,
    ) -> R {
        let Self(f, dep) = self;

        let dep_eq_memoed_dep;
        if eq(memoed_dep, &dep) {
            *memoed_dep = dep; // TODO: should we update memoed dep even if the new dep eq the memoed dep?
            dep_eq_memoed_dep = true;
        } else {
            *memoed_dep = dep;
            dep_eq_memoed_dep = false;
        }

        run(f, memoed_dep, dep_eq_memoed_dep)
    }
}

pin_project_lite::pin_project!(
    #[derive(Debug)]
    pub struct CompoundState<S, T> {
        #[pin]
        pub reactive: S,
        pub non_reactive: T,
    }
);

impl<S, T> CompoundState<S, T> {
    pub fn pin_project(self: Pin<&mut Self>) -> CompoundState<Pin<&mut S>, &mut T> {
        let this = self.project();
        CompoundState {
            reactive: this.reactive,
            non_reactive: this.non_reactive,
        }
    }
}

impl<S: StateUnmount, T> StateUnmount for CompoundState<S, T> {
    fn state_unmount(self: Pin<&mut Self>) {
        self.project().reactive.state_unmount()
    }
}

enum Never {}
pub struct Kind<K, Dep>(Never, PhantomData<(K, Dep)>);

impl<K: UnpinnedRenderStateKind, Dep> UnpinnedRenderStateKind for Kind<K, Dep> {
    type UnpinnedUiHandle<R: RenderHtml + ?Sized> = K::UnpinnedUiHandle<R>;
    type UnpinnedState<R: RenderHtml + ?Sized> = CompoundState<K::UnpinnedState<R>, Dep>;
}

impl<K: UnpinnedRenderStateKindPollRender, Dep> UnpinnedRenderStateKindPollRender for Kind<K, Dep> {
    fn unpinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        CompoundState {
            reactive,
            non_reactive: _,
        }: &mut Self::UnpinnedState<R>,
        ui_handle: &mut Self::UnpinnedUiHandle<R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        K::unpinned_poll_render(renderer, reactive, ui_handle, cx)
    }
}

pin_project_lite::pin_project!(
    pub struct PinnedState<S, NRS> {
        #[pin]
        reactive: Option<S>,
        non_reactive: NRS,
    }
);

impl<S: StateUnmount, NRS> StateUnmount for PinnedState<S, NRS> {
    fn state_unmount(self: Pin<&mut Self>) {
        self.project_state().reactive.state_unmount()
    }
}

impl<S, NRS> PinnedState<S, NRS> {
    pub fn project_state(self: Pin<&mut Self>) -> CompoundState<Pin<&mut S>, &mut NRS> {
        let this = self.project();
        CompoundState {
            reactive: this.reactive.as_pin_mut().unwrap(),
            non_reactive: this.non_reactive,
        }
    }
}

impl<K: PinnedRenderStateKind, Dep> PinnedRenderStateKind for Kind<K, Dep> {
    type PinnedUiHandle<R: RenderHtml + ?Sized> = K::PinnedUiHandle<R>;
    type PinnedState<R: RenderHtml + ?Sized> = PinnedState<K::PinnedState<R>, Dep>;
}

impl<K: PinnedRenderStateKindPollRender, Dep> PinnedRenderStateKindPollRender for Kind<K, Dep> {
    fn pinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        state: Pin<&mut Self::PinnedState<R>>,
        ui_handle: &mut Self::PinnedUiHandle<R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        let state = state.project_state().reactive;
        K::pinned_poll_render(renderer, state, ui_handle, cx)
    }
}

pub struct RenderInit<F>(F);

impl<
    F: for<'a> FnOnce1<&'a Dep, Output: CsrElement<RenderStateKind = K>>,
    Dep,
    K: PinnedRenderStateKind,
    Ctx: ?Sized + HtmlRenderContext,
> RenderInitPinned<&mut Ctx, PinnedState<K::PinnedState<Ctx::Renderer>, Dep>> for RenderInit<F>
{
    type Output = K::PinnedUiHandle<Ctx::Renderer>;

    fn render_init_pinned(
        self,
        render_context: &mut Ctx,
        state: Pin<&mut PinnedState<K::PinnedState<Ctx::Renderer>, Dep>>,
    ) -> Self::Output {
        let Self(f) = self;
        let mut state = state.project();
        let dep = state.non_reactive;
        let (state_init, render_init) = f(dep).pinned_render_init(render_context.renderer_mut());
        state.reactive.set(Some(state_init));
        let state = state.reactive.as_pin_mut().unwrap();
        render_context.map_mut_render_context(|render_context| {
            render_init.render_init_pinned(render_context, state)
        })
    }
}

impl<
    F: for<'a> FnOnce1<&'a Dep, Output: CsrElement<RenderStateKind = K>>,
    Dep: PartialEq,
    K: RenderStateKind,
> CsrElement for Memo<F, Dep>
{
    type RenderStateKind = Kind<K, Dep>;
    type PinnedRenderInit<R: ?Sized + RenderHtml> = RenderInit<F>;

    fn pinned_render_init<Renderer: ?Sized + RenderHtml>(
        //
        self,
        _: &mut Renderer,
    ) -> (
        //
        experimental::PinnedStateOfKind<Renderer, Self::RenderStateKind>,
        Self::PinnedRenderInit<Renderer>,
    ) {
        let Self(f, dep) = self;
        (
            PinnedState {
                reactive: None,
                non_reactive: dep,
            },
            RenderInit(f),
        )
    }

    fn pinned_render_init_by_reusing<Ctx: ?Sized + HtmlRenderContext>(
        self,
        render_context: &mut Ctx,
        reused_state: Pin<&mut PinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>>,
        unmounted_ui_handle: experimental::PinnedUnmountedUiHandleOfKind<
            Ctx::Renderer,
            Self::RenderStateKind,
        >,
    ) -> PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        let CompoundState {
            reactive: reused_state,
            non_reactive: memoed_dep,
        } = reused_state.project_state();

        self.map_memoed(memoed_dep, Dep::eq, |f, dep, unchanged| {
            if unchanged {
                render_context.map_mut_render_context(|render_context| {
                    unmounted_ui_handle.mount(render_context)
                })
            } else {
                f(dep).pinned_render_init_by_reusing(
                    render_context,
                    reused_state,
                    unmounted_ui_handle,
                )
            }
        })
    }

    fn pinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        state: Pin<&mut PinnedStateOfKind<Renderer, Self::RenderStateKind>>,
        ui_handle: &mut PinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
        let CompoundState {
            reactive: state,
            non_reactive: memoed_dep,
        } = state.project_state();

        _ = self.map_changed_memo(memoed_dep, PartialEq::ne, |f, dep| {
            f(dep).pinned_render_update(renderer, state, ui_handle)
        })
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
        let Self(f, dep) = self;

        let (state, ui_handle) = f(&dep).unpinned_render_init(render_context);

        (
            CompoundState {
                reactive: state,
                non_reactive: dep,
            },
            ui_handle,
        )
    }

    fn unpinned_render_init_by_reusing<Ctx: ?Sized + HtmlRenderContext>(
        self,
        render_context: &mut Ctx,
        CompoundState {
            reactive: reused_state,
            non_reactive: memoed_dep,
        }: &mut experimental::UnpinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        unmounted_ui_handle: experimental::UnpinnedUnmountedUiHandleOfKind<
            Ctx::Renderer,
            Self::RenderStateKind,
        >,
    ) -> experimental::UnpinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        self.map_memoed(memoed_dep, Dep::eq, |f, dep, unchanged| {
            if unchanged {
                render_context.map_mut_render_context(|render_context| {
                    unmounted_ui_handle.mount(render_context)
                })
            } else {
                f(dep).unpinned_render_init_by_reusing(
                    render_context,
                    reused_state,
                    unmounted_ui_handle,
                )
            }
        })
    }

    fn unpinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        CompoundState {
            reactive: state,
            non_reactive: memoed_dep,
        }: &mut experimental::UnpinnedStateOfKind<Renderer, Self::RenderStateKind>,
        ui_handle: &mut experimental::UnpinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
        _ = self.map_changed_memo(memoed_dep, PartialEq::eq, |f, dep| {
            f(dep).unpinned_render_update(renderer, state, ui_handle)
        })
    }
}

pub struct RenderInitProvideFirstArgument<F, A>(F, A);

impl<F, A> RenderInitProvideFirstArgument<F, A> {
    fn into_f<Dep, K>(self) -> impl for<'a> FnOnce(&'a Dep) -> <F as FnOnce2<A, &'a Dep>>::Output_
    where
        F: for<'a> FnOnce2<A, &'a Dep, Output: CsrElement<RenderStateKind = K>>,
        K: PinnedRenderStateKind,
    {
        let Self(f, first_arg) = self;
        |dep: &_| f(first_arg, dep)
    }
}

impl<
    F: for<'a> FnOnce2<A, &'a Dep, Output: CsrElement<RenderStateKind = K>>,
    A,
    Dep,
    K: PinnedRenderStateKind,
    Ctx: ?Sized + HtmlRenderContext,
> RenderInitPinned<&mut Ctx, PinnedState<K::PinnedState<Ctx::Renderer>, Dep>>
    for RenderInitProvideFirstArgument<F, A>
{
    type Output = K::PinnedUiHandle<Ctx::Renderer>;

    fn render_init_pinned(
        self,
        render_context: &mut Ctx,
        state: Pin<&mut PinnedState<K::PinnedState<Ctx::Renderer>, Dep>>,
    ) -> Self::Output {
        RenderInit(self.into_f::<Dep, K>()).render_init_pinned(render_context, state)
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
    type PinnedRenderInit<R: ?Sized + RenderHtml> = RenderInitProvideFirstArgument<F, A>;

    fn pinned_render_init<Renderer: ?Sized + RenderHtml>(
        //
        self,
        _: &mut Renderer,
    ) -> (
        //
        PinnedStateOfKind<Renderer, Self::RenderStateKind>,
        Self::PinnedRenderInit<Renderer>,
    ) {
        let Self(f, first_arg, dep) = self;
        (
            PinnedState {
                reactive: None,
                non_reactive: dep,
            },
            RenderInitProvideFirstArgument(f, first_arg),
        )
    }

    proxy_csr_element_without_pinned_render_init!(|this| this.into_memo());
}
