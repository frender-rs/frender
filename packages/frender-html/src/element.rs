use std::{pin::Pin, task::Poll};

use frender_common::reactive_value::AsOptionMut;
use frender_dom::{render::RenderContext, ui_handle::UiHandle, StateUnmount};

use crate::RenderHtml;

pub trait HtmlRenderContext: RenderContext<Renderer: RenderHtml> {}
impl<Ctx: ?Sized + RenderContext<Renderer: RenderHtml>> HtmlRenderContext for Ctx {}

pub trait PinnedRenderStateKind {
    /// Ui handles that are renderer-specific and **NOT** pinned in pinned environment.
    type PinnedUiHandle<R: RenderHtml + ?Sized>: UiHandle<R>;

    /// State in pinned environment.
    ///
    /// [`Default`] is required so that the state can be constructed and then pinned before [`CsrElement::pinned_render_init`].
    /// Note that [`UnpinnedRenderStateKind::UnpinnedReactiveState`] requires `Default` for a different reason.
    type PinnedState<R: RenderHtml + ?Sized>: StateUnmount;
    type PinnedStateDefault<R: RenderHtml + ?Sized>: Default + AsOptionMut<Self::PinnedState<R>> + StateUnmount;
}

pub trait PinnedRenderStateKindPollRender: PinnedRenderStateKind {
    fn pinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        state: Pin<&mut Self::PinnedState<R>>,
        ui_handle: &mut Self::PinnedUiHandle<R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()>;
}

pub trait UnpinnedRenderStateKind {
    /// Ui handles that are renderer-specific and not pinned in unpinned environment.
    type UnpinnedUiHandle<R: RenderHtml + ?Sized>: UiHandle<R>;

    /// State in unpinned environment.
    ///
    /// [`Unpin`] is required so that [`StateUnmount`] can be used without defining another unpinned variant
    /// (caller can safely create a `Pin<&mut _>` from unpinned places
    /// and then call [`StateUnmount::state_unmount`]).
    type UnpinnedState<R: RenderHtml + ?Sized>: StateUnmount + Unpin;

    /// [`Default`] is required so that [`RenderStateKind`](CsrElement::RenderStateKind) of `Option<impl CsrElement>`
    /// don't need to wrap `UnpinnedReactiveState` with `Option`.
    /// This optimizes `impl CsrElement for Option<impl CsrElement<RenderStateKind: UnpinnedRenderStateKind<UnpinnedState<_>: Default>>>`.
    /// Note that [`PinnedRenderStateKind::PinnedReactiveState`] requires `Default` for a different reason.
    type UnpinnedStateDefault<R: RenderHtml + ?Sized>: Default + AsOptionMut<Self::UnpinnedState<R>> + StateUnmount + Unpin;
}

pub trait UnpinnedRenderStateKindPollRender: UnpinnedRenderStateKind {
    fn unpinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        state: &mut Self::UnpinnedState<R>,
        ui_handle: &mut Self::UnpinnedUiHandle<R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()>;
}

/// Trait alias for experimental traits [`UnpinnedRenderStateKindPollRender`] + [`PinnedRenderStateKindPollRender`].
pub trait RenderStateKind: UnpinnedRenderStateKindPollRender + PinnedRenderStateKindPollRender {}
impl<K: ?Sized + UnpinnedRenderStateKindPollRender + PinnedRenderStateKindPollRender> RenderStateKind for K {}

pub type PinnedUiHandleOfKind<R, K> = <K as PinnedRenderStateKind>::PinnedUiHandle<R>;
pub type PinnedUnmountedUiHandleOfKind<R, K> = <<K as PinnedRenderStateKind>::PinnedUiHandle<R> as UiHandle<R>>::Unmounted;
pub type PinnedStateOfKind<R, K> = <K as PinnedRenderStateKind>::PinnedState<R>;
pub type PinnedStateDefaultOfKind<R, K> = <K as PinnedRenderStateKind>::PinnedStateDefault<R>;

pub type UnpinnedUiHandleOfKind<R, K> = <K as UnpinnedRenderStateKind>::UnpinnedUiHandle<R>;
pub type UnpinnedUnmountedUiHandleOfKind<R, K> = <<K as UnpinnedRenderStateKind>::UnpinnedUiHandle<R> as UiHandle<R>>::Unmounted;
pub type UnpinnedStateOfKind<R, K> = <K as UnpinnedRenderStateKind>::UnpinnedState<R>;
pub type UnpinnedStateDefaultOfKind<R, K> = <K as UnpinnedRenderStateKind>::UnpinnedStateDefault<R>;

pub trait CsrElement {
    type RenderStateKind: RenderStateKind;

    /// The implementation should _initialize_ `state_default` so that [`AsOptionMut::<PinnedState>::as_option_mut(state_default).is_some()`](AsOptionMut)
    /// or future usage will panic.
    /// Caller of this method cannot consider this requirement as a safety guarantee.
    fn pinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        state_default: Pin<&mut PinnedStateDefaultOfKind<Ctx::Renderer, Self::RenderStateKind>>,
    ) -> PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>;

    fn pinned_render_init_by_reusing<Ctx: ?Sized + HtmlRenderContext>(
        self,
        render_context: &mut Ctx,
        reused_state: Pin<&mut PinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>>,
        unmounted_ui_handle: PinnedUnmountedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
    ) -> PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>;

    fn pinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        state: Pin<&mut PinnedStateOfKind<Renderer, Self::RenderStateKind>>,
        ui_handle: &mut PinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    );

    fn unpinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
    ) -> (
        //
        UnpinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        UnpinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
    );

    fn unpinned_render_init_by_reusing<Ctx: ?Sized + HtmlRenderContext>(
        self,
        render_context: &mut Ctx,
        reused_state: &mut UnpinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        unmounted_ui_handle: UnpinnedUnmountedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
    ) -> UnpinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>;

    fn unpinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        state: &mut UnpinnedStateOfKind<Renderer, Self::RenderStateKind>,
        ui_handle: &mut UnpinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    );
}

#[macro_export]
macro_rules! proxy_csr_element {
    (|$this:pat_param| $expr:expr) => {
        fn pinned_render_init<Ctx: ?Sized + $crate::HtmlRenderContext>(
            //
            self,
            render_context: &mut Ctx,
            states: $crate::__private::PinMutRenderInitStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
        ) -> $crate::__private::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
            let $this = self;
            $expr.pinned_render_init(render_context, states)
        }

        fn unpinned_render_init<Ctx: ?Sized + $crate::HtmlRenderContext>(
            //
            self,
            render_context: &mut Ctx,
        ) -> $crate::__private::UnpinnedRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer> {
            let $this = self;
            $expr.unpinned_render_init(render_context)
        }

        $crate::proxy_csr_element_render_update!(|$this| $expr);
    };
}

#[macro_export]
macro_rules! proxy_csr_element_render_update {
    (|$this:pat_param| $expr:expr) => {
        fn pinned_render_update<Ctx: ?Sized + $crate::HtmlRenderContext>(
            //
            self,
            render_context: &mut Ctx,
            states: $crate::__private::PinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
        ) {
            let $this = self;
            $expr.pinned_render_update(render_context, states)
        }

        fn unpinned_render_update<Ctx: ?Sized + $crate::HtmlRenderContext>(
            //
            self,
            render_context: &mut Ctx,
            states: $crate::__private::UnpinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
        ) {
            let $this = self;
            $expr.unpinned_render_update(render_context, states)
        }
    };
}

#[cfg(any(test, doctest))]
mod tests {
    /// ```compile_fail
    /// # use frender_element::Element;
    /// # fn __(v: (impl Element,impl Element,impl Element,impl Element,impl Element,impl Element,impl Element,impl Element,impl Element,impl Element,impl Element,impl Element,impl Element,)) -> impl Element {
    /// #   v
    /// # }
    /// # _ = __((1,2,3,4,5,6,7,8,9,10,11,12,13));
    /// ```
    enum _TupleMaxElements {}

    #[test]
    fn tuple_max_elements() {
        use crate::Element;
        fn __(
            v: (
                impl Element,
                impl Element,
                impl Element,
                impl Element,
                impl Element,
                impl Element,
                impl Element,
                impl Element,
                impl Element,
                impl Element,
                impl Element,
                impl Element,
            ),
        ) -> impl Element {
            v
        }
        _ = __((1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12));
    }
}
