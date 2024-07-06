use darling::{util::WithOriginal, FromMeta};

use crate::utils::ValueOrPath;

#[derive(Debug, FromMeta)]
pub struct ComponentMainOptions {
    pub get_dom_element: WithOriginal<ValueOrPath<syn::LitStr>, syn::Meta>,
}

pub type MainOptionsWithOriginal = darling::util::WithOriginal<ComponentMainOptions, syn::Meta>;

#[derive(Debug, FromMeta, Default)]
pub struct ComponentOptions {
    #[darling(default)]
    pub main: Option<MainOptionsWithOriginal>,
    /// Defaults to `::frender`
    pub frender_path: Option<syn::Path>,
    pub ssr_only: darling::util::Flag,
}
