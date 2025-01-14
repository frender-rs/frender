use async_str_iter::IntoAsyncStrIterator;

use crate::{DomTokens, DomTokensStateUnmount};

// TODO: TempStr

pub struct State<S>(S);

// TODO: figure out a better design for non-chainable dom tokens
impl<S> DomTokensStateUnmount for State<S> {
    fn dom_tokens_state_unmount(_: &mut Self, dom_token_list: &mut impl crate::DomTokenList) {
        dom_token_list.set_value("");
    }
}

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
        type State = State<Self>;

        fn dom_tokens_render_init(
            this: Self,
            dom_token_list: &mut impl crate::DomTokenList,
        ) -> Self::State {
            let value = this.as_ref();

            dom_token_list.set_value(value);

            State(this)
        }

        fn dom_tokens_render_update(
            this: Self,
            dom_token_list: &mut impl crate::DomTokenList,
            state: &mut Self::State,
        ) {
            if state.0 == this {
                return;
            }

            *state = Self::dom_tokens_render_init(this, dom_token_list)
        }

        type DomTokensIntoAsyncStrIter = async_str_iter::any_str::IterAnyStr<Self>;

        fn dom_tokens_into_async_str_iter(this: Self) -> Self::DomTokensIntoAsyncStrIter {
            async_str_iter::any_str::AnyStr(this).into_async_str_iterator()
        }
    }
);
