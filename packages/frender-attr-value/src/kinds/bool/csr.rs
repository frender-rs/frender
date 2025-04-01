use crate::csr::{
    const_some::impl_csr_attr_value_for_const_some, CsrAttrValue, CsrAttrValueState,
    UpdateAttrValue,
};

use super::{BoolAsAttrValue, EmptyAsTrue};

impl CsrAttrValue<bool> for EmptyAsTrue {
    impl_csr_attr_value_for_const_some!((()) as bool);
}

pub struct BoolAsAttrValueState(bool);

impl CsrAttrValueState for BoolAsAttrValueState {
    fn attribute_is_known_as_absent(&self) -> bool {
        !self.0
    }
}

impl CsrAttrValue<bool> for BoolAsAttrValue {
    type State = BoolAsAttrValueState;

    fn render_init_on_absent_attribute(
        Self(this): Self,
        updater: impl UpdateAttrValue<Kind = bool>,
    ) -> Self::State {
        if this {
            updater.set(())
        }
        BoolAsAttrValueState(this)
    }

    fn render_init(Self(this): Self, updater: impl UpdateAttrValue<Kind = bool>) -> Self::State {
        if this {
            updater.set(())
        } else {
            updater.remove()
        }
        BoolAsAttrValueState(this)
    }

    fn render_init_by_reusing_on_absent_attribute(
        this: Self,
        updater: impl UpdateAttrValue<Kind = bool>,
        state: &mut Self::State,
    ) {
        *state = Self::render_init_on_absent_attribute(this, updater)
    }

    fn render_init_by_reusing(
        this: Self,
        updater: impl UpdateAttrValue<Kind = bool>,
        state: &mut Self::State,
    ) {
        *state = Self::render_init(this, updater)
    }

    fn render_update(
        this: Self,
        updater: impl UpdateAttrValue<Kind = bool>,
        state: &mut Self::State,
    ) {
        if this.0 == state.0 {
            return;
        }

        *state = Self::render_init(this, updater)
    }
}
