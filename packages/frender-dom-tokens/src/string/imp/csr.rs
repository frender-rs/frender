use frender_reactive_value::temp_ref::TempRef;

use crate::csr::{CsrDomTokens, DomTokenList, DomTokensStateUnmount};

use super::super::KnownNonReactiveStr;
use super::NonReactiveStrIntoDomTokens;

pub struct State<S>(S);

// TODO: figure out a better design for non-chainable dom tokens
impl<S> DomTokensStateUnmount for State<S> {
    fn dom_tokens_state_unmount(_: &mut Self, dom_token_list: &mut impl DomTokenList) {
        dom_token_list.set_value("");
    }
}

impl<S: KnownNonReactiveStr> CsrDomTokens for NonReactiveStrIntoDomTokens<S> {
    type State = State<S::Cache>;

    fn dom_tokens_render_init(this: Self, dom_token_list: &mut impl DomTokenList) -> Self::State {
        let (state, ()) = S::into_cache_and_render(this.0, renderer(dom_token_list));

        State(state)
    }

    fn dom_tokens_render_init_with_old_state(
        this: Self,
        dom_token_list: &mut impl DomTokenList,
        old_state: &mut Self::State,
    ) {
        Self::dom_tokens_render_update(this, dom_token_list, old_state)
    }

    fn dom_tokens_render_update(
        this: Self,
        dom_token_list: &mut impl DomTokenList,
        State(state): &mut Self::State,
    ) {
        S::update_into_cache_and_render(this.0, renderer(dom_token_list), state)
    }
}

fn renderer(dom_token_list: &mut impl DomTokenList) -> impl '_ + FnOnce(TempRef<str>) {
    |TempRef(value)| dom_token_list.set_value(value)
}
