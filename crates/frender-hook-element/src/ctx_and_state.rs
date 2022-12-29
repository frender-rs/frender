use std::{any::Any, pin::Pin};

use frender_core::UpdateRenderState;

pub struct ContextAndState<'a, Ctx, State: ?Sized> {
    context: &'a mut Ctx,
    state: Pin<&'a mut State>,
}

impl<'a, Ctx, State: ?Sized> ContextAndState<'a, Ctx, State> {
    pub fn new(context: &'a mut Ctx, state: Pin<&'a mut State>) -> Self {
        Self { context, state }
    }
}

impl<'a, Ctx> ContextAndState<'a, Ctx, dyn Any> {
    #[inline]
    pub fn render<E: UpdateRenderState<Ctx>>(self, element: E) -> ContextAndState<'a, Ctx, E::State>
    where
        E::State: 'static,
    {
        let mut this = self.downcast_state::<E::State>().unwrap();
        element.update_render_state(this.context, this.state.as_mut());

        this
    }

    pub fn downcast_state<S: Any>(self) -> Option<ContextAndState<'a, Ctx, S>> {
        let Self { context, state } = self;

        // SAFETY: get_unchecked_mut is never used to mutate state
        let state = unsafe { state.get_unchecked_mut() };
        let state = state.downcast_mut::<S>()?;
        // SAFETY: state comes from a Pin<&mut dyn Any>
        let state = unsafe { Pin::new_unchecked(state) };

        Some(ContextAndState { context, state })
    }
}
