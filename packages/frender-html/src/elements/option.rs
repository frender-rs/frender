use std::{pin::Pin, task::Poll};

use frender_dom::{
    render::RenderContext,
    ui_handle::{UiHandle, UnmountedUiHandle},
    StateUnmount as _,
};

use crate::{
    element::{PinMutRenderInitStates, PinnedRenderStateKind, PinnedRenderStateKindPollRender, RenderStates, UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender},
    kinds::UiHandleWithNonReactiveState,
    CsrElement, HtmlRenderContext, RenderHtml,
};

// region: ui handle

pub enum UiHandleMaybe<M, U> {
    BeforeMounted,
    Mounted(M),
    Unmounted(U),
}

impl<M, U> UiHandleMaybe<M, U> {
    fn mount_unmounted<Ctx: ?Sized + HtmlRenderContext>(&mut self, render_context: &mut Ctx) -> &mut M
    where
        U: UnmountedUiHandle<Ctx::Renderer, Mounted = M>,
    {
        let UiHandleMaybe::Unmounted(unmounted) = self.take() else { unreachable!() };

        *self = render_context.map_mut_render_context(|render_context| Self::Mounted(unmounted.mount(render_context)));

        match self {
            UiHandleMaybe::Mounted(mounted) => mounted,
            _ => unreachable!(),
        }
    }

    pub fn take(&mut self) -> Self {
        std::mem::replace(self, Self::BeforeMounted)
    }

    /// Returns `true` if mounted in this call.
    pub fn mount_in_place<Ctx: ?Sized>(&mut self, render_context: &mut Ctx) -> bool
    where
        Ctx: RenderContext,
        U: UnmountedUiHandle<Ctx::Renderer, Mounted = M>,
    {
        match self {
            UiHandleMaybe::Unmounted(_) => {
                let UiHandleMaybe::Unmounted(unmounted) = self.take() else { unreachable!() };
                *self = Self::Mounted(render_context.map_mut_render_context(|render_context| unmounted.mount(render_context)));
                true
            }
            _ => false,
        }
    }

    /// Returns `true` if unmounted in this call.
    /// Returns `false` if already unmounted before.
    pub fn unmount_in_place<R: ?Sized>(&mut self, renderer: &mut R) -> bool
    where
        M: UiHandle<R, Unmounted = U>,
    {
        match self {
            UiHandleMaybe::Mounted(_) => {
                let UiHandleMaybe::Mounted(mounted) = self.take() else { unreachable!() };
                *self = Self::Unmounted(mounted.unmount(renderer));
                true
            }
            _ => false,
        }
    }
}

pub type UiHandleMaybeMounted<M, R> = UiHandleMaybe<M, <M as UiHandle<R>>::Unmounted>;

pub struct UnmountedUiHandleMaybe<U>(Option<U>);

impl<U, R: ?Sized> UnmountedUiHandle<R> for UnmountedUiHandleMaybe<U>
where
    U: UnmountedUiHandle<R>,
{
    type Mounted = UiHandleMaybe<U::Mounted, U>;

    fn mount(self, render_context: &mut <R>::RenderContext<'_>) -> Self::Mounted
    where
        R: frender_dom::render::RenderWithContext,
    {
        match self.0 {
            Some(unmounted) => UiHandleMaybe::Mounted(unmounted.mount(render_context)),
            None => UiHandleMaybe::BeforeMounted,
        }
    }
}

impl<M, U, R: ?Sized> UiHandle<R> for UiHandleMaybe<M, U>
where
    M: UiHandle<R, Unmounted = U>,
    U: UnmountedUiHandle<R, Mounted = M>,
{
    type Unmounted = UnmountedUiHandleMaybe<U>;

    fn unmount(self, renderer: &mut R) -> <Self as UiHandle<R>>::Unmounted {
        match self {
            UiHandleMaybe::BeforeMounted => UnmountedUiHandleMaybe(None),
            UiHandleMaybe::Mounted(mounted) => UnmountedUiHandleMaybe(Some(mounted.unmount(renderer))),
            UiHandleMaybe::Unmounted(unmounted) => UnmountedUiHandleMaybe(Some(unmounted)),
        }
    }

    fn reposition(&mut self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: frender_dom::render::RenderWithContext,
    {
        if let UiHandleMaybe::Mounted(m) = self {
            m.reposition(render_context)
        }
    }

    fn check_and_move_cursor(&self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: frender_dom::render::RenderWithContext,
    {
        if let UiHandleMaybe::Mounted(m) = self {
            m.check_and_move_cursor(render_context);
        }
    }

    fn assert_cursor_is_at_self(&self, render_context: &<R>::RenderContext<'_>)
    where
        R: frender_dom::render::RenderWithContext,
    {
        if let UiHandleMaybe::Mounted(m) = self {
            m.assert_cursor_is_at_self(render_context);
        }
    }
}

// endregion

// region: kind

pub struct Kind<K>(super::Kind<K>);

impl<K: UnpinnedRenderStateKind> UnpinnedRenderStateKind for Kind<K> {
    type UnpinnedUiHandle<R: RenderHtml + ?Sized> = Option<UiHandleWithNonReactiveState<K::UnpinnedUiHandle<R>, K::UnpinnedNonReactiveState<R>>>;
    type UnpinnedNonReactiveState<R: RenderHtml + ?Sized> = ();
    type UnpinnedReactiveState = K::UnpinnedReactiveState;
}

impl<K: UnpinnedRenderStateKindPollRender> UnpinnedRenderStateKindPollRender for Kind<K> {
    fn unpinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        states: crate::element::UnpinnedMutRenderStatesOfKind<Self, R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        let RenderStates {
            ui_handle,
            non_reactive_state: (),
            reactive_state,
        } = states;

        if let Some(UiHandleWithNonReactiveState { ui_handle, non_reactive_state }) = ui_handle {
            K::unpinned_poll_render(
                renderer,
                RenderStates {
                    ui_handle,
                    non_reactive_state,
                    reactive_state,
                },
                cx,
            )
        } else {
            Poll::Ready(())
        }
    }
}

impl<K: PinnedRenderStateKind> PinnedRenderStateKind for Kind<K> {
    type PinnedUiHandle<R: RenderHtml + ?Sized> = Option<K::PinnedUiHandle<R>>;
    type PinnedNonReactiveState<R: RenderHtml + ?Sized> = K::PinnedNonReactiveState<R>;
    type PinnedReactiveState = K::PinnedReactiveState;
}

impl<K: PinnedRenderStateKindPollRender> PinnedRenderStateKindPollRender for Kind<K> {
    fn pinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        states: crate::element::PinnedMutRenderStatesOfKind<Self, R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        let RenderStates {
            ui_handle,
            non_reactive_state,
            reactive_state,
        } = states;

        if let Some(ui_handle) = ui_handle {
            K::pinned_poll_render(
                renderer,
                RenderStates {
                    ui_handle,
                    non_reactive_state,
                    reactive_state,
                },
                cx,
            )
        } else {
            Poll::Ready(())
        }
    }
}

// endregion

impl<E: CsrElement> CsrElement for Option<E> {
    type RenderStateKind = Kind<E::RenderStateKind>;

    fn pinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        states: crate::element::PinMutRenderInitStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) -> crate::element::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        match self {
            Some(this) => Some(this.pinned_render_init(render_context, states)),
            None => None,
        }
    }

    fn pinned_render_update<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        RenderStates {
            ui_handle,
            mut non_reactive_state,
            reactive_state,
        }: crate::element::PinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) {
        match (self, ui_handle) {
            (None, ui_handle) => {
                if let Some(ui_handle) = ui_handle.take() {
                    // NonReactiveState is dropped and set to default
                    non_reactive_state.set(Default::default());

                    // ReactiveState is state_unmounted but not set to default
                    reactive_state.state_unmount();

                    // ui handle is unmounted and dropped
                    _ = ui_handle.unmount(render_context.renderer_mut());
                }
            }
            (Some(this), ui_handle @ None) => {
                *ui_handle = Some(this.pinned_render_init(render_context, PinMutRenderInitStates { non_reactive_state, reactive_state }));
            }
            (Some(this), Some(ui_handle)) => this.pinned_render_update(
                render_context,
                RenderStates {
                    ui_handle,
                    non_reactive_state,
                    reactive_state,
                },
            ),
        }
    }

    fn unpinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
    ) -> crate::element::UnpinnedRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer> {
        match self {
            Some(this) => {
                let RenderStates {
                    ui_handle,
                    non_reactive_state,
                    reactive_state,
                } = this.unpinned_render_init(render_context);
                RenderStates {
                    ui_handle: Some(UiHandleWithNonReactiveState { ui_handle, non_reactive_state }),
                    non_reactive_state: (),
                    reactive_state,
                }
            }
            None => RenderStates {
                ui_handle: None,
                non_reactive_state: (),
                reactive_state: Default::default(), // Note default reactive state is used when None.render_init()
            },
        }
    }

    fn unpinned_render_update<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        states: crate::element::UnpinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) {
        let RenderStates {
            ui_handle,
            non_reactive_state: (),
            reactive_state,
        } = states;

        match (self, ui_handle) {
            (None, ui_handle) => {
                if let Some(UiHandleWithNonReactiveState { ui_handle, non_reactive_state }) = ui_handle.take() {
                    // NonReactiveState is dropped
                    drop(non_reactive_state);

                    // ReactiveState is state_unmounted but not set to default
                    Pin::new(reactive_state).state_unmount();

                    // ui handle is unmounted and dropped
                    _ = ui_handle.unmount(render_context.renderer_mut());
                }
            }
            (Some(this), ui_handle @ None) => {
                let states = this.unpinned_render_init(render_context);
                *reactive_state = states.reactive_state;
                *ui_handle = Some(UiHandleWithNonReactiveState {
                    ui_handle: states.ui_handle,
                    non_reactive_state: states.non_reactive_state,
                });
            }
            (Some(this), Some(UiHandleWithNonReactiveState { ui_handle, non_reactive_state })) => this.unpinned_render_update(
                render_context,
                RenderStates {
                    ui_handle,
                    non_reactive_state,
                    reactive_state,
                },
            ),
        }
    }
}
