use proc_macro2::TokenStream;
use syn::{parse::Parse, punctuated::Punctuated};

use crate::utils::{
    grouped::{Braced, Bracketed, Parenthesized},
    kw::PrefixKeyword,
};

use super::kw;

#[derive(Clone)]
pub struct FieldDeclarationMaybeDetailsMethod {
    pub name: syn::Ident,
    pub deref_star_token: Option<syn::Token![*]>,
}

#[derive(Clone)]
pub struct FieldDeclarationMaybeDetailsImpl<K> {
    pub keyword: K,
    pub element_pat_ident: syn::Ident,
    pub rest: TokenStream,
}

impl<K: Parse> Parse for FieldDeclarationMaybeDetailsImpl<K> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            keyword: input.parse()?,
            element_pat_ident: input.parse()?,
            rest: input.parse()?,
        })
    }
}

#[derive(Clone)]
pub struct FieldDeclarationMaybeDetails {
    pub html_prop_name: Option<syn::LitStr>,
    pub impl_update: Option<Bracketed<FieldDeclarationMaybeDetailsImpl<kw::update>>>,
    pub impl_remove: Option<Bracketed<FieldDeclarationMaybeDetailsImpl<kw::remove>>>,
    pub html_element_method: Option<FieldDeclarationMaybeDetailsMethod>,
}

impl Parse for FieldDeclarationMaybeDetails {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut custom_impl: Option<_> = None;
        Ok(Self {
            html_prop_name: input.parse()?,
            impl_update: {
                if input.peek(syn::token::Bracket) {
                    let content;
                    let group_token = syn::bracketed!(content in input);

                    if content.peek(kw::update) {
                        Some(Bracketed {
                            group_token,
                            content: content.parse()?,
                        })
                    } else {
                        custom_impl = Some((group_token, content));
                        None
                    }
                } else {
                    None
                }
            },
            impl_remove: {
                if custom_impl.is_none() {
                    if input.peek(syn::token::Bracket) {
                        let content;
                        let group_token = syn::bracketed!(content in input);
                        custom_impl = Some((group_token, content))
                    }
                }
                if let Some((group_token, content)) = custom_impl {
                    Some(Bracketed {
                        group_token,
                        content: content.parse()?,
                    })
                } else {
                    None
                }
            },
            html_element_method: {
                if let Some(name) = input.parse()? {
                    Some(FieldDeclarationMaybeDetailsMethod {
                        name,
                        deref_star_token: input.parse()?,
                    })
                } else {
                    None
                }
            },
        })
    }
}

#[derive(Clone)]
pub struct FieldDeclarationDomImpl {
    pub impl_token: syn::Token![impl],
    pub pat: proc_macro2::TokenTree,
    pub impl_body: proc_macro2::TokenTree,
}

impl Parse for FieldDeclarationDomImpl {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            impl_token: input.parse()?,
            pat: input.parse()?,
            impl_body: input.parse()?,
        })
    }
}

#[derive(Clone)]
pub struct FieldDeclarationDomStateImpl {
    pub colon_token: syn::Token![:],
    /// dom state bounds
    pub bounds: Bracketed<TokenStream>,
    pub eq_token: syn::Token![=],
    pub initial_value: Parenthesized<syn::Expr>,
}

#[derive(Clone)]
pub struct FieldDeclarationDomState {
    pub pin: Option<kw::pin>,
    pub lt: syn::Token![<],
    pub ty: syn::Type,
    pub gt: syn::Token![>],
    pub implementation: Option<FieldDeclarationDomStateImpl>,
}

impl Parse for FieldDeclarationDomState {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            pin: input.parse()?,
            lt: input.parse()?,
            ty: input.parse()?,
            gt: input.parse()?,
            implementation: {
                if let Some(colon_token) = input.parse()? {
                    Some(FieldDeclarationDomStateImpl {
                        colon_token,
                        bounds: input.parse()?,
                        eq_token: input.parse()?,
                        initial_value: input.parse()?,
                    })
                } else {
                    None
                }
            },
        })
    }
}

#[derive(Clone)]
pub struct FieldDeclarationDomDefinitions {
    /// dom bounds
    pub bounds: Option<PrefixKeyword<kw::bounds, Bracketed<TokenStream>>>,
    pub state: Option<PrefixKeyword<kw::state, FieldDeclarationDomState>>,
    pub implementation: FieldDeclarationDomImpl,
}

impl Parse for FieldDeclarationDomDefinitions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            bounds: input.parse()?,
            state: input.parse()?,
            implementation: input.parse()?,
        })
    }
}

#[derive(Clone)]
pub struct FieldDeclarationDefinitions {
    pub dom_keyword: kw::dom,
    pub dom_definitions: Braced<FieldDeclarationDomDefinitions>,
}

impl Parse for FieldDeclarationDefinitions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            dom_keyword: input.parse()?,
            dom_definitions: input.parse()?,
        })
    }
}

#[derive(Clone)]
pub struct FieldDeclarationFull {
    pub bounds: Option<Bracketed<TokenStream>>,
    pub colon_token: syn::Token![:],
    pub initial_type: syn::Type,
    pub eq_token: syn::Token![=],
    pub initial_value: syn::Expr,
    pub fat_arrow_token: syn::Token![=>],
    pub definitions: Braced<FieldDeclarationDefinitions>,
}

impl Parse for FieldDeclarationFull {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            bounds: input.parse()?,
            colon_token: input.parse()?,
            initial_type: input.parse()?,
            eq_token: input.parse()?,
            initial_value: input.parse()?,
            fat_arrow_token: input.parse()?,
            definitions: input.parse()?,
        })
    }
}

#[derive(Clone)]
pub enum FieldDeclaration {
    Maybe {
        question_token: syn::Token![?],
        ty: syn::Type,
        details: Option<Braced<FieldDeclarationMaybeDetails>>,
    },
    Full(FieldDeclarationFull),
}

impl Parse for FieldDeclaration {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Some(question_token) = input.parse()? {
            Ok(Self::Maybe {
                question_token,
                ty: input.parse()?,
                details: input.parse()?,
            })
        } else {
            Ok(Self::Full(input.parse()?))
        }
    }
}

#[derive(Clone)]
pub struct Field {
    pub attrs: Vec<syn::Attribute>,
    pub name: syn::Ident,
    pub declaration: FieldDeclaration,
}

impl Parse for Field {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attrs: input.call(syn::Attribute::parse_outer)?,
            name: input.parse()?,
            declaration: input.parse()?,
        })
    }
}

pub struct Fields(pub Punctuated<Field, syn::Token![,]>);

impl Parse for Fields {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse_terminated(Field::parse).map(Self)
    }
}

pub struct ComponentNames {
    pub colon_token: syn::Token![:],
    pub names: Punctuated<syn::Ident, syn::Token![,]>,
}

pub struct DomElement {
    pub ty: syn::Type,
    pub component_names: Option<ComponentNames>,
}

impl DomElement {
    pub fn component_names(&self) -> Option<&Punctuated<syn::Ident, syn::Token![,]>> {
        self.component_names.as_ref().map(|v| &v.names)
    }
}

impl Parse for DomElement {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            ty: input.parse()?,
            component_names: if let Some(colon_token) = input.parse()? {
                Some(ComponentNames {
                    colon_token,
                    names: input.parse_terminated(syn::Ident::parse)?,
                })
            } else {
                None
            },
        })
    }
}

pub struct IntrinsicComponentPropsData {
    pub attrs: Vec<syn::Attribute>,
    pub vis: syn::Visibility,
    pub struct_token: syn::Token![struct],
    pub name: syn::Ident,
    pub dom_element: Parenthesized<DomElement>,
    pub fields: Braced<Fields>,
    pub inherits: Vec<Bracketed<IntrinsicComponentPropsData>>,
}

impl Parse for IntrinsicComponentPropsData {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attrs: input.call(syn::Attribute::parse_outer)?,
            vis: input.parse()?,
            struct_token: input.parse()?,
            name: input.parse()?,
            dom_element: input.parse()?,
            fields: input.parse()?,
            inherits: {
                let mut inherits = vec![];

                while let Some(inherit) = input.parse()? {
                    inherits.push(inherit)
                }

                inherits
            },
        })
    }
}
