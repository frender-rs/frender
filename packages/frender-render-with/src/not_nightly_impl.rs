use std::task::Poll;

use frender_html::{
    dom::{render::RenderWithContext, ui_handle::UiHandle},
    experimental::{
        PinnedRenderStateKind, PinnedRenderStateKindPollRender, RenderStates,
        UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender,
    },
    kinds::UiHandleWithNonReactiveState,
    StateUnmount,
};

use super::*;

// region: dyn safe UiHandle

trait DynSafeUnmountedUiHandle<Renderer: ?Sized> {
    fn mount<'a>(
        self: Box<Self>,
        render_context: &mut Renderer::RenderContext<'_>,
    ) -> Box<dyn 'a + DynSafeUiHandle<Renderer>>
    where
        Self: 'a,
        Renderer: 'a,
        Renderer: RenderWithContext;
}

impl<R: ?Sized, T: UnmountedUiHandle<R>> DynSafeUnmountedUiHandle<R> for T
where
    // Note
    T::Mounted: PollRender<R>,
{
    fn mount<'a>(
        self: Box<Self>,
        render_context: &mut <R>::RenderContext<'_>,
    ) -> Box<dyn 'a + DynSafeUiHandle<R>>
    where
        Self: 'a,
        R: 'a,
        R: RenderWithContext,
    {
        Box::new(T::mount(*self, render_context))
    }
}

// Note the `PollRender<Renderer>` bound
trait DynSafeUiHandle<Renderer: ?Sized>: PollRender<Renderer> {
    fn upcast_mut<'a>(&mut self) -> &mut (dyn 'a + Any)
    where
        Self: 'a;

    fn unmount<'a>(
        self: Box<Self>,
        renderer: &mut Renderer,
    ) -> Box<dyn 'a + DynSafeUnmountedUiHandle<Renderer>>
    where
        Self: 'a,
        Renderer: 'a;

    fn reposition(&mut self, render_context: &mut Renderer::RenderContext<'_>)
    where
        Renderer: RenderWithContext;

    fn check_and_move_cursor(&self, render_context: &mut Renderer::RenderContext<'_>)
    where
        Renderer: RenderWithContext;

    fn assert_cursor_is_at_self(&self, render_context: &Renderer::RenderContext<'_>)
    where
        Renderer: RenderWithContext;
}

impl<R: ?Sized, T: UiHandle<R> + PollRender<R>> DynSafeUiHandle<R> for T {
    fn upcast_mut<'a>(&mut self) -> &mut (dyn 'a + Any)
    where
        Self: 'a,
    {
        self
    }

    fn unmount<'a>(self: Box<Self>, renderer: &mut R) -> Box<dyn 'a + DynSafeUnmountedUiHandle<R>>
    where
        Self: 'a,
        R: 'a,
    {
        Box::new(T::unmount(*self, renderer))
    }

    fn reposition(&mut self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: RenderWithContext,
    {
        T::reposition(self, render_context)
    }

    fn check_and_move_cursor(&self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: RenderWithContext,
    {
        T::check_and_move_cursor(self, render_context)
    }

    fn assert_cursor_is_at_self(&self, render_context: &<R>::RenderContext<'_>)
    where
        R: RenderWithContext,
    {
        T::assert_cursor_is_at_self(self, render_context)
    }
}

// endregion
// region: BoxDynUiHandle

pub struct BoxDynUiHandle<'a, R: 'a + ?Sized>(Box<dyn 'a + DynSafeUiHandle<R>>);
pub struct BoxDynUnmountedUiHandle<'a, R: 'a + ?Sized>(Box<dyn 'a + DynSafeUnmountedUiHandle<R>>);

impl<'a, R: 'a + ?Sized> UnmountedUiHandle<R> for BoxDynUnmountedUiHandle<'a, R> {
    type Mounted = BoxDynUiHandle<'a, R>;

    fn mount(self, render_context: &mut <R>::RenderContext<'_>) -> Self::Mounted
    where
        R: RenderWithContext,
    {
        BoxDynUiHandle(self.0.mount(render_context))
    }
}

impl<'a, R: 'a + ?Sized> UiHandle<R> for BoxDynUiHandle<'a, R> {
    type Unmounted = BoxDynUnmountedUiHandle<'a, R>;

    fn unmount(self, renderer: &mut R) -> Self::Unmounted {
        BoxDynUnmountedUiHandle(self.0.unmount(renderer))
    }

    fn reposition(&mut self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: RenderWithContext,
    {
        self.0.reposition(render_context)
    }

    fn check_and_move_cursor(&self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: RenderWithContext,
    {
        self.0.check_and_move_cursor(render_context)
    }

    fn assert_cursor_is_at_self(&self, render_context: &<R>::RenderContext<'_>)
    where
        R: RenderWithContext,
    {
        self.0.assert_cursor_is_at_self(render_context)
    }
}

// endregion
// region: ReactiveState

trait AnyStateUnmount: StateUnmount {
    fn upcast_mut<'a>(&mut self) -> &mut (dyn 'a + Any)
    where
        Self: 'a;
}
impl<T: StateUnmount> AnyStateUnmount for T {
    fn upcast_mut<'a>(&mut self) -> &mut (dyn 'a + Any)
    where
        Self: 'a,
    {
        self
    }
}

pub struct OptionBoxReactiveState(Option<Box<dyn AnyStateUnmount + Unpin>>);

impl Default for OptionBoxReactiveState {
    fn default() -> Self {
        Self(None)
    }
}

impl StateUnmount for OptionBoxReactiveState {
    fn state_unmount(self: Pin<&mut Self>) {
        if let Some(this) = &mut self.get_mut().0 {
            Pin::new(this.as_mut()).state_unmount();
        }
    }
}

// endregion
// region: dyn safe poll_render

struct TypedReactiveState<S, K> {
    reactive_state: S,
    kind: PhantomData<K>,
}

impl<S, K> Unpin for TypedReactiveState<S, K> {}

impl<S: StateUnmount + Unpin, K> StateUnmount for TypedReactiveState<S, K> {
    fn state_unmount(self: Pin<&mut Self>) {
        S::state_unmount(Pin::new(&mut self.get_mut().reactive_state))
    }
}

trait PollRender<R: ?Sized> {
    fn poll_render(
        //
        &mut self,
        renderer: &mut R,
        reactive_state: &mut OptionBoxReactiveState,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()>;
}

type UiHandleOfKind<K, R> = UiHandleWithNonReactiveState<
    <K as UnpinnedRenderStateKind>::UnpinnedUiHandle<R>,
    (
        <K as UnpinnedRenderStateKind>::UnpinnedNonReactiveState<R>,
        PhantomData<K>,
    ),
>;

impl<K: UnpinnedRenderStateKindPollRender, R: ?Sized + RenderHtml> PollRender<R>
    for UiHandleOfKind<K, R>
where
    // Note
    K: 'static,
{
    fn poll_render(
        //
        &mut self,
        renderer: &mut R,
        reactive_state: &mut OptionBoxReactiveState,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        let Self {
            ui_handle,
            non_reactive_state: (non_reactive_state, PhantomData),
        } = self;

        let reactive_state = reactive_state
            .0
            .get_or_insert_with(|| Box::new(<K::UnpinnedReactiveState>::default()));

        let reactive_state = reactive_state.as_mut().upcast_mut().downcast_mut().unwrap();

        K::unpinned_poll_render(
            renderer,
            RenderStates {
                ui_handle,
                non_reactive_state,
                reactive_state,
            },
            cx,
        )
    }
}

//

enum Never {}
pub struct Kind(Never);

impl UnpinnedRenderStateKind for Kind {
    type UnpinnedUiHandle<R: RenderHtml + ?Sized> = BoxDynUiHandle<'static, R>;
    type UnpinnedNonReactiveState<R: RenderHtml + ?Sized> = ();
    type UnpinnedReactiveState = OptionBoxReactiveState;
    // TODO: State should be statically typed and stacked allocated without Pin<Box<dyn __>> with [impl Trait in type aliases](https://github.com/rust-lang/rust/issues/63063)
}

impl UnpinnedRenderStateKindPollRender for Kind {
    fn unpinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        RenderStates {
            ui_handle,
            non_reactive_state: (),
            reactive_state,
        }: frender_html::experimental::UnpinnedMutRenderStatesOfKind<Self, R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        ui_handle.0.poll_render(renderer, reactive_state, cx)
    }
}

impl PinnedRenderStateKind for Kind {
    type PinnedUiHandle<R: RenderHtml + ?Sized> = BoxDynUiHandle<'static, R>;
    type PinnedNonReactiveState<R: RenderHtml + ?Sized> = ();
    type PinnedReactiveState = OptionBoxReactiveState;
}

impl PinnedRenderStateKindPollRender for Kind {
    fn pinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        RenderStates {
            ui_handle,
            non_reactive_state,
            reactive_state,
        }: frender_html::experimental::PinnedMutRenderStatesOfKind<Self, R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        Self::unpinned_poll_render(
            renderer,
            RenderStates {
                ui_handle,
                non_reactive_state: non_reactive_state.get_mut(),
                reactive_state: reactive_state.get_mut(),
            },
            cx,
        )
    }
}

impl<F: IntoFnOnceRenderWithContext> CsrElement for RenderWith<F> {
    type RenderStateKind = Kind;

    fn pinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        states: frender_html::experimental::PinMutRenderInitStatesOfKind<
            Self::RenderStateKind,
            Ctx::Renderer,
        >,
    ) -> frender_html::experimental::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>
    {
        todo!()
    }

    fn pinned_render_update<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        states: frender_html::experimental::PinnedMutRenderStatesOfKind<
            Self::RenderStateKind,
            Ctx::Renderer,
        >,
    ) {
        todo!()
    }

    fn unpinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
    ) -> UnpinnedRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer> {
        let f = self.0.into_fn_once_render_with_context::<Ctx>();
        // f(CsrRenderContext { render_context })
        todo!()
    }

    fn unpinned_render_update<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        states: frender_html::experimental::UnpinnedMutRenderStatesOfKind<
            Self::RenderStateKind,
            Ctx::Renderer,
        >,
    ) {
        todo!()
    }

    #[cfg(a)]
    fn render_update_maybe_reposition<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        render_state: Pin<&mut frender_html::RenderStateOfContext<Self::RenderStateKind, Ctx>>,
        force_reposition: bool,
    ) {
        let state = render_state.get_mut();

        let phantom_state = PhantomData;

        fn default_pin_box_dyn_render_state_with_phantom_hint<
            Renderer: ?Sized + RenderHtml,
            K: RenderStateKindUnpinned + 'static,
        >(
            _: PhantomData<K>,
        ) -> Pin<Box<dyn 'static + AnyRenderState<Renderer>>> {
            Box::pin(<K::UnpinnedRenderState<Renderer> as Default>::default())
        }

        let state = state.get_or_insert_with(|| PinBoxDynRenderState {
            render_state: default_pin_box_dyn_render_state_with_phantom_hint::<Ctx::Renderer, _>(
                phantom_state,
            ),
        });

        let render_state = state.render_state.as_mut().get_mut();
        let render_state = render_state.upcast_mut_dyn_any();

        let f = self.0.into_fn_once_render_with_context::<Ctx>();
        let rendered: Rendered<_> = f(CsrRenderContext {
            render_context,
            render_state,
            force_reposition,
        });
        rendered.type_check(phantom_state);
    }
}
