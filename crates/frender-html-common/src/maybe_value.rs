use crate::{attr::MaybeIntoHtmlAttributeValue, impl_many, StringValue};

pub trait ValueUpdater<V: ?Sized> {
    fn update(self, value: &V);
    fn remove(self);
}

impl<V: ?Sized, U: FnOnce(&V), R: FnOnce()> ValueUpdater<V> for (U, R) {
    fn update(self, value: &V) {
        self.0(value)
    }

    fn remove(self) {
        self.1()
    }
}

pub trait MaybeValue<V: ?Sized>: MaybeIntoHtmlAttributeValue<V> {
    type UpdateWithState: Default;

    fn update_with_state(
        this: Self,
        state: &mut Self::UpdateWithState,
        updater: impl ValueUpdater<V>,
    );
}

impl<S: StringValue> MaybeValue<str> for S {
    type UpdateWithState = Option<S>;

    fn update_with_state(
        this: Self,
        state: &mut Self::UpdateWithState,
        updater: impl ValueUpdater<str>,
    ) {
        if let Some(state) = state {
            if state.as_ref() == this.as_ref() {
                return;
            }
        }

        updater.update(this.as_ref());
        *state = Some(this);
    }
}

/// Temporary strings are cloned to cache
impl<S: std::borrow::Borrow<str>> MaybeValue<str> for frender_common::TempStr<S> {
    type UpdateWithState = Option<String>;

    fn update_with_state(
        this: Self,
        state: &mut Self::UpdateWithState,
        updater: impl ValueUpdater<str>,
    ) {
        let s = this.0.borrow();
        if state.as_deref() == Some(s) {
            return;
        }

        s.clone_into(state.get_or_insert_with(Default::default));

        updater.update(s);
    }
}

impl<V: ?Sized> MaybeValue<V> for () {
    type UpdateWithState = ();

    fn update_with_state((): Self, (): &mut Self::UpdateWithState, _: impl ValueUpdater<V>) {}
}

impl<T: MaybeValue<V>, V: ?Sized> MaybeValue<V> for Option<T> {
    type UpdateWithState = T::UpdateWithState;

    fn update_with_state(
        this: Self,
        state: &mut Self::UpdateWithState,
        updater: impl ValueUpdater<V>,
    ) {
        if let Some(this) = this {
            T::update_with_state(this, state, updater)
        } else {
            updater.remove();
            *state = Default::default()
        }
    }
}

impl_many!(
    impl<__> MaybeValue<Self>
        for each_of![i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64]
    {
        type UpdateWithState = Option<Self>;

        fn update_with_state(
            this: Self,
            state: &mut Self::UpdateWithState,
            updater: impl ValueUpdater<Self>,
        ) {
            if let Some(state) = state {
                if *state == this {
                    return;
                }
            }

            updater.update(&this);
            *state = Some(this);
        }
    }
);

impl MaybeValue<bool> for bool {
    type UpdateWithState = Option<Self>;

    fn update_with_state(
        this: Self,
        state: &mut Self::UpdateWithState,
        updater: impl ValueUpdater<Self>,
    ) {
        if let Some(state) = state {
            if *state == this {
                return;
            }
        }

        updater.update(&this);
        *state = Some(this);
    }
}

#[cfg(feature = "either")]
pub mod either {
    use ::either::Either;

    use super::*;

    pub enum EitherState<L, R> {
        Left(L),
        Right(R),
    }

    impl<L, R> EitherState<L, R> {
        fn get_left_or_insert_default(&mut self) -> &mut L
        where
            L: Default,
        {
            match self {
                EitherState::Left(this) => this,
                this @ EitherState::Right(_) => {
                    *this = Self::Left(Default::default());
                    if let Self::Left(this) = this {
                        this
                    } else {
                        unreachable!()
                    }
                }
            }
        }

        fn get_right_or_insert_default(&mut self) -> &mut R
        where
            R: Default,
        {
            match self {
                EitherState::Right(this) => this,
                this @ EitherState::Left(_) => {
                    *this = Self::Right(Default::default());
                    if let Self::Right(this) = this {
                        this
                    } else {
                        unreachable!()
                    }
                }
            }
        }
    }

    impl<L: Default, R: Default> Default for EitherState<L, R> {
        fn default() -> Self {
            Self::Left(Default::default())
        }
    }

    impl<V: ?Sized, L: MaybeValue<V>, R: MaybeValue<V>> MaybeValue<V> for Either<L, R> {
        type UpdateWithState = EitherState<L::UpdateWithState, R::UpdateWithState>;

        fn update_with_state(
            this: Self,
            state: &mut Self::UpdateWithState,
            updater: impl ValueUpdater<V>,
        ) {
            match this {
                Either::Left(this) => {
                    L::update_with_state(this, state.get_left_or_insert_default(), updater)
                }
                Either::Right(this) => {
                    R::update_with_state(this, state.get_right_or_insert_default(), updater)
                }
            }
        }
    }
}
