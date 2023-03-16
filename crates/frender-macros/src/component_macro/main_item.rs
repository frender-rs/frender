use darling::ToTokens;
use proc_macro2::{Span, TokenStream};
use quote::quote_spanned;
use syn::spanned::Spanned;

use crate::component_data::ComponentMainOptions;

pub struct MainItem<'a> {
    pub span_default: Span,
    pub span_fn_ident: Span,
    pub hook_element_path: &'a syn::Path,
    pub options: &'a ComponentMainOptions,
    pub vis: &'a syn::Visibility,
    pub expr_get_element: &'a dyn ToTokens,
}

impl<'a> MainItem<'a> {
    pub fn into_ts(self) -> TokenStream {
        let Self {
            span_default,
            span_fn_ident,
            hook_element_path,
            options:
                ComponentMainOptions {
                    //
                    get_dom_element,
                },
            vis,
            expr_get_element,
        } = self;

        let ident = proc_macro2::Ident::new("main", span_fn_ident);

        let path_dom = quote_spanned! {get_dom_element.original.path().span()=>
            ::frender_dom::spawn_mount_to_dom_element
        };
        let get_dom_element = &get_dom_element.parsed;

        quote_spanned! {span_default=>
            #vis fn #ident() {
                #hook_element_path #path_dom(
                    #expr_get_element,
                    #get_dom_element
                )
            }
        }
    }
}
