use crate::{attr::MaybeIntoHtmlAttributeValue, bool_to_str, MaybeValue, ValueUpdater};

/// See html attribute [spellcheck](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/spellcheck).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Spellcheck(pub bool);

impl Spellcheck {
    /// empty string or true, which indicates that the element should be, if possible, checked for spelling errors
    pub const EMPTY: Self = Self(true);
}

impl MaybeIntoHtmlAttributeValue<Spellcheck> for Spellcheck {
    type HtmlAttributeValue = <bool as MaybeIntoHtmlAttributeValue<Spellcheck>>::HtmlAttributeValue;

    fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
        MaybeIntoHtmlAttributeValue::<Spellcheck>::maybe_into_html_attribute_value(this.0)
    }
}

impl MaybeIntoHtmlAttributeValue<Spellcheck> for bool {
    type HtmlAttributeValue =
        <&'static str as MaybeIntoHtmlAttributeValue<str>>::HtmlAttributeValue;

    fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
        <&'static str as MaybeIntoHtmlAttributeValue<str>>::maybe_into_html_attribute_value(
            bool_to_str(this),
        )
    }
}

/// `true` is mapped to `"true". `false` is mapped to "false"`.
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
}
