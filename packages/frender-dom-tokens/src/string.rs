use async_str_iter::IntoAsyncStrIterator;

use crate::DomTokens;

// TODO: TempStr

frender_common::impl_many!(
    impl<__> DomTokens
        for each_of![
            &'static str,
            String,
            std::borrow::Cow<'static, str>,
            std::rc::Rc<str>,
            std::sync::Arc<str>,
        ]
    {
        type UpdateWithState = Option<Self>;

        fn update_with_state(
            this: Self,
            dom_token_list: &mut impl crate::DomTokenList,
            state: &mut Self::UpdateWithState,
        ) {
            if let Some(state) = state {
                if *state == this {
                    return;
                }
            }

            let value = this.as_ref();

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
);
