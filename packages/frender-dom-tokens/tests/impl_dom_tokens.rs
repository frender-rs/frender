#![cfg(feature = "experimental")]
#![cfg(feature = "ssr")]

use frender_dom_tokens::{
    constness::HasConstKnownPossibleDomTokens, impl_dom_tokens_for, DomToken,
};

use self::utils::ssr::{collect_dom_tokens, collect_dom_tokens_prefix_space};

pub mod utils;

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
            //
            collect_dom_tokens(ConstDomTokens).await,
            "a b c"
        );

        assert_eq!(
            collect_dom_tokens_prefix_space(ConstDomTokens).await,
            " a b c"
        );
    });
}
