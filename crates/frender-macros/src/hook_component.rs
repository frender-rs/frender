use hooks_derive_core::DetectedHooksTokens;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::Parse;

use crate::utils::{grouped::Parenthesized, kw::PrefixKeyword};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(no_data);
    custom_keyword!(no_data_ty);
    custom_keyword!(prepend);
}

/// ```plain
/// macro(crate::consume_detected_hooks)
/// no_data(crate::NoData)
/// $( prepend(...) )?
/// {
///     // statements
/// }
/// ```
pub struct DetectHooks {
    macro_path: PrefixKeyword<syn::Token![macro], Parenthesized<TokenStream>>,
    expr_no_data: PrefixKeyword<kw::no_data, Parenthesized<TokenStream>>,
    type_no_data: Option<PrefixKeyword<kw::no_data_ty, Parenthesized<TokenStream>>>,
    prepend: Option<PrefixKeyword<kw::prepend, Parenthesized<TokenStream>>>,
    block: syn::Block,
}

impl Parse for DetectHooks {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            macro_path: input.parse()?,
            expr_no_data: input.parse()?,
            type_no_data: input.parse()?,
            prepend: input.parse()?,
            block: input.parse()?,
        })
    }
}

impl DetectHooks {
    pub fn into_ts(self, hooks_core_path: impl ToTokens) -> TokenStream {
        let mut stmts = self.block.stmts;
        let detected_hooks = hooks_derive_core::detect_hooks(stmts.iter_mut(), &hooks_core_path);
        let DetectedHooksTokens {
            data_expr,
            fn_arg_data_pat,
            fn_stmts_extract_data,
        } = hooks_derive_core::detected_hooks_to_tokens(
            detected_hooks,
            hooks_core_path,
            self.expr_no_data.content.content,
            self.type_no_data.map(|ty| ty.content.content),
            self.macro_path.keyword.span,
        );

        let prepend = self.prepend.map(|p| p.content.content);
        let macro_path = self.macro_path.content.content;

        quote! {
            #macro_path ! {
                #prepend
                data_expr(#data_expr)
                fn_arg_data_pat(#fn_arg_data_pat)
                fn_stmts_extract_data(#fn_stmts_extract_data)
                fn_stmts(#(#stmts)*)
            }
        }
    }
}
