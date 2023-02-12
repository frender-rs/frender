use quote::{quote_spanned, ToTokens};

use crate::err::OutputError;

use super::component_data::*;

impl ComponentDefinition {
    pub fn into_ts(self) -> proc_macro::TokenStream {
        let Self {
            errors,
            options:
                ComponentOptions {
                    //
                    main,
                    render_ctx,
                    hook_element_path,
                },
            item_fn,
        } = self;

        let main_block = if let Some(main) = main {
            let span = main.span();
            let ComponentMainOptions {
                //
                mount_element_id: _,
            } = main.as_ref();

            let ts = quote_spanned! {span=>
                fn main() {

                }
            };

            Some(ts)
        } else {
            None
        };

        let errors = errors
            .output_error()
            .map(darling::Error::write_errors)
            .map(proc_macro::TokenStream::from);

        let hook_element_path = hook_element_path.map_or_else(default_hook_element_path, |path| {
            path.into_token_stream().into()
        });

        let mut out = expand_def_component(hook_element_path, render_ctx.macro_name(), item_fn);
        out.extend(main_block.map(proc_macro::TokenStream::from));
        out.extend(errors);

        out
    }
}

/// For `render_output_ty` = `Option<MyElement>`, extract `MyElement`.
/// Otherwise clone `render_output_ty`
fn infer_custom_element_ty(render_output_ty: &syn::Type) -> syn::Type {
    if let syn::Type::Path(path_ty) = render_output_ty {
        let p = &path_ty.path;
        if p.leading_colon.is_none() && p.segments.len() == 1 {
            let seg = &p.segments[0];
            if seg.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(bracketed_args) = &seg.arguments {
                    if bracketed_args.args.len() == 1 {
                        if let syn::GenericArgument::Type(ty) = &bracketed_args.args[0] {
                            return ty.clone();
                        }
                    }
                }
            }
        }
    }

    render_output_ty.clone()
}

fn expand_def_component(
    hook_element_path: proc_macro::TokenStream,
    macro_name: &str,
    input_fn: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenTree};

    let mut out = hook_element_path;
    out.extend([
        TokenTree::from(Punct::new(':', Spacing::Joint)),
        Punct::new(':', Spacing::Alone).into(),
        Ident::new(macro_name, Span::call_site()).into(),
        Punct::new('!', Spacing::Alone).into(),
        Group::new(Delimiter::Brace, input_fn).into(),
    ]);

    out
}

/// `::frender::hook_element`
fn default_hook_element_path() -> proc_macro::TokenStream {
    use proc_macro::{Ident, Punct, Spacing, Span, TokenTree};

    proc_macro::TokenStream::from_iter([
        TokenTree::from(Punct::new(':', Spacing::Joint)),
        Punct::new(':', Spacing::Alone).into(),
        Ident::new("frender", Span::call_site()).into(),
        TokenTree::from(Punct::new(':', Spacing::Joint)),
        Punct::new(':', Spacing::Alone).into(),
        Ident::new("hook_element", Span::call_site()).into(),
    ])
}
