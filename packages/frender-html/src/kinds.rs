use std::{marker::PhantomData, task::Poll};

use frender_dom::ui_handle::{UiHandle, UnmountedUiHandle};

use crate::{
    element::{PinnedRenderStateKind, PinnedRenderStateKindPollRender, RenderStates, UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender},
    RenderHtml,
};

pub enum KindOfNoState {}

impl PinnedRenderStateKind for KindOfNoState {
    type PinnedUiHandle<R: RenderHtml + ?Sized> = ();
    type PinnedNonReactiveState<R: RenderHtml + ?Sized> = ();
    type PinnedReactiveState = ();
}

impl PinnedRenderStateKindPollRender for KindOfNoState {
    fn pinned_poll_render<R: RenderHtml + ?Sized>(
        //
        _: &mut R,
        _: crate::element::PinnedMutRenderStatesOfKind<Self, R>,
        _: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        Poll::Ready(())
    }
}

impl UnpinnedRenderStateKind for KindOfNoState {
    type UnpinnedUiHandle<R: RenderHtml + ?Sized> = ();
    type UnpinnedNonReactiveState<R: RenderHtml + ?Sized> = ();
    type UnpinnedReactiveState = ();
}

impl UnpinnedRenderStateKindPollRender for KindOfNoState {
    fn unpinned_poll_render<R: RenderHtml + ?Sized>(
        //
        _: &mut R,
        _: crate::element::UnpinnedMutRenderStatesOfKind<Self, R>,
        _: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        Poll::Ready(())
    }
}

// impl<ElType: ?Sized + behavior_type_traits::Element> RenderStateWithPehKind<ElType> for KindOfNoState {
//     type RenderStateWithPeh<R: RenderHtml + ?Sized> = ();
//     type RenderStateWithPehUnpinned<R: RenderHtml + ?Sized> = ();
// }

pub struct KindOfNonReactive<T: Default>(KindOfNoState, PhantomData<T>);

// impl<T: Default, ElType: ?Sized + behavior_type_traits::Element> RenderStateWithPehKind<ElType> for KindOfNonReactive<T> {
//     type RenderStateWithPeh<R: RenderHtml + ?Sized> = NonReactiveRenderState<T>;
//     type RenderStateWithPehUnpinned<R: RenderHtml + ?Sized> = NonReactiveRenderState<T>;
// }

// region: ui handle
pub struct UiHandleWithNonReactiveState<T, C> {
    pub ui_handle: T,
    pub non_reactive_state: C,
}

impl<T, C> UiHandleWithNonReactiveState<T, C> {
    pub fn map_ui_handle<U>(self, f: impl FnOnce(T) -> U) -> UiHandleWithNonReactiveState<U, C> {
        UiHandleWithNonReactiveState {
            ui_handle: f(self.ui_handle),
            non_reactive_state: self.non_reactive_state,
        }
    }
}

impl<T: UnmountedUiHandle<R>, C, R: ?Sized> UnmountedUiHandle<R> for UiHandleWithNonReactiveState<T, C> {
    type Mounted = UiHandleWithNonReactiveState<T::Mounted, C>;

    fn mount(self, render_context: &mut <R>::RenderContext<'_>) -> Self::Mounted
    where
        R: frender_dom::render::RenderWithContext,
    {
        self.map_ui_handle(|text| text.mount(render_context))
    }
}

impl<T: UiHandle<R>, C, R: ?Sized> UiHandle<R> for UiHandleWithNonReactiveState<T, C> {
    type Unmounted = UiHandleWithNonReactiveState<T::Unmounted, C>;

    fn unmount(self, renderer: &mut R) -> Self::Unmounted {
        self.map_ui_handle(|text| text.unmount(renderer))
    }

    fn reposition(&mut self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: frender_dom::render::RenderWithContext,
    {
        self.ui_handle.reposition(render_context)
    }

    fn check_and_move_cursor(&self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: frender_dom::render::RenderWithContext,
    {
        self.ui_handle.check_and_move_cursor(render_context)
    }

    fn assert_cursor_is_at_self(&self, render_context: &<R>::RenderContext<'_>)
    where
        R: frender_dom::render::RenderWithContext,
    {
        self.ui_handle.assert_cursor_is_at_self(render_context)
    }
}

// endregion

// region: KindUnpinned
enum Never {}
pub struct KindUnpinned<K: UnpinnedRenderStateKind>(Never, PhantomData<K>);

impl<K: UnpinnedRenderStateKind> UnpinnedRenderStateKind for KindUnpinned<K> {
    type UnpinnedUiHandle<R: RenderHtml + ?Sized> = K::UnpinnedUiHandle<R>;
    type UnpinnedNonReactiveState<R: RenderHtml + ?Sized> = K::UnpinnedNonReactiveState<R>;
    type UnpinnedReactiveState = K::UnpinnedReactiveState;
}

impl<K: UnpinnedRenderStateKindPollRender> UnpinnedRenderStateKindPollRender for KindUnpinned<K> {
    fn unpinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        states: crate::element::UnpinnedMutRenderStatesOfKind<Self, R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        K::unpinned_poll_render(renderer, states, cx)
    }
}

impl<K: UnpinnedRenderStateKind> PinnedRenderStateKind for KindUnpinned<K> {
    type PinnedUiHandle<R: RenderHtml + ?Sized> = UiHandleWithNonReactiveState<K::UnpinnedUiHandle<R>, K::UnpinnedNonReactiveState<R>>;
    type PinnedNonReactiveState<R: RenderHtml + ?Sized> = ();
    type PinnedReactiveState = K::UnpinnedReactiveState;
}

impl<K: UnpinnedRenderStateKindPollRender> PinnedRenderStateKindPollRender for KindUnpinned<K> {
    fn pinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        states: crate::element::PinnedMutRenderStatesOfKind<Self, R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        let RenderStates {
            ui_handle: UiHandleWithNonReactiveState { ui_handle, non_reactive_state },
            non_reactive_state: _,
            reactive_state,
        } = states;

        K::unpinned_poll_render(
            renderer,
            RenderStates {
                ui_handle,
                non_reactive_state,
                reactive_state: reactive_state.get_mut(),
            },
            cx,
        )
    }
}
// endregion
