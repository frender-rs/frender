/// - [`Empty`](frender_common::Empty)
///
///   As [documented](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/contenteditable#value),
///   if the attribute is given without a value, like <label contenteditable>Example Label</label>, its value is treated as an empty string,
///   which is the same as `"true"`.
///
/// - [`bool`]
///
///   `true` is mapped to `"true". `false` is mapped to "false"`.
///
/// - strings
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentEditable {}

impl crate::csr::ValueKind for ContentEditable {
    type Value<'a> = &'a str;
}

mod empty {
    mod ssr {
        use frender_common::Empty;

        use crate::{html::ContentEditable, ssr::MaybeIntoHtmlAttributeValue};

        impl MaybeIntoHtmlAttributeValue<ContentEditable> for Empty {
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
            html::ContentEditable,
        };

        impl MaybeValue<ContentEditable> for Empty {
            // whether initialized
            type UpdateWithState = bool;

            fn update_with_state(
                _: Self,
                state: &mut Self::UpdateWithState,
                updater: impl ValueUpdater<ContentEditable>,
            ) {
                if !*state {
                    *state = true;
                    updater.update("")
                }
            }

            fn state_could_skip_remove(state: &Self::UpdateWithState) -> bool {
                !*state
            }
        }
    }
}

mod bool {
    mod ssr {
        use crate::{
            html::{bool_to_str, ContentEditable},
            ssr::MaybeIntoHtmlAttributeValue,
        };

        impl MaybeIntoHtmlAttributeValue<ContentEditable> for bool {
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
            html::{bool_to_str, ContentEditable},
        };

        impl MaybeValue<ContentEditable> for bool {
            type UpdateWithState = Option<Self>;

            fn update_with_state(
                this: Self,
                state: &mut Self::UpdateWithState,
                updater: impl ValueUpdater<ContentEditable>,
            ) {
                if *state == Some(this) {
                    return;
                }
                *state = Some(this);
                updater.update(bool_to_str(this));
            }

            fn state_could_skip_remove(state: &Self::UpdateWithState) -> bool {
                state.is_none()
            }
        }
    }
}

mod string {
    mod ssr {
        use frender_common::{TempStr, ToStaticStr};

        use crate::{
            html::ContentEditable, ssr::MaybeIntoHtmlAttributeValue, string::KnownStaticStr,
        };

        impl<V: KnownStaticStr> MaybeIntoHtmlAttributeValue<ContentEditable> for V {
            type HtmlAttributeValue =
                <Self as MaybeIntoHtmlAttributeValue<str>>::HtmlAttributeValue;

            fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
                <Self as MaybeIntoHtmlAttributeValue<str>>::maybe_into_html_attribute_value(this)
            }
        }

        impl<S: ToStaticStr> MaybeIntoHtmlAttributeValue<ContentEditable> for TempStr<S> {
            type HtmlAttributeValue =
                <Self as MaybeIntoHtmlAttributeValue<str>>::HtmlAttributeValue;

            fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
                <Self as MaybeIntoHtmlAttributeValue<str>>::maybe_into_html_attribute_value(this)
            }
        }
    }

    mod csr {
        use frender_common::{ToAsRefStr, ToStaticCache};

        use crate::{
            csr::{MaybeValue, ValueUpdater},
            html::ContentEditable,
            string::KnownStaticStr,
        };

        struct UpdateStr<U: ValueUpdater<ContentEditable>>(U);

        impl<U: ValueUpdater<ContentEditable>> ValueUpdater<str> for UpdateStr<U> {
            fn update(self, value: &str) {
                self.0.update(value)
            }

            fn remove(self) {
                self.0.remove()
            }
        }

        impl<V: KnownStaticStr> MaybeValue<ContentEditable> for V {
            type UpdateWithState = <Self as MaybeValue<str>>::UpdateWithState;

            fn update_with_state(
                this: Self,
                state: &mut Self::UpdateWithState,
                updater: impl ValueUpdater<ContentEditable>,
            ) {
                <Self as MaybeValue<str>>::update_with_state(this, state, UpdateStr(updater))
            }

            fn state_could_skip_remove(state: &Self::UpdateWithState) -> bool {
                <Self as MaybeValue<str>>::state_could_skip_remove(state)
            }
        }

        impl<S: ToStaticCache + ToAsRefStr> MaybeValue<ContentEditable> for frender_common::TempStr<S> {
            type UpdateWithState = <Self as MaybeValue<str>>::UpdateWithState;

            fn update_with_state(
                this: Self,
                state: &mut Self::UpdateWithState,
                updater: impl ValueUpdater<ContentEditable>,
            ) {
                <Self as MaybeValue<str>>::update_with_state(this, state, UpdateStr(updater))
            }

            fn state_could_skip_remove(state: &Self::UpdateWithState) -> bool {
                <Self as MaybeValue<str>>::state_could_skip_remove(state)
            }
        }
    }
}
