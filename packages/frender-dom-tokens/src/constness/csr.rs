use std::marker::PhantomData;

use crate::csr::{CsrDomTokens, DomTokenList, DomTokensStateUnmount};

use super::{ConstDomTokens, HasConstDomTokens};

pub struct State<T: ?Sized + HasConstDomTokens>(pub(super) PhantomData<T>);

impl<T: ?Sized + HasConstDomTokens> DomTokensStateUnmount for State<T> {
    fn dom_tokens_state_unmount(_: &mut Self, dom_token_list: &mut impl DomTokenList) {
        T::DOM_TOKENS
            .as_ref()
            .iter()
            .for_each(|t| dom_token_list.remove_1(*t))
    }
}

impl<T: ?Sized + HasConstDomTokens> CsrDomTokens for ConstDomTokens<T> {
    type State = State<T>;

    fn dom_tokens_render_init(_: Self, dom_token_list: &mut impl DomTokenList) -> Self::State {
        T::DOM_TOKENS
            .as_ref()
            .iter()
            .for_each(|t| dom_token_list.add_1(*t));

        State(PhantomData)
    }

    fn dom_tokens_render_update(_: Self, _: &mut impl DomTokenList, _: &mut Self::State) {
        // Does nothing
    }
}
