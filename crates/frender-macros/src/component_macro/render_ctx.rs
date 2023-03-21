use darling::ToTokens;
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};

use crate::component_data::RenderCtx;

fn ts_bounds(
    ctx_type: TokenStream,
    span: Span,
    hook_element_path: impl ToTokens,
    bounds: Option<&TokenStream>,
    state_it: Option<&syn::TypeImplTrait>,
) -> TokenStream {
    let plus = if bounds.is_some() {
        Some(quote_spanned!(span=> +))
    } else {
        None
    };

    let state_eq_it = match state_it {
        Some(syn::TypeImplTrait { impl_token, bounds }) if !bounds.is_empty() => {
            let span = impl_token.span;

            Some(quote_spanned! {span=>
                State =
                #impl_token
                #hook_element_path::frender_core::RenderState
                +
                #bounds
            })
        }
        _ => None,
    };

    quote_spanned! {span=>
        #hook_element_path::frender_core::UpdateRenderState<
            #hook_element_path #ctx_type,
            #state_eq_it
        >
        #plus
        #bounds
    }
}

fn dom_ctx(span: Span) -> TokenStream {
    quote_spanned! {span=>
        ::frender_dom::Dom
    }
}

fn ssr_ctx(span: Span) -> TokenStream {
    quote_spanned! {span=>
        ::frender_ssr::AnySsrContext
    }
}

impl RenderCtx {
    pub fn ssr_enabled(&self) -> bool {
        match self {
            RenderCtx::Ssr => true,
            RenderCtx::Dom => false,
            RenderCtx::SsrAndDom => true,
        }
    }

    pub fn dom_enabled(&self) -> bool {
        match self {
            RenderCtx::Ssr => false,
            RenderCtx::Dom => true,
            RenderCtx::SsrAndDom => true,
        }
    }

    pub fn to_ctx_ty(&self, span: Span, hook_element_path: impl ToTokens) -> TokenStream {
        let ctx_ty = match self {
            RenderCtx::Ssr => ssr_ctx(span),
            RenderCtx::Dom => dom_ctx(span),
            RenderCtx::SsrAndDom => todo!("ssr+dom"),
        };

        quote!(
            #hook_element_path
            #ctx_ty
        )
    }

    pub fn to_render_bounds(
        &self,
        span: Span,
        hook_element_path: &impl ToTokens,
        bounds: Option<&TokenStream>,
        state_it: Option<&syn::TypeImplTrait>,
    ) -> TokenStream {
        let mut out = quote_spanned!(span=> impl);

        if self.ssr_enabled() {
            out.extend(ts_bounds(
                ssr_ctx(span),
                span,
                hook_element_path,
                bounds,
                state_it,
            ))
        }

        if let RenderCtx::SsrAndDom = self {
            out.extend(quote_spanned!(span=> + ))
        }

        if self.dom_enabled() {
            out.extend(ts_bounds(
                dom_ctx(span),
                span,
                hook_element_path,
                bounds,
                state_it,
            ))
        }

        out
    }
}
