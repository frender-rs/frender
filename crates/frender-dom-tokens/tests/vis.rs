use frender_dom_tokens::{impl_dom_tokens_for, Empty};

pub struct PubTokens;

impl_dom_tokens_for!(|_: PubTokens| dom_tokens!(
    "a",
    "a0 a1 a2",
    if false {
        "b"
    },
    if true { "c" } else { "d" },
    match None::<()> {
        Some(_) => "e",
        None => "f",
    },
    Empty as Empty,
));

mod private {
    use frender_dom_tokens::{impl_dom_tokens_for, Empty};

    struct PrivateTokens;

    impl_dom_tokens_for!(|_: PrivateTokens| dom_tokens!(
        "a",
        "a0 a1 a2",
        if false {
            "b"
        },
        if true { "c" } else { "d" },
        match None::<()> {
            Some(_) => "e",
            None => "f",
        },
        Empty as Empty,
    ));

    pub(crate) struct CrateTokens;

    impl_dom_tokens_for!(|_: CrateTokens| dom_tokens!(
        "a",
        "a0 a1 a2",
        if false {
            "b"
        },
        if true { "c" } else { "d" },
        match None::<()> {
            Some(_) => "e",
            None => "f",
        },
        Empty as Empty,
    ));
}
