// Empty implements AttrValue<bool>, AttrValue<ContentEditable>, AttrValue<Spellcheck>

use frender_common::Empty;

use crate::{
    csr::{MaybeValue, ValueUpdater},
    ssr::MaybeIntoHtmlAttributeValue,
};

impl MaybeIntoHtmlAttributeValue<bool> for Empty {
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
        updater: impl ValueUpdater<bool>,
    ) {
        if !*state {
            *state = true;
            updater.update(true)
        }
    }

    fn state_could_skip_remove(state: &Self::UpdateWithState) -> bool {
        !*state
    }
}
