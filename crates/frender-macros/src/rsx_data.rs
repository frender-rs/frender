use darling::ToTokens;
use proc_macro2::Span;
use syn::{braced, parse::Parse, punctuated::Punctuated, spanned::Spanned};

pub struct RsxCratePath {
    pub at: syn::Token![@],
    pub bracket: syn::token::Bracket,
    pub path: proc_macro2::TokenStream,
}

pub struct RsxElement {
    pub start_lt: syn::Token![<],
    pub component_type: RsxComponentType,
    pub props: Vec<RsxProp>,
    pub key: Option<RsxKey>,
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
            RsxEndElementComponentType::Underscore(us) => Some(us.span()),
            RsxEndElementComponentType::Explicit(cp) => cp.optional_span(),
        }
    }
}

impl RsxEndElementComponentType {
    pub fn match_start(
        &self,
        start: &RsxComponentType,
        start_span: Span,
        end_span: Span,
    ) -> syn::Result<()> {
        let matched = match self {
            RsxEndElementComponentType::Underscore(_) => true,
            RsxEndElementComponentType::Explicit(end) => match start {
                RsxComponentType::Fragment(_) => match end {
                    RsxComponentType::Fragment(_) => true,
                    _ => false,
                },
                RsxComponentType::Path(start) => match end {
                    RsxComponentType::Path(end) => {
                        end == start
                            || end.get_ident().map_or(false, |end_ident| {
                                start.leading_colon.is_none()
                                    && start.segments.len() == 3
                                    && start.segments[0].arguments.is_none()
                                    && start.segments[0].ident == "self"
                                    && start.segments[1].arguments.is_none()
                                    && start.segments[1].ident == "intrinsic_components"
                                    && start.segments[2].arguments.is_none()
                                    && start.segments[2].ident == end_ident.to_string()
                            })
                    }
                    _ => false,
                },
            },
        };

        if matched {
            Ok(())
        } else {
            let mut err = syn::Error::new(start_span, format!("<{}> not properly enclosed", start));
            err.combine(syn::Error::new(
                end_span,
                format!("Expect </_> or </{}>, but got </{}>", start, self),
            ));
            Err(err)
        }
    }
}

impl Parse for RsxEndElementComponentType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let us: Option<syn::Token![_]> = input.parse()?;
        if let Some(us) = us {
            Ok(Self::Underscore(us))
        } else {
            input.parse().map(Self::Explicit)
        }
    }
}

impl Parse for RsxElement {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let start_lt: syn::Token![<] = input.parse()?;
        let component_type: RsxComponentType = input.parse()?;

        let mut props = Vec::new();
        let mut key: Option<RsxKey> = None;

        loop {
            let start_gt: Option<syn::Token![>]> = input.parse()?;
            if let Some(start_gt) = start_gt {
                // <a> props end but there might be children

                let mut children = Vec::new();
                loop {
                    if input.peek(syn::Token![<]) {
                        if input.peek2(syn::Token![/]) {
                            // </a>
                            let end_lt: syn::Token![<] = input.parse()?;
                            let end_slash: syn::Token![/] = input.parse()?;
                            let end_component_type: RsxEndElementComponentType = input.parse()?;
                            let end_gt: syn::Token![>] = input.parse()?;
                            end_component_type.match_start(
                                &component_type,
                                component_type
                                    .optional_span()
                                    .unwrap_or_else(|| start_lt.span()),
                                end_component_type
                                    .optional_span()
                                    .unwrap_or_else(|| end_slash.span()),
                            )?;

                            return Ok(Self {
                                start_lt,
                                component_type,
                                key,
                                props,
                                children: RsxElementChildren::Yes {
                                    start_gt,
                                    children,
                                    end_lt,
                                    end_slash,
                                    end_component_type,
                                    end_gt,
                                },
                            });
                        } else {
                            // <a> <span> child elements
                            children.push(input.parse().map(RsxChild::Element)?);
                        }
                    } else {
                        // { expr } or "lit"
                        let le = input.parse()?;
                        children.push(RsxChild::LitOrBraced(le));
                    }
                }
            }

            let slash: Option<syn::Token![/]> = input.parse()?;
            if let Some(slash) = slash {
                // <a /> element end with no children
                let start_gt: syn::Token![>] = input.parse()?;

                return Ok(Self {
                    start_lt,
                    key,
                    props,
                    component_type,
                    children: RsxElementChildren::No { slash, start_gt },
                });
            }

            // parse key or props
            let kp: RsxKeyOrProp = input.parse()?;

            match kp {
                RsxKeyOrProp::Key(k) => {
                    if let Some(key) = key {
                        static MSG: &str = "key should only be specified once";
                        let mut err = syn::Error::new(key.name.span(), MSG);
                        err.combine(syn::Error::new(k.name.span(), MSG));
                        return Err(err);
                    } else {
                        key = Some(k);
                    }
                }
                RsxKeyOrProp::Prop(p) => props.push(p),
            };
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
            RsxComponentType::Path(tp) => write!(f, "{}", tp.to_token_stream().to_string()),
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
fn ident_is_intrinsic_component(ident: &syn::Ident) -> bool {
    let s = ident.to_string();
    let c = s.chars().nth(0).unwrap();
    c >= 'a' && c <= 'z'
    // match c {
    //     'a'..='z' => true,
    //     _ => false,
    // }
}

impl Parse for RsxComponentType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Token![>]) {
            // <> fragment
            Ok(Self::Fragment(None))
        } else {
            let frag: Option<syn::Token![#]> = input.parse()?;

            if let Some(frag) = frag {
                // <#> fragment
                Ok(Self::Fragment(Some(frag)))
            } else {
                let p: syn::Path = input.parse()?;

                Ok(Self::Path(p))
            }
        }
    }
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

impl Parse for RsxKeyOrProp {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let prop: RsxProp = input.parse()?;

        if prop.name == "key" {
            if let Some(value) = prop.value {
                Ok(Self::Key(RsxKey {
                    name: prop.name,
                    value,
                }))
            } else {
                Err(syn::Error::new(
                    prop.name.span(),
                    "value of key must be provided",
                ))
            }
        } else {
            Ok(Self::Prop(prop))
        }
    }
}

pub struct RsxPropValue {
    pub eq: syn::Token![=],
    pub value: LitOrBraced,
}

impl Parse for RsxProp {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: syn::Ident = input.parse()?;

        let value = {
            let eq: Option<syn::Token![=]> = input.parse()?;
            if let Some(eq) = eq {
                Some(RsxPropValue {
                    eq,
                    value: input.parse()?,
                })
            } else {
                None
            }
        };

        Ok(RsxProp { name, value })
    }
}

pub enum LitOrBraced {
    Lit(syn::Lit),
    Braced {
        brace: syn::token::Brace,
        inner: proc_macro2::TokenStream,
    },
}

impl Parse for LitOrBraced {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::token::Brace) {
            let expr;
            let brace = braced!(expr in input);

            Ok(Self::Braced {
                brace,
                inner: expr.parse()?,
            })
        } else {
            input.parse().map(Self::Lit)
        }
    }
}

pub enum RsxChild {
    LitOrBraced(LitOrBraced),
    Element(RsxElement),
}

impl Parse for RsxChild {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Token![<]) {
            input.parse().map(Self::Element)
        } else {
            input.parse().map(Self::LitOrBraced)
        }
    }
}

pub struct OptionalCratePathAndRsxChild {
    pub crate_path: Option<RsxCratePath>,
    pub child: RsxChild,
}

impl Parse for OptionalCratePathAndRsxChild {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let crate_path_at: Option<syn::Token![@]> = input.parse()?;

        let crate_path = if let Some(at) = crate_path_at {
            let path;

            Some(RsxCratePath {
                at,
                bracket: syn::bracketed!(path in input),
                path: path.parse()?,
            })
        } else {
            None
        };

        let child = input.parse()?;

        Ok(Self { crate_path, child })
    }
}
