use frender_html_common::{
    attr::MaybeIntoHtmlAttributeValue, ContentEditable, MaybeValue, Spellcheck,
};

/// Indicates an attribute is present but has an empty value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Empty;

impl MaybeIntoHtmlAttributeValue<bool> for Empty {
    type HtmlAttributeValue = async_str_iter::empty::Empty;

    fn maybe_into_html_attribute_value(Self: Self) -> Option<Self::HtmlAttributeValue> {
        Some(async_str_iter::empty::Empty)
    }
}

/// As [documented](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/contenteditable#value),
/// if the attribute is given without a value, like <label contenteditable>Example Label</label>, its value is treated as an empty string.
impl MaybeIntoHtmlAttributeValue<ContentEditable<'static>> for Empty {
    type HtmlAttributeValue = async_str_iter::empty::Empty;

    fn maybe_into_html_attribute_value(Self: Self) -> Option<Self::HtmlAttributeValue> {
        Some(async_str_iter::empty::Empty)
    }
}

impl MaybeIntoHtmlAttributeValue<Spellcheck> for Empty {
    type HtmlAttributeValue = async_str_iter::empty::Empty;

    fn maybe_into_html_attribute_value(Self: Self) -> Option<Self::HtmlAttributeValue> {
        Some(async_str_iter::empty::Empty)
    }
}

impl MaybeValue<bool> for Empty {
    // whether initialized
    type UpdateWithState = bool;

    fn update_with_state(
        _: Self,
        state: &mut Self::UpdateWithState,
        updater: impl frender_html_common::ValueUpdater<bool>,
    ) {
        if !*state {
            *state = true;
            updater.update(true)
        }
    }
}

/// As [documented](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/contenteditable#value),
/// if the attribute is given without a value, like <label contenteditable>Example Label</label>, its value is treated as an empty string,
/// which is the same as `"true"`.
impl MaybeValue<ContentEditable<'static>> for Empty {
    // whether initialized
    type UpdateWithState = bool;

    fn update_with_state(
        _: Self,
        state: &mut Self::UpdateWithState,
        updater: impl frender_html_common::ValueUpdater<ContentEditable<'static>>,
    ) {
        if !*state {
            *state = true;
            updater.update(ContentEditable::EMPTY)
        }
    }
}

impl MaybeValue<Spellcheck> for Empty {
    // whether initialized
    type UpdateWithState = bool;

    fn update_with_state(
        _: Self,
        state: &mut Self::UpdateWithState,
        updater: impl frender_html_common::ValueUpdater<Spellcheck>,
    ) {
        if !*state {
            *state = true;
            updater.update(Spellcheck::EMPTY)
        }
    }
}
