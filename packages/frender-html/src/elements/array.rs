use std::{pin::Pin, task::Poll};

use frender_common::utils::pin_project_iter_mut_array;

use crate::{
    element::{self, PinnedRenderStateKind, PinnedRenderStateKindPollRender, UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender},
    CsrElement, HtmlRenderContext, RenderHtml,
};

// region: kind

pub struct Kind<K, const N: usize>(super::Kind<K>);

impl<K: UnpinnedRenderStateKind, const N: usize> UnpinnedRenderStateKind for Kind<K, N> {
    type UnpinnedUiHandle<R: RenderHtml + ?Sized> = [K::UnpinnedUiHandle<R>; N];
    type UnpinnedState<R: RenderHtml + ?Sized> = [K::UnpinnedState<R>; N];
}

impl<K: UnpinnedRenderStateKindPollRender, const N: usize> UnpinnedRenderStateKindPollRender for Kind<K, N> {
    fn unpinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        state: &mut Self::UnpinnedState<R>,
        ui_handle: &mut Self::UnpinnedUiHandle<R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        crate::utils::poll_each(
            ui_handle
                .iter_mut()
                .zip(state.iter_mut())
                //
                .map(|(ui_handle, state)| K::unpinned_poll_render(renderer, state, ui_handle, cx)),
        )
    }
}

impl<K: PinnedRenderStateKind, const N: usize> PinnedRenderStateKind for Kind<K, N> {
    type PinnedUiHandle<R: RenderHtml + ?Sized> = [K::PinnedUiHandle<R>; N];
    type PinnedState<R: RenderHtml + ?Sized> = [K::PinnedState<R>; N];
}

impl<K: PinnedRenderStateKindPollRender, const N: usize> PinnedRenderStateKindPollRender for Kind<K, N> {
    fn pinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        state: Pin<&mut Self::PinnedState<R>>,
        ui_handle: &mut Self::PinnedUiHandle<R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        crate::utils::poll_each(
            ui_handle
                .iter_mut()
                .zip(pin_project_iter_mut_array(state))
                .map(|(ui_handle, state)| K::pinned_poll_render(renderer, state, ui_handle, cx)),
        )
    }
}

// endregion

impl<E: CsrElement, const N: usize> CsrElement for [E; N] {
    type RenderStateKind = Kind<E::RenderStateKind, N>;
    type PinnedRenderInit<R: ?Sized + RenderHtml> = [E::PinnedRenderInit<R>; N];

    fn pinned_render_init<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
    ) -> (
        //
        element::PinnedStateOfKind<Renderer, Self::RenderStateKind>,
        Self::PinnedRenderInit<Renderer>,
    ) {
        use arrayvec::ArrayVec;

        let (states, render_inits) = self
            .into_iter()
            //
            .map(|el| el.pinned_render_init(renderer))
            .unzip::<_, _, ArrayVec<_, N>, ArrayVec<_, N>>();

        let (
            //
            Ok(states),
            Ok(render_inits),
        ) = (states.into_inner(), render_inits.into_inner())
        else {
            unreachable!()
        };

        (states, render_inits)
    }

    fn pinned_render_init_by_reusing<Ctx: ?Sized + HtmlRenderContext>(
        self,
        render_context: &mut Ctx,
        reused_state: Pin<&mut element::PinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>>,
        unmounted_ui_handle: element::PinnedUnmountedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
    ) -> element::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        let reused_state = pin_project_iter_mut_array(reused_state);
        let mut iter = reused_state.zip(unmounted_ui_handle);
        self.map(|el| {
            let (reused_state, unmounted_ui_handle) = iter.next().unwrap();

            el.pinned_render_init_by_reusing(render_context, reused_state, unmounted_ui_handle)
        })
    }

    fn pinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        state: Pin<&mut element::PinnedStateOfKind<Renderer, Self::RenderStateKind>>,
        ui_handle: &mut element::PinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
        self.into_iter()
            //
            .zip(pin_project_iter_mut_array(state))
            .zip(ui_handle)
            .for_each(|((el, state), ui_handle)| el.pinned_render_update(renderer, state, ui_handle))
    }

    fn unpinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
    ) -> (
        //
        element::UnpinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        element::UnpinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
    ) {
        use arrayvec::ArrayVec;

        let (states, ui_handles) = self
            .into_iter()
            //
            .map(|el| el.unpinned_render_init(render_context))
            .unzip::<_, _, ArrayVec<_, N>, ArrayVec<_, N>>();

        let (
            //
            Ok(states),
            Ok(ui_handles),
        ) = (states.into_inner(), ui_handles.into_inner())
        else {
            unreachable!()
        };

        (states, ui_handles)
    }

    fn unpinned_render_init_by_reusing<Ctx: ?Sized + HtmlRenderContext>(
        self,
        render_context: &mut Ctx,
        reused_state: &mut element::UnpinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        unmounted_ui_handle: element::UnpinnedUnmountedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
    ) -> element::UnpinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        let mut iter = reused_state.iter_mut().zip(unmounted_ui_handle.into_iter());
        self.map(|el| {
            let (reused_state, unmounted_ui_handle) = iter.next().unwrap();
            el.unpinned_render_init_by_reusing(render_context, reused_state, unmounted_ui_handle)
        })
    }

    fn unpinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        state: &mut element::UnpinnedStateOfKind<Renderer, Self::RenderStateKind>,
        ui_handle: &mut element::UnpinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
        self.into_iter()
            .zip(state)
            .zip(ui_handle)
            .for_each(|((el, state), ui_handle)| el.unpinned_render_update(renderer, state, ui_handle))
    }
}
