use frender_macro_utils::grouped::{Braced, Parenthesized};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse::Parse, punctuated::Punctuated};

use crate::kw;

#[derive(Clone)]
/// `trait Bounds = ...;`
pub struct FieldDeclarationWithCommonBoundsTraitBounds {
    pub trait_token: syn::Token![trait],
    pub ident: kw::Bounds,
    pub eq_token: syn::Token![=],
    pub bounds: syn::punctuated::Punctuated<syn::TypeParamBound, syn::Token![+]>,
    pub where_clause: Option<syn::WhereClause>,
    pub semi_token: syn::Token![;],
}

impl FieldDeclarationWithCommonBoundsTraitBounds {
    pub fn parse_some(input: syn::parse::ParseStream) -> syn::Result<Option<Self>> {
        Ok(
            if input.peek(syn::Token![trait]) && input.peek2(kw::Bounds) {
                Some(input.parse()?)
            } else {
                None
            },
        )
    }
}

impl Parse for FieldDeclarationWithCommonBoundsTraitBounds {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            trait_token: input.parse()?,
            ident: input.parse()?,
            eq_token: input.parse()?,
            bounds: {
                let mut bounds = syn::punctuated::Punctuated::new();
                loop {
                    if input.peek(syn::Token![where]) || input.peek(syn::Token![;]) {
                        break;
                    }
                    bounds.push_value(input.parse()?);
                    if input.peek(syn::Token![where]) || input.peek(syn::Token![;]) {
                        break;
                    }
                    bounds.push_punct(input.parse()?);
                }

                bounds
            },
            where_clause: input.parse()?,
            semi_token: input.parse()?,
        })
    }
}

#[derive(Clone)]
pub struct FieldDeclarationWithCommonBoundsCsrFnCommonArgs {
    pub this: TypedPatType<syn::Token![Self]>,
    pub comma_1: syn::Token![,],
    pub element: TypedPatType<syn::Token![_]>,
    pub comma_2: syn::Token![,],
    pub children_ctx: TypedPatType<syn::Token![_]>,
}

#[derive(Clone)]
pub struct FieldDeclarationWithCommonBoundsCsrFnInitializeArgs {
    pub args: FieldDeclarationWithCommonBoundsCsrFnCommonArgs,
    pub comma_trailing: Option<syn::Token![,]>,
}

impl Parse for FieldDeclarationWithCommonBoundsCsrFnInitializeArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            args: input.parse()?,
            comma_trailing: input.parse()?,
        })
    }
}

impl Parse for FieldDeclarationWithCommonBoundsCsrFnCommonArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            this: input.parse()?,
            comma_1: input.parse()?,
            element: input.parse()?,
            comma_2: input.parse()?,
            children_ctx: input.parse()?,
        })
    }
}

type ColonToken = syn::Token![:];

#[derive(Clone)]
pub struct TypedPatType<Type> {
    pub pat: syn::Pat,
    pub colon_token: ColonToken,
    pub ty: Type,
}

impl<Type: ToTokens> ToTokens for TypedPatType<Type> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.pat.to_tokens(tokens);
        self.colon_token.to_tokens(tokens);
        self.ty.to_tokens(tokens);
    }
}

impl<Type: Parse> Parse for TypedPatType<Type> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            pat: input.parse()?,
            colon_token: input.parse()?,
            ty: input.parse()?,
        })
    }
}

#[derive(Clone)]
pub struct FieldDeclarationWithCommonBoundsCsrFnUpdateArgs {
    pub pre_args: FieldDeclarationWithCommonBoundsCsrFnCommonArgs,
    pub comma_3: syn::Token![,],
    pub state: TypedPatType<syn::Token![_]>,
    pub comma_trailing: Option<syn::Token![,]>,
}

impl Parse for FieldDeclarationWithCommonBoundsCsrFnUpdateArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            pre_args: input.parse()?,
            comma_3: input.parse()?,
            state: input.parse()?,
            comma_trailing: input.parse()?,
        })
    }
}

#[derive(Clone)]
pub struct FieldDeclarationWithCommonBoundsCsrFnInitialize {
    pub fn_token: syn::Token![fn],
    pub ident: kw::initialize,
    pub args: Parenthesized<FieldDeclarationWithCommonBoundsCsrFnInitializeArgs>,
    pub arrow_token: syn::Token![->],
    pub ty_state: syn::Type,
    pub block: syn::Block,
}

impl Parse for FieldDeclarationWithCommonBoundsCsrFnInitialize {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            fn_token: input.parse()?,
            ident: input.parse()?,
            args: input.parse()?,
            arrow_token: input.parse()?,
            ty_state: input.parse()?,
            block: input.parse()?,
        })
    }
}

#[derive(Clone)]
pub struct FieldDeclarationWithCommonBoundsCsrFnUpdate {
    pub fn_token: syn::Token![fn],
    pub ident: kw::update,
    pub args: Parenthesized<FieldDeclarationWithCommonBoundsCsrFnUpdateArgs>,
    pub block: syn::Block,
}

impl Parse for FieldDeclarationWithCommonBoundsCsrFnUpdate {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            fn_token: input.parse()?,
            ident: input.parse()?,
            args: input.parse()?,
            block: input.parse()?,
        })
    }
}

#[derive(Clone)]
pub struct FieldDeclarationWithCommonBoundsCsrContent {
    pub fn_initialize: FieldDeclarationWithCommonBoundsCsrFnInitialize,
    pub fn_update: FieldDeclarationWithCommonBoundsCsrFnUpdate,
}

impl Parse for FieldDeclarationWithCommonBoundsCsrContent {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            fn_initialize: input.parse()?,
            fn_update: input.parse()?,
        })
    }
}

#[derive(Clone)]
pub struct FnIntoAttrs {
    pub fn_token: syn::Token![fn],
    pub ident: kw::into_attrs,
    pub paren_token: syn::token::Paren,
    pub inputs: TypedPatType<syn::Token![Self]>,
    pub arrow_token: syn::Token![->],
    pub ty_into_iter_attrs: syn::Type,
    pub block: syn::Block,
}

impl Parse for FnIntoAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let inputs;
        Ok(Self {
            fn_token: input.parse()?,
            ident: input.parse()?,
            paren_token: syn::parenthesized!(inputs in input),
            inputs: inputs.parse()?,
            arrow_token: input.parse()?,
            ty_into_iter_attrs: input.parse()?,
            block: input.parse()?,
        })
    }
}

#[derive(Clone)]
pub struct FieldDeclarationWithCommonBoundsSsrContent {
    pub fn_into_iter_attrs: FnIntoAttrs,
}

impl Parse for FieldDeclarationWithCommonBoundsSsrContent {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            fn_into_iter_attrs: input.parse()?,
        })
    }
}

type ImplToken = syn::Token![impl];

/// `impl csr { ... }`
#[derive(Clone)]
pub struct FieldDeclarationWithBoundsImpl<ImplIdent, Content> {
    pub impl_token: ImplToken,
    pub impl_ident: ImplIdent,
    pub where_clause: Option<syn::WhereClause>,
    pub content: Braced<Content>,
}

impl<ModIdent: Parse, Content: Parse> Parse for FieldDeclarationWithBoundsImpl<ModIdent, Content> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            impl_token: input.parse()?,
            impl_ident: input.parse()?,
            where_clause: input.parse()?,
            content: input.parse()?,
        })
    }
}

#[derive(Clone)]
pub struct FieldDeclarationWithBoundsInitial {
    pub const_token: syn::Token![const],
    pub ident: syn::Token![_],
    pub colon_token: syn::Token![:],
    pub ty: syn::Type,
    pub eq_token: syn::Token![=],
    pub expr: syn::Expr,
    pub semi_token: syn::Token![;],
}

impl FieldDeclarationWithBoundsInitial {
    pub fn parse_some(input: syn::parse::ParseStream) -> syn::Result<Option<Self>> {
        if input.peek(syn::Token![const]) {
            input.parse().map(Some)
        } else {
            Ok(None)
        }
    }
}

impl Parse for FieldDeclarationWithBoundsInitial {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            const_token: input.parse()?,
            ident: input.parse()?,
            colon_token: input.parse()?,
            ty: input.parse()?,
            eq_token: input.parse()?,
            expr: input.parse()?,
            semi_token: input.parse()?,
        })
    }
}

type FieldDeclarationWithBoundsImplCsr =
    FieldDeclarationWithBoundsImpl<kw::csr, FieldDeclarationWithCommonBoundsCsrContent>;

type FieldDeclarationWithBoundsImplSsr =
    FieldDeclarationWithBoundsImpl<kw::ssr, FieldDeclarationWithCommonBoundsSsrContent>;

#[derive(Clone)]
pub struct BoundsPath {
    pub mod_path: syn::Path,
    pub generic_args: Option<syn::AngleBracketedGenericArguments>,
}

impl Parse for BoundsPath {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            mod_path: input.call(syn::Path::parse_mod_style)?,
            generic_args: if input.peek(syn::Token![<])
                || (input.peek(syn::Token![::]) && input.peek2(syn::Token![<]))
            {
                Some(input.parse()?)
            } else {
                None
            },
        })
    }
}

#[derive(Clone)]
pub struct FieldDeclarationWithBoundsDetailsSimple {
    pub bounds: (kw::bounds, syn::Token![as], BoundsPath),
    pub attr_name: Option<(syn::Token![,], kw::attr_name, syn::Token![=], syn::Expr)>,
    pub imps: Vec<(syn::Token![,], syn::Ident, Braced<TokenStream>)>,
    pub trailing_comma: Option<syn::Token![,]>,
}

impl Parse for FieldDeclarationWithBoundsDetailsSimple {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            bounds: (input.parse()?, input.parse()?, input.parse()?),
            attr_name: if input.peek(syn::Token![,]) && input.peek2(kw::attr_name) {
                Some((
                    input.parse()?,
                    input.parse()?,
                    input.parse()?,
                    input.parse()?,
                ))
            } else {
                None
            },
            imps: {
                let mut imps = vec![];

                while input.peek(syn::Token![,]) && input.peek2(syn::Ident) {
                    imps.push((input.parse()?, input.parse()?, input.parse()?));
                }

                imps
            },
            trailing_comma: input.parse()?,
        })
    }
}

#[derive(Clone)]
pub struct FieldDeclarationWithBoundsDetailsFull {
    pub common_bounds: Option<FieldDeclarationWithCommonBoundsTraitBounds>,
    pub initial: Option<FieldDeclarationWithBoundsInitial>,
    pub csr: FieldDeclarationWithBoundsImplCsr,
    pub ssr: FieldDeclarationWithBoundsImplSsr,
}

impl FieldDeclarationWithBoundsDetailsFull {
    pub fn csr_content(&self) -> &FieldDeclarationWithCommonBoundsCsrContent {
        &self.csr.content.content
    }

    pub fn ssr_content(&self) -> &FieldDeclarationWithCommonBoundsSsrContent {
        &self.ssr.content.content
    }
}

impl Parse for FieldDeclarationWithBoundsDetailsFull {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            common_bounds: input.call(FieldDeclarationWithCommonBoundsTraitBounds::parse_some)?,
            initial: input.call(FieldDeclarationWithBoundsInitial::parse_some)?,
            csr: input.parse()?,
            ssr: input.parse()?,
        })
    }
}

#[derive(Clone)]
pub struct FieldDeclarationWithBounds {
    pub colon: syn::Token![:],
    pub ident: kw::bounds,
    pub bang_token: syn::Token![!],
    pub delimiter: syn::MacroDelimiter,
    pub details: FieldDeclarationWithBoundsDetails,
}

#[derive(Clone)]
pub enum FieldDeclarationWithBoundsDetails {
    Full(Braced<FieldDeclarationWithBoundsDetailsFull>),
    Simple(FieldDeclarationWithBoundsDetailsSimple),
}

impl Parse for FieldDeclarationWithBoundsDetails {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(if let Some(full) = input.parse()? {
            Self::Full(full)
        } else {
            Self::Simple(input.parse()?)
        })
    }
}

impl Parse for FieldDeclarationWithBounds {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let colon = input.parse()?;
        let syn::Macro {
            path,
            bang_token,
            delimiter,
            tokens,
        } = input.parse()?;

        let ident = if let Some(bounds) =
            path.get_ident()
                .and_then(|ident| if ident == "bounds" { Some(ident) } else { None })
        {
            kw::bounds(bounds.span())
        } else {
            return Err(syn::Error::new_spanned(
                path,
                "macro path should be `bounds`",
            ));
        };

        Ok(Self {
            colon,
            ident,
            bang_token,
            delimiter,
            details: syn::parse2(tokens)?,
        })
    }
}
