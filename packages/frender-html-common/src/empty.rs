use frender_common::Empty;

impl crate::MaybeStringValue for Empty {
    type StringValue = async_str_iter::never::Never;

    fn maybe_string_value(Self: Self) -> Option<Self::StringValue> {
        None
    }
}

impl crate::IntoOneStringOrEmpty for Empty {
    type OneStringOrEmpty = async_str_iter::empty::Empty;

    fn into_one_string_or_empty(Self: Self) -> Self::OneStringOrEmpty {
        async_str_iter::empty::Empty
    }
}

mod attr {
    use frender_common::Empty;

    use crate::{attr::MaybeIntoHtmlAttributeValue, ContentEditable, MaybeValue, Spellcheck};

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
            updater: impl crate::ValueUpdater<bool>,
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
            updater: impl crate::ValueUpdater<ContentEditable<'static>>,
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
            updater: impl crate::ValueUpdater<Spellcheck>,
        ) {
            if !*state {
                *state = true;
                updater.update(Spellcheck::EMPTY)
            }
        }
    }
}
