use frender_dom::csr::{render::RenderWithContext, UiHandle, UnmountedUiHandle};

pub enum EitherUiHandle<A, B> {
    A(A),
    B(B),
}

impl<A: UnmountedUiHandle<R>, B: UnmountedUiHandle<R>, R: ?Sized> UnmountedUiHandle<R> for EitherUiHandle<A, B> {
    type Mounted = EitherUiHandle<A::Mounted, B::Mounted>;

    fn mount(self, render_context: &mut <R>::RenderContext<'_>) -> Self::Mounted
    where
        R: RenderWithContext,
    {
        match self {
            EitherUiHandle::A(a) => EitherUiHandle::A(a.mount(render_context)),
            EitherUiHandle::B(b) => EitherUiHandle::B(b.mount(render_context)),
        }
    }
}

impl<A: UiHandle<R>, B: UiHandle<R>, R: ?Sized> UiHandle<R> for EitherUiHandle<A, B> {
    type Unmounted = EitherUiHandle<A::Unmounted, B::Unmounted>;

    fn unmount(self, renderer: &mut R) -> Self::Unmounted {
        match self {
            EitherUiHandle::A(a) => EitherUiHandle::A(a.unmount(renderer)),
            EitherUiHandle::B(b) => EitherUiHandle::B(b.unmount(renderer)),
        }
    }

    fn reposition(&mut self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: RenderWithContext,
    {
        match self {
            EitherUiHandle::A(a) => a.reposition(render_context),
            EitherUiHandle::B(b) => b.reposition(render_context),
        }
    }

    fn check_and_move_cursor(&self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: RenderWithContext,
    {
        match self {
            EitherUiHandle::A(a) => a.check_and_move_cursor(render_context),
            EitherUiHandle::B(b) => b.check_and_move_cursor(render_context),
        }
    }

    fn assert_cursor_is_at_self(&self, render_context: &<R>::RenderContext<'_>)
    where
        R: RenderWithContext,
    {
        match self {
            EitherUiHandle::A(this) => this.assert_cursor_is_at_self(render_context),
            EitherUiHandle::B(this) => this.assert_cursor_is_at_self(render_context),
        }
    }
}
