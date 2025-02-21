use frender_ssr::html::assert::HtmlAttributeEqValueOrEmpty;

use crate::{FormControlValueKind, ProvideFormControlValue};

pub trait InputValueKindSsr: FormControlValueKind {
    type IntoInputValueAttrValue<V: ProvideFormControlValue<Self>>: HtmlAttributeEqValueOrEmpty;
    fn into_input_value_attr_value<V: ProvideFormControlValue<Self>>(
        v: V,
        input_type: &str,
    ) -> Self::IntoInputValueAttrValue<V>;
}
