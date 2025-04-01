use crate::{
    csr::{CsrAttrValue, CsrAttrValueState, UpdateAttrValue},
    AttrValueKind,
};

use super::EitherAttrValue;

pub struct State<A, B>(Result<A, B>);

impl<A: CsrAttrValueState, B: CsrAttrValueState> CsrAttrValueState for State<A, B> {
    fn attribute_is_known_as_absent(&self) -> bool {
        match &self.0 {
            Ok(state) => A::attribute_is_known_as_absent(state),
            Err(state) => B::attribute_is_known_as_absent(state),
        }
    }
}

impl<V: ?Sized + AttrValueKind, A: CsrAttrValue<V>, B: CsrAttrValue<V>> CsrAttrValue<V>
    for EitherAttrValue<A, B>
{
    type State = State<A::State, B::State>;

    fn render_init_on_absent_attribute(
        this: Self,
        updater: impl UpdateAttrValue<Kind = V>,
    ) -> Self::State {
        State(match this {
            EitherAttrValue::A(this) => Ok(A::render_init_on_absent_attribute(this, updater)),
            EitherAttrValue::B(this) => Err(B::render_init_on_absent_attribute(this, updater)),
        })
    }

    fn render_init(this: Self, updater: impl UpdateAttrValue<Kind = V>) -> Self::State {
        State(match this {
            EitherAttrValue::A(this) => Ok(A::render_init(this, updater)),
            EitherAttrValue::B(this) => Err(B::render_init(this, updater)),
        })
    }

    fn render_init_by_reusing_on_absent_attribute(
        this: Self,
        updater: impl UpdateAttrValue<Kind = V>,
        State(state): &mut Self::State,
    ) {
        match (this, &mut *state) {
            (EitherAttrValue::A(this), Ok(state)) => {
                A::render_init_by_reusing_on_absent_attribute(this, updater, state)
            }
            (EitherAttrValue::B(this), Err(state)) => {
                B::render_init_by_reusing_on_absent_attribute(this, updater, state)
            }
            (EitherAttrValue::A(this), Err(_)) => {
                *state = Ok(A::render_init_on_absent_attribute(this, updater))
            }
            (EitherAttrValue::B(this), Ok(_)) => {
                *state = Err(B::render_init_on_absent_attribute(this, updater))
            }
        }
    }

    fn render_init_by_reusing(
        this: Self,
        updater: impl UpdateAttrValue<Kind = V>,
        State(state): &mut Self::State,
    ) {
        match (this, &mut *state) {
            (EitherAttrValue::A(this), Ok(state)) => {
                A::render_init_by_reusing(this, updater, state)
            }
            (EitherAttrValue::B(this), Err(state)) => {
                B::render_init_by_reusing(this, updater, state)
            }
            (EitherAttrValue::A(this), Err(_)) => {
                *state = Ok(A::render_init_on_absent_attribute(this, updater))
            }
            (EitherAttrValue::B(this), Ok(_)) => {
                *state = Err(B::render_init_on_absent_attribute(this, updater))
            }
        }
    }

    fn render_update(
        this: Self,
        updater: impl UpdateAttrValue<Kind = V>,
        State(state): &mut Self::State,
    ) {
        match (this, &mut *state) {
            (EitherAttrValue::A(this), Ok(state)) => A::render_update(this, updater, state),
            (EitherAttrValue::B(this), Err(state)) => B::render_update(this, updater, state),
            (EitherAttrValue::A(this), Err(old_state)) => {
                *state = Ok(if B::State::attribute_is_known_as_absent(old_state) {
                    A::render_init_on_absent_attribute(this, updater)
                } else {
                    A::render_init(this, updater)
                })
            }
            (EitherAttrValue::B(this), Ok(old_state)) => {
                *state = Err(if A::State::attribute_is_known_as_absent(old_state) {
                    B::render_init_on_absent_attribute(this, updater)
                } else {
                    B::render_init(this, updater)
                })
            }
        }
    }
}
