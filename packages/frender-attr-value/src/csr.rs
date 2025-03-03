use crate::{AttrValue, AttrValueKind};

pub trait UpdateAttrValue {
    type Kind: ?Sized + AttrValueKind;

    fn set(self, value: <Self::Kind as AttrValueKind>::AttrValue<'_>);

    fn remove(self);
}

/// Unlike CsrStyle and CsrDomTokens, AttrValue doesn't allow chaining.
/// Thus, [`CsrAttrValue`] doesn't have method `remove_with_state`.
pub trait CsrAttrValue<AK: ?Sized + AttrValueKind>: Sized + AttrValue<AK> {
    type State;

    fn update_absent_attribute_value_into_state(
        this: Self,
        updater: impl UpdateAttrValue<Kind = AK>,
    ) -> Self::State {
        Self::update_attribute_value_into_state(this, updater)
    }

    fn update_attribute_value_into_state(
        this: Self,
        updater: impl UpdateAttrValue<Kind = AK>,
    ) -> Self::State;

    fn can_skip_update(this: &Self, state: &Self::State) -> bool;

    fn update_attribute_value_with_state(
        this: Self,
        updater: impl UpdateAttrValue<Kind = AK>,
        state: &mut Self::State,
    ) {
        if Self::can_skip_update(&this, state) {
            return;
        }
        Self::force_update_attribute_value_with_state(this, updater, state)
    }

    fn force_update_attribute_value_with_state(
        this: Self,
        updater: impl UpdateAttrValue<Kind = AK>,
        state: &mut Self::State,
    ) {
        *state = if Self::attribute_is_known_as_absent(state) {
            Self::update_absent_attribute_value_into_state(this, updater)
        } else {
            Self::update_attribute_value_into_state(this, updater)
        }
    }

    /// Returning `true` implies that the attribute is absent.
    /// Returning `false` implies that the attribute might be absent.
    ///
    /// This method is to optimize `impl CsrAttrValue for Option, Either`
    ///
    /// Always returning `false` is correct.
    fn attribute_is_known_as_absent(state: &Self::State) -> bool {
        let _ = state;
        false
    }

    fn update_attribute_value_with_option_state(
        this: Self,
        updater: impl UpdateAttrValue<Kind = AK>,
        state: &mut Option<Self::State>,
    ) {
        if let Some(state) = state {
            Self::update_attribute_value_with_state(this, updater, state)
        } else {
            *state = Some(Self::update_absent_attribute_value_into_state(
                this, updater,
            ))
        }
    }
}

#[doc(hidden)]
pub mod __private {
    pub use frender_common::expand;
}

pub(crate) mod cached_some;

pub(crate) mod macros;
