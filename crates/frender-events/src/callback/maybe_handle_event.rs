use crate::{Callable, IsCallable};

#[derive(Clone, Copy)]
pub enum NeverCalled {}

impl IsCallable for NeverCalled {}
impl<Args: super::sealed::Tuple> Callable<Args> for NeverCalled {
    type Output = ();

    fn call_fn(&self, _: Args) -> Self::Output {
        match *self {}
    }
}

pub trait StatedEvent {
    type State;
}

pub trait MaybeHandleEvent<E: ?Sized + StatedEvent>: PartialEq {
    type State;
    type Callable: Clone + for<'e> Callable<(&'e E,)>;

    fn initialize_handle_event_state(
        this: Self,
        new_event_state: impl FnOnce(&Self::Callable) -> E::State,
    ) -> Self::State;

    fn update_handle_event_state(
        this: Self,
        state: &mut Self::State,
        new_event_state: impl FnOnce(&Self::Callable) -> E::State,
    );
}

impl<E: ?Sized + StatedEvent> MaybeHandleEvent<E> for () {
    type State = ();
    type Callable = NeverCalled;

    fn initialize_handle_event_state(
        _: Self,
        _: impl FnOnce(&Self::Callable) -> <E as StatedEvent>::State,
    ) -> Self::State {
    }

    fn update_handle_event_state(
        _: Self,
        _: &mut Self::State,
        _: impl FnOnce(&Self::Callable) -> <E as StatedEvent>::State,
    ) {
    }
}

impl<E: ?Sized + StatedEvent, C: PartialEq + Clone + for<'e> Callable<(&'e E,)>> MaybeHandleEvent<E>
    for C
{
    type State = (E::State, C);
    type Callable = C;

    fn initialize_handle_event_state(
        this: Self,
        new_event_state: impl FnOnce(&Self::Callable) -> <E as StatedEvent>::State,
    ) -> Self::State {
        (new_event_state(&this), this)
    }

    fn update_handle_event_state(
        this: Self,
        state: &mut Self::State,
        new_event_state: impl FnOnce(&Self::Callable) -> <E as StatedEvent>::State,
    ) {
        if this != state.1 {
            *state = Self::initialize_handle_event_state(this, new_event_state)
        }
    }
}

impl<E: ?Sized + StatedEvent, M: MaybeHandleEvent<E>> MaybeHandleEvent<E> for Option<M> {
    type State = Option<M::State>;
    type Callable = M::Callable;

    fn initialize_handle_event_state(
        this: Self,
        new_event_state: impl FnOnce(&Self::Callable) -> <E as StatedEvent>::State,
    ) -> Self::State {
        this.map(|this| M::initialize_handle_event_state(this, new_event_state))
    }

    fn update_handle_event_state(
        this: Self,
        state: &mut Self::State,
        new_event_state: impl FnOnce(&Self::Callable) -> <E as StatedEvent>::State,
    ) {
        match (this, state) {
            (None, None) => {}
            (None, state @ Some(_)) => *state = None,
            (Some(this), Some(state)) => M::update_handle_event_state(this, state, new_event_state),
            (Some(this), state @ None) => {
                *state = Some(M::initialize_handle_event_state(this, new_event_state))
            }
        }
    }
}
