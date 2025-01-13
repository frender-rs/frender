/// ## impl [`AttrValue<ContentEditable>`](crate::AttrValue) for
///
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

        use crate::{html::ContentEditable, ssr::SsrAttrValue};

        impl SsrAttrValue<ContentEditable> for Empty {
            type HtmlAttributeValue = async_str_iter::empty::Empty;

            fn maybe_into_html_attribute_value(Self: Self) -> Option<Self::HtmlAttributeValue> {
                Some(async_str_iter::empty::Empty)
            }
        }
    }

    mod csr {
        use frender_common::Empty;

        use crate::{
            csr::CsrAttrValue, html::ContentEditable, impl_csr_attr_value_for_unit_struct,
        };

        impl CsrAttrValue<ContentEditable> for Empty {
            impl_csr_attr_value_for_unit_struct!(("") as ContentEditable);
        }
    }
}

mod bool {
    mod ssr {
        use crate::{
            html::{bool_to_str, ContentEditable},
            ssr::SsrAttrValue,
        };

        impl SsrAttrValue<ContentEditable> for bool {
            type HtmlAttributeValue = <&'static str as SsrAttrValue<str>>::HtmlAttributeValue;

            fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
                <&'static str as SsrAttrValue<str>>::maybe_into_html_attribute_value(bool_to_str(
                    this,
                ))
            }
        }
    }

    mod csr {
        use crate::{
            csr::CsrAttrValue,
            html::{bool_to_str, ContentEditable},
            impl_csr_attr_value_with_cache,
        };

        impl CsrAttrValue<ContentEditable> for bool {
            type State = Self;

            impl_csr_attr_value_with_cache!(
                kind![ContentEditable],
                set = |this| bool_to_str(this),
                eq = Self::eq,
            );
        }
    }
}

mod string {
    mod ssr {
        use crate::{html::ContentEditable, ssr::SsrAttrValue, values::str::KnownSsrStr};

        impl<V: KnownSsrStr> SsrAttrValue<ContentEditable> for V {
            type HtmlAttributeValue = <Self as SsrAttrValue<str>>::HtmlAttributeValue;

            fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
                <Self as SsrAttrValue<str>>::maybe_into_html_attribute_value(this)
            }
        }
    }

    mod csr {

        use crate::{
            csr::{CsrAttrValue, UpdateAttrValue},
            html::ContentEditable,
            values::str::KnownCsrStr,
        };

        struct UpdateStr<U: UpdateAttrValue<Kind = ContentEditable>>(U);

        impl<U: UpdateAttrValue<Kind = ContentEditable>> UpdateAttrValue for UpdateStr<U> {
            type Kind = str;

            fn set(self, value: &str) {
                self.0.set(value)
            }

            fn remove(self) {
                self.0.remove()
            }
        }

        impl<V: KnownCsrStr> CsrAttrValue<ContentEditable> for V {
            type State = <Self as CsrAttrValue<str>>::State;

            fn update_attribute_value_into_state(
                this: Self,
                updater: impl UpdateAttrValue<Kind = ContentEditable>,
            ) -> Self::State {
                <Self as CsrAttrValue<str>>::update_attribute_value_into_state(
                    this,
                    UpdateStr(updater),
                )
            }

            fn can_skip_update(this: &Self, state: &Self::State) -> bool {
                <Self as CsrAttrValue<str>>::can_skip_update(this, state)
            }

            fn update_attribute_value_with_state(
                this: Self,
                updater: impl UpdateAttrValue<Kind = ContentEditable>,
                state: &mut Self::State,
            ) {
                <Self as CsrAttrValue<str>>::update_attribute_value_with_state(
                    this,
                    UpdateStr(updater),
                    state,
                )
            }

            fn force_update_attribute_value_with_state(
                this: Self,
                updater: impl UpdateAttrValue<Kind = ContentEditable>,
                state: &mut Self::State,
            ) {
                <Self as CsrAttrValue<str>>::force_update_attribute_value_with_state(
                    this,
                    UpdateStr(updater),
                    state,
                )
            }
        }
    }
}
