use darling::{ast::NestedMeta, util::WithOriginal, FromMeta};

use crate::utils::value_or_path::ValueOrPath;

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

pub struct ComponentDefinition {
    pub errors: Vec<darling::Error>,
    pub options: ComponentOptions,
    pub item_fn: syn::ItemFn,
}

impl ComponentDefinition {
    pub fn from_attrs_and_fn(attr_args: &[NestedMeta], item_fn: syn::ItemFn) -> Self {
        let mut errors = vec![];
        let options = match ComponentOptions::from_list(attr_args) {
            Ok(v) => v,
            Err(err) => {
                errors.push(err);
                ComponentOptions::default()
            }
        };

        Self {
            options,
            item_fn,
            errors,
        }
    }
}
