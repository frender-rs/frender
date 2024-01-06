use std::pin::Pin;

use either::Either;

use crate::{render_state::either::EitherRenderState, Element, RenderHtml, RenderState};

macro_rules! update_either {
    ($_self:ident . $method:ident($peh:ident, $ctx:ident, $state:ident $(, $arg:expr)?)) => {{
        let mut $state = $state.project_inner();
        match $_self {
            Either::Left(e) => {
                if let Either::Right(other_state) = $state.as_mut().as_pin_mut() {
                    other_state.unmount($peh, $ctx);
                    $state.set(Either::Left(Default::default()))
                }

                let state = if let Either::Left(state) = $state.as_pin_mut() {
                    state
                } else {
                    unreachable!();
                };

                e.$method($peh, $ctx, state $(, $arg)?)
            }
            Either::Right(e) => {
                if let Either::Left(other_state) = $state.as_mut().as_pin_mut() {
                    other_state.unmount($peh, $ctx);
                    $state.set(Either::Right(Default::default()))
                }

                let state = if let Either::Right(state) = $state.as_pin_mut() {
                    state
                } else {
                    unreachable!();
                };

                e.$method($peh, $ctx, state $(, $arg)?)
            }
        }
    }};
}

macro_rules! unpinned_update_either {
    ($_self:ident . $method:ident($peh:ident, $ctx:ident, $state:ident $(, $arg:expr)?)) => {{
        let $state = $state.inner_mut();
        match $_self {
            Either::Left(e) => {
                if let Either::Right(other_state) = $state {
                    Pin::new(other_state).unmount($peh, $ctx);
                    *$state = Either::Left(Default::default());
                }

                let state = if let Either::Left(state) = $state {
                    state
                } else {
                    unreachable!();
                };

                e.$method($peh, $ctx, state $(, $arg)?)
            }
            Either::Right(e) => {
                if let Either::Left(other_state) = $state {
                    Pin::new(other_state).unmount($peh, $ctx);
                    *$state = Either::Right(Default::default());
                }

                let state = if let Either::Right(state) = $state {
                    state
                } else {
                    unreachable!();
                };

                e.$method($peh, $ctx, state $(, $arg)?)
            }
        }
    }};
}

impl<L, R> Element for Either<L, R>
where
    L: Element,
    R: Element,
{
    type RenderState<PEH: ?Sized, Renderer: RenderHtml + ?Sized> = EitherRenderState<L::RenderState<PEH, Renderer>, R::RenderState<PEH, Renderer>>;

    fn render_update<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(self, peh: &mut PEH, renderer: &mut Renderer, render_state: Pin<&mut Self::RenderState<PEH, Renderer>>) {
        update_either!(self.render_update(peh, renderer, render_state))
    }

    fn render_update_force_reposition<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(self, peh: &mut PEH, renderer: &mut Renderer, render_state: Pin<&mut Self::RenderState<PEH, Renderer>>) {
        update_either!(self.render_update_force_reposition(peh, renderer, render_state))
    }

    fn render_update_maybe_reposition<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(self, peh: &mut PEH, renderer: &mut Renderer, render_state: Pin<&mut Self::RenderState<PEH, Renderer>>, force_reposition: bool) {
        update_either!(self.render_update_maybe_reposition(peh, renderer, render_state, force_reposition))
    }

    type UnpinnedRenderState<PEH: ?Sized, Renderer: RenderHtml + ?Sized> = EitherRenderState<L::UnpinnedRenderState<PEH, Renderer>, R::UnpinnedRenderState<PEH, Renderer>>;

    fn unpinned_render_update<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(self, peh: &mut PEH, renderer: &mut Renderer, render_state: &mut Self::UnpinnedRenderState<PEH, Renderer>) {
        unpinned_update_either!(self.unpinned_render_update(peh, renderer, render_state))
    }

    fn unpinned_render_update_force_reposition<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(self, peh: &mut PEH, renderer: &mut Renderer, render_state: &mut Self::UnpinnedRenderState<PEH, Renderer>) {
        unpinned_update_either!(self.unpinned_render_update_force_reposition(peh, renderer, render_state))
    }

    fn unpinned_render_update_maybe_reposition<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(
        self,
        peh: &mut PEH,
        renderer: &mut Renderer,
        render_state: &mut Self::UnpinnedRenderState<PEH, Renderer>,
        force_reposition: bool,
    ) {
        unpinned_update_either!(self.unpinned_render_update_maybe_reposition(peh, renderer, render_state, force_reposition))
    }
}
