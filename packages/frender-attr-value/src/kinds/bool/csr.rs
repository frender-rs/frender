use frender_common::Empty;

use crate::{
    csr::{CsrAttrValue, UpdateAttrValue},
    impl_csr_attr_value_for_unit_struct,
};

impl CsrAttrValue<bool> for Empty {
    impl_csr_attr_value_for_unit_struct!((()) as bool);
}

impl CsrAttrValue<bool> for bool {
    type State = Self;

    fn update_absent_attribute_value_into_state(
        this: Self,
        updater: impl UpdateAttrValue<Kind = bool>,
    ) -> Self::State {
        if this {
            updater.set(())
        }
        this
    }

    fn update_attribute_value_into_state(
        this: Self,
        updater: impl UpdateAttrValue<Kind = bool>,
    ) -> Self::State {
        if this {
            updater.set(())
        } else {
            updater.remove()
        }
        this
    }

    fn can_skip_update(this: &Self, state: &Self::State) -> bool {
        *this == *state
    }

    fn attribute_is_known_as_absent(state: &Self::State) -> bool {
        !*state
    }
}
