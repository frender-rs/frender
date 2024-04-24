use frender_html_common::{
    attr::MaybeIntoHtmlAttributeEqValueOrEmpty, content_editable::ContentEditable,
    MaybeValue,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Omitted;

impl MaybeIntoHtmlAttributeEqValueOrEmpty<bool> for Omitted {
    type HtmlAttributeEqValueOrEmpty = async_str_iter::empty::Empty;

    fn maybe_into_html_attribute_eq_value_or_empty(
        Self: Self,
    ) -> Option<Self::HtmlAttributeEqValueOrEmpty> {
        Some(async_str_iter::empty::Empty)
    }
}

/// As [documented](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/contenteditable#value),
/// if the attribute is given without a value, like <label contenteditable>Example Label</label>, its value is treated as an empty string.
impl MaybeIntoHtmlAttributeEqValueOrEmpty<ContentEditable> for Omitted {
    type HtmlAttributeEqValueOrEmpty = async_str_iter::empty::Empty;

    fn maybe_into_html_attribute_eq_value_or_empty(
        Self: Self,
    ) -> Option<Self::HtmlAttributeEqValueOrEmpty> {
        Some(async_str_iter::empty::Empty)
    }
}

impl MaybeValue<bool> for Omitted {
    // whether initialized
    type UpdateWithState = bool;

    fn update_with_state(
        _: Self,
        state: &mut Self::UpdateWithState,
        updater: impl frender_html_common::ValueUpdater<bool>,
    ) {
        if !*state {
            updater.update(&true)
        }
    }
}

/// As [documented](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/contenteditable#value),
/// if the attribute is given without a value, like <label contenteditable>Example Label</label>, its value is treated as an empty string,
/// which is the same as `"true"`.
impl MaybeValue<ContentEditable> for Omitted {
    // whether initialized
    type UpdateWithState = bool;

    fn update_with_state(
        _: Self,
        state: &mut Self::UpdateWithState,
        updater: impl frender_html_common::ValueUpdater<ContentEditable>,
    ) {
        if !*state {
            updater.update(&ContentEditable::True)
        }
    }
}
