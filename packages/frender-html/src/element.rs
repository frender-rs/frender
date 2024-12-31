use std::{pin::Pin, task::Poll};

use frender_dom::{render::RenderContext, ui_handle::UiHandle, StateUnmount};

use crate::RenderHtml;

pub trait HtmlRenderContext: RenderContext<Renderer = Self::HtmlRenderer> {
    type HtmlRenderer: RenderHtml + ?Sized;
}

impl<Ctx: ?Sized + RenderContext> HtmlRenderContext for Ctx
where
    Ctx::Renderer: RenderHtml,
{
    type HtmlRenderer = Ctx::Renderer;
}

// pub trait StatePollRender<U: ?Sized, R: ?Sized> {
//     fn state_poll_render(self: Pin<&mut Self>, ui_handle: &mut U, renderer: &mut R) -> Poll<()>;
// }

// + StatePollRender<Self, R>
/// A synonymous trait for [`StatePollRender`] with ui handle as Self.

// impl<UH: ?Sized, S: ?Sized + StatePollRender<Self, R>, R: ?Sized> UiHandlePollRender<S, R> for UH {
//     #[inline(always)]
//     fn ui_handle_poll_render(&mut self, state: &mut S, renderer: &mut R) -> Poll<()> {
//         state.state_poll_render(self, renderer)
//     }
// }

pub trait PinnedRenderStateKind {
    /// Ui handles that are renderer-specific and **NOT** pinned in pinned environment.
    type PinnedUiHandle<R: RenderHtml + ?Sized>: UiHandle<R>;

    // TODO: should `*NonReactiveState` and `*ReactiveState` be merged as one `*State`?
    /// Renderer-specific non-reactive state in pinned environment.
    type PinnedNonReactiveState<R: RenderHtml + ?Sized>: Default;
    /// Renderer-agnostic reactive state in pinned environment.
    ///
    /// [`Default`] is required so that the state can be constructed and then pinned before [`CsrElement::pinned_render_init`].
    /// Note that [`UnpinnedRenderStateKind::UnpinnedReactiveState`] requires `Default` for a different reason.
    type PinnedReactiveState: StateUnmount + Default;
}

pub trait PinnedRenderStateKindPollRender: PinnedRenderStateKind {
    fn pinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        states: PinnedMutRenderStatesOfKind<Self, R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()>;
}

pub trait UnpinnedRenderStateKind {
    /// Ui handles that are renderer-specific and not pinned in unpinned environment.
    type UnpinnedUiHandle<R: RenderHtml + ?Sized>: UiHandle<R>;
    /// Renderer-specific non-reactive state in unpinned environment.
    type UnpinnedNonReactiveState<R: RenderHtml + ?Sized>;
    /// Renderer-agnostic reactive state in unpinned environment.
    ///
    /// [`Default`] is required so that [`RenderStateKind`](CsrElement::RenderStateKind) of `Option<impl CsrElement>`
    /// don't need to wrap `UnpinnedReactiveState` with `Option`.
    /// Note that [`PinnedRenderStateKind::PinnedReactiveState`] requires `Default` for a different reason.
    ///
    /// [`Unpin`] is required so that [`StateUnmount`] can be used
    /// (caller can safely create a `Pin<&mut _>` from unpinned places
    /// and then call [`StateUnmount::state_unmount`]).
    ///
    /// Another solution is to split trait [`StateUnmount`] into pinned and unpinned variants,
    /// then we don't need the `Unpin` bound.
    type UnpinnedReactiveState: StateUnmount + Default + Unpin;
}

pub trait UnpinnedRenderStateKindPollRender: UnpinnedRenderStateKind {
    fn unpinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        states: UnpinnedMutRenderStatesOfKind<Self, R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()>;
}

/// Trait alias for experimental traits [`UnpinnedRenderStateKindPollRender`] + [`PinnedRenderStateKindPollRender`].
pub trait RenderStateKind: UnpinnedRenderStateKindPollRender + PinnedRenderStateKindPollRender {}
impl<K: ?Sized + UnpinnedRenderStateKindPollRender + PinnedRenderStateKindPollRender> RenderStateKind for K {}

pub type PinnedMutRenderStatesOfKind<'a, Kind, Renderer> = RenderStates<
    //
    &'a mut <Kind as PinnedRenderStateKind>::PinnedUiHandle<Renderer>,
    Pin<&'a mut <Kind as PinnedRenderStateKind>::PinnedNonReactiveState<Renderer>>,
    Pin<&'a mut <Kind as PinnedRenderStateKind>::PinnedReactiveState>,
>;

pub type UnpinnedMutRenderStatesOfKind<'a, Kind, Renderer> = RenderStates<
    //
    &'a mut <Kind as UnpinnedRenderStateKind>::UnpinnedUiHandle<Renderer>,
    &'a mut <Kind as UnpinnedRenderStateKind>::UnpinnedNonReactiveState<Renderer>,
    &'a mut <Kind as UnpinnedRenderStateKind>::UnpinnedReactiveState,
>;

pub type UnpinnedRenderStatesOfKind<Kind, Renderer> = RenderStates<
    //
    <Kind as UnpinnedRenderStateKind>::UnpinnedUiHandle<Renderer>,
    <Kind as UnpinnedRenderStateKind>::UnpinnedNonReactiveState<Renderer>,
    <Kind as UnpinnedRenderStateKind>::UnpinnedReactiveState,
>;

pub struct RenderStates<UH, NRS, RS> {
    pub ui_handle: UH,
    pub non_reactive_state: NRS,
    pub reactive_state: RS,
}

pub type PinnedUiHandleOfKind<R, K> = <K as PinnedRenderStateKind>::PinnedUiHandle<R>;
pub type UnpinnedUiHandleOfKind<R, K> = <K as UnpinnedRenderStateKind>::UnpinnedUiHandle<R>;

pub struct PinMutRenderInitStates<'a, NRS, RS> {
    pub non_reactive_state: Pin<&'a mut NRS>,
    pub reactive_state: Pin<&'a mut RS>,
}

pub type PinMutRenderInitStatesOfKind<'a, Kind, Renderer> = PinMutRenderInitStates<'a, <Kind as PinnedRenderStateKind>::PinnedNonReactiveState<Renderer>, <Kind as PinnedRenderStateKind>::PinnedReactiveState>;

pub trait CsrElement {
    type RenderStateKind: RenderStateKind;

    fn pinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        states: PinMutRenderInitStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) -> PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>;

    fn pinned_render_update<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        states: PinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    );

    fn unpinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
    ) -> UnpinnedRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>;

    fn unpinned_render_update<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        states: UnpinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
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
