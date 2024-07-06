use proc_macro2::Span;
use syn::{braced, ext::IdentExt, parse::Parse, spanned::Spanned};

use super::*;

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
                                    && start.segments[2].ident == *end_ident
                            })
                    }
                    _ => false,
                },
            },
        };

        if matched {
            Ok(())
        } else {
            let mut err = syn::Error::new(start_span, format!("<{start}> not properly enclosed"));
            err.combine(syn::Error::new(
                end_span,
                format!("Expect </_> or </{start}>, but got </{self}>"),
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
                // TODO: record the error and fallback
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

fn should_use_raw(ident_str: &str) -> bool {
    match ident_str {
        // Copied from https://docs.rs/proc-macro2/1.0.53/src/proc_macro2/fallback.rs.html#784-789
        "_" | "super" | "self" | "Self" | "crate" => false,
        #[rustfmt::skip]
        #[allow(unreachable_patterns)]
        // Copied from https://docs.rs/syn/2.0.5/src/syn/ident.rs.html#57-71
        // Based on https://doc.rust-lang.org/1.65.0/reference/keywords.html
        "abstract" | "as" | "async" | "await" | "become" | "box" | "break" |
        "const" | "continue" | "crate" | "do" | "dyn" | "else" | "enum" |
        "extern" | "false" | "final" | "fn" | "for" | "if" | "impl" | "in" |
        "let" | "loop" | "macro" | "match" | "mod" | "move" | "mut" |
        "override" | "priv" | "pub" | "ref" | "return" | "Self" | "self" |
        "static" | "struct" | "super" | "trait" | "true" | "try" | "type" |
        "typeof" | "unsafe" | "unsized" | "use" | "virtual" | "where" |
        "while" | "yield" => true,
        _ => false,
    }
}

impl Parse for RsxProp {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: syn::Ident = {
            let name = input.call(syn::Ident::parse_any)?;
            let ident_str = name.to_string();
            if should_use_raw(&ident_str) {
                syn::Ident::new_raw(&ident_str, name.span())
            } else {
                name
            }
        };

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

impl Parse for RsxChild {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Token![<]) {
            input.parse().map(Self::Element)
        } else {
            input.parse().map(Self::LitOrBraced)
        }
    }
}

impl Parse for OptionalCratePathAndRsxChild {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Parse::parse(input).map(Self)
    }
}
