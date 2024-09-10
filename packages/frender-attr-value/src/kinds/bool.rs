mod kind {
    use crate::csr::ValueKind;

    impl ValueKind for bool {
        /// Boolean attributes doesn't have value.
        type Value<'a> = ();
    }
}

mod empty {
    mod ssr {
        use frender_common::Empty;

        use crate::ssr::SsrAttrValue;

        impl SsrAttrValue<bool> for Empty {
            type HtmlAttributeValue = async_str_iter::empty::Empty;

            fn maybe_into_html_attribute_value(Self: Self) -> Option<Self::HtmlAttributeValue> {
                Some(async_str_iter::empty::Empty)
            }
        }
    }

    mod csr {
        use crate::impl_csr_attr_value_for_unit_struct;

        use frender_common::Empty;

        use crate::csr::CsrAttrValue;

        /// For boolean attributes, [`Empty`] indicates a present attribute with empty value.
        impl CsrAttrValue<bool> for Empty {
            impl_csr_attr_value_for_unit_struct!((()) as bool);
        }
    }
}

mod impl_bool {
    mod ssr {
        use crate::ssr::SsrAttrValue;

        impl SsrAttrValue<bool> for bool {
            type HtmlAttributeValue = async_str_iter::empty::Empty;

            fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
                this.then_some(async_str_iter::empty::Empty)
            }
        }
    }

    mod csr {
        use crate::csr::{CsrAttrValue, UpdateAttrValue};

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
    }
}
