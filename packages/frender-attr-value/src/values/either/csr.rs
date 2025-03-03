use crate::{
    csr::{CsrAttrValue, UpdateAttrValue},
    AttrValueKind,
};

use super::EitherAttrValue;

impl<V: ?Sized + AttrValueKind, A: CsrAttrValue<V>, B: CsrAttrValue<V>> CsrAttrValue<V>
    for EitherAttrValue<A, B>
{
    type State = Result<A::State, B::State>;

    fn update_absent_attribute_value_into_state(
        this: Self,
        updater: impl UpdateAttrValue<Kind = V>,
    ) -> Self::State {
        match this {
            EitherAttrValue::A(this) => {
                Ok(A::update_absent_attribute_value_into_state(this, updater))
            }
            EitherAttrValue::B(this) => {
                Err(B::update_absent_attribute_value_into_state(this, updater))
            }
        }
    }

    fn update_attribute_value_into_state(
        this: Self,
        updater: impl UpdateAttrValue<Kind = V>,
    ) -> Self::State {
        match this {
            EitherAttrValue::A(this) => Ok(A::update_attribute_value_into_state(this, updater)),
            EitherAttrValue::B(this) => Err(B::update_attribute_value_into_state(this, updater)),
        }
    }

    fn can_skip_update(this: &Self, state: &Self::State) -> bool {
        match (this, state) {
            (EitherAttrValue::A(this), Ok(state)) => A::can_skip_update(this, state),
            (EitherAttrValue::B(this), Err(state)) => B::can_skip_update(this, state),
            _ => false,
        }
    }

    fn update_attribute_value_with_state(
        this: Self,
        updater: impl UpdateAttrValue<Kind = V>,
        state: &mut Self::State,
    ) {
        match (this, &mut *state) {
            (EitherAttrValue::A(this), Ok(state)) => {
                A::update_attribute_value_with_state(this, updater, state)
            }
            (EitherAttrValue::B(this), Err(state)) => {
                B::update_attribute_value_with_state(this, updater, state)
            }
            (EitherAttrValue::A(this), Err(old_state)) => {
                *state = Ok(if B::attribute_is_known_as_absent(old_state) {
                    A::update_absent_attribute_value_into_state(this, updater)
                } else {
                    A::update_attribute_value_into_state(this, updater)
                })
            }
            (EitherAttrValue::B(this), Ok(old_state)) => {
                *state = Err(if A::attribute_is_known_as_absent(old_state) {
                    B::update_absent_attribute_value_into_state(this, updater)
                } else {
                    B::update_attribute_value_into_state(this, updater)
                })
            }
        }
    }

    fn force_update_attribute_value_with_state(
        this: Self,
        updater: impl UpdateAttrValue<Kind = V>,
        state: &mut Self::State,
    ) {
        match (this, &mut *state) {
            (EitherAttrValue::A(this), Ok(state)) => {
                A::force_update_attribute_value_with_state(this, updater, state)
            }
            (EitherAttrValue::B(this), Err(state)) => {
                B::force_update_attribute_value_with_state(this, updater, state)
            }
            (EitherAttrValue::A(this), Err(old_state)) => {
                *state = Ok(if B::attribute_is_known_as_absent(old_state) {
                    A::update_absent_attribute_value_into_state(this, updater)
                } else {
                    A::update_attribute_value_into_state(this, updater)
                })
            }
            (EitherAttrValue::B(this), Ok(old_state)) => {
                *state = Err(if A::attribute_is_known_as_absent(old_state) {
                    B::update_absent_attribute_value_into_state(this, updater)
                } else {
                    B::update_attribute_value_into_state(this, updater)
                })
            }
        }
    }

    fn attribute_is_known_as_absent(state: &Self::State) -> bool {
        match state {
            Ok(state) => A::attribute_is_known_as_absent(state),
            Err(state) => B::attribute_is_known_as_absent(state),
        }
    }
}
