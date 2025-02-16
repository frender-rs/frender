use std::{any::Any, marker::PhantomData, pin::Pin, task::Poll};

use frender_html::{
    dom::{
        render::RenderWithContext,
        ui_handle::{UiHandle, UnmountedUiHandle},
    },
    experimental::{
        self, RenderInitPinned, UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender,
    },
    kinds::KindUnpinned,
    CsrElement, HtmlRenderContext, RenderHtml, StateUnmount,
};

use crate::{
    CsrRenderContext, CsrRenderContextInner, IntoFnOnceRenderWithContext, RenderWith, Rendered,
    RenderedInner,
};

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

impl<R: ?Sized, T: UnmountedUiHandle<R>> DynSafeUnmountedUiHandle<R> for T {
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

trait DynSafeUiHandle<Renderer: ?Sized> {
    fn upcast_mut_ui_handle<'a>(&mut self) -> &mut (dyn 'a + Any)
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

impl<R: ?Sized, T: UiHandle<R>> DynSafeUiHandle<R> for T {
    fn upcast_mut_ui_handle<'a>(&mut self) -> &mut (dyn 'a + Any)
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

impl<'a, R: 'a + ?Sized> BoxDynUiHandle<'a, R> {
    fn upcast_mut_ui_handle(&mut self) -> &mut (dyn 'a + Any) {
        self.0.upcast_mut_ui_handle()
    }

    fn from_sized<U: 'a + UiHandle<R>>(u: Box<U>) -> Self {
        Self(u)
    }
}
impl<R: 'static + ?Sized> BoxDynUiHandle<'static, R> {
    fn downcast_mut<U: 'static>(&mut self) -> Option<&mut U> {
        self.upcast_mut_ui_handle().downcast_mut()
    }
}
// endregion
// region: ReactiveState
trait AnyState<R: ?Sized>: PollRender<R> {
    fn state_state_unmount(self: Pin<&mut Self>);
    fn state_upcast_mut<'a>(&mut self) -> &mut (dyn 'a + Any)
    where
        Self: 'a;
}

pub struct BoxDynState<'a, R: ?Sized>(Box<dyn 'a + AnyState<R> + Unpin>);

impl<'a, R: ?Sized> BoxDynState<'a, R> {
    fn dummy() -> Self {
        Self(Box::new(DummyState))
    }

    fn upcast_mut_state(&mut self) -> &mut (dyn 'a + Any)
    where
        R: 'a,
    {
        AnyState::state_upcast_mut(self.as_mut_unboxed())
    }

    fn as_mut_unboxed(&mut self) -> &mut (dyn 'a + AnyState<R> + Unpin) {
        &mut *self.0
    }

    fn from_sized<S: 'a + AnyState<R> + Unpin>(s: Box<S>) -> Self {
        Self(s)
    }

    fn new<K: UnpinnedRenderStateKindPollRender + 'static>(state: K::UnpinnedState<R>) -> Self
    where
        R: RenderHtml,
    {
        Self::from_sized(Box::new(TypedState {
            kind: PhantomData::<K>,
            state,
        }))
    }
}

impl<'a, R: ?Sized> StateUnmount for BoxDynState<'a, R> {
    fn state_unmount(self: Pin<&mut Self>) {
        Pin::new(self.get_mut().as_mut_unboxed()).state_state_unmount();
    }
}

// endregion
// region: TypedState
struct TypedState<S, K> {
    kind: PhantomData<K>,
    state: S,
}

impl<S, K> Unpin for TypedState<S, K> {}
// endregion
// region: dyn safe poll_render
trait PollRender<R: ?Sized> {
    fn poll_render(
        //
        &mut self,
        renderer: &mut R,
        ui_handle: &mut BoxDynUiHandle<'static, R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()>;
}

impl<K: UnpinnedRenderStateKindPollRender, R: ?Sized + RenderHtml> PollRender<R>
    for TypedState<K::UnpinnedState<R>, K>
where
    // Note
    K: 'static,
{
    fn poll_render(
        //
        &mut self,
        renderer: &mut R,
        ui_handle: &mut BoxDynUiHandle<'static, R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        K::unpinned_poll_render(
            renderer,
            &mut self.state,
            ui_handle.downcast_mut().unwrap(),
            cx,
        )
    }
}
impl<K: UnpinnedRenderStateKindPollRender, R: ?Sized + RenderHtml> AnyState<R>
    for TypedState<K::UnpinnedState<R>, K>
where
    // Note
    K: 'static,
{
    fn state_state_unmount(self: Pin<&mut Self>) {
        Pin::new(&mut self.get_mut().state).state_unmount()
    }

    fn state_upcast_mut<'a>(&mut self) -> &mut (dyn 'a + Any)
    where
        Self: 'a,
    {
        &mut self.state
    }
}
// endregion
// region: DummyState
struct DummyState;

impl<R: ?Sized> PollRender<R> for DummyState {
    fn poll_render(
        //
        &mut self,
        _: &mut R,
        _: &mut BoxDynUiHandle<'static, R>,
        _: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        unreachable!()
    }
}

impl<R: ?Sized> AnyState<R> for DummyState {
    fn state_state_unmount(self: Pin<&mut Self>) {
        unreachable!()
    }
    fn state_upcast_mut<'a>(&mut self) -> &mut (dyn 'a + Any)
    where
        Self: 'a,
    {
        unreachable!()
    }
}
// endregion
// region: kind
enum Never {}
pub struct Kind(Never);

impl UnpinnedRenderStateKind for Kind {
    type UnpinnedUiHandle<R: RenderHtml + ?Sized> = BoxDynUiHandle<'static, R>;
    type UnpinnedState<R: RenderHtml + ?Sized> = BoxDynState<'static, R>;
    // TODO: State should be statically typed and stacked allocated without Pin<Box<dyn __>> with [impl Trait in type aliases](https://github.com/rust-lang/rust/issues/63063)
}

impl UnpinnedRenderStateKindPollRender for Kind {
    fn unpinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        state: &mut Self::UnpinnedState<R>,
        ui_handle: &mut Self::UnpinnedUiHandle<R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        state.as_mut_unboxed().poll_render(renderer, ui_handle, cx)
    }
}
// endregion
// region: RenderInit
pub struct RenderInit<F>(F);

impl<F: IntoFnOnceRenderWithContext, Ctx: ?Sized + HtmlRenderContext>
    RenderInitPinned<&mut Ctx, BoxDynState<'static, Ctx::Renderer>> for RenderInit<F>
{
    type Output = BoxDynUiHandle<'static, Ctx::Renderer>;

    fn render_init_pinned(
        self,
        render_context: &mut Ctx,
        state: Pin<&mut BoxDynState<'static, Ctx::Renderer>>,
    ) -> Self::Output {
        let ui_handle;
        (*state.get_mut(), ui_handle) = RenderWith(self.0).unpinned_render_init(render_context);
        ui_handle
    }
}
// endregion

impl<F: IntoFnOnceRenderWithContext> CsrElement for RenderWith<F> {
    type RenderStateKind = KindUnpinned<Kind>;
    type PinnedRenderInit<R: ?Sized + RenderHtml> = RenderInit<F>;

    fn pinned_render_init<Renderer: ?Sized + RenderHtml>(
        //
        self,
        _: &mut Renderer,
    ) -> (
        //
        experimental::PinnedStateOfKind<Renderer, Self::RenderStateKind>,
        Self::PinnedRenderInit<Renderer>,
    ) {
        (BoxDynState::dummy(), RenderInit(self.0))
    }

    fn pinned_render_init_by_reusing<Ctx: ?Sized + HtmlRenderContext>(
        self,
        render_context: &mut Ctx,
        reused_state: Pin<
            &mut experimental::PinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        >,
        unmounted_ui_handle: experimental::PinnedUnmountedUiHandleOfKind<
            Ctx::Renderer,
            Self::RenderStateKind,
        >,
    ) -> experimental::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        self.unpinned_render_init_by_reusing(
            render_context,
            reused_state.get_mut(),
            unmounted_ui_handle,
        )
    }

    fn pinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        state: Pin<&mut experimental::PinnedStateOfKind<Renderer, Self::RenderStateKind>>,
        ui_handle: &mut experimental::PinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
        self.unpinned_render_update(renderer, state.get_mut(), ui_handle)
    }

    fn unpinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
    ) -> (
        //
        experimental::UnpinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        experimental::UnpinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
    ) {
        fn render_init<K: UnpinnedRenderStateKindPollRender + 'static, R: ?Sized + RenderHtml>(
            this: Rendered<K, R>,
        ) -> (BoxDynState<'static, R>, BoxDynUiHandle<'static, R>) {
            let Rendered(::core::marker::PhantomData, RenderedInner::Init((state, ui_handle))) =
                this
            else {
                unreachable!()
            };

            (
                BoxDynState::<R>::new::<K>(state),
                BoxDynUiHandle::from_sized(Box::new(ui_handle)),
            )
        }

        let f = self.0.into_fn_once_render_with_context::<Ctx::Renderer>();
        render_context.map_mut_render_context(|render_context| {
            render_init(f(CsrRenderContext(CsrRenderContextInner::Init(
                render_context,
            ))))
        })
    }

    fn unpinned_render_init_by_reusing<Ctx: ?Sized + HtmlRenderContext>(
        self,
        render_context: &mut Ctx,
        reused_state: &mut experimental::UnpinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        unmounted_ui_handle: experimental::UnpinnedUnmountedUiHandleOfKind<
            Ctx::Renderer,
            Self::RenderStateKind,
        >,
    ) -> experimental::UnpinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        // TODO: is mount-then-update correct?
        let mut ui_handle = render_context
            .map_mut_render_context(|render_context| unmounted_ui_handle.mount(render_context));
        self.unpinned_render_update(render_context.renderer_mut(), reused_state, &mut ui_handle);
        ui_handle
    }

    fn unpinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        state: &mut BoxDynState<'static, Renderer>,
        ui_handle: &mut BoxDynUiHandle<'static, Renderer>,
    ) {
        let f = self.0.into_fn_once_render_with_context::<Renderer>();
        let Rendered(self::PhantomData, RenderedInner::Update) =
            f(CsrRenderContext(CsrRenderContextInner::Update {
                renderer,
                state: state.upcast_mut_state(),
                ui_handle: ui_handle.upcast_mut_ui_handle(),
            }))
        else {
            unreachable!()
        };
    }
}
