use super::UpdateValueWithState;

pub trait MaybeUpdateValueWithState<V: ?Sized> {
    type State: Default;

    fn maybe_update_value_with_state(
        this: Self,
        state: &mut Self::State,
        update: impl FnOnce(&V),
        remove: impl FnOnce(),
    );
}

impl<V: ?Sized> MaybeUpdateValueWithState<V> for () {
    type State = ();

    #[inline(always)]
    fn maybe_update_value_with_state(
        _: Self,
        _: &mut Self::State,
        _: impl FnOnce(&V),
        _: impl FnOnce(),
    ) {
    }
}

impl<T: UpdateValueWithState<V>, V: ?Sized> MaybeUpdateValueWithState<V> for Option<T> {
    type State = T::State;

    fn maybe_update_value_with_state(
        this: Self,
        state: &mut Self::State,
        update: impl FnOnce(&V),
        remove: impl FnOnce(),
    ) {
        match this {
            Some(this) => T::update_value_with_state(this, state, update),
            None => {
                remove();
                *state = Default::default();
            }
        }
    }
}

