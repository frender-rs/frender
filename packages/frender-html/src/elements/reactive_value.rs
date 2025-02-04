use std::marker::PhantomData;

use frender_common::reactive_value::{ReactiveValue, ReactiveValueKind, ReactiveValueRenderInitPinned, ReactiveValueState};
use frender_dom::{ui_handle::UiHandle, StateUnmount};

use crate::{
    element::{CsrElement, CsrElementRenderInitPinned, PinnedRenderInitKind, PinnedRenderStateKind, PinnedRenderStateKindPollRender, UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender},
    stateless_render::{StatelessRender, StatelessRenderStateKind},
    RenderHtml,
};

mod known;

pub trait ReactiveValueWithKind: ReactiveValue<Self::ReactiveValueKind> {
    type ReactiveValueKind: ?Sized + ReactiveValueKind;
}

pub struct ReactiveValueIntoElement<V: ReactiveValueWithKind>(pub V);

struct StateKind<PS, US, VK: ?Sized>(super::Kind<(PS, US, VK)>);
struct InitKind<PRI, US, VK: ?Sized>(super::Kind<(PRI, US, VK)>);

struct RenderInit<PRI: ReactiveValueRenderInitPinned<VK>, VK: ?Sized + ReactiveValueKind>(PRI, PhantomData<VK>);

impl<
        //
        PRI: ReactiveValueRenderInitPinned<VK>,
        VK: ?Sized + ReactiveValueKind,
        R: ?Sized + RenderHtml,
        UH: UiHandle<R>,
    > CsrElementRenderInitPinned<R> for RenderInit<PRI, VK>
where
    PRI::State: StateUnmount,
    for<'a> VK::Value<'a>: StatelessRender<StatelessRenderStateKind: StatelessRenderStateKind<UiHandle<R> = UH>>,
{
    type UiHandle = UH;
    type State = PRI::State;

    fn render_init_pinned(self, render_context: &mut R::RenderContext<'_>, state: std::pin::Pin<&mut Self::State>) -> Self::UiHandle {
        PRI::render_init_pinned(
            //
            self.0,
            |value| StatelessRender::stateless_render_init(value, render_context),
            state,
        )
    }
}

impl<PS: StateUnmount, VK: ?Sized + ReactiveValueKind, US, StatelessK: StatelessRenderStateKind> PinnedRenderStateKind for StateKind<PS, US, VK>
where
    for<'a> VK::Value<'a>: StatelessRender<StatelessRenderStateKind = StatelessK>,
{
    type PinnedUiHandle<R: crate::RenderHtml + ?Sized> = StatelessK::UiHandle<R>;
    type PinnedState<R: crate::RenderHtml + ?Sized> = PS;
}

impl<PRI: ReactiveValueRenderInitPinned<VK>, VK: ?Sized + ReactiveValueKind, US, StatelessK: StatelessRenderStateKind> PinnedRenderInitKind for InitKind<PRI, US, VK>
where
    PRI::State: StateUnmount,
    for<'a> VK::Value<'a>: StatelessRender<StatelessRenderStateKind = StatelessK>,
{
    type PinnedRenderStateKind = StateKind<PRI::State, US, VK>;
    type PinnedRenderInit<R: crate::RenderHtml + ?Sized> = RenderInit<PRI, VK>;
}

impl<PS: ReactiveValueState<ReactiveValueKind = VK>, VK: ?Sized + ReactiveValueKind, US, StatelessK: StatelessRenderStateKind> PinnedRenderStateKindPollRender for StateKind<PS, US, VK>
where
    for<'a> VK::Value<'a>: StatelessRender<StatelessRenderStateKind = StatelessK>,
{
    fn pinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        state: std::pin::Pin<&mut Self::PinnedState<R>>,
        ui_handle: &mut Self::PinnedUiHandle<R>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        PS::poll_render(
            //
            state,
            |v| StatelessRender::stateless_render_update(v, renderer, ui_handle),
            cx,
        )
    }
}

impl<PS, US: StateUnmount + Unpin, VK: ?Sized + ReactiveValueKind, StatelessK: StatelessRenderStateKind> UnpinnedRenderStateKind for StateKind<PS, US, VK>
where
    for<'a> VK::Value<'a>: StatelessRender<StatelessRenderStateKind = StatelessK>,
{
    type UnpinnedUiHandle<R: RenderHtml + ?Sized> = StatelessK::UiHandle<R>;
    type UnpinnedState<R: RenderHtml + ?Sized> = US;
}

impl<PS, US: ReactiveValueState<ReactiveValueKind = VK> + Unpin, VK: ?Sized + ReactiveValueKind, StatelessK: StatelessRenderStateKind> UnpinnedRenderStateKindPollRender for StateKind<PS, US, VK>
where
    for<'a> VK::Value<'a>: StatelessRender<StatelessRenderStateKind = StatelessK>,
{
    fn unpinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        state: &mut Self::UnpinnedState<R>,
        ui_handle: &mut Self::UnpinnedUiHandle<R>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        US::poll_render(
            //
            state,
            |v| StatelessRender::stateless_render_update(v, renderer, ui_handle),
            cx,
        )
    }
}

impl<V: ReactiveValueWithKind, StatelessK: StatelessRenderStateKind> CsrElement for ReactiveValueIntoElement<V>
where
    for<'a> <V::ReactiveValueKind as ReactiveValueKind>::Value<'a>: StatelessRender<StatelessRenderStateKind = StatelessK>,
{
    type RenderStateKind = StateKind<V::PinnedState, V::UnpinnedState, V::ReactiveValueKind>;
    type RenderInitKind = InitKind<V::PinnedRenderInit, V::UnpinnedState, V::ReactiveValueKind>;

    fn pinned_render_init<Renderer: ?Sized + crate::RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
    ) -> (
        //
        crate::element::PinnedStateOfKind<Renderer, Self::RenderStateKind>,
        crate::element::PinnedRenderInitOfKind<Renderer, Self::RenderInitKind>,
    ) {
        1
    }

    fn pinned_render_init_by_reusing<Ctx: ?Sized + crate::HtmlRenderContext>(
        self,
        render_context: &mut Ctx,
        reused_state: std::pin::Pin<&mut crate::element::PinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>>,
        unmounted_ui_handle: crate::element::PinnedUnmountedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
    ) -> crate::element::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        todo!()
    }

    fn pinned_render_update<Renderer: ?Sized + crate::RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        state: std::pin::Pin<&mut crate::element::PinnedStateOfKind<Renderer, Self::RenderStateKind>>,
        ui_handle: &mut crate::element::PinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
        todo!()
    }

    fn unpinned_render_init<Ctx: ?Sized + crate::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
    ) -> (
        //
        crate::element::UnpinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        crate::element::UnpinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
    ) {
        todo!()
    }

    fn unpinned_render_init_by_reusing<Ctx: ?Sized + crate::HtmlRenderContext>(
        self,
        render_context: &mut Ctx,
        reused_state: &mut crate::element::UnpinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        unmounted_ui_handle: crate::element::UnpinnedUnmountedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
    ) -> crate::element::UnpinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        todo!()
    }

    fn unpinned_render_update<Renderer: ?Sized + crate::RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        state: &mut crate::element::UnpinnedStateOfKind<Renderer, Self::RenderStateKind>,
        ui_handle: &mut crate::element::UnpinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
        todo!()
    }
}
