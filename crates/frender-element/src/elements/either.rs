use std::pin::Pin;

use either::Either;

use crate::{Element, RenderState};

impl<Renderer, L: RenderState<Renderer>, R: RenderState<Renderer>> RenderState<Renderer>
    for Either<L, R>
{
    fn unmount(self: Pin<&mut Self>, renderer: &mut Renderer) {
        match self.as_pin_mut() {
            Either::Left(s) => s.unmount(renderer),
            Either::Right(s) => s.unmount(renderer),
        }
    }

    fn state_unmount(self: std::pin::Pin<&mut Self>) {
        match self.as_pin_mut() {
            Either::Left(s) => s.state_unmount(),
            Either::Right(s) => s.state_unmount(),
        }
    }

    fn poll_render(
        self: Pin<&mut Self>,
        renderer: &mut Renderer,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        match self.as_pin_mut() {
            Either::Left(s) => s.poll_render(renderer, cx),
            Either::Right(s) => s.poll_render(renderer, cx),
        }
    }
}

macro_rules! update_either {
    ($_self:ident . $method:ident($ctx:ident, $state:ident $(, $arg:expr)?)) => {{
        let mut $state = $state.project().inner;
        match $_self {
            Either::Left(e) => {
                if let Either::Right(other_state) = $state.as_mut().as_pin_mut() {
                    other_state.unmount($ctx);
                    $state.set(Either::Left(Default::default()))
                }

                let state = if let Either::Left(state) = $state.as_pin_mut() {
                    state
                } else {
                    unreachable!();
                };

                e.$method($ctx, state $(, $arg)?)
            }
            Either::Right(e) => {
                if let Either::Left(other_state) = $state.as_mut().as_pin_mut() {
                    other_state.unmount($ctx);
                    $state.set(Either::Right(Default::default()))
                }

                let state = if let Either::Right(state) = $state.as_pin_mut() {
                    state
                } else {
                    unreachable!();
                };

                e.$method($ctx, state $(, $arg)?)
            }
        }
    }};
}

macro_rules! unpinned_update_either {
    ($_self:ident . $method:ident($ctx:ident, $state:ident $(, $arg:expr)?)) => {{
        let $state = &mut $state.inner;
        match $_self {
            Either::Left(e) => {
                if let Either::Right(other_state) = $state {
                    Pin::new(other_state).unmount($ctx);
                    *$state = Either::Left(Default::default());
                }

                let state = if let Either::Left(state) = $state {
                    state
                } else {
                    unreachable!();
                };

                e.$method($ctx, state $(, $arg)?)
            }
            Either::Right(e) => {
                if let Either::Left(other_state) = $state {
                    Pin::new(other_state).unmount($ctx);
                    *$state = Either::Right(Default::default());
                }

                let state = if let Either::Right(state) = $state {
                    state
                } else {
                    unreachable!();
                };

                e.$method($ctx, state $(, $arg)?)
            }
        }
    }};
}

pin_project_lite::pin_project!(
    pub struct EitherState<A, B> {
        #[pin]
        inner: Either<A, B>,
    }
);

impl<A: Default, B: Default> Default for EitherState<A, B> {
    fn default() -> Self {
        EitherState {
            inner: Either::Left(A::default()),
        }
    }
}

impl<R, A: RenderState<R>, B: RenderState<R>> RenderState<R> for EitherState<A, B> {
    fn unmount(self: Pin<&mut Self>, renderer: &mut R) {
        self.project().inner.unmount(renderer)
    }

    fn state_unmount(self: Pin<&mut Self>) {
        self.project().inner.state_unmount()
    }

    fn poll_render(
        self: Pin<&mut Self>,
        renderer: &mut R,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        self.project().inner.poll_render(renderer, cx)
    }
}

impl<L, R> Element for Either<L, R>
where
    L: Element,
    R: Element,
{
    type RenderState<Renderer: frender_html::RenderHtml> =
        EitherState<L::RenderState<Renderer>, R::RenderState<Renderer>>;

    fn render_update<Renderer: frender_html::RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: Pin<&mut Self::RenderState<Renderer>>,
    ) {
        update_either!(self.render_update(renderer, render_state))
    }

    fn render_update_force_reposition<Renderer: frender_html::RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: Pin<&mut Self::RenderState<Renderer>>,
    ) {
        update_either!(self.render_update_force_reposition(renderer, render_state))
    }

    fn render_update_maybe_reposition<Renderer: frender_html::RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: Pin<&mut Self::RenderState<Renderer>>,
        force_reposition: bool,
    ) {
        update_either!(self.render_update_maybe_reposition(
            renderer,
            render_state,
            force_reposition
        ))
    }

    type UnpinnedRenderState<Renderer: frender_html::RenderHtml> =
        EitherState<L::UnpinnedRenderState<Renderer>, R::UnpinnedRenderState<Renderer>>;

    fn unpinned_render_update<Renderer: frender_html::RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: &mut Self::UnpinnedRenderState<Renderer>,
    ) {
        unpinned_update_either!(self.unpinned_render_update(renderer, render_state))
    }

    fn unpinned_render_update_force_reposition<Renderer: frender_html::RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: &mut Self::UnpinnedRenderState<Renderer>,
    ) {
        unpinned_update_either!(self.unpinned_render_update_force_reposition(renderer, render_state))
    }

    fn unpinned_render_update_maybe_reposition<Renderer: frender_html::RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: &mut Self::UnpinnedRenderState<Renderer>,
        force_reposition: bool,
    ) {
        unpinned_update_either!(self.unpinned_render_update_maybe_reposition(
            renderer,
            render_state,
            force_reposition
        ))
    }
}
