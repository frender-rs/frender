use proc_macro2::Span;
use quote::ToTokens;
use syn::spanned::Spanned;

pub struct RsxElement {
    pub start_lt: syn::Token![<],
    pub component_type: RsxComponentType,
    pub props: Vec<RsxProp>,
    pub key: Option<RsxKey>,
    pub children: RsxElementChildren,
}

/// `component_type` is a path, no `key`.
pub struct PureRsxElement {
    pub start_lt: syn::Token![<],
    pub component_path: syn::Path,
    pub props: Vec<RsxProp>,
    pub children: RsxElementChildren,
}

pub enum RsxElementChildren {
    /// <a />
    No {
        slash: syn::Token![/],
        start_gt: syn::Token![>],
    },
    /// `<a></a>` or `<a></_>`
    Yes {
        start_gt: syn::Token![>],
        children: Vec<RsxChild>,
        end_lt: syn::Token![<],
        end_slash: syn::Token![/],
        end_component_type: RsxEndElementComponentType,
        end_gt: syn::Token![>],
    },
}

impl RsxElementChildren {
    pub fn start_gt(&self) -> &syn::Token![>] {
        match self {
            RsxElementChildren::No { start_gt, .. } => start_gt,
            RsxElementChildren::Yes { start_gt, .. } => start_gt,
        }
    }

    pub fn unwrap_children(self) -> Option<Vec<RsxChild>> {
        match self {
            RsxElementChildren::No { .. } => None,
            RsxElementChildren::Yes { children, .. } => {
                if children.is_empty() {
                    None
                } else {
                    Some(children)
                }
            }
        }
    }

    pub fn unwrap_children_and_span(self) -> Option<(Vec<RsxChild>, Span)> {
        match self {
            RsxElementChildren::No { .. } => None,
            RsxElementChildren::Yes {
                children, start_gt, ..
            } => {
                if children.is_empty() {
                    None
                } else {
                    Some((children, start_gt.span))
                }
            }
        }
    }
}

pub enum RsxEndElementComponentType {
    Underscore(syn::Token![_]),
    Explicit(RsxComponentType),
}

impl std::fmt::Display for RsxEndElementComponentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RsxEndElementComponentType::Underscore(_) => write!(f, "_"),
            RsxEndElementComponentType::Explicit(cp) => cp.fmt(f),
        }
    }
}

impl RsxEndElementComponentType {
    pub fn optional_span(&self) -> Option<Span> {
        match self {
            RsxEndElementComponentType::Underscore(us) => Some(us.span),
            RsxEndElementComponentType::Explicit(cp) => cp.optional_span(),
        }
    }
}

pub enum RsxComponentType {
    /// `<>` or `<#>`
    Fragment(Option<syn::Token![#]>),
    Path(syn::Path),
}

impl std::fmt::Display for RsxComponentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RsxComponentType::Fragment(t) => {
                write!(f, "{}", if t.is_some() { "#" } else { "" })
            }
            RsxComponentType::Path(tp) => write!(f, "{}", tp.to_token_stream()),
        }
    }
}

impl RsxComponentType {
    pub fn optional_span(&self) -> Option<Span> {
        match self {
            RsxComponentType::Fragment(t) => t.as_ref().map(Spanned::span),
            RsxComponentType::Path(tp) => Some(tp.span()),
        }
    }
}

#[inline]
pub fn ident_is_intrinsic_component(ident: &syn::Ident) -> bool {
    let s = ident.to_string();
    let c = s.chars().next().unwrap();
    ('a'..='z').contains(&c)
    // match c {
    //     'a'..='z' => true,
    //     _ => false,
    // }
}

pub struct RsxProp {
    pub name: syn::Ident,
    pub value: Option<RsxPropValue>,
}

/// key={value}
pub struct RsxKey {
    pub name: syn::Ident,
    pub value: RsxPropValue,
}

pub enum RsxKeyOrProp {
    Key(RsxKey),
    Prop(RsxProp),
}

pub enum LitOrBraced {
    Lit(syn::Lit),
    Braced {
        brace: syn::token::Brace,
        inner: proc_macro2::TokenStream,
    },
}

pub struct RsxPropValue {
    pub eq: syn::Token![=],
    pub value: LitOrBraced,
}

pub enum RsxChild {
    LitOrBraced(LitOrBraced),
    Element(RsxElement),
}

pub type OptionalCratePathAndRsxChild = crate::utils::prefix_path::PrefixPath<RsxChild>;
