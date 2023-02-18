pub trait MaybeUpdateValueWithState<V: ?Sized> {
    type State;

    fn initialize_state_and_update(
        this: Self,
        update: impl FnOnce(&V),
        remove: impl FnOnce(),
    ) -> Self::State;

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
    fn initialize_state_and_update(_: Self, _: impl FnOnce(&V), _: impl FnOnce()) -> Self::State {}

    #[inline(always)]
    fn maybe_update_value_with_state(
        _: Self,
        _: &mut Self::State,
        _: impl FnOnce(&V),
        _: impl FnOnce(),
    ) {
    }
}

impl<T: MaybeUpdateValueWithState<V>, V: ?Sized> MaybeUpdateValueWithState<V> for Option<T> {
    type State = Option<T::State>;

    fn initialize_state_and_update(
        this: Self,
        update: impl FnOnce(&V),
        remove: impl FnOnce(),
    ) -> Self::State {
        match this {
            Some(this) => Some(T::initialize_state_and_update(this, update, remove)),
            None => {
                remove();
                None
            }
        }
    }

    fn maybe_update_value_with_state(
        this: Self,
        state: &mut Self::State,
        update: impl FnOnce(&V),
        remove: impl FnOnce(),
    ) {
        match (this, state) {
            (Some(this), Some(state)) => {
                T::maybe_update_value_with_state(this, state, update, remove)
            }
            (Some(this), state) => {
                *state = Some(T::initialize_state_and_update(this, update, remove))
            }
            (None, state) => {
                remove();
                *state = None;
            }
        }
    }
}

/// No cache
impl MaybeUpdateValueWithState<str> for &str {
    type State = ();

    #[inline(always)]
    fn initialize_state_and_update(
        this: Self,
        update: impl FnOnce(&str),
        _: impl FnOnce(),
    ) -> Self::State {
        update(this)
    }

    #[inline(always)]
    fn maybe_update_value_with_state(
        this: Self,
        _: &mut Self::State,
        update: impl FnOnce(&str),
        _: impl FnOnce(),
    ) {
        update(this)
    }
}

/// Cache if self is [`Cow::Owned`]
impl MaybeUpdateValueWithState<str> for std::borrow::Cow<'_, str> {
    /// `None` means the previous value is borrowed and not cached.
    type State = Option<String>;

    fn initialize_state_and_update(
        this: Self,
        update: impl FnOnce(&str),
        _: impl FnOnce(),
    ) -> Self::State {
        update(&this);
        match this {
            std::borrow::Cow::Borrowed(_) => None,
            std::borrow::Cow::Owned(s) => Some(s),
        }
    }

    fn maybe_update_value_with_state(
        this: Self,
        state: &mut Self::State,
        update: impl FnOnce(&str),
        _: impl FnOnce(),
    ) {
        match (state, this) {
            (Some(state), this) if *this == **state => {}
            (state, std::borrow::Cow::Owned(this)) => {
                update(&this);
                *state = Some(this);
            }
            (state, std::borrow::Cow::Borrowed(this)) => {
                *state = None;
                update(this)
            }
        }
    }
}

impl<S: frender_core::StaticStr> MaybeUpdateValueWithState<str> for frender_core::StaticText<S> {
    type State = S;

    fn initialize_state_and_update(
        this: Self,
        update: impl FnOnce(&str),
        _: impl FnOnce(),
    ) -> Self::State {
        update(&this);
        this.0
    }

    fn maybe_update_value_with_state(
        this: Self,
        state: &mut Self::State,
        update: impl FnOnce(&str),
        _: impl FnOnce(),
    ) {
        if **state == *this.0 {
            return;
        }
        update(&this);
        *state = this.0;
    }
}

impl MaybeUpdateValueWithState<str> for String {
    type State = String;

    fn initialize_state_and_update(
        this: Self,
        update: impl FnOnce(&str),
        _: impl FnOnce(),
    ) -> Self::State {
        update(&this);
        this
    }

    fn maybe_update_value_with_state(
        this: Self,
        state: &mut Self::State,
        update: impl FnOnce(&str),
        _: impl FnOnce(),
    ) {
        if *state == this {
            return;
        }
        update(&this);
        *state = this;
    }
}

macro_rules! auto_impl_update {
    ($($ty:ty),* $(,)?) => {$(
        impl MaybeUpdateValueWithState<$ty> for $ty {
            type State = $ty;

            fn initialize_state_and_update(
                this: Self,
                update: impl FnOnce(&$ty),
                _: impl FnOnce(),
            ) -> Self::State {
                update(&this);
                this
            }

            fn maybe_update_value_with_state(
                this: Self,
                state: &mut Self::State,
                update: impl FnOnce(&$ty),
                _: impl FnOnce(),
            ) {
                if *state == this {
                    return;
                }
                update(&this);
                *state = this;
            }
        }
    )*};
}

auto_impl_update! {
    i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize,
    f32, f64,
    bool,
}
