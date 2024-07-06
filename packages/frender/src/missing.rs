use frender_html_common::{attr::MaybeIntoHtmlAttributeValue, MaybeValue, ValueKind};

/// Indicates an attribute is missing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Missing;

impl<VK: ValueKind> MaybeIntoHtmlAttributeValue<VK> for Missing {
    type HtmlAttributeValue = async_str_iter::never::Never;

    fn maybe_into_html_attribute_value(Self: Self) -> Option<Self::HtmlAttributeValue> {
        None
    }
}

impl<VK: ValueKind> MaybeValue<VK> for Missing {
    // whether initialized
    type UpdateWithState = bool;

    fn update_with_state(
        Self: Self,
        state: &mut Self::UpdateWithState,
        updater: impl frender_html_common::ValueUpdater<VK>,
    ) {
        if !*state {
            *state = true;
            updater.remove()
        }
    }
}
