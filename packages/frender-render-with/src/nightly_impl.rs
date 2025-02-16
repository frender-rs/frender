use std::{marker::PhantomData, pin::Pin};

use frender_html::{
    experimental::{
        self, PinnedRenderStateKind, PinnedRenderStateKindPollRender, RenderInitPinned,
        UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender, UnpinnedStateOfKind,
        UnpinnedUiHandleOfKind,
    },
    CsrElement, HtmlRenderContext, RenderHtml, StateUnmount,
};

use crate::{
    CsrRenderContext, CsrRenderContextInner, FnOnceRenderWithContext, IntoFnOnceRenderWithContext,
    RenderWith, Rendered, RenderedInner,
};

type KindOf<R, T> = <<T as NamedIntoFnOnceRenderWithContext>::NamedIntoFnOnceRenderWithContext<
        R
    > as FnOnceRenderWithContext<R>>::OutputRenderStateKind;

pub trait NamedIntoFnOnceRenderWithContext: IntoFnOnceRenderWithContext + Sized {
    type NamedIntoFnOnceRenderWithContext<R: RenderHtml + ?Sized>: FnOnceRenderWithContext<
        R,
        // OutputRenderStateKind = Self::NamedRenderStateKind,
    >;

    fn named_into_fn_once_render_with_context<R: RenderHtml + ?Sized>(
        self,
    ) -> Self::NamedIntoFnOnceRenderWithContext<R>;

    fn render_init<R: ?Sized + RenderHtml>(
        self,
        render_context: &mut R::RenderContext<'_>,
    ) -> (
        UnpinnedStateOfKind<R, KindOf<R, Self>>,
        UnpinnedUiHandleOfKind<R, KindOf<R, Self>>,
    ) {
        let f = self.named_into_fn_once_render_with_context::<R>();
        let Rendered(self::PhantomData, RenderedInner::Init(out)) = f(CsrRenderContext(
            CsrRenderContextInner::Init(render_context),
        )) else {
            unreachable!()
        };
        out
    }

    fn render_update<R: ?Sized + RenderHtml>(
        self,
        renderer: &mut R,
        state: &mut UnpinnedStateOfKind<R, KindOf<R, Self>>,
        ui_handle: &mut UnpinnedUiHandleOfKind<R, KindOf<R, Self>>,
    ) {
        let f = self.named_into_fn_once_render_with_context();
        let Rendered(self::PhantomData, RenderedInner::Update) =
            f(CsrRenderContext(CsrRenderContextInner::Update {
                renderer,
                state,
                ui_handle,
            }))
        else {
            unreachable!()
        };
    }
}

impl<F: IntoFnOnceRenderWithContext> NamedIntoFnOnceRenderWithContext for F {
    type NamedIntoFnOnceRenderWithContext<R: RenderHtml + ?Sized> = impl FnOnceRenderWithContext<R>;

    fn named_into_fn_once_render_with_context<R: RenderHtml + ?Sized>(
        self,
    ) -> Self::NamedIntoFnOnceRenderWithContext<R> {
        self.into_fn_once_render_with_context()
    }
}

enum Never {}
pub struct Kind<F: NamedIntoFnOnceRenderWithContext>(Never, PhantomData<F>);

pub struct PinnedState<S>(Option<S>);

impl<S> PinnedState<S> {
    const DUMMY: Self = Self(None);

    fn as_mut_state(&mut self) -> &mut S {
        self.0.as_mut().unwrap()
    }
}

impl<S: StateUnmount + Unpin> StateUnmount for PinnedState<S> {
    fn state_unmount(self: Pin<&mut Self>) {
        Pin::new(self.get_mut().as_mut_state()).state_unmount()
    }
}

impl<F: NamedIntoFnOnceRenderWithContext> UnpinnedRenderStateKind for Kind<F> {
    type UnpinnedUiHandle<R: RenderHtml + ?Sized> =
        <KindOf<R, F> as UnpinnedRenderStateKind>::UnpinnedUiHandle<R>;
    type UnpinnedState<R: RenderHtml + ?Sized> =
        <KindOf<R, F> as UnpinnedRenderStateKind>::UnpinnedState<R>;
}

impl<F: NamedIntoFnOnceRenderWithContext> UnpinnedRenderStateKindPollRender for Kind<F> {
    fn unpinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        state: &mut Self::UnpinnedState<R>,
        ui_handle: &mut Self::UnpinnedUiHandle<R>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        <KindOf<R, F>>::unpinned_poll_render(renderer, state, ui_handle, cx)
    }
}

impl<F: NamedIntoFnOnceRenderWithContext> PinnedRenderStateKind for Kind<F> {
    type PinnedUiHandle<R: RenderHtml + ?Sized> =
        <Self as UnpinnedRenderStateKind>::UnpinnedUiHandle<R>;
    type PinnedState<R: RenderHtml + ?Sized> =
        PinnedState<<Self as UnpinnedRenderStateKind>::UnpinnedState<R>>;
}

impl<F: NamedIntoFnOnceRenderWithContext> PinnedRenderStateKindPollRender for Kind<F> {
    fn pinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        state: Pin<&mut Self::PinnedState<R>>,
        ui_handle: &mut Self::PinnedUiHandle<R>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        Self::unpinned_poll_render(renderer, state.get_mut().as_mut_state(), ui_handle, cx)
    }
}

// region: RenderInit
pub struct RenderInit<F: IntoFnOnceRenderWithContext>(F);

impl<F: IntoFnOnceRenderWithContext, Ctx: ?Sized + HtmlRenderContext>
    RenderInitPinned<
        &mut Ctx,
        PinnedState<UnpinnedStateOfKind<Ctx::Renderer, KindOf<Ctx::Renderer, F>>>,
    > for RenderInit<F>
{
    type Output =
        <KindOf<Ctx::Renderer, F> as UnpinnedRenderStateKind>::UnpinnedUiHandle<Ctx::Renderer>;

    fn render_init_pinned(
        self,
        render_context: &mut Ctx,
        dummy_state: Pin<
            &mut PinnedState<UnpinnedStateOfKind<Ctx::Renderer, KindOf<Ctx::Renderer, F>>>,
        >,
    ) -> Self::Output {
        let (state, ui_handle) = RenderWith(self.0).unpinned_render_init(render_context);
        dummy_state.get_mut().0 = Some(state);
        ui_handle
    }
}
// endregion

impl<F: IntoFnOnceRenderWithContext> CsrElement for RenderWith<F> {
    type RenderStateKind = Kind<F>;
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
        (PinnedState::DUMMY, RenderInit(self.0))
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
    ) -> experimental::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        self.unpinned_render_init_by_reusing(
            render_context,
            reused_state.get_mut().as_mut_state(),
            unmounted_ui_handle,
        )
    }

    fn pinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        state: Pin<&mut experimental::PinnedStateOfKind<Renderer, Self::RenderStateKind>>,
        ui_handle: &mut experimental::PinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
        self.unpinned_render_update(renderer, state.get_mut().as_mut_state(), ui_handle)
    }

    fn unpinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
    ) -> (
        //
        UnpinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        experimental::UnpinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
    ) {
        render_context.map_mut_render_context(|render_context| self.0.render_init(render_context))
    }

    fn unpinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        state: &mut UnpinnedStateOfKind<Renderer, Self::RenderStateKind>,
        ui_handle: &mut experimental::UnpinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
        self.0.render_update(renderer, state, ui_handle)
    }

    fn unpinned_render_init_by_reusing<Ctx: ?Sized + HtmlRenderContext>(
        self,
        render_context: &mut Ctx,
        reused_state: &mut experimental::UnpinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        unmounted_ui_handle: experimental::UnpinnedUnmountedUiHandleOfKind<
            Ctx::Renderer,
            Self::RenderStateKind,
        >,
    ) -> experimental::UnpinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        use frender_html::dom::ui_handle::UnmountedUiHandle as _;

        // TODO: is mount-then-update correct?
        let mut ui_handle = render_context
            .map_mut_render_context(|render_context| unmounted_ui_handle.mount(render_context));

        self.unpinned_render_update(render_context.renderer_mut(), reused_state, &mut ui_handle);

        ui_handle
    }
}
