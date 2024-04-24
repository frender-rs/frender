use crate::{attr::MaybeIntoHtmlAttributeEqValueOrEmpty, MaybeUpdateValueWithState, StringValue};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentEditable {
    /// true or an empty string, which indicates that the element is editable.
    True,
    /// false, which indicates that the element is not editable.
    False,
    /// plaintext-only, which indicates that the element's raw text is editable, but rich text formatting is disabled.
    PlaintextOnly,
}

pub trait MaybeContentEditable: MaybeIntoHtmlAttributeEqValueOrEmpty<ContentEditable> {
    type UpdateWithState: Default;

    fn update_with_state(
        this: Self,
        updater: impl crate::ValueUpdater<str>,
        state: &mut Self::UpdateWithState,
    );
}

impl<V: StringValue> MaybeIntoHtmlAttributeEqValueOrEmpty<ContentEditable> for V {
    type HtmlAttributeEqValueOrEmpty =
        <V as MaybeIntoHtmlAttributeEqValueOrEmpty<str>>::HtmlAttributeEqValueOrEmpty;

    fn maybe_into_html_attribute_eq_value_or_empty(
        this: Self,
    ) -> Option<Self::HtmlAttributeEqValueOrEmpty> {
        <V as  MaybeIntoHtmlAttributeEqValueOrEmpty<str>>::maybe_into_html_attribute_eq_value_or_empty(this)
    }
}

impl MaybeIntoHtmlAttributeEqValueOrEmpty<ContentEditable> for bool {
    type HtmlAttributeEqValueOrEmpty =
        <&'static str as MaybeIntoHtmlAttributeEqValueOrEmpty<str>>::HtmlAttributeEqValueOrEmpty;

    fn maybe_into_html_attribute_eq_value_or_empty(
        this: Self,
    ) -> Option<Self::HtmlAttributeEqValueOrEmpty> {
        <&'static str as MaybeIntoHtmlAttributeEqValueOrEmpty<str>>::maybe_into_html_attribute_eq_value_or_empty(bool_to_str(this))
    }
}

impl<V: StringValue> MaybeContentEditable for V {
    type UpdateWithState = <V as MaybeUpdateValueWithState<str>>::UpdateWithState;

    fn update_with_state(
        this: Self,
        updater: impl crate::ValueUpdater<str>,
        state: &mut Self::UpdateWithState,
    ) {
        <V as MaybeUpdateValueWithState<str>>::update_with_state(this, state, updater)
    }
}

fn bool_to_str(this: bool) -> &'static str {
    if this {
        "true"
    } else {
        "false"
    }
}

impl MaybeContentEditable for bool {
    type UpdateWithState = Option<Self>;

    fn update_with_state(
        this: Self,
        updater: impl crate::ValueUpdater<str>,
        state: &mut Self::UpdateWithState,
    ) {
        if *state == Some(this) {
            return;
        }
        *state = Some(this);
        updater.update(bool_to_str(this));
    }
}

impl<V: MaybeContentEditable> MaybeContentEditable for Option<V> {
    type UpdateWithState = V::UpdateWithState;

    fn update_with_state(
        this: Self,
        updater: impl crate::ValueUpdater<str>,
        state: &mut Self::UpdateWithState,
    ) {
        if let Some(this) = this {
            V::update_with_state(this, updater, state);
        } else {
            *state = Default::default();
            updater.remove();
        }
    }
}

impl MaybeContentEditable for () {
    type UpdateWithState = ();

    fn update_with_state(
        (): Self,
        _: impl crate::ValueUpdater<str>,
        (): &mut Self::UpdateWithState,
    ) {
    }
}
