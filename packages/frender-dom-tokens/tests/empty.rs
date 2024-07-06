use frender_dom_tokens::{dom_tokens, ChainableDomTokens, DomTokens};
use utils::dom_token_list::DomTokenListNever;

use crate::utils::ssr::{collect_dom_tokens, collect_dom_tokens_prefix_space};

pub mod utils;

const fn empty() -> impl ChainableDomTokens + Copy {
    dom_tokens!()
}

#[test]
fn ssr() {
    futures_lite::future::block_on(async {
        assert_eq!(collect_dom_tokens(empty()).await, "");
        assert_eq!(collect_dom_tokens_prefix_space(empty()).await, "");
    })
}

#[test]
fn csr() {
    let dom_token_list = &mut DomTokenListNever;
    let state = &mut Default::default();

    DomTokens::update_with_state(empty(), dom_token_list, state);
    DomTokens::update_with_state(empty(), dom_token_list, state);
}

mod impl_dom_tokens {
    use frender_dom_tokens::{impl_dom_tokens_for, ChainableDomTokens, DomTokens};

    use crate::utils::{
        dom_token_list::DomTokenListNever,
        ssr::{collect_dom_tokens, collect_dom_tokens_prefix_space},
    };

    #[derive(Debug, Clone, Copy)]
    struct MyEmpty;
    impl_dom_tokens_for!(|_: MyEmpty| dom_tokens!());

    const fn empty() -> impl ChainableDomTokens + Copy {
        MyEmpty
    }

    #[test]
    fn ssr() {
        futures_lite::future::block_on(async {
            assert_eq!(collect_dom_tokens(empty()).await, "");
            assert_eq!(collect_dom_tokens_prefix_space(empty()).await, "");
        })
    }

    #[test]
    fn csr() {
        let dom_token_list = &mut DomTokenListNever;
        let state = &mut Default::default();

        DomTokens::update_with_state(empty(), dom_token_list, state);
        DomTokens::update_with_state(empty(), dom_token_list, state);
    }
}
