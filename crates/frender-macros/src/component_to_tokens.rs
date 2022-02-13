use quote::ToTokens;

use super::component_data::*;

impl ToTokens for ComponentDefinition {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        todo!()
    }
}
