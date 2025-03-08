use frender_dom_tokens::{impl_dom_tokens_for, values::Empty};

pub struct PubTokens;

impl_dom_tokens_for!(|self: PubTokens| dom_tokens!(
    "a",
    "a0 a1 a2",
    if (false) {
        "b"
    },
    if (true) { "c" } else { "d" },
    match (None::<()>) {
        Some(_) => "e",
        None => "f",
    },
    {
        {
            Empty
        }
    } as Empty,
));

#[test]
fn r#const() {
    use frender_dom_tokens::constness::HasConstKnownPossibleDomTokens;
    let a = PubTokens::KNOWN_POSSIBLE_DOM_TOKENS_ARRAY_VEC.as_slice();
    assert_eq!(a, ["a", "a0", "a1", "a2", "b", "c", "d", "e", "f"]);
}

mod private {
    use frender_dom_tokens::{impl_dom_tokens_for, values::Empty};

    struct PrivateTokens;

    impl_dom_tokens_for!(|self: PrivateTokens| dom_tokens!(
        "a",
        "a0 a1 a2",
        if (false) {
            "b"
        },
        if (true) { "c" } else { "d" },
        match (None::<()>) {
            Some(_) => "e",
            None => "f",
        },
        {
            {
                Empty
            }
        } as Empty,
    ));

    pub(crate) struct CrateTokens;

    impl_dom_tokens_for!(|self: CrateTokens| dom_tokens!(
        "a",
        "a0 a1 a2",
        if (false) {
            "b"
        },
        if (true) { "c" } else { "d" },
        match (None::<()>) {
            Some(_) => "e",
            None => "f",
        },
        {
            {
                Empty
            }
        } as Empty,
    ));

    #[test]
    fn r#const() {
        use frender_dom_tokens::constness::HasConstKnownPossibleDomTokens;

        assert_eq!(
            PrivateTokens::KNOWN_POSSIBLE_DOM_TOKENS_ARRAY_VEC.as_slice(),
            ["a", "a0", "a1", "a2", "b", "c", "d", "e", "f"]
        );

        assert_eq!(
            CrateTokens::KNOWN_POSSIBLE_DOM_TOKENS_ARRAY_VEC.as_slice(),
            ["a", "a0", "a1", "a2", "b", "c", "d", "e", "f"]
        );
    }
}
