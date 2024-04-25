use crate::{attr::MaybeIntoHtmlAttributeValue, MaybeValue, StringValue};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContentEditable<'a>(pub &'a str);

impl ContentEditable<'static> {
    pub const EMPTY: Self = Self("");
}

pub trait MaybeContentEditable: MaybeIntoHtmlAttributeValue<ContentEditable<'static>> {
    type UpdateWithState: Default;

    fn update_with_state(
        this: Self,
        updater: impl crate::ValueUpdater<str>,
        state: &mut Self::UpdateWithState,
    );
}

impl<V: StringValue> MaybeIntoHtmlAttributeValue<ContentEditable<'static>> for V {
    type HtmlAttributeValue = <V as MaybeIntoHtmlAttributeValue<str>>::HtmlAttributeValue;

    fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
        <V as MaybeIntoHtmlAttributeValue<str>>::maybe_into_html_attribute_value(this)
    }
}

impl MaybeIntoHtmlAttributeValue<ContentEditable<'static>> for bool {
    type HtmlAttributeValue =
        <&'static str as MaybeIntoHtmlAttributeValue<str>>::HtmlAttributeValue;

    fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
        <&'static str as MaybeIntoHtmlAttributeValue<str>>::maybe_into_html_attribute_value(
            bool_to_str(this),
        )
    }
}

impl<V: StringValue> MaybeContentEditable for V {
    type UpdateWithState = <V as MaybeValue<str>>::UpdateWithState;

    fn update_with_state(
        this: Self,
        updater: impl crate::ValueUpdater<str>,
        state: &mut Self::UpdateWithState,
    ) {
        <V as MaybeValue<str>>::update_with_state(this, state, updater)
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
