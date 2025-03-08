use crate::{dom_token::DomToken, IntoDomTokens};

/// See [DOMTokenList](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList).
pub trait DomTokenList {
    fn set_value(&mut self, value: &str);
    fn add_1(&mut self, token: DomToken);
    fn remove_1(&mut self, token: DomToken);
    fn replace(&mut self, old_token: DomToken, new_token: DomToken);
}

pub trait DomTokensStateUnmount {
    fn dom_tokens_state_unmount(state: &mut Self, dom_token_list: &mut impl DomTokenList);
}

pub trait CsrDomTokens {
    type State: DomTokensStateUnmount;

    fn dom_tokens_render_init(this: Self, dom_token_list: &mut impl DomTokenList) -> Self::State;
    fn dom_tokens_render_init_with_old_state(
        this: Self,
        dom_token_list: &mut impl DomTokenList,
        old_state: &mut Self::State,
    ) where
        Self: Sized,
    {
        *old_state = Self::dom_tokens_render_init(this, dom_token_list)
    }

    fn dom_tokens_render_update(
        this: Self,
        dom_token_list: &mut impl DomTokenList,
        state: &mut Self::State,
    );
}

impl<T: IntoDomTokens> CsrDomTokens for T {
    type State = <T::IntoDomTokens as CsrDomTokens>::State;

    fn dom_tokens_render_init(this: Self, dom_token_list: &mut impl DomTokenList) -> Self::State {
        <T::IntoDomTokens>::dom_tokens_render_init(this.into_dom_tokens(), dom_token_list)
    }

    fn dom_tokens_render_init_with_old_state(
        this: Self,
        dom_token_list: &mut impl DomTokenList,
        old_state: &mut Self::State,
    ) {
        <T::IntoDomTokens>::dom_tokens_render_init_with_old_state(
            this.into_dom_tokens(),
            dom_token_list,
            old_state,
        )
    }

    fn dom_tokens_render_update(
        this: Self,
        dom_token_list: &mut impl DomTokenList,
        state: &mut Self::State,
    ) {
        <T::IntoDomTokens>::dom_tokens_render_update(this.into_dom_tokens(), dom_token_list, state)
    }
}
