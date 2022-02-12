use proc_macro2::Span;
use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};
use syn::{braced, parse::Parse, punctuated::Punctuated, spanned::Spanned};

pub struct RsxElement {
    pub start_lt: syn::Token![<],
    pub component_type: RsxComponentType,
    pub props: Punctuated<RsxProp, syn::token::Group>,
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
        children: Punctuated<RsxChild, syn::token::Group>,
        end_lt: syn::Token![<],
        end_slash: syn::Token![/],
        end_component_type: RsxEndElementComponentType,
        end_gt: syn::Token![>],
    },
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
                RsxComponentType::Intrinsic(start) => match end {
                    RsxComponentType::Intrinsic(end) => end == start,
                    _ => false,
                },
                RsxComponentType::TypePath(start) => match end {
                    RsxComponentType::TypePath(end) => end == start,
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

        let mut props = Punctuated::new();
        let mut key: Option<RsxKey> = None;

        loop {
            let start_gt: Option<syn::Token![>]> = input.parse()?;
            if let Some(start_gt) = start_gt {
                // <a> props end but there might be children

                let mut children = Punctuated::new();
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
                        children.push(RsxChild::LitOrBracedExpr(le));
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
                        static msg: &str = "key should only be specified once";
                        let mut err = syn::Error::new(key.name.span(), msg);
                        err.combine(syn::Error::new(k.name.span(), msg));
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
    Intrinsic(syn::Ident),
    TypePath(syn::TypePath),
}

impl std::fmt::Display for RsxComponentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RsxComponentType::Fragment(t) => {
                write!(f, "{}", if t.is_some() { "#" } else { "" })
            }
            RsxComponentType::Intrinsic(id) => write!(f, "{}", id),
            RsxComponentType::TypePath(tp) => write!(f, "{:?}", tp),
        }
    }
}

impl RsxComponentType {
    pub fn optional_span(&self) -> Option<Span> {
        match self {
            RsxComponentType::Fragment(t) => t.as_ref().map(Spanned::span),
            RsxComponentType::Intrinsic(id) => Some(id.span()),
            RsxComponentType::TypePath(tp) => Some(tp.span()),
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
                let p: syn::TypePath = input.parse()?;

                if p.qself.is_none()
                    && p.path
                        .get_ident()
                        .map_or(false, ident_is_intrinsic_component)
                {
                    // <a> intrinsic component
                    Ok(Self::Intrinsic(p.path.segments[0].ident.clone()))
                } else {
                    // <MyComp>
                    Ok(Self::TypePath(p))
                }
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
    /// if is some,
    /// then value itself will be passed without
    /// `IntoPropValue::into_prop_value` or `AsKey::as_key`
    pub colon: Option<syn::Token![:]>,
    pub eq: syn::Token![=],
    pub value: LitOrBracedExpr,
}

impl Parse for RsxProp {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: syn::Ident = input.parse()?;
        let colon: Option<syn::Token![:]> = input.parse()?;

        let value = if colon.is_some() {
            let eq: syn::Token![=] = input.parse()?;
            Some(RsxPropValue {
                colon,
                eq,
                value: input.parse()?,
            })
        } else {
            let eq: Option<syn::Token![=]> = input.parse()?;
            if let Some(eq) = eq {
                Some(RsxPropValue {
                    colon: None,
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

pub enum LitOrBracedExpr {
    Lit(syn::Lit),
    BracedExpr {
        brace: syn::token::Brace,
        expr: syn::Expr,
    },
}

impl Parse for LitOrBracedExpr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::token::Brace) {
            let expr;
            let brace = braced!(expr in input);

            Ok(Self::BracedExpr {
                brace,
                expr: expr.parse()?,
            })
        } else {
            input.parse().map(Self::Lit)
        }
    }
}

pub enum RsxChild {
    LitOrBracedExpr(LitOrBracedExpr),
    Element(RsxElement),
}

impl Parse for RsxChild {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Token![<]) {
            input.parse().map(Self::Element)
        } else {
            input.parse().map(Self::LitOrBracedExpr)
        }
    }
}