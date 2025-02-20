use std::{marker::PhantomData, pin::Pin};

use frender_common::{
    reactive_value::{ReactiveValueRenderInitPinned, ReactiveValueState, ReactiveValueWithKind, RenderInitPinned, ReusableRendererOfKind},
    value_kind::ValueKind,
};
use frender_dom::csr::{StateUnmount, UnmountedUiHandle};

use crate::{
    element::{CsrElement, PinnedRenderStateKind, PinnedRenderStateKindPollRender, UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender},
    stateless_render::{StatelessRender, StatelessRenderStateKind, StatelessUiHandleOfKind},
    HtmlRenderContext, RenderHtml,
};

mod known;

pub struct ReactiveValueIntoElement<V: ReactiveValueWithKind>(pub V);

pub struct Kind<PS, US, VK: ?Sized>(super::Kind<(PS, US, VK)>);

pub struct RenderInit<PRI, VK: ?Sized + ValueKind>(PRI, PhantomData<VK>);

impl<
        //
        PRI: ReactiveValueRenderInitPinned<VK, S>,
        VK: ?Sized + ValueKind,
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

impl<PS: StateUnmount, VK: ?Sized + ValueKind, US, StatelessK: StatelessRenderStateKind> PinnedRenderStateKind for Kind<PS, US, VK>
where
    for<'a> VK::Value<'a>: StatelessRender<StatelessRenderStateKind = StatelessK>,
{
    type PinnedUiHandle<R: crate::RenderHtml + ?Sized> = StatelessK::UiHandle<R>;
    type PinnedState<R: crate::RenderHtml + ?Sized> = PS;
}

impl<PS: ReactiveValueState<ReactiveValueKind = VK>, VK: ?Sized + ValueKind, US, StatelessK: StatelessRenderStateKind> PinnedRenderStateKindPollRender for Kind<PS, US, VK>
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

impl<PS, US: StateUnmount + Unpin, VK: ?Sized + ValueKind, StatelessK: StatelessRenderStateKind> UnpinnedRenderStateKind for Kind<PS, US, VK>
where
    for<'a> VK::Value<'a>: StatelessRender<StatelessRenderStateKind = StatelessK>,
{
    type UnpinnedUiHandle<R: RenderHtml + ?Sized> = StatelessK::UiHandle<R>;
    type UnpinnedState<R: RenderHtml + ?Sized> = US;
}

impl<PS, US: ReactiveValueState<ReactiveValueKind = VK> + Unpin, VK: ?Sized + ValueKind, StatelessK: StatelessRenderStateKind> UnpinnedRenderStateKindPollRender for Kind<PS, US, VK>
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
            into_renderer::<VK, R>(renderer, ui_handle),
            cx,
        )
    }
}

fn into_renderer<'a, VK: ?Sized + ValueKindStatelessRender, R: ?Sized + RenderHtml>(
    //
    renderer: &'a mut R,
    ui_handle: &'a mut StatelessUiHandleOfKind<R, VK::ValueKindStatelessRenderStateKind>,
) -> impl 'a + FnMut(VK::Value<'_>) {
    |v| StatelessRender::stateless_render_update(v, renderer, ui_handle)
}

pub trait ValueKindStatelessRender: for<'a> ValueKind<Value<'a>: StatelessRender<StatelessRenderStateKind = Self::ValueKindStatelessRenderStateKind>> {
    type ValueKindStatelessRenderStateKind: StatelessRenderStateKind;
}

impl<VK: ?Sized + for<'a> ValueKind<Value<'a>: StatelessRender<StatelessRenderStateKind = K>>, K: StatelessRenderStateKind> ValueKindStatelessRender for VK {
    type ValueKindStatelessRenderStateKind = K;
}

impl<V: ReactiveValueWithKind> CsrElement for ReactiveValueIntoElement<V>
where
    V::ReactiveValueKind: ValueKindStatelessRender,
{
    type RenderStateKind = Kind<V::PinnedState, V::UnpinnedState, V::ReactiveValueKind>;
    type PinnedRenderInit<R: ?Sized + RenderHtml> = RenderInit<
        //
        V::PinnedRenderInit,
        V::ReactiveValueKind,
    >;

    fn pinned_render_init<Renderer: ?Sized + RenderHtml>(
        //
        self,
        _: &mut Renderer,
    ) -> (
        //
        crate::element::PinnedStateOfKind<Renderer, Self::RenderStateKind>,
        Self::PinnedRenderInit<Renderer>,
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
        _ = self.0.pinned_render_update(
            //
            into_renderer::<V::ReactiveValueKind, Renderer>(renderer, ui_handle),
            state,
        )
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
        self.0.unpinned_render_init(|v| StatelessRender::stateless_render_init(v, render_context))
    }

    fn unpinned_render_init_by_reusing<Ctx: ?Sized + crate::HtmlRenderContext>(
        self,
        render_context: &mut Ctx,
        reused_state: &mut crate::element::UnpinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        unmounted_ui_handle: crate::element::UnpinnedUnmountedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
    ) -> crate::element::UnpinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        let ui_handle = render_context.map_mut_render_context(|render_context| unmounted_ui_handle.mount(render_context));
        self.0.unpinned_render_init_by_reusing(
            ReusableRenderer {
                renderer: render_context.renderer_mut(),
                ui_handle,
            },
            reused_state,
        )
    }

    fn unpinned_render_update<Renderer: ?Sized + crate::RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        state: &mut crate::element::UnpinnedStateOfKind<Renderer, Self::RenderStateKind>,
        ui_handle: &mut crate::element::UnpinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
        _ = self.0.unpinned_render_update(
            //
            into_renderer::<V::ReactiveValueKind, Renderer>(renderer, ui_handle),
            state,
        )
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
        VK: ?Sized + ValueKind,
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
        // TODO: check when debug_assertions
        let _ = provide_value;

        self.ui_handle
    }
}
