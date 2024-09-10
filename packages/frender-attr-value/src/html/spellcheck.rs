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
        use crate::{html::Spellcheck, ssr::SsrAttrValue};

        impl SsrAttrValue<Spellcheck> for Spellcheck {
            type HtmlAttributeValue = <bool as SsrAttrValue<Spellcheck>>::HtmlAttributeValue;

            fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
                SsrAttrValue::<Spellcheck>::maybe_into_html_attribute_value(this.0)
            }
        }
    }

    mod csr {
        use crate::{csr::CsrAttrValue, html::Spellcheck, impl_csr_attr_value_with_cache};

        impl CsrAttrValue<Spellcheck> for Spellcheck {
            type State = bool;

            impl_csr_attr_value_with_cache!(
                kind![Spellcheck],
                set = |this| this,
                into_cache = this.0,
                eq = |this, cache| this.0 == *cache,
            );
        }
    }
}

mod bool {
    mod ssr {
        use crate::{
            html::{bool_to_str, Spellcheck},
            ssr::SsrAttrValue,
        };

        impl SsrAttrValue<Spellcheck> for bool {
            type HtmlAttributeValue = <&'static str as SsrAttrValue<str>>::HtmlAttributeValue;

            fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
                <&'static str as SsrAttrValue<str>>::maybe_into_html_attribute_value(bool_to_str(
                    this,
                ))
            }
        }
    }

    mod csr {
        use crate::{csr::CsrAttrValue, html::Spellcheck, impl_csr_attr_value_with_cache};

        impl CsrAttrValue<Spellcheck> for bool {
            type State = Self;

            impl_csr_attr_value_with_cache!(
                kind![Spellcheck],
                set = |this| Spellcheck(this),
                eq = Self::eq,
            );
        }
    }
}

mod empty {
    mod ssr {
        use frender_common::Empty;

        use crate::{html::Spellcheck, ssr::SsrAttrValue};

        impl SsrAttrValue<Spellcheck> for Empty {
            type HtmlAttributeValue = async_str_iter::empty::Empty;

            fn maybe_into_html_attribute_value(Self: Self) -> Option<Self::HtmlAttributeValue> {
                Some(async_str_iter::empty::Empty)
            }
        }
    }

    mod csr {
        use frender_common::Empty;

        use crate::{csr::CsrAttrValue, html::Spellcheck, impl_csr_attr_value_for_unit_struct};

        impl CsrAttrValue<Spellcheck> for Empty {
            impl_csr_attr_value_for_unit_struct!((Spellcheck::EMPTY) as Spellcheck);
        }
    }
}
