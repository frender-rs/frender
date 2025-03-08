#![cfg(feature = "experimental")]
#![cfg(feature = "csr")]
#![cfg(feature = "ssr")]

pub mod utils;

use utils::{
    dom_token_list::{DomTokenListAddRemove, DomTokenListNever},
    ssr::{collect_dom_tokens, collect_dom_tokens_prefix_space},
};

mod literal {
    use frender_dom_tokens::{dom_tokens, experimental::csr::CsrDomTokens, ChainableDomTokens};

    use super::*;

    const fn value() -> impl ChainableDomTokens + Copy {
        dom_tokens!("literal")
    }

    #[test]
    fn ssr() {
        futures_lite::future::block_on(async {
            assert_eq!(collect_dom_tokens(value()).await, "literal");
            assert_eq!(collect_dom_tokens_prefix_space(value()).await, " literal");
        })
    }

    #[test]
    fn csr() {
        let dom_token_list = &mut DomTokenListAddRemove::default();
        assert!(dom_token_list.tokens.is_empty());
        let mut state = CsrDomTokens::dom_tokens_render_init(value(), dom_token_list);
        assert_eq!(dom_token_list.tokens, ["literal"]);
        CsrDomTokens::dom_tokens_render_update(value(), dom_token_list, &mut state);
        assert_eq!(dom_token_list.tokens, ["literal"]);
    }
}

mod array_of_literals {
    use frender_dom_tokens::{dom_tokens, experimental::csr::CsrDomTokens, ChainableDomTokens};

    use super::*;

    const fn value() -> impl ChainableDomTokens + Copy {
        dom_tokens!("literal-0 literal-1 literal-2")
    }

    #[test]
    fn ssr() {
        futures_lite::future::block_on(async {
            assert_eq!(
                collect_dom_tokens(value()).await,
                "literal-0 literal-1 literal-2"
            );

            assert_eq!(
                collect_dom_tokens_prefix_space(value()).await,
                " literal-0 literal-1 literal-2"
            );
        })
    }

    #[test]
    fn csr() {
        let dom_token_list = &mut DomTokenListAddRemove::default();
        assert!(dom_token_list.tokens.is_empty());
        let mut state = CsrDomTokens::dom_tokens_render_init(value(), dom_token_list);
        let state = &mut state;
        assert_eq!(
            dom_token_list.tokens,
            ["literal-0", "literal-1", "literal-2"]
        );
        CsrDomTokens::dom_tokens_render_update(value(), dom_token_list, state);
        assert_eq!(
            dom_token_list.tokens,
            ["literal-0", "literal-1", "literal-2"]
        );
    }
}

mod r#if {
    use frender_dom_tokens::{dom_tokens, experimental::csr::CsrDomTokens, ChainableDomTokens};

    use crate::{
        utils::ssr::{collect_dom_tokens, collect_dom_tokens_prefix_space},
        DomTokenListNever,
    };

    use super::DomTokenListAddRemove;

    const fn value(predicate: bool) -> impl ChainableDomTokens + Copy {
        dom_tokens!(if (predicate) {
            "a"
        })
    }

    #[test]
    fn ssr() {
        futures_lite::future::block_on(async {
            assert_eq!(collect_dom_tokens(value(false)).await, "");
            assert_eq!(collect_dom_tokens(value(true)).await, "a");

            assert_eq!(collect_dom_tokens_prefix_space(value(false)).await, "");
            assert_eq!(collect_dom_tokens_prefix_space(value(true)).await, " a");
        })
    }

    #[test]
    fn csr() {
        let dom_token_list = &mut DomTokenListAddRemove::default();

        assert!(dom_token_list.tokens.is_empty());
        let mut state = CsrDomTokens::dom_tokens_render_init(value(true), dom_token_list);
        let state = &mut state;
        assert_eq!(dom_token_list.tokens, ["a"]);
        CsrDomTokens::dom_tokens_render_update(value(true), &mut DomTokenListNever, state);
        CsrDomTokens::dom_tokens_render_update(value(true), dom_token_list, state);
        assert_eq!(dom_token_list.tokens, ["a"]);

        CsrDomTokens::dom_tokens_render_update(value(false), dom_token_list, state);
        assert!(dom_token_list.tokens.is_empty());

        CsrDomTokens::dom_tokens_render_update(value(false), &mut DomTokenListNever, state);
    }
}

mod if_else {
    use frender_dom_tokens::{dom_tokens, experimental::csr::CsrDomTokens, ChainableDomTokens};

    use super::*;

    const fn value(predicate: bool) -> impl ChainableDomTokens + Copy {
        dom_tokens!(if (!!predicate) { "a" } else { "b c" })
    }

    #[test]
    fn ssr() {
        futures_lite::future::block_on(async {
            assert_eq!(collect_dom_tokens(value(false)).await, "b c");
            assert_eq!(collect_dom_tokens(value(true)).await, "a");

            assert_eq!(collect_dom_tokens_prefix_space(value(false)).await, " b c");
            assert_eq!(collect_dom_tokens_prefix_space(value(true)).await, " a");
        })
    }

    #[test]
    fn csr() {
        let dom_token_list = &mut DomTokenListAddRemove::default();
        assert!(dom_token_list.tokens.is_empty());

        let mut state = CsrDomTokens::dom_tokens_render_init(value(true), dom_token_list);
        let state = &mut state;
        assert_eq!(dom_token_list.tokens, ["a"]);
        CsrDomTokens::dom_tokens_render_update(value(true), &mut DomTokenListNever, state);
        CsrDomTokens::dom_tokens_render_update(value(true), dom_token_list, state);
        assert_eq!(dom_token_list.tokens, ["a"]);

        CsrDomTokens::dom_tokens_render_update(value(false), dom_token_list, state);
        assert_eq!(dom_token_list.tokens, ["b", "c"]);

        CsrDomTokens::dom_tokens_render_update(value(false), &mut DomTokenListNever, state);
    }
}

mod r#match {
    use frender_dom_tokens::{dom_tokens, experimental::csr::CsrDomTokens, ChainableDomTokens};

    use super::*;

    pub enum Theme {
        Dark,
        Light,
        Contrast { colorful: bool },
    }

    const fn value(theme: Theme) -> impl ChainableDomTokens + Copy {
        dom_tokens!(match (theme) {
            Theme::Dark => "dark",
            Theme::Light => "light",
            Theme::Contrast { colorful } => dom_tokens!(
                "contrast",
                if (colorful) {
                    "colorful"
                }
            ),
        })
    }

    #[test]
    fn ssr() {
        futures_lite::future::block_on(async {
            assert_eq!(collect_dom_tokens(value(Theme::Dark)).await, "dark");
            assert_eq!(collect_dom_tokens(value(Theme::Light)).await, "light");
            assert_eq!(
                collect_dom_tokens(value(Theme::Contrast { colorful: false })).await,
                "contrast"
            );
            assert_eq!(
                collect_dom_tokens(value(Theme::Contrast { colorful: true })).await,
                "contrast colorful"
            );

            assert_eq!(
                collect_dom_tokens_prefix_space(value(Theme::Dark)).await,
                " dark"
            );
            assert_eq!(
                collect_dom_tokens_prefix_space(value(Theme::Light)).await,
                " light"
            );
            assert_eq!(
                collect_dom_tokens_prefix_space(value(Theme::Contrast { colorful: false })).await,
                " contrast"
            );
            assert_eq!(
                collect_dom_tokens_prefix_space(value(Theme::Contrast { colorful: true })).await,
                " contrast colorful"
            );
        })
    }

    #[test]
    fn csr() {
        let dom_token_list = &mut DomTokenListAddRemove::default();
        assert!(dom_token_list.tokens.is_empty());
        let mut state = CsrDomTokens::dom_tokens_render_init(value(Theme::Dark), dom_token_list);
        let state = &mut state;
        assert_eq!(dom_token_list.tokens, ["dark"]);
        CsrDomTokens::dom_tokens_render_update(value(Theme::Dark), &mut DomTokenListNever, state);
        CsrDomTokens::dom_tokens_render_update(value(Theme::Dark), dom_token_list, state);
        assert_eq!(dom_token_list.tokens, ["dark"]);

        CsrDomTokens::dom_tokens_render_update(value(Theme::Light), dom_token_list, state);
        assert_eq!(dom_token_list.tokens, ["light"]);

        CsrDomTokens::dom_tokens_render_update(
            value(Theme::Contrast { colorful: true }),
            dom_token_list,
            state,
        );
        assert_eq!(dom_token_list.tokens, ["contrast", "colorful"]);

        CsrDomTokens::dom_tokens_render_update(
            value(Theme::Contrast { colorful: true }),
            &mut DomTokenListNever,
            state,
        );

        CsrDomTokens::dom_tokens_render_update(
            value(Theme::Contrast { colorful: false }),
            dom_token_list,
            state,
        );
        assert_eq!(dom_token_list.tokens, ["contrast"]);
    }
}

mod r#as {
    use frender_dom_tokens::{
        dom_tokens, experimental::csr::CsrDomTokens, impl_dom_tokens_for, ChainableDomTokens,
    };

    use super::*;

    #[derive(Debug, Clone, Copy)]
    struct MyDomTokens;
    impl_dom_tokens_for!(|self: MyDomTokens| "light");

    const fn value() -> impl ChainableDomTokens + Copy {
        dom_tokens!({
            {
                MyDomTokens
            }
        })
    }

    #[test]
    fn ssr() {
        futures_lite::future::block_on(async {
            assert_eq!(collect_dom_tokens(value()).await, "light");

            assert_eq!(collect_dom_tokens_prefix_space(value()).await, " light");
        })
    }

    #[test]
    fn csr() {
        let dom_token_list = &mut DomTokenListAddRemove::default();
        assert!(dom_token_list.tokens.is_empty());
        let mut state = CsrDomTokens::dom_tokens_render_init(value(), dom_token_list);
        let state = &mut state;
        assert_eq!(dom_token_list.tokens, ["light"]);
        CsrDomTokens::dom_tokens_render_update(value(), &mut DomTokenListNever, state);
        CsrDomTokens::dom_tokens_render_update(value(), dom_token_list, state);
        assert_eq!(dom_token_list.tokens, ["light"]);
    }
}
