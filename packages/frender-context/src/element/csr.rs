use std::{marker::PhantomData, pin::Pin, task::Poll};

use frender_html::csr::{
    experimental::{
        self, HtmlRenderContext, PinnedRenderStateKind, PinnedRenderStateKindPollRender,
        RenderHtml, RenderInitPinned, UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender,
    },
    CsrElement, StateUnmount,
};

use crate::{ContextKey, ContextKeyInner};

use super::{ElementWithContext, IntoContextValue};

struct ContextKeyAndValue<C: ContextKeyInner + 'static> {
    context_key: &'static ContextKey<C>,
    value: C::SwapValue,
}

impl<C: ContextKeyInner + 'static> ContextKeyAndValue<C> {
    fn same_context_key_or_insert(
        &mut self,
        context_key: &'static ContextKey<C>,
        into_value: impl IntoContextValue<ContextValue = C::Value>,
    ) {
        if self.context_key.is_same_as(context_key) {
            C::update_swap_value_lazily(&mut self.value, into_value);
        } else {
            self.context_key = context_key;
            self.value = C::make_swap_value(into_value.into_context_value());
        }
    }

    fn provide<R>(&mut self, f: impl FnOnce() -> R) -> R {
        self.context_key.provide_value(&mut self.value, f)
    }
}

pin_project_lite::pin_project!(
    #[project = StateProj]
    pub struct State<S, C: ContextKeyInner>
    where
        C: 'static,
    {
        #[pin]
        state: S,
        context_key_and_value: ContextKeyAndValue<C>,
    }
);

impl<S: StateUnmount, C: ContextKeyInner> StateUnmount for State<S, C> {
    fn state_unmount(self: Pin<&mut Self>) {
        self.project().state.state_unmount()
    }
}

// region: kind

enum Never {}
pub struct Kind<C: ContextKeyInner + 'static, K>(Never, PhantomData<(C, K)>);

impl<C: ContextKeyInner + 'static, K: UnpinnedRenderStateKind> UnpinnedRenderStateKind
    for Kind<C, K>
{
    type UnpinnedUiHandle<R: RenderHtml + ?Sized> = K::UnpinnedUiHandle<R>;
    type UnpinnedState<R: RenderHtml + ?Sized> = State<K::UnpinnedState<R>, C>;
}

impl<C: ContextKeyInner, K: UnpinnedRenderStateKindPollRender> UnpinnedRenderStateKindPollRender
    for Kind<C, K>
{
    fn unpinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        State {
            state,
            context_key_and_value,
        }: &mut Self::UnpinnedState<R>,
        ui_handle: &mut Self::UnpinnedUiHandle<R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        context_key_and_value.provide(|| K::unpinned_poll_render(renderer, state, ui_handle, cx))
    }
}

impl<C: ContextKeyInner + 'static, K: PinnedRenderStateKind> PinnedRenderStateKind for Kind<C, K> {
    type PinnedUiHandle<R: RenderHtml + ?Sized> = K::PinnedUiHandle<R>;
    type PinnedState<R: RenderHtml + ?Sized> = State<K::PinnedState<R>, C>;
}

impl<C: ContextKeyInner + 'static, K: PinnedRenderStateKindPollRender>
    PinnedRenderStateKindPollRender for Kind<C, K>
{
    fn pinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        state: Pin<&mut Self::PinnedState<R>>,
        ui_handle: &mut Self::PinnedUiHandle<R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        let StateProj {
            state,
            context_key_and_value,
        } = state.project();

        context_key_and_value.provide(|| K::pinned_poll_render(renderer, state, ui_handle, cx))
    }
}
// endregion

impl<T, Inner: 'static + ContextKeyInner<Value = T>, F: IntoContextValue<ContextValue = T>, FE>
    ElementWithContext<Inner, F, FE>
{
    fn into_ctx_and_get_element(self) -> (ContextKeyAndValue<Inner>, FE) {
        let Self {
            context_key,
            into_value,
            get_element,
        } = self;
        let context_key_and_value = ContextKeyAndValue {
            context_key,
            value: Inner::make_swap_value(into_value.into_context_value()),
        };
        (context_key_and_value, get_element)
    }
}

pub struct RenderInit<T>(pub T);

impl<T: RenderInitPinned<R, S>, R, S, C: ContextKeyInner + 'static> RenderInitPinned<R, State<S, C>>
    for RenderInit<T>
{
    type Output = T::Output;
    fn render_init_pinned(self, renderer: R, state: Pin<&mut State<S, C>>) -> Self::Output {
        let StateProj {
            state,
            context_key_and_value,
        } = state.project();
        context_key_and_value.provide(|| self.0.render_init_pinned(renderer, state))
    }
}

impl<
        T,
        Inner: 'static + ContextKeyInner<Value = T>,
        F: IntoContextValue<ContextValue = T>,
        E: CsrElement,
        FE: FnOnce() -> E,
    > CsrElement for ElementWithContext<Inner, F, FE>
{
    type RenderStateKind = Kind<Inner, E::RenderStateKind>;
    type PinnedRenderInit<R: ?Sized + RenderHtml> = RenderInit<E::PinnedRenderInit<R>>;

    fn pinned_render_init<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
    ) -> (
        //
        experimental::PinnedStateOfKind<Renderer, Self::RenderStateKind>,
        Self::PinnedRenderInit<Renderer>,
    ) {
        let (mut context_key_and_value, get_element) = self.into_ctx_and_get_element();
        let (state, render_init) =
            context_key_and_value.provide(|| get_element().pinned_render_init(renderer));

        (
            State {
                state,
                context_key_and_value,
            },
            RenderInit(render_init),
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
    ) -> experimental::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        let Self {
            context_key,
            into_value,
            get_element,
        } = self;
        let StateProj {
            state,
            context_key_and_value,
        } = reused_state.project();
        context_key_and_value.same_context_key_or_insert(context_key, into_value);
        context_key_and_value.provide(|| {
            get_element().pinned_render_init_by_reusing(render_context, state, unmounted_ui_handle)
        })
    }

    fn pinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        state: Pin<&mut experimental::PinnedStateOfKind<Renderer, Self::RenderStateKind>>,
        ui_handle: &mut experimental::PinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
        let Self {
            context_key,
            into_value,
            get_element,
        } = self;
        let StateProj {
            state,
            context_key_and_value,
        } = state.project();
        context_key_and_value.same_context_key_or_insert(context_key, into_value);
        context_key_and_value
            .provide(|| get_element().pinned_render_update(renderer, state, ui_handle))
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
        let (mut context_key_and_value, get_element) = self.into_ctx_and_get_element();

        let (state, ui_handle) =
            context_key_and_value.provide(|| get_element().unpinned_render_init(render_context));

        (
            State {
                state,
                context_key_and_value,
            },
            ui_handle,
        )
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
        let Self {
            context_key,
            into_value,
            get_element,
        } = self;
        let State {
            state,
            context_key_and_value,
        } = reused_state;
        context_key_and_value.same_context_key_or_insert(context_key, into_value);
        context_key_and_value.provide(|| {
            get_element().unpinned_render_init_by_reusing(
                render_context,
                state,
                unmounted_ui_handle,
            )
        })
    }

    fn unpinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        State {
            state,
            context_key_and_value,
        }: &mut experimental::UnpinnedStateOfKind<Renderer, Self::RenderStateKind>,
        ui_handle: &mut experimental::UnpinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
        let Self {
            context_key,
            into_value,
            get_element,
        } = self;
        context_key_and_value.same_context_key_or_insert(context_key, into_value);

        context_key_and_value
            .provide(|| get_element().unpinned_render_update(renderer, state, ui_handle))
    }
}
