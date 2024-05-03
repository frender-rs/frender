//! inline anonymous `dom_tokens!(...)` can be used in const context.

use async_str_iter::ext::AsyncStrIteratorExt as _;
use frender_dom_tokens::{dom_tokens, ConstPossibleDomTokens, DomTokens};

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
) -> impl DomTokens + Copy + ConstPossibleDomTokens {
    dom_tokens!(
        "my-btn",
        if circle {
            "circle"
        },
        if array {
            ["a", "b"]
        },
        if dark { "dark" } else { "light" }
    )
}

#[test]
fn out() {
    futures_lite::future::block_on(async {
        {
            let out: String = DomTokens::dom_tokens_into_async_str_iter(tokens(Classes {
                circle: true,
                array: true,
                dark: true,
            }))
            .collect()
            .await;

            assert_eq!(out, "my-btn circle a b dark")
        }

        {
            let out: String = DomTokens::dom_tokens_into_async_str_iter(tokens(Classes {
                circle: true,
                array: true,
                dark: false,
            }))
            .collect()
            .await;

            assert_eq!(out, "my-btn circle a b light")
        }

        {
            let out: String = DomTokens::dom_tokens_into_async_str_iter(tokens(Classes {
                circle: true,
                array: false,
                dark: true,
            }))
            .collect()
            .await;

            assert_eq!(out, "my-btn circle dark")
        }

        {
            let out: String = DomTokens::dom_tokens_into_async_str_iter(tokens(Classes {
                circle: true,
                array: false,
                dark: false,
            }))
            .collect()
            .await;

            assert_eq!(out, "my-btn circle light")
        }

        {
            let out: String = DomTokens::dom_tokens_into_async_str_iter(tokens(Classes {
                circle: false,
                array: true,
                dark: true,
            }))
            .collect()
            .await;

            assert_eq!(out, "my-btn a b dark")
        }

        {
            let out: String = DomTokens::dom_tokens_into_async_str_iter(tokens(Classes {
                circle: false,
                array: true,
                dark: false,
            }))
            .collect()
            .await;

            assert_eq!(out, "my-btn a b light")
        }

        {
            let out: String = DomTokens::dom_tokens_into_async_str_iter(tokens(Classes {
                circle: false,
                array: false,
                dark: true,
            }))
            .collect()
            .await;

            assert_eq!(out, "my-btn dark")
        }

        {
            let out: String = DomTokens::dom_tokens_into_async_str_iter(tokens(Classes {
                circle: false,
                array: false,
                dark: false,
            }))
            .collect()
            .await;

            assert_eq!(out, "my-btn light")
        }
    })
}
