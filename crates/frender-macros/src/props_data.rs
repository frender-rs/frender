use proc_macro2::Span;
use syn::{
    parse::Parse,
    punctuated::{Pair, Punctuated},
    spanned::Spanned,
};

pub struct PropsDefinition {
    pub attrs: Vec<syn::Attribute>,
    pub vis: syn::Visibility,
    pub struct_token: syn::Token![struct],
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub fields: PropsFields,
    // pub semi_token: Option<Token![;]>,
}

impl Parse for PropsDefinition {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut attrs = input.call(syn::Attribute::parse_outer)?;
        let vis = input.parse::<syn::Visibility>()?;
        let struct_token = input.parse::<syn::Token![struct]>()?;
        let ident = input.parse::<syn::Ident>()?;
        let generics = input.parse::<syn::Generics>()?;
        let (where_clause, fields) = data_struct(input, &mut attrs)?;
        Ok(PropsDefinition {
            attrs,
            vis,
            struct_token,
            ident,
            generics: syn::Generics {
                where_clause,
                ..generics
            },
            fields,
        })
    }
}

pub fn data_struct(
    input: syn::parse::ParseStream,
    attrs: &mut Vec<syn::Attribute>,
) -> syn::Result<(Option<syn::WhereClause>, PropsFields)> {
    let where_clause: Option<syn::WhereClause> = input.parse()?;

    if input.peek(syn::token::Brace) {
        let fields = parse_braced(input, attrs)?;
        Ok((where_clause, fields))
    } else {
        Err(syn::Error::new(
            input.span(),
            "Expect braced fields { }. Props can only be a struct with named fields.",
        ))
    }
}

pub fn parse_braced(
    input: syn::parse::ParseStream,
    attrs: &mut Vec<syn::Attribute>,
) -> syn::Result<PropsFields> {
    let content;
    let brace_token = syn::braced!(content in input);
    attrs.extend(syn::Attribute::parse_inner(&content)?);
    let named = content.parse_terminated(PropsField::parse_named)?;
    Ok(PropsFields { brace_token, named })
}

pub struct PropsFields {
    pub brace_token: syn::token::Brace,
    pub named: Punctuated<PropsField, syn::Token![,]>,
}

pub struct PropsField {
    /// Attributes tagged on the field.
    pub attrs: Vec<syn::Attribute>,

    /// Visibility of the field.
    pub vis: syn::Visibility,

    /// Name of the field
    pub ident: syn::Ident,
    // whether the field is optional
    pub question: Option<syn::Token![?]>,
    pub type_and_builder: PropsFieldTypeAndBuilder,
}

impl PropsField {
    pub fn parse_named(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attrs: input.call(syn::Attribute::parse_outer)?,
            vis: input.parse()?,
            ident: input.parse()?,
            question: input.parse()?,
            type_and_builder: input.parse()?,
        })
    }
}

pub enum PropsFieldTypeAndBuilder {
    /// ```text
    /// name: String
    /// ```
    ///
    /// or
    ///
    /// ```text
    /// name?: String
    /// ```
    ImplicitBuilder {
        colon: syn::Token![:],
        ty: syn::Type,
    },
    /// ```text
    /// children? <TNode: Node>(value: TNode) -> Children
    ///     where TNode: Debug {
    ///     value.into_react_children_js()
    /// }
    /// ```
    Explicit(PropsFieldTypeAndBuilderExplicit),
    /// `children` or `children?`
    ImplicitTypeAndBuilder,
}

/// See [PropsFieldTypeAndBuilder::Explicit]
pub struct PropsFieldTypeAndBuilderExplicit {
    /// fn return type. This will be the field type
    pub ty: syn::Type,
    pub builder: PropsFieldBuilder,
}

#[derive(Clone)]
pub struct PropsFieldBuilder {
    pub generics: syn::Generics,
    pub paren: syn::token::Paren,
    pub fn_arg: Pair<syn::PatType, syn::Token![,]>,
    pub arrow: syn::Token![->],
    /// builder fn block
    pub fn_block: syn::Block,
}

impl PropsFieldBuilder {
    pub fn new(span: Span, arg_ty: syn::Type) -> Self {
        let id = syn::Ident::new("prop_value", span);
        let fn_arg = syn::PatType {
            attrs: vec![],
            pat: Box::new(syn::Pat::Ident(syn::PatIdent {
                attrs: vec![],
                by_ref: None,
                mutability: None,
                ident: id.clone(),
                subpat: None,
            })),
            colon_token: syn::Token![:](span),
            ty: Box::new(arg_ty),
        };
        let fn_arg: Pair<syn::PatType, syn::Token![,]> = Pair::End(fn_arg);
        let paren = syn::token::Paren { span };
        Self {
            generics: syn::Generics::default(),
            paren,
            fn_arg,
            arrow: syn::Token![->](span),
            fn_block: syn::Block {
                brace_token: syn::token::Brace { span },
                stmts: vec![syn::Stmt::Expr(syn::Expr::Path(syn::ExprPath {
                    attrs: vec![],
                    path: syn::Path {
                        leading_colon: None,
                        segments: Punctuated::from_iter([syn::PathSegment::from(id)]),
                    },
                    qself: None,
                }))],
            },
        }
    }
}

impl Parse for PropsFieldTypeAndBuilder {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let colon: Option<syn::Token![:]> = input.parse()?;
        if let Some(colon) = colon {
            let ty = input.parse()?;
            Ok(Self::ImplicitBuilder { colon, ty })
        } else if input.peek(syn::Token![<]) || input.peek(syn::token::Paren) {
            input.parse().map(Self::Explicit)
        } else {
            Ok(Self::ImplicitTypeAndBuilder)
        }
    }
}

impl Parse for PropsFieldTypeAndBuilderExplicit {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let generics = input.parse::<syn::Generics>()?;
        let args;
        let paren = syn::parenthesized!(args in input);

        let mut args: Punctuated<_, syn::Token![,]> = args.parse_terminated(syn::FnArg::parse)?;

        if args.len() != 1 {
            return Err(syn::Error::new(
                paren.span,
                "Props field's custom builder fn need exactly one argument",
            ));
        }
        let fn_arg = args.pop().unwrap();

        let (fn_arg, comma) = fn_arg.into_tuple();

        let fn_arg = match fn_arg {
            syn::FnArg::Receiver(r) => {
                return Err(syn::Error::new(
                    r.span(),
                    "Props field's custom builder fn can't have `self` argument",
                ))
            }
            syn::FnArg::Typed(fn_arg) => Pair::new(fn_arg, comma),
        };

        let arrow = input.parse()?;
        let ty = input.parse()?;
        let fn_block = input.parse()?;

        Ok(Self {
            ty,
            builder: PropsFieldBuilder {
                generics,
                paren,
                fn_arg,
                arrow,
                fn_block,
            },
        })
    }
}
