use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::Parse;

use crate::utils::grouped::Bracketed;

use super::kw;

#[derive(Clone)]
pub struct FieldDeclarationMaybeDetails {
    pub html_prop_name: Option<syn::LitStr>,
    pub impl_update: Option<Bracketed<FieldDeclarationMaybeDetailsImpl<kw::update>>>,
    pub impl_remove: Option<Bracketed<FieldDeclarationMaybeDetailsImpl<kw::remove>>>,
    pub html_element_method: Option<syn::Ident>,
}

type ColonToken = syn::Token![:];
type OrToken = syn::Token![|];
type CommaToken = syn::Token![,];
type Dot2Token = syn::Token![..];

/// A sub set of [`syn::FieldPat`].
#[derive(Clone)]
pub struct ImplClosureFieldPat {
    pub member: syn::Ident,
    pub colon_token: Option<ColonToken>,
    pub pat: Box<syn::Pat>,
}

impl ImplClosureFieldPat {
    fn try_from_field_pat(field_pat: syn::FieldPat) -> syn::Result<Self> {
        let syn::FieldPat {
            attrs,
            member,
            colon_token,
            pat,
        } = field_pat;

        if !attrs.is_empty() {
            return Err(syn::Error::new_spanned(
                quote!(#(#attrs)*),
                "attributes are not allowed here",
            ));
        }

        let member = match member {
            syn::Member::Named(member) => member,
            syn::Member::Unnamed(member) => {
                return Err(syn::Error::new_spanned(member, "field must be named"));
            }
        };

        Ok(Self {
            member,
            colon_token,
            pat,
        })
    }
}

/// A sub set of [`syn::PatStruct`].
#[derive(Clone)]
pub struct ImplClosurePatStruct {
    pub path: syn::token::SelfType,
    pub brace_token: syn::token::Brace,
    pub fields: syn::punctuated::Punctuated<ImplClosureFieldPat, CommaToken>,
    pub dot2_token: Option<Dot2Token>,
}

pub(crate) struct ImplClosurePatStructFieldsIntoMap {
    inner: std::collections::HashMap<String, ImplClosureFieldPat>,
}

impl Drop for ImplClosurePatStructFieldsIntoMap {
    fn drop(&mut self) {
        if self.inner.is_empty() {
            return;
        }
        let this = std::mem::replace(
            self,
            ImplClosurePatStructFieldsIntoMap {
                inner: Default::default(),
            },
        );
        this.assert_empty().unwrap()
    }
}

impl ImplClosurePatStructFieldsIntoMap {
    pub(crate) fn assert_empty(mut self) -> syn::Result<()> {
        let inner = std::mem::take(&mut self.inner);
        let mut errors = None::<syn::Error>;
        for unused in inner.into_values() {
            let error = syn::Error::new_spanned(unused.member, "field not used");
            if let Some(errors) = &mut errors {
                errors.combine(error);
            } else {
                errors = Some(error);
            }
        }

        errors.map_or(Ok(()), Err)
    }

    pub(crate) fn take(&mut self, field_name: &str) -> Option<ImplClosureFieldPat> {
        self.inner.remove_entry(field_name).map(|(_, v)| v)
    }

    pub(crate) fn take_or(
        &mut self,
        field_name: &str,
        f: impl FnOnce(&syn::Ident) -> (Option<syn::Token![:]>, Box<syn::Pat>),
    ) -> ImplClosureFieldPat {
        self.inner.remove_entry(field_name).map_or_else(
            || {
                let member = syn::Ident::new(field_name, proc_macro2::Span::call_site());
                let (colon_token, pat) = f(&member);
                ImplClosureFieldPat {
                    member,
                    colon_token,
                    pat,
                }
            },
            |(_, v)| v,
        )
    }

    pub(crate) fn take_or_ident(&mut self, field_name: &str) -> ImplClosureFieldPat {
        self.take_or(field_name, |member| {
            (
                None,
                Box::new(syn::Pat::Ident(syn::PatIdent {
                    attrs: vec![],
                    by_ref: None,
                    mutability: None,
                    ident: member.clone(),
                    subpat: None,
                })),
            )
        })
    }

    pub(crate) fn take_or_wild(&mut self, field_name: &str) -> ImplClosureFieldPat {
        self.take_or(field_name, |member| {
            (
                None,
                Box::new(syn::Pat::Wild(syn::PatWild {
                    attrs: vec![],
                    underscore_token: Default::default(),
                })),
            )
        })
    }
}

impl ImplClosurePatStruct {
    pub(crate) fn fields_into_map(self) -> ImplClosurePatStructFieldsIntoMap {
        ImplClosurePatStructFieldsIntoMap {
            inner: self
                .fields
                .into_iter()
                .map(|field| (field.member.to_string(), field))
                .collect(),
        }
    }

    pub(crate) fn fields_to_map(&self) -> ImplClosurePatStructFieldsIntoMap {
        ImplClosurePatStructFieldsIntoMap {
            inner: self
                .fields
                .iter()
                .map(|field| (field.member.to_string(), field.clone()))
                .collect(),
        }
    }
}

impl Parse for ImplClosurePatStruct {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let pat: syn::Pat = input.parse()?;

        if let syn::Pat::Struct(syn::PatStruct {
            attrs,
            path,
            brace_token,
            fields,
            dot2_token,
        }) = pat
        {
            if !attrs.is_empty() {
                return Err(syn::Error::new_spanned(
                    quote!(#(#attrs)*),
                    "struct pattern here cannot have attributes",
                ));
            }

            let path = match path.get_ident() {
                Some(ident) if *ident == "Self" => syn::Token![Self](ident.span()),
                _ => {
                    return Err(syn::Error::new_spanned(
                        path,
                        "struct path here must be `Self`",
                    ));
                }
            };

            let fields = fields
                .into_pairs()
                .map(|pair| {
                    let (field_pat, comma) = pair.into_tuple();
                    Ok(syn::punctuated::Pair::new(
                        ImplClosureFieldPat::try_from_field_pat(field_pat)?,
                        comma,
                    ))
                })
                .collect::<syn::Result<_>>()?;

            Ok(Self {
                path,
                brace_token,
                fields,
                dot2_token,
            })
        } else {
            return Err(syn::Error::new_spanned(pat, "expect struct pattern"));
        }
    }
}

/// A sub set of [`syn::ExprClosure`].
#[derive(Clone)]
pub struct ImplClosure<Inputs = ImplClosurePatStruct> {
    pub or1_token: OrToken,
    pub inputs: Inputs,
    pub comma: Option<CommaToken>,
    pub or2_token: OrToken,
    // pub output: syn::ReturnType,
    pub body: Box<syn::Expr>,
}

impl<Inputs: Parse> Parse for ImplClosure<Inputs> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            or1_token: input.parse()?,
            inputs: input.parse()?,
            comma: input.parse()?,
            or2_token: input.parse()?,
            // output: input.parse()?,
            body: input.parse()?,
        })
    }
}

#[derive(Clone)]
pub struct FieldDeclarationMaybeDetailsImpl<K> {
    pub keyword: K,
    pub impl_closure: ImplClosure,
}

impl<K: Parse> Parse for FieldDeclarationMaybeDetailsImpl<K> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            keyword: input.parse()?,
            impl_closure: input.parse()?,
        })
    }
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
                    Some(name)
                } else {
                    None
                }
            },
        })
    }
}
