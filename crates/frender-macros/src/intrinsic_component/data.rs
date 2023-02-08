use proc_macro2::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

use crate::utils::{
    grouped::{Braced, Bracketed, Parenthesized},
    kw::PrefixKeyword,
};

use super::{kw, FieldDeclarationInherit, FieldDeclarationMaybe, IntrinsicComponentPropsVirtual};

#[derive(Clone)]
pub struct FieldDeclarationDomImpl {
    pub impl_token: syn::Token![impl],
    pub impl_body: Braced<TokenStream>,
}

impl Parse for FieldDeclarationDomImpl {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            impl_token: input.parse()?,
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
    Maybe(FieldDeclarationMaybe),
    Full(FieldDeclarationFull),
    Inherit(FieldDeclarationInherit),
}

impl FieldDeclaration {
    #[must_use]
    pub fn is_inherit(&self) -> bool {
        matches!(self, Self::Inherit(..))
    }

    pub fn as_inherit(&self) -> Option<&FieldDeclarationInherit> {
        if let Self::Inherit(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl FieldDeclarationFull {
    pub fn dom_state(&self) -> Option<&FieldDeclarationDomState> {
        self.definitions
            .content
            .dom_definitions
            .content
            .state
            .as_ref()
            .map(|b| &b.content)
    }
}

impl Parse for FieldDeclaration {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Some(question_token) = input.parse()? {
            Ok(Self::Maybe(FieldDeclarationMaybe {
                question_token,
                by_ref: input.parse()?,
                ty: input.parse()?,
                details: input.parse()?,
            }))
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

#[derive(Clone)]
pub struct Fields(pub Punctuated<Field, syn::Token![,]>);

impl Fields {
    pub fn prepend(&mut self, mut prepend_fields: Fields) {
        prepend_fields.0.extend(std::mem::take(&mut self.0));
        *self = prepend_fields;
    }
}

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
    pub inherits: Vec<Bracketed<IntrinsicComponentProps>>,
}

pub enum IntrinsicComponentProps {
    Virtual(IntrinsicComponentPropsVirtual),
    Data(IntrinsicComponentPropsData),
}

impl IntrinsicComponentProps {
    pub fn fields_mut(&mut self) -> &mut Fields {
        match self {
            IntrinsicComponentProps::Virtual(v) => &mut v.fields.content,
            IntrinsicComponentProps::Data(d) => &mut d.fields.content,
        }
    }
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
            inherits: input.call(Bracketed::parse_many)?,
        })
    }
}

impl Parse for IntrinsicComponentProps {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Token![virtual]) {
            input.parse().map(Self::Virtual)
        } else {
            input.parse().map(Self::Data)
        }
    }
}
