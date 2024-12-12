use std::{pin::Pin, task::Poll};

use frender_dom::render_state::array::ArrayRenderState;

use crate::{
    element::{PinMutRenderInitStates, PinnedRenderStateKind, PinnedRenderStateKindPollRender, RenderStates, UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender},
    kinds::UiHandleWithNonReactiveState,
    CsrElement, HtmlRenderContext, RenderHtml, StateUnmount,
};

// region: kind

pub struct Kind<K, const N: usize>(super::Kind<K>);

impl<K: UnpinnedRenderStateKind, const N: usize> UnpinnedRenderStateKind for Kind<K, N> {
    type UnpinnedUiHandle<R: RenderHtml + ?Sized> = [UiHandleWithNonReactiveState<K::UnpinnedUiHandle<R>, K::UnpinnedNonReactiveState<R>>; N];
    type UnpinnedNonReactiveState<R: RenderHtml + ?Sized> = ();
    type UnpinnedReactiveState = ArrayRenderState<K::UnpinnedReactiveState, N>;
}

impl<K: UnpinnedRenderStateKindPollRender, const N: usize> UnpinnedRenderStateKindPollRender for Kind<K, N> {
    fn unpinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        RenderStates {
            ui_handle,
            non_reactive_state: (),
            reactive_state,
        }: crate::element::UnpinnedMutRenderStatesOfKind<Self, R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        let mut res = Poll::Ready(());

        for (
            //
            UiHandleWithNonReactiveState { ui_handle, non_reactive_state },
            reactive_state,
        ) in ui_handle.iter_mut().zip(reactive_state.0.iter_mut())
        {
            if let Poll::Pending = K::unpinned_poll_render(
                renderer,
                RenderStates {
                    ui_handle,
                    non_reactive_state,
                    reactive_state,
                },
                cx,
            ) {
                res = Poll::Pending
            }
        }

        res
    }
}

impl<K: PinnedRenderStateKind, const N: usize> PinnedRenderStateKind for Kind<K, N> {
    type PinnedUiHandle<R: RenderHtml + ?Sized> = [K::PinnedUiHandle<R>; N];
    type PinnedNonReactiveState<R: RenderHtml + ?Sized> = ArrayRenderState<K::PinnedNonReactiveState<R>, N>;
    type PinnedReactiveState = ArrayRenderState<K::PinnedReactiveState, N>;
}

impl<K: PinnedRenderStateKindPollRender, const N: usize> PinnedRenderStateKindPollRender for Kind<K, N> {
    fn pinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        RenderStates {
            ui_handle,
            non_reactive_state,
            reactive_state,
        }: crate::element::PinnedMutRenderStatesOfKind<Self, R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        let mut res = Poll::Ready(());

        for (
            //
            ui_handle,
            (non_reactive_state, reactive_state),
        ) in ui_handle.iter_mut().zip(non_reactive_state.iter_pin_mut().zip(reactive_state.iter_pin_mut()))
        {
            if let Poll::Pending = K::pinned_poll_render(
                renderer,
                RenderStates {
                    ui_handle,
                    non_reactive_state,
                    reactive_state,
                },
                cx,
            ) {
                res = Poll::Pending
            }
        }

        res
    }
}

// endregion

impl<E: CsrElement, const N: usize> CsrElement for [E; N] {
    type RenderStateKind = Kind<E::RenderStateKind, N>;

    fn pinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        PinMutRenderInitStates { non_reactive_state, reactive_state }: crate::element::PinMutRenderInitStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) -> crate::element::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        let mut states = non_reactive_state.iter_pin_mut().zip(reactive_state.iter_pin_mut());

        // This relies on a documented feature of <[_; N]>::map():
        // > ..., with function f applied to each element in order
        self.map(|this| {
            let (non_reactive_state, reactive_state) = states.next().unwrap();
            this.pinned_render_init(render_context, PinMutRenderInitStates { non_reactive_state, reactive_state })
        })
    }

    fn pinned_render_update<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        RenderStates {
            ui_handle,
            non_reactive_state,
            reactive_state,
        }: crate::element::PinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
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

    fn unpinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
    ) -> crate::element::UnpinnedRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer> {
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

    fn unpinned_render_update<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        RenderStates {
            ui_handle,
            non_reactive_state: (),
            reactive_state: ArrayRenderState(reactive_state),
        }: crate::element::UnpinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
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
