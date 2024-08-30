use async_str_iter::ext::AsyncStrIteratorExt;
use frender_dom_tokens::{
    constness::HasConstKnownPossibleDomTokens, impl_dom_tokens_for, ChainableDomTokens, DomToken,
    DomTokens,
};

struct ConstDomTokens;

enum ConstDomTokensMarker {}

impl_dom_tokens_for!(|self: ConstDomTokens| {
    #[const_marker(ConstDomTokensMarker)]
    ["a", "b", "c"]
});

#[test]
fn const_dom_tokens() {
    assert_eq!(
        ConstDomTokens::KNOWN_POSSIBLE_DOM_TOKENS_ARRAY_VEC.as_slice(),
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
