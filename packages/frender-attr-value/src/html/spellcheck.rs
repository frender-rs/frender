/// See html attribute [spellcheck](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/spellcheck).
///
/// ## Types that impl [`AttrValue<Spellcheck>`]
///
/// - [`Spellcheck`]
/// - [`bool`]
///
///   `true` is mapped to `"true". `false` is mapped to "false"`.
///
/// - [`Empty`](frender_common::Empty)
///
///   An empty string, which is the same as `true`.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Spellcheck(pub bool);

impl Spellcheck {
    /// empty string or true, which indicates that the element should be, if possible, checked for spelling errors
    pub const EMPTY: Self = Self(true);
}

impl crate::csr::ValueKind for Spellcheck {
    type Value<'a> = Spellcheck;
}

mod impl_spellcheck {
    mod ssr {
        use crate::{html::Spellcheck, ssr::MaybeIntoHtmlAttributeValue};

        impl MaybeIntoHtmlAttributeValue<Spellcheck> for Spellcheck {
            type HtmlAttributeValue =
                <bool as MaybeIntoHtmlAttributeValue<Spellcheck>>::HtmlAttributeValue;

            fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
                MaybeIntoHtmlAttributeValue::<Spellcheck>::maybe_into_html_attribute_value(this.0)
            }
        }
    }

    mod csr {
        use crate::{
            csr::{MaybeValue, ValueUpdater},
            html::Spellcheck,
        };

        impl MaybeValue<Spellcheck> for Spellcheck {
            type UpdateWithState = Option<bool>;

            fn update_with_state(
                this: Self,
                state: &mut Self::UpdateWithState,
                updater: impl ValueUpdater<Spellcheck>,
            ) {
                <bool>::update_with_state(this.0, state, updater)
            }

            fn state_could_skip_remove(state: &Self::UpdateWithState) -> bool {
                <bool as MaybeValue<Spellcheck>>::state_could_skip_remove(state)
            }
        }
    }
}

mod bool {
    mod ssr {
        use crate::{
            html::{bool_to_str, Spellcheck},
            ssr::MaybeIntoHtmlAttributeValue,
        };

        impl MaybeIntoHtmlAttributeValue<Spellcheck> for bool {
            type HtmlAttributeValue =
                <&'static str as MaybeIntoHtmlAttributeValue<str>>::HtmlAttributeValue;

            fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
                <&'static str as MaybeIntoHtmlAttributeValue<str>>::maybe_into_html_attribute_value(
                    bool_to_str(this),
                )
            }
        }
    }

    mod csr {
        use crate::{
            csr::{MaybeValue, ValueUpdater},
            html::Spellcheck,
        };

        impl MaybeValue<Spellcheck> for bool {
            type UpdateWithState = Option<Self>;

            fn update_with_state(
                this: Self,
                state: &mut Self::UpdateWithState,
                updater: impl ValueUpdater<Spellcheck>,
            ) {
                if *state == Some(this) {
                    return;
                }
                *state = Some(this);
                updater.update(Spellcheck(this));
            }

            fn state_could_skip_remove(state: &Self::UpdateWithState) -> bool {
                state.is_none()
            }
        }
    }
}

mod empty {
    mod ssr {
        use frender_common::Empty;

        use crate::{html::Spellcheck, ssr::MaybeIntoHtmlAttributeValue};

        impl MaybeIntoHtmlAttributeValue<Spellcheck> for Empty {
            type HtmlAttributeValue = async_str_iter::empty::Empty;

            fn maybe_into_html_attribute_value(Self: Self) -> Option<Self::HtmlAttributeValue> {
                Some(async_str_iter::empty::Empty)
            }
        }
    }

    mod csr {
        use frender_common::Empty;

        use crate::{
            csr::{MaybeValue, ValueUpdater},
            html::Spellcheck,
        };

        impl MaybeValue<Spellcheck> for Empty {
            // whether initialized
            type UpdateWithState = bool;

            fn update_with_state(
                _: Self,
                state: &mut Self::UpdateWithState,
                updater: impl ValueUpdater<Spellcheck>,
            ) {
                if !*state {
                    *state = true;
                    updater.update(Spellcheck::EMPTY)
                }
            }

            fn state_could_skip_remove(state: &Self::UpdateWithState) -> bool {
                !*state
            }
        }
    }
}
