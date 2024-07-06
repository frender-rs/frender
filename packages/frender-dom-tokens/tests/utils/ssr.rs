use async_str_iter::ext::AsyncStrIteratorExt as _;
use frender_dom_tokens::{ChainableDomTokens, DomTokens};

pub async fn collect_dom_tokens(v: impl DomTokens) -> String {
    DomTokens::dom_tokens_into_async_str_iter(v).collect().await
}

pub async fn collect_dom_tokens_prefix_space(v: impl ChainableDomTokens) -> String {
    ChainableDomTokens::dom_tokens_prefix_space_into_async_str_iter(v)
        .collect()
        .await
}
