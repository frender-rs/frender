use frender_common::{impl_many, ToAsRefStr, ToStaticCache};

use crate::string::KnownStaticStr;

pub use self::value_kind::ValueKind;

mod value_kind;

pub trait ValueUpdater<VK: ?Sized + ValueKind> {
    fn update(self, value: VK::Value<'_>);
    fn remove(self);
}

// Unlike CsrStyle and CsrDomTokens, MaybeValue doesn't have remove_with_state.
pub trait MaybeValue<V: ?Sized + ValueKind> {
    type UpdateWithState: Default;

    fn update_with_state(
        this: Self,
        state: &mut Self::UpdateWithState,
        updater: impl ValueUpdater<V>,
    );

    fn state_could_skip_remove(state: &Self::UpdateWithState) -> bool;
}

impl<T: MaybeValue<V>, V: ?Sized + ValueKind> MaybeValue<V> for Option<T> {
    type UpdateWithState = T::UpdateWithState;

    fn update_with_state(
        this: Self,
        state: &mut Self::UpdateWithState,
        updater: impl ValueUpdater<V>,
    ) {
        if let Some(this) = this {
            T::update_with_state(this, state, updater)
        } else if !Self::state_could_skip_remove(state) {
            updater.remove();
            *state = Default::default()
        }
    }

    fn state_could_skip_remove(state: &Self::UpdateWithState) -> bool {
        T::state_could_skip_remove(state)
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

            updater.update(this);
            *state = Some(this);
        }

        fn state_could_skip_remove(state: &Self::UpdateWithState) -> bool {
            state.is_none()
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

        updater.update(this);
        *state = Some(this);
    }

    fn state_could_skip_remove(state: &Self::UpdateWithState) -> bool {
        state.is_none()
    }
}

#[cfg(feature = "either")]
pub mod either {
    use ::either::Either;
    use frender_common::either::EitherState;

    use super::*;

    impl<V: ?Sized + ValueKind, L: MaybeValue<V>, R: MaybeValue<V>> MaybeValue<V> for Either<L, R> {
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
