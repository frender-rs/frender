#![cfg(feature = "ssr")]
#![cfg(feature = "experimental")]
//! inline anonymous `dom_tokens!(...)` can be used in const context.

use frender_dom_tokens::{constness::HasConstKnownPossibleDomTokens, dom_tokens, DomTokens};

use self::utils::ssr::collect_dom_tokens;

pub mod utils;

struct Classes {
    circle: bool,
    array: bool,
    dark: bool,
}

const fn tokens(
    Classes {
        circle,
        array,
        dark,
    }: Classes,
) -> impl DomTokens + Copy + HasConstKnownPossibleDomTokens {
    dom_tokens!(
        "my-btn",
        if (circle) {
            "circle"
        },
        if (array) {
            "a b"
        },
        if (dark) { "dark" } else { "light" }
    )
}

#[test]
fn out() {
    futures_lite::future::block_on(async {
        {
            let out: String = collect_dom_tokens(tokens(Classes {
                circle: true,
                array: true,
                dark: true,
            }))
            .await;

            assert_eq!(out, "my-btn circle a b dark")
        }

        {
            let out: String = collect_dom_tokens(tokens(Classes {
                circle: true,
                array: true,
                dark: false,
            }))
            .await;

            assert_eq!(out, "my-btn circle a b light")
        }

        {
            let out: String = collect_dom_tokens(tokens(Classes {
                circle: true,
                array: false,
                dark: true,
            }))
            .await;

            assert_eq!(out, "my-btn circle dark")
        }

        {
            let out: String = collect_dom_tokens(tokens(Classes {
                circle: true,
                array: false,
                dark: false,
            }))
            .await;

            assert_eq!(out, "my-btn circle light")
        }

        {
            let out: String = collect_dom_tokens(tokens(Classes {
                circle: false,
                array: true,
                dark: true,
            }))
            .await;

            assert_eq!(out, "my-btn a b dark")
        }

        {
            let out: String = collect_dom_tokens(tokens(Classes {
                circle: false,
                array: true,
                dark: false,
            }))
            .await;

            assert_eq!(out, "my-btn a b light")
        }

        {
            let out: String = collect_dom_tokens(tokens(Classes {
                circle: false,
                array: false,
                dark: true,
            }))
            .await;

            assert_eq!(out, "my-btn dark")
        }

        {
            let out: String = collect_dom_tokens(tokens(Classes {
                circle: false,
                array: false,
                dark: false,
            }))
            .await;

            assert_eq!(out, "my-btn light")
        }
    })
}
