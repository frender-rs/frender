use crate::{
    csr::{CsrAttrValue, CsrAttrValueState, UpdateAttrValue},
    AttrValueKind,
};

use super::OptionAttrValue;

pub struct State<T>(Option<T>);

impl<T: CsrAttrValueState> CsrAttrValueState for State<T> {
    fn attribute_is_known_as_absent(&self) -> bool {
        match &self.0 {
            None => true,
            Some(state) => T::attribute_is_known_as_absent(state),
        }
    }
}

impl<T: CsrAttrValue<V>, V: ?Sized + AttrValueKind> CsrAttrValue<V> for OptionAttrValue<T> {
    type State = State<T::State>;

    fn render_init_on_absent_attribute(
        Self(this): Self,
        updater: impl UpdateAttrValue<Kind = V>,
    ) -> Self::State {
        State(match this {
            Some(this) => Some(T::render_init_on_absent_attribute(this, updater)),
            None => {
                // the attribute is absent, so we don't need to remove it
                None
            }
        })
    }

    fn render_init(Self(this): Self, updater: impl UpdateAttrValue<Kind = V>) -> Self::State {
        State(match this {
            Some(this) => Some(T::render_init(this, updater)),
            None => {
                updater.remove();
                None
            }
        })
    }

    fn render_init_by_reusing_on_absent_attribute(
        Self(this): Self,
        updater: impl UpdateAttrValue<Kind = V>,
        State(state): &mut Self::State,
    ) {
        match (this, state) {
            (None, state) => {
                *state = None;
                // already absent
            }
            (Some(this), Some(state)) => {
                T::render_init_by_reusing_on_absent_attribute(this, updater, state)
            }
            (Some(this), state @ None) => {
                *state = Some(T::render_init_on_absent_attribute(this, updater))
            }
        }
    }

    fn render_init_by_reusing(
        Self(this): Self,
        updater: impl UpdateAttrValue<Kind = V>,
        State(state): &mut Self::State,
    ) {
        match (this, state) {
            (None, state) => {
                *state = None;
                updater.remove();
            }
            (Some(this), Some(state)) => {
                T::render_init_by_reusing_on_absent_attribute(this, updater, state)
            }
            (Some(this), state @ None) => {
                *state = Some(T::render_init_on_absent_attribute(this, updater))
            }
        }
    }

    fn render_update(
        Self(this): Self,
        updater: impl UpdateAttrValue<Kind = V>,
        State(state): &mut Self::State,
    ) {
        match (this, state) {
            (None, None) => {} // skip
            (Some(this), Some(state)) => T::render_update(this, updater, state),
            (Some(this), state @ None) => {
                *state = Some(T::render_init_on_absent_attribute(this, updater))
            }
            (None, state @ Some(_)) => {
                *state = None;
                updater.remove();
            }
        }
    }
}
