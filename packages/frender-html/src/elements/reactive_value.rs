use std::{marker::PhantomData, pin::Pin};

use frender_common::reactive_value::{ReactiveValue, ReactiveValueKind, ReactiveValueRenderInitPinned, ReactiveValueState, RenderInitPinned, ReusableRendererOfKind};
use frender_dom::{ui_handle::UnmountedUiHandle, StateUnmount};

use crate::{
    element::{CsrElement, PinnedRenderStateKind, PinnedRenderStateKindPollRender, UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender},
    stateless_render::{StatelessRender, StatelessRenderStateKind},
    HtmlRenderContext, RenderHtml,
};

mod known;

pub trait ReactiveValueWithKind: ReactiveValue<Self::ReactiveValueKind> {
    type ReactiveValueKind: ?Sized + ReactiveValueKind;
}

pub struct ReactiveValueIntoElement<V: ReactiveValueWithKind>(pub V);

struct Kind<PS, US, VK: ?Sized>(super::Kind<(PS, US, VK)>);

struct RenderInit<PRI, VK: ?Sized + ReactiveValueKind>(PRI, PhantomData<VK>);

impl<
        //
        PRI: ReactiveValueRenderInitPinned<VK, S>,
        VK: ?Sized + ReactiveValueKind,
        Ctx: ?Sized + HtmlRenderContext,
        UH,
        S: ?Sized,
    > RenderInitPinned<&mut Ctx, S> for RenderInit<PRI, VK>
where
    for<'a> VK::Value<'a>: StatelessRender<StatelessRenderStateKind: StatelessRenderStateKind<UiHandle<Ctx::Renderer> = UH>>,
{
    type Output = UH;

    fn render_init_pinned(self, render_context: &mut Ctx, state: Pin<&mut S>) -> Self::Output {
        PRI::RenderInitPinned::from(self.0).render_init_pinned(
            //
            |v| StatelessRender::stateless_render_init(v, render_context),
            state,
        )
    }
}

impl<PS: StateUnmount, VK: ?Sized + ReactiveValueKind, US, StatelessK: StatelessRenderStateKind> PinnedRenderStateKind for Kind<PS, US, VK>
where
    for<'a> VK::Value<'a>: StatelessRender<StatelessRenderStateKind = StatelessK>,
{
    type PinnedUiHandle<R: crate::RenderHtml + ?Sized> = StatelessK::UiHandle<R>;
    type PinnedState<R: crate::RenderHtml + ?Sized> = PS;
}

impl<PS: ReactiveValueState<ReactiveValueKind = VK>, VK: ?Sized + ReactiveValueKind, US, StatelessK: StatelessRenderStateKind> PinnedRenderStateKindPollRender for Kind<PS, US, VK>
where
    for<'a> VK::Value<'a>: StatelessRender<StatelessRenderStateKind = StatelessK>,
{
    fn pinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        state: Pin<&mut Self::PinnedState<R>>,
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

impl<PS, US: StateUnmount + Unpin, VK: ?Sized + ReactiveValueKind, StatelessK: StatelessRenderStateKind> UnpinnedRenderStateKind for Kind<PS, US, VK>
where
    for<'a> VK::Value<'a>: StatelessRender<StatelessRenderStateKind = StatelessK>,
{
    type UnpinnedUiHandle<R: RenderHtml + ?Sized> = StatelessK::UiHandle<R>;
    type UnpinnedState<R: RenderHtml + ?Sized> = US;
}

impl<PS, US: ReactiveValueState<ReactiveValueKind = VK> + Unpin, VK: ?Sized + ReactiveValueKind, StatelessK: StatelessRenderStateKind> UnpinnedRenderStateKindPollRender for Kind<PS, US, VK>
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
            Pin::new(state),
            |v| StatelessRender::stateless_render_update(v, renderer, ui_handle),
            cx,
        )
    }
}

impl<V: ReactiveValueWithKind, StatelessK: StatelessRenderStateKind> CsrElement for ReactiveValueIntoElement<V>
where
    for<'a> <V::ReactiveValueKind as ReactiveValueKind>::Value<'a>: StatelessRender<StatelessRenderStateKind = StatelessK>,
{
    type RenderStateKind = Kind<V::PinnedState, V::UnpinnedState, V::ReactiveValueKind>;
    type PinnedRenderInit<R: ?Sized + RenderHtml> = RenderInit<
        //
        V::PinnedRenderInit,
        V::ReactiveValueKind,
    >;

    fn pinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        _: &mut Ctx,
    ) -> (
        //
        crate::element::PinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        Self::PinnedRenderInit<Ctx::Renderer>,
    ) {
        let (state, render_init) = self.0.pinned_render_init();
        (state, RenderInit(render_init, PhantomData))
    }

    fn pinned_render_init_by_reusing<Ctx: ?Sized + crate::HtmlRenderContext>(
        self,
        render_context: &mut Ctx,
        reused_state: Pin<&mut crate::element::PinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>>,
        unmounted_ui_handle: crate::element::PinnedUnmountedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
    ) -> crate::element::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        let ui_handle = render_context.map_mut_render_context(|render_context| unmounted_ui_handle.mount(render_context));
        self.0.pinned_render_init_by_reusing(
            ReusableRenderer {
                renderer: render_context.renderer_mut(),
                ui_handle,
            },
            reused_state,
        )
    }

    fn pinned_render_update<Renderer: ?Sized + crate::RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        state: Pin<&mut crate::element::PinnedStateOfKind<Renderer, Self::RenderStateKind>>,
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

struct ReusableRenderer<'a, R: ?Sized, UH> {
    ui_handle: UH,
    renderer: &'a mut R,
}

impl<
        //
        R: ?Sized + RenderHtml,
        UH,
        VK: ?Sized + ReactiveValueKind,
        StatelessK: StatelessRenderStateKind<UiHandle<R> = UH>,
    > ReusableRendererOfKind<VK> for ReusableRenderer<'_, R, UH>
where
    for<'a> VK::Value<'a>: StatelessRender<StatelessRenderStateKind = StatelessK>,
{
    type Output = UH;

    fn render(mut self, value: VK::Value<'_>) -> Self::Output {
        StatelessRender::stateless_render_update(value, self.renderer, &mut self.ui_handle);
        self.ui_handle
    }

    fn reuse(self, provide_value: impl frender_common::reactive_value::ProvideValueOfKind<VK>) -> Self::Output {
        // TODO: check if debug_assertions
        let _ = provide_value;

        self.ui_handle
    }
}
