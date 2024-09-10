mod ssr {
    use crate::ssr::SsrAttrValue;

    impl<AttributeType: ?Sized, V: SsrAttrValue<AttributeType>> SsrAttrValue<AttributeType>
        for Option<V>
    {
        type HtmlAttributeValue = V::HtmlAttributeValue;

        fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
            this.and_then(V::maybe_into_html_attribute_value)
        }
    }
}

mod csr {
    use crate::csr::{CsrAttrValue, UpdateAttrValue, ValueKind};

    impl<T: CsrAttrValue<V>, V: ?Sized + ValueKind> CsrAttrValue<V> for Option<T> {
        type State = Option<T::State>;

        fn update_absent_attribute_value_into_state(
            this: Self,
            updater: impl UpdateAttrValue<Kind = V>,
        ) -> Self::State {
            match this {
                Some(this) => Some(T::update_absent_attribute_value_into_state(this, updater)),
                None => {
                    // the attribute is absent, so we don't need to remove it
                    None
                }
            }
        }

        fn update_attribute_value_into_state(
            this: Self,
            updater: impl UpdateAttrValue<Kind = V>,
        ) -> Self::State {
            match this {
                Some(this) => Some(T::update_attribute_value_into_state(this, updater)),
                None => {
                    updater.remove();
                    None
                }
            }
        }

        fn can_skip_update(this: &Self, state: &Self::State) -> bool {
            match (this, state) {
                (None, None) => true,
                (Some(this), Some(state)) => T::can_skip_update(this, state),
                _ => false,
            }
        }

        fn update_attribute_value_with_state(
            this: Self,
            updater: impl UpdateAttrValue<Kind = V>,
            state: &mut Self::State,
        ) {
            match (this, state) {
                (None, None) => {} // skip
                (Some(this), Some(state)) => {
                    T::update_attribute_value_with_state(this, updater, state)
                }
                (Some(this), state @ None) => {
                    *state = Some(T::update_absent_attribute_value_into_state(this, updater))
                }
                (None, state @ Some(_)) => {
                    *state = None;
                    updater.remove();
                }
            }
        }

        fn force_update_attribute_value_with_state(
            this: Self,
            updater: impl UpdateAttrValue<Kind = V>,
            state: &mut Self::State,
        ) {
            match (this, state) {
                (None, state) => {
                    *state = None;
                    updater.remove();
                }
                (Some(this), Some(state)) => {
                    T::force_update_attribute_value_with_state(this, updater, state)
                }
                (Some(this), state @ None) => {
                    *state = Some(T::update_absent_attribute_value_into_state(this, updater))
                }
            }
        }

        fn attribute_is_known_as_absent(state: &Self::State) -> bool {
            match state {
                None => true,
                Some(state) => T::attribute_is_known_as_absent(state),
            }
        }
    }
}
