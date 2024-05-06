use async_str_iter::IntoAsyncStrIterator;

use crate::DomTokens;

trait StringValue: AsRef<str> + IntoAsyncStrIterator {}

frender_common::impl_many!(
    impl<__> StringValue
        for each_of![
            &str,
            String,
            std::borrow::Cow<'_, str>,
            std::rc::Rc<str>,
            std::sync::Arc<str>,
        ]
    {
    }
);

impl<S: StringValue> DomTokens for S {
    type UpdateWithState = Option<Self>;

    fn update_with_state(
        this: Self,
        dom_token_list: &mut impl crate::DomTokenList,
        state: &mut Self::UpdateWithState,
    ) {
        let value = this.as_ref();
        if let Some(state) = state {
            if state.as_ref() == value {
                return;
            }
        }

        dom_token_list.set_value(value);

        *state = Some(this);
    }

    fn remove_with_state(
        dom_token_list: &mut impl crate::DomTokenList,
        state: &mut Self::UpdateWithState,
    ) {
        dom_token_list.set_value("");
        *state = None;
    }

    type DomTokensIntoAsyncStrIter = async_str_iter::any_str::IterAnyStr<Self>;

    fn dom_tokens_into_async_str_iter(this: Self) -> Self::DomTokensIntoAsyncStrIter {
        async_str_iter::any_str::AnyStr(this).into_async_str_iterator()
    }
}
