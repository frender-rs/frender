use crate::{attr::MaybeIntoHtmlAttributeValue, MaybeValue, StringValue, ValueUpdater};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContentEditable<'a>(pub &'a str);

impl ContentEditable<'static> {
    pub const EMPTY: Self = Self("");
}

impl<'a> MaybeIntoHtmlAttributeValue<ContentEditable<'static>> for ContentEditable<'a> {
    type HtmlAttributeValue = <&'a str as MaybeIntoHtmlAttributeValue<str>>::HtmlAttributeValue;

    fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
        <&'a str as MaybeIntoHtmlAttributeValue<str>>::maybe_into_html_attribute_value(this.0)
    }
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

impl<V: StringValue> MaybeValue<ContentEditable<'static>> for V {
    type UpdateWithState = <V as MaybeValue<str>>::UpdateWithState;

    fn update_with_state(
        this: Self,
        state: &mut Self::UpdateWithState,
        updater: impl ValueUpdater<ContentEditable<'static>>,
    ) {
        struct UpdateStr<U: ValueUpdater<ContentEditable<'static>>>(U);

        impl<U: ValueUpdater<ContentEditable<'static>>> ValueUpdater<str> for UpdateStr<U> {
            fn update(self, value: <str as crate::ValueKind>::Value<'_>) {
                self.0.update(ContentEditable(value))
            }

            fn remove(self) {
                self.0.remove()
            }
        }

        <V as MaybeValue<str>>::update_with_state(this, state, UpdateStr(updater))
    }
}

fn bool_to_str(this: bool) -> &'static str {
    if this {
        "true"
    } else {
        "false"
    }
}

/// `true` is mapped to `"true". `false` is mapped to "false"`.
impl MaybeValue<ContentEditable<'static>> for bool {
    type UpdateWithState = Option<Self>;

    fn update_with_state(
        this: Self,
        state: &mut Self::UpdateWithState,
        updater: impl ValueUpdater<ContentEditable<'static>>,
    ) {
        if *state == Some(this) {
            return;
        }
        *state = Some(this);
        updater.update(ContentEditable(bool_to_str(this)));
    }
}
