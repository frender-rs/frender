use frender_dom_tokens::DomTokenList;

#[derive(Default)]
struct DomTokenListAddOnly {
    tokens: Vec<String>,
}

impl DomTokenList for DomTokenListAddOnly {
    fn set_value(&mut self, _: &str) {
        unreachable!()
    }

    fn add_1(&mut self, token: frender_dom_tokens::DomToken) {
        assert!(self.tokens.is_empty());
        self.tokens.push(token.as_str().to_owned())
    }

    fn remove_1(&mut self, _: frender_dom_tokens::DomToken) {
        unreachable!()
    }

    fn replace(&mut self, _: frender_dom_tokens::DomToken, _: frender_dom_tokens::DomToken) {
        unreachable!()
    }
}

mod literal {
    use async_str_iter::ext::AsyncStrIteratorExt as _;
    use frender_dom_tokens::{dom_tokens, ChainableDomTokens, DomTokens};

    use super::DomTokenListAddOnly;

    const fn value() -> impl ChainableDomTokens + Copy {
        dom_tokens!("literal")
    }

    #[test]
    fn ssr() {
        futures_lite::future::block_on(async {
            {
                let out: String = DomTokens::dom_tokens_into_async_str_iter(value())
                    .collect()
                    .await;

                assert_eq!(out, "literal");
            }

            {
                let out: String =
                    ChainableDomTokens::dom_tokens_prefix_space_into_async_str_iter(value())
                        .collect()
                        .await;

                assert_eq!(out, " literal");
            }
        })
    }

    #[test]
    fn csr() {
        let dom_token_list = &mut DomTokenListAddOnly::default();
        let state = &mut Default::default();
        assert!(dom_token_list.tokens.is_empty());
        DomTokens::update_with_state(value(), dom_token_list, state);
        assert_eq!(dom_token_list.tokens, ["literal"]);
        DomTokens::update_with_state(value(), dom_token_list, state);
        assert_eq!(dom_token_list.tokens, ["literal"]);
    }
}
