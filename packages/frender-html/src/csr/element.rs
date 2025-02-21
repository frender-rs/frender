use std::{pin::Pin, task::Poll};

use frender_common::reactive_value::RenderInitPinned;
use frender_dom::csr::{render::RenderContext, StateUnmount, UiHandle};

use crate::html::RenderHtml;

pub trait HtmlRenderContext: RenderContext<Renderer: RenderHtml> {}
impl<Ctx: ?Sized + RenderContext<Renderer: RenderHtml>> HtmlRenderContext for Ctx {}

pub trait PinnedRenderStateKind {
    /// Ui handles that are renderer-specific and **NOT** pinned in pinned environment.
    type PinnedUiHandle<R: RenderHtml + ?Sized>: UiHandle<R>;

    /// State in pinned environment.
    type PinnedState<R: RenderHtml + ?Sized>: StateUnmount;
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

/// Trait alias for experimental traits.
pub trait RenderStateKind: UnpinnedRenderStateKindPollRender + PinnedRenderStateKindPollRender {}
impl<K: ?Sized + UnpinnedRenderStateKindPollRender + PinnedRenderStateKindPollRender> RenderStateKind for K {}

pub type PinnedUiHandleOfKind<R, K> = <K as PinnedRenderStateKind>::PinnedUiHandle<R>;
pub type PinnedUnmountedUiHandleOfKind<R, K> = <<K as PinnedRenderStateKind>::PinnedUiHandle<R> as UiHandle<R>>::Unmounted;
pub type PinnedStateOfKind<R, K> = <K as PinnedRenderStateKind>::PinnedState<R>;

pub type UnpinnedUiHandleOfKind<R, K> = <K as UnpinnedRenderStateKind>::UnpinnedUiHandle<R>;
pub type UnpinnedUnmountedUiHandleOfKind<R, K> = <<K as UnpinnedRenderStateKind>::UnpinnedUiHandle<R> as UiHandle<R>>::Unmounted;
pub type UnpinnedStateOfKind<R, K> = <K as UnpinnedRenderStateKind>::UnpinnedState<R>;

pub trait CsrElement {
    type RenderStateKind: RenderStateKind;

    type PinnedRenderInit<R: ?Sized + RenderHtml>: for<'a, 'b> RenderInitPinned<
        //
        &'a mut R::RenderContext<'b>,
        PinnedStateOfKind<R, Self::RenderStateKind>,
        Output = PinnedUiHandleOfKind<R, Self::RenderStateKind>,
    >;

    /// The caller should make sure mutated `render_context` is then passed to [`RenderInitPinned::render_init_pinned`].
    fn pinned_render_init<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
    ) -> (
        //
        PinnedStateOfKind<Renderer, Self::RenderStateKind>,
        Self::PinnedRenderInit<Renderer>,
    );

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
        fn pinned_render_init<Renderer: ?::core::marker::Sized + $crate::__private::RenderHtml>(
            //
            self,
            renderer: &mut Renderer,
        ) -> (
            //
            $crate::__private::PinnedStateOfKind<Renderer, Self::RenderStateKind>,
            Self::PinnedRenderInit<Renderer>,
        ) {
            let $this = self;
            $expr.pinned_render_init(renderer)
        }

        $crate::proxy_csr_element_without_pinned_render_init!(|$this| $expr);
    };
}

#[macro_export]
macro_rules! proxy_csr_element_without_pinned_render_init {
    (|$this:pat_param| $expr:expr) => {
        fn pinned_render_init_by_reusing<Ctx: ?Sized + $crate::__private::HtmlRenderContext>(
            self,
            render_context: &mut Ctx,
            reused_state: ::core::pin::Pin<&mut $crate::__private::PinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>>,
            unmounted_ui_handle: $crate::__private::PinnedUnmountedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
        ) -> $crate::__private::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
            let $this = self;
            $expr.pinned_render_init_by_reusing(render_context, reused_state, unmounted_ui_handle)
        }

        fn unpinned_render_init<Ctx: ?Sized + $crate::__private::HtmlRenderContext>(
            //
            self,
            render_context: &mut Ctx,
        ) -> (
            $crate::__private::UnpinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
            $crate::__private::UnpinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
        ) {
            let $this = self;
            $expr.unpinned_render_init(render_context)
        }

        fn unpinned_render_init_by_reusing<Ctx: ?Sized + $crate::__private::HtmlRenderContext>(
            self,
            render_context: &mut Ctx,
            reused_state: &mut $crate::__private::UnpinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
            unmounted_ui_handle: $crate::__private::UnpinnedUnmountedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
        ) -> $crate::__private::UnpinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
            let $this = self;
            $expr.unpinned_render_init_by_reusing(render_context, reused_state, unmounted_ui_handle)
        }

        $crate::proxy_csr_element_render_update!(|$this| $expr);
    };
}

#[macro_export]
macro_rules! proxy_csr_element_render_update {
    (|$this:pat_param| $expr:expr) => {
        fn pinned_render_update<Renderer: ?Sized + $crate::__private::RenderHtml>(
            //
            self,
            renderer: &mut Renderer,
            state: ::core::pin::Pin<&mut $crate::__private::PinnedStateOfKind<Renderer, Self::RenderStateKind>>,
            ui_handle: &mut $crate::__private::PinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
        ) {
            let $this = self;
            $expr.pinned_render_update(renderer, state, ui_handle)
        }

        fn unpinned_render_update<Renderer: ?Sized + $crate::__private::RenderHtml>(
            //
            self,
            renderer: &mut Renderer,
            state: &mut $crate::__private::UnpinnedStateOfKind<Renderer, Self::RenderStateKind>,
            ui_handle: &mut $crate::__private::UnpinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
        ) {
            let $this = self;
            $expr.unpinned_render_update(renderer, state, ui_handle)
        }
    };
}

#[cfg(any(test, doctest))]
mod tests {
    /// ```compile_fail,E0277
    /// # use frender_html::CsrElement as Element;
    /// # fn __(v: (impl Element,impl Element,impl Element,impl Element,impl Element,impl Element,impl Element,impl Element,impl Element,impl Element,impl Element,impl Element,impl Element,)) -> impl Element {
    /// #   v
    /// # }
    /// # _ = __((1,2,3,4,5,6,7,8,9,10,11,12,13));
    /// ```
    enum _TupleMaxElements {}

    #[test]
    fn tuple_max_elements() {
        use crate::csr::CsrElement as Element;
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
