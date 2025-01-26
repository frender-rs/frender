use std::{pin::Pin, task::Poll};

use frender_common::utils::pin_project_iter_mut_array;
use frender_dom::render_state::array::ArrayRenderState;

use crate::{
    element::{self, PinnedRenderStateKind, PinnedRenderStateKindPollRender, UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender},
    kinds::UiHandleWithNonReactiveState,
    CsrElement, HtmlRenderContext, RenderHtml,
};

// region: kind

pub struct Kind<K, const N: usize>(super::Kind<K>);

impl<K: UnpinnedRenderStateKind, const N: usize> UnpinnedRenderStateKind for Kind<K, N> {
    type UnpinnedUiHandle<R: RenderHtml + ?Sized> = [K::UnpinnedUiHandle<R>; N];
    type UnpinnedState<R: RenderHtml + ?Sized> = [K::UnpinnedState<R>; N];

    // We have to believe `Option<Self::UnpinnedState<R>>` is better than `[Self::UnpinnedStateDefault<R>; N]` in most cases. The latter requires CsrElementStateDefault.
    // This might be optimized with an assoc type `trait CsrElement { type UnpinnedStateDefaultArray<const N: usize>: AsOptionMut<[Self::UnpinnedState<R>; N]>; }` but I think that's too much.
    type UnpinnedStateDefault<R: RenderHtml + ?Sized> = Option<Self::UnpinnedState<R>>;
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
    type PinnedStateDefault<R: RenderHtml + ?Sized> = [K::PinnedStateDefault<R>; N];
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

    fn pinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        state_default: Pin<&mut element::PinnedStateDefaultOfKind<Ctx::Renderer, Self::RenderStateKind>>,
    ) -> element::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        let mut states = non_reactive_state.iter_pin_mut().zip(reactive_state.iter_pin_mut());

        // This relies on a documented feature of <[_; N]>::map():
        // > ..., with function f applied to each element in order
        self.map(|this| {
            let (non_reactive_state, reactive_state) = states.next().unwrap();
            this.pinned_render_init(render_context, PinMutRenderInitStates { non_reactive_state, reactive_state })
        })
    }

    fn pinned_render_init_by_reusing<Ctx: ?Sized + HtmlRenderContext>(
        self,
        render_context: &mut Ctx,
        reused_state: Pin<&mut element::PinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>>,
        unmounted_ui_handle: element::PinnedUnmountedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
    ) -> element::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        todo!()
    }

    fn pinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        state: Pin<&mut element::PinnedStateOfKind<Renderer, Self::RenderStateKind>>,
        ui_handle: &mut element::PinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
        todo!()
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
        todo!()
    }

    fn unpinned_render_init_by_reusing<Ctx: ?Sized + HtmlRenderContext>(
        self,
        render_context: &mut Ctx,
        reused_state: &mut element::UnpinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        unmounted_ui_handle: element::UnpinnedUnmountedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
    ) -> element::UnpinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        todo!()
    }

    fn unpinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        state: &mut element::UnpinnedStateOfKind<Renderer, Self::RenderStateKind>,
        ui_handle: &mut element::UnpinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
        todo!()
    }

    fn pinned_render_init_a<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        PinMutRenderInitStates { non_reactive_state, reactive_state }: element::PinMutRenderInitStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) -> element::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        let mut states = non_reactive_state.iter_pin_mut().zip(reactive_state.iter_pin_mut());

        // This relies on a documented feature of <[_; N]>::map():
        // > ..., with function f applied to each element in order
        self.map(|this| {
            let (non_reactive_state, reactive_state) = states.next().unwrap();
            this.pinned_render_init(render_context, PinMutRenderInitStates { non_reactive_state, reactive_state })
        })
    }

    fn pinned_render_update_<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        RenderStates {
            ui_handle,
            non_reactive_state,
            reactive_state,
        }: element::PinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) {
        self.into_iter()
            .zip(ui_handle.iter_mut().zip(non_reactive_state.iter_pin_mut().zip(reactive_state.iter_pin_mut())))
            .for_each(|(this, (ui_handle, (non_reactive_state, reactive_state)))| {
                this.pinned_render_update(
                    render_context,
                    RenderStates {
                        ui_handle,
                        non_reactive_state,
                        reactive_state,
                    },
                )
            })
    }

    fn unpinned_render_init_<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
    ) -> element::UnpinnedRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer> {
        let mut reactive_states = ArrayRenderState::default();

        let ui_handle = {
            let mut reactive_states = reactive_states.0.iter_mut();
            self.map(|this| {
                let RenderStates {
                    ui_handle,
                    non_reactive_state,
                    reactive_state,
                } = this.unpinned_render_init(render_context);
                *reactive_states.next().unwrap() = reactive_state;
                UiHandleWithNonReactiveState { ui_handle, non_reactive_state }
            })
        };
        RenderStates {
            ui_handle,
            non_reactive_state: (),
            reactive_state: reactive_states,
        }
    }

    fn unpinned_render_update_<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        RenderStates {
            ui_handle,
            non_reactive_state: (),
            reactive_state: ArrayRenderState(reactive_state),
        }: element::UnpinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) {
        self.into_iter()
            .zip(ui_handle.iter_mut().zip(reactive_state.iter_mut()))
            .for_each(|(this, (UiHandleWithNonReactiveState { ui_handle, non_reactive_state }, reactive_state))| {
                this.unpinned_render_update(
                    render_context,
                    RenderStates {
                        ui_handle,
                        non_reactive_state,
                        reactive_state,
                    },
                )
            })
    }
}
