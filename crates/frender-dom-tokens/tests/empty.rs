use async_str_iter::ext::AsyncStrIteratorExt as _;
use frender_dom_tokens::{dom_tokens, ChainableDomTokens, DomTokenList, DomTokens};

const fn empty() -> impl ChainableDomTokens + Copy {
    dom_tokens!()
}

#[test]
fn ssr() {
    futures_lite::future::block_on(async {
        {
            let out: String = DomTokens::dom_tokens_into_async_str_iter(empty())
                .collect()
                .await;

            assert_eq!(out, "");
        }

        {
            let out: String =
                ChainableDomTokens::dom_tokens_prefix_space_into_async_str_iter(empty())
                    .collect()
                    .await;

            assert_eq!(out, "");
        }
    })
}

#[test]
fn csr() {
    struct DomTokenListUntouched;

    impl DomTokenList for DomTokenListUntouched {
        fn set_value(&mut self, _: &str) {
            unreachable!()
        }

        fn add_1(&mut self, _: frender_dom_tokens::DomToken) {
            unreachable!()
        }

        fn remove_1(&mut self, _: frender_dom_tokens::DomToken) {
            unreachable!()
        }

        fn replace(&mut self, _: frender_dom_tokens::DomToken, _: frender_dom_tokens::DomToken) {
            unreachable!()
        }
    }

    let dom_token_list = &mut DomTokenListUntouched;
    let state = &mut Default::default();

    DomTokens::update_with_state(empty(), dom_token_list, state);
    DomTokens::update_with_state(empty(), dom_token_list, state);
}
