use std::{marker::PhantomData, task::Poll};

use frender_html::{
    experimental::{
        self, PinnedRenderStateKind, PinnedRenderStateKindPollRender, RenderStates,
        UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender,
    },
    kinds::UiHandleWithNonReactiveState,
    CsrElement, RenderHtml,
};

use crate::{ContextKey, ContextKeyInner};

use super::{ElementWithContext, IntoContextValue};

pub struct ContextKeyAndValue<C: ContextKeyInner + 'static> {
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

// region: kind

enum Never {}
pub struct Kind<C: ContextKeyInner + 'static, K>(Never, PhantomData<(C, K)>);

impl<C: ContextKeyInner + 'static, K: UnpinnedRenderStateKind> UnpinnedRenderStateKind
    for Kind<C, K>
{
    type UnpinnedUiHandle<R: RenderHtml + ?Sized> =
        UiHandleWithNonReactiveState<K::UnpinnedUiHandle<R>, ContextKeyAndValue<C>>;
    type UnpinnedNonReactiveState<R: RenderHtml + ?Sized> = K::UnpinnedNonReactiveState<R>;
    type UnpinnedReactiveState = K::UnpinnedReactiveState;
}

impl<C: ContextKeyInner, K: UnpinnedRenderStateKindPollRender> UnpinnedRenderStateKindPollRender
    for Kind<C, K>
{
    fn unpinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        RenderStates {
            ui_handle:
                UiHandleWithNonReactiveState {
                    ui_handle,
                    non_reactive_state: context_key_and_value,
                },
            non_reactive_state,
            reactive_state,
        }: experimental::UnpinnedMutRenderStatesOfKind<Self, R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        context_key_and_value.provide(|| {
            K::unpinned_poll_render(
                renderer,
                RenderStates {
                    ui_handle,
                    non_reactive_state,
                    reactive_state,
                },
                cx,
            )
        })
    }
}

impl<C: ContextKeyInner + 'static, K: PinnedRenderStateKind> PinnedRenderStateKind for Kind<C, K> {
    type PinnedUiHandle<R: RenderHtml + ?Sized> =
        UiHandleWithNonReactiveState<K::PinnedUiHandle<R>, ContextKeyAndValue<C>>;
    type PinnedNonReactiveState<R: RenderHtml + ?Sized> = K::PinnedNonReactiveState<R>;
    type PinnedReactiveState = K::PinnedReactiveState;
}

impl<C: ContextKeyInner + 'static, K: PinnedRenderStateKindPollRender>
    PinnedRenderStateKindPollRender for Kind<C, K>
{
    fn pinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        RenderStates {
            ui_handle:
                UiHandleWithNonReactiveState {
                    ui_handle,
                    non_reactive_state: context_key_and_value,
                },
            non_reactive_state,
            reactive_state,
        }: experimental::PinnedMutRenderStatesOfKind<Self, R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        context_key_and_value.provide(|| {
            K::pinned_poll_render(
                renderer,
                RenderStates {
                    ui_handle,
                    non_reactive_state,
                    reactive_state,
                },
                cx,
            )
        })
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

impl<
        T,
        Inner: 'static + ContextKeyInner<Value = T>,
        F: IntoContextValue<ContextValue = T>,
        E: CsrElement,
        FE: FnOnce() -> E,
    > CsrElement for ElementWithContext<Inner, F, FE>
{
    type RenderStateKind = Kind<Inner, E::RenderStateKind>;

    fn pinned_render_init<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        states: experimental::PinMutRenderInitStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) -> experimental::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        let (mut context_key_and_value, get_element) = self.into_ctx_and_get_element();
        let ui_handle = context_key_and_value
            .provide(|| get_element().pinned_render_init(render_context, states));

        UiHandleWithNonReactiveState {
            ui_handle,
            non_reactive_state: context_key_and_value,
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
                    non_reactive_state: context_key_and_value,
                },
            non_reactive_state,
            reactive_state,
        }: experimental::PinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) {
        let Self {
            context_key,
            into_value,
            get_element,
        } = self;
        context_key_and_value.same_context_key_or_insert(context_key, into_value);
        context_key_and_value.provide(|| {
            get_element().pinned_render_update(
                render_context,
                RenderStates {
                    ui_handle,
                    non_reactive_state,
                    reactive_state,
                },
            );
        })
    }

    fn unpinned_render_init<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
    ) -> experimental::UnpinnedRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer> {
        let (mut context_key_and_value, get_element) = self.into_ctx_and_get_element();

        let RenderStates {
            ui_handle,
            non_reactive_state,
            reactive_state,
        } = context_key_and_value.provide(|| get_element().unpinned_render_init(render_context));

        RenderStates {
            ui_handle: UiHandleWithNonReactiveState {
                ui_handle,
                non_reactive_state: context_key_and_value,
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
                    non_reactive_state: context_key_and_value,
                },
            non_reactive_state,
            reactive_state,
        }: experimental::UnpinnedMutRenderStatesOfKind<
            Self::RenderStateKind,
            Ctx::Renderer,
        >,
    ) {
        let Self {
            context_key,
            into_value,
            get_element,
        } = self;
        context_key_and_value.same_context_key_or_insert(context_key, into_value);

        context_key_and_value.provide(|| {
            get_element().unpinned_render_update(
                render_context,
                RenderStates {
                    ui_handle,
                    non_reactive_state,
                    reactive_state,
                },
            );
        })
    }
}
