use async_str_iter::ext::AsyncStrIteratorExt;
use frender_dom_tokens::{
    impl_dom_tokens_for, ChainableDomTokens, ConstPossibleDomTokens, DomToken, DomTokens,
};

struct ConstDomTokens;

impl_dom_tokens_for!(|_: ConstDomTokens| dom_tokens!["a", "b", "c"]);

#[test]
fn const_dom_tokens() {
    assert_eq!(
        <ConstDomTokens as ConstPossibleDomTokens>::POSSIBLE_DOM_TOKENS.as_slice(),
        ["a", "b", "c"].map(DomToken::new_const)
    );

    futures_lite::future::block_on(async {
        assert_eq!(
            DomTokens::dom_tokens_into_async_str_iter(ConstDomTokens)
                .collect::<String>()
                .await,
            "a b c"
        );

        assert_eq!(
            ChainableDomTokens::dom_tokens_prefix_space_into_async_str_iter(ConstDomTokens)
                .collect::<String>()
                .await,
            " a b c"
        );
    });
}
