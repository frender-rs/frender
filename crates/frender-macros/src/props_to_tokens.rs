use std::borrow::Cow;

use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};
use syn::{
    parse_quote_spanned,
    punctuated::{Pair, Punctuated},
    spanned::Spanned,
};

use crate::err::OutputError;

use super::props_data::*;

impl PropsDefinitionWithOptions {
    pub fn into_tokens(self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            errors,
            options: PropsOptions {},
            definition:
                PropsDefinition {
                    vis,
                    attrs,
                    struct_token,
                    ident,
                    generics,
                    fields,
                },
        } = self;

        let span = ident.span();

        let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
        let tp = &generics.params;

        let fields = match fields.to_fields_impl() {
            Ok(v) => v,
            Err(err) => return tokens.append_all(err.into_compile_error()),
        };

        // struct MyProps { ... }
        let t_struct_props = {
            let props_struct_fields = fields
                .iter()
                .map(PropsFieldImpl::to_props_struct_field_tokens);

            quote! {
                #(#attrs)*
                #vis #struct_token #ident #type_generics #where_clause {
                    #(#props_struct_fields),*
                }
            }
        };

        let info = fields.iter().map(|f| f.to_info(&ident)).collect::<Vec<_>>();
        let (required, optional) = info
            .iter()
            .partition::<Vec<_>, _>(|v| v.builder_type_param.is_some());

        let fields_all_optional = required.is_empty();

        let builder_ident = if fields_all_optional {
            Cow::Borrowed(&ident)
        } else {
            Cow::Owned(quote::format_ident!("{}Builder", ident))
        };

        let builder_init = info.iter().map(|v| &v.builder_init);

        let (initial_builder_tps, t_impl_builder) = {
            if fields_all_optional {
                // all fields are optional
                // the MyProps struct can be the builder for itself
                let builder_fns = info.iter().map(
                    |PropsFieldInfo {
                         ident,
                         ty: _,
                         builder,
                         ..
                     }| {
                        let PropsFieldBuilder {
                            arrow,
                            fn_arg,
                            generics,
                            fn_block,
                            ..
                        } = builder.as_ref();
                        let (gp, _, where_clause) = generics.split_for_impl();
                        quote! {
                            #vis fn #ident #gp (mut self, #fn_arg) #arrow Self
                                #where_clause {
                                #![allow(unused_braces)]
                                self.#ident = #fn_block;
                                self
                            }
                        }
                    },
                );

                let t_impl_builder = quote! {
                    impl #impl_generics ::frender::react::PropsBuilder::<Self> for #ident #type_generics
                        #where_clause {
                        #[inline]
                        fn build(self) -> Self {
                            self
                        }
                    }
                    impl #impl_generics #builder_ident #type_generics #where_clause {
                        #(#builder_fns)*
                    }
                };

                (type_generics.to_token_stream(), t_impl_builder)
            } else {
                let (builder_tp, builder_valid_type): (Vec<_>, Vec<_>) = required
                    .iter()
                    .map(|v| {
                        let v = v.builder_type_param.as_ref().unwrap();
                        (&v.0, &v.1)
                    })
                    .unzip();
                let builder_tp_ts = builder_tp
                    .iter()
                    .map(|v| v.to_token_stream())
                    .collect::<Vec<_>>();
                let builder_tp_ts = builder_tp_ts.iter().collect::<Vec<_>>();
                let (
                    builder_impl_generics,
                    builder_type_params,
                    builder_valid_type_params,
                    type_params_with_comma,
                ) = {
                    let tp_comma = if tp.empty_or_trailing() {
                        None
                    } else {
                        Some(syn::Token![,](span))
                    };

                    let type_params =
                        Punctuated::<_, syn::Token![,]>::from_iter(tp.pairs().map(|p| {
                            let (gp, c) = p.into_tuple();
                            let ident_or_lt = match gp {
                                syn::GenericParam::Type(t) => t.ident.to_token_stream(),
                                syn::GenericParam::Lifetime(lt) => lt.lifetime.to_token_stream(),
                                syn::GenericParam::Const(c) => c.ident.to_token_stream(),
                            };
                            Pair::new(ident_or_lt, c)
                        }));

                    let type_params_with_comma = quote!(#type_params #tp_comma);

                    (
                        quote!(<#tp #tp_comma #(#builder_tp),*>),
                        quote!(<#type_params_with_comma #(#builder_tp),*>),
                        quote!(<#type_params_with_comma #(#builder_valid_type),*>),
                        type_params_with_comma,
                    )
                };

                let not_set = quote!(::frender::react::__private::required_prop::PropertyNotSet);
                let not_set = (0..(required.len())).map(|_| &not_set);
                let initial_builder_tps = quote!(
                    <#type_params_with_comma #(#not_set),*>
                );
                let required_field_defs = required.iter().map(|v| {
                    let ident = v.ident;
                    let ty = &v.builder_type_param.as_ref().unwrap().0;
                    quote!(#ident : #ty)
                });
                let optional_field_defs = optional.iter().map(|v| {
                    let ident = v.ident;
                    let ty = v.ty;
                    quote!(#ident : #ty)
                });

                let required_field_set = required
                    .iter()
                    .map(|PropsFieldInfo { ident, .. }| quote!( #ident : self.#ident.0 ));
                let required_field_set = quote!(#(#required_field_set),*);

                let optional_field_set = optional
                    .iter()
                    .map(|PropsFieldInfo { ident, .. }| quote!( #ident : self.#ident ));
                let optional_field_set = quote!(#(#optional_field_set),*);

                let required_field_set_builder = required
                    .iter()
                    .map(|PropsFieldInfo { ident, .. }| quote!( #ident : self.#ident ))
                    .collect::<Vec<_>>();
                let required_field_set_wrapped = quote!(
                    ::frender::react::__private::required_prop::PropertyToBeSet(value)
                );

                let required_field_fns = required.iter().enumerate().map(|(idx, v)| {
                    let PropsFieldInfo {
                        ident, ty, builder, ..
                    } = v;

                    let required_field_set_wrapped = quote!(#ident : #required_field_set_wrapped);
                    let mut field_set = required_field_set_builder.iter().collect::<Vec<_>>();
                    field_set[idx] = &required_field_set_wrapped;

                    let PropsFieldBuilder {
                        arrow,
                        fn_arg,
                        generics,
                        fn_block,
                        ..
                    } = builder.as_ref();
                    let (gp, _, where_clause) = generics.split_for_impl();

                    let tp_new = {
                        let mut tp_new = builder_tp_ts.clone();
                        tp_new[idx] = builder_valid_type[idx];
                        tp_new
                    };

                    quote! {
                        #vis fn #ident #gp (self, #fn_arg ) #arrow #builder_ident < #type_params_with_comma #(#tp_new),* >
                            #where_clause {
                                // #![allow(unused_braces)]
                                let value: #ty = #fn_block;
                                #builder_ident {
                                    #(#field_set),*
                                    ,
                                    #optional_field_set
                                }
                        }
                    }
                });

                let optional_field_fns = optional.iter().map(|v| {
                    let PropsFieldInfo { ident, builder, .. } = v;
                    let PropsFieldBuilder {
                        arrow,
                        fn_arg,
                        generics,
                        fn_block,
                        ..
                    } = builder.as_ref();
                    let (gp, _, where_clause) = generics.split_for_impl();

                    quote! {
                        #vis fn #ident #gp (mut self, #fn_arg ) #arrow Self
                            #where_clause {
                                #![allow(unused_braces)]
                                self.#ident = #fn_block;
                                self
                        }
                    }
                });

                let t_impl_builder = quote_spanned! {span=>
                    #[allow(non_camel_case_types)]
                    #vis #struct_token #builder_ident #builder_impl_generics #where_clause {
                        #(#required_field_defs),*
                        ,
                        #(#optional_field_defs),*
                    }

                    impl #impl_generics ::frender::react::PropsBuilder::<#ident #type_generics> for
                        #builder_ident #builder_valid_type_params
                        #where_clause {
                        fn build(self) -> #ident #type_generics {
                            #ident {
                                #required_field_set,
                                #optional_field_set
                            }
                        }
                    }

                    #[allow(non_camel_case_types)]
                    impl #builder_impl_generics #builder_ident #builder_type_params #where_clause {
                        #(#required_field_fns)*
                        #(#optional_field_fns)*
                    }
                };

                (initial_builder_tps, t_impl_builder)
            }
        };

        let t_impl_props = quote_spanned! {span=>
            impl ::frender::react::Props for #ident {
                type InitialBuilder = #builder_ident #initial_builder_tps;

                fn init_builder() -> Self::InitialBuilder {
                    Self::InitialBuilder {
                        #(#builder_init),*
                    }
                }
            }
        };

        tokens.append_all(t_struct_props);
        tokens.append_all(t_impl_props);
        tokens.append_all(t_impl_builder);

        if let Some(error) = errors.output_error() {
            tokens.append_all(error.write_errors());
        }
    }
}

impl PropsFields {
    fn to_fields_impl(&self) -> syn::Result<Vec<PropsFieldImpl>> {
        let fields = self.named.iter().map(|f| f.to_field_impl());

        let mut res = Ok(Vec::with_capacity(fields.len()));

        for f in fields {
            match &mut res {
                Ok(vec) => match f {
                    Ok(f) => {
                        vec.push(f);
                    }
                    Err(err) => res = Err(err),
                },
                Err(err) => {
                    if let Err(e) = f {
                        err.combine(e);
                    }
                }
            }
        }

        res
    }
}

struct PropsFieldImpl<'a> {
    /// Attributes tagged on the field.
    pub attrs: &'a Vec<syn::Attribute>,

    /// Visibility of the field.
    pub vis: &'a syn::Visibility,

    /// Name of the field
    pub ident: &'a syn::Ident,
    // whether the field is optional
    pub question: &'a Option<syn::Token![?]>,
    pub colon: Cow<'a, syn::Token![:]>,
    pub ty: Cow<'a, syn::Type>,
    pub builder: Cow<'a, PropsFieldBuilder>,
}

struct PropsFieldInfo<'a> {
    ident: &'a syn::Ident,
    ty: &'a Cow<'a, syn::Type>,
    /// If `None`, this field is optional
    ///
    /// ```ignore
    /// struct MyBuilder<MyBuilder__name> {
    ///     //           =============== ident
    ///     name: MyBuilder__name
    /// }
    /// ```
    ///
    /// ```ignore
    /// impl PropsBuilder<MyProps> for MyBuilder<PropertyToBeSet<String>> {
    ///     //                                   ======================= type
    /// }
    /// ```
    builder_type_param: Option<(syn::Ident, proc_macro2::TokenStream)>,
    /// ```ignore
    /// # MyBuilder {
    ///     optional_prop: Default::default(),
    ///     required_prop: PropertyNotSet
    /// # }
    /// ```
    builder_init: proc_macro2::TokenStream,
    builder: &'a Cow<'a, PropsFieldBuilder>,
}

impl<'a> PropsFieldImpl<'a> {
    #[inline]
    pub fn as_required_field(&self) -> Option<&Self> {
        if self.is_optional() {
            None
        } else {
            Some(self)
        }
    }

    #[inline]
    pub fn is_optional(&self) -> bool {
        self.question.is_some()
    }

    #[inline]
    pub fn is_required(&self) -> bool {
        self.question.is_none()
    }

    pub fn to_info(&self, props_ident: &syn::Ident) -> PropsFieldInfo {
        let PropsFieldImpl {
            // vis,
            ident,
            // question,
            colon,
            ty,
            builder,
            ..
        } = self;
        if self.is_optional() {
            PropsFieldInfo {
                ident,
                ty,
                builder_init: quote!( #ident #colon Default::default() ),
                builder_type_param: None,
                builder,
            }
        } else {
            let span = ident.span();
            let tp_ident = quote::format_ident!("{}Builder__{}", props_ident, ident, span = span);
            let builder_init = quote_spanned! {span=>
                    #ident #colon ::frender::react::__private::required_prop::PropertyNotSet
            };

            let valid_type = quote_spanned! {span=>
                ::frender::react::__private::required_prop::PropertyToBeSet::< #ty >
            };
            let builder_type_param = (tp_ident, valid_type);
            PropsFieldInfo {
                ident,
                ty,
                builder_type_param: Some(builder_type_param),
                builder_init,
                builder,
            }
        }
    }

    pub fn to_props_struct_field_tokens(&self) -> proc_macro2::TokenStream {
        let PropsFieldImpl {
            attrs,
            vis,
            ident,
            // question,
            colon,
            ty,
            // builder,
            ..
        } = self;
        quote! {
            #(#attrs)*
            #vis #ident #colon #ty
        }
    }
}

impl PropsField {
    fn to_field_impl(&self) -> syn::Result<PropsFieldImpl<'_>> {
        let span = self.ident.span();
        let (colon, ty, builder) = match &self.type_and_builder {
            PropsFieldTypeAndBuilder::ImplicitBuilder { colon, ty } => (
                Cow::Borrowed(colon),
                Cow::Borrowed(ty),
                Cow::Owned(PropsFieldBuilder::new(span, ty.clone())),
            ),
            PropsFieldTypeAndBuilder::Explicit(ret) => (
                Cow::Owned(syn::Token![:](span)),
                Cow::Borrowed(&ret.ty),
                Cow::Borrowed(&ret.builder),
            ),
            PropsFieldTypeAndBuilder::ImplicitTypeAndBuilder => {
                let ret = infer_field_type_and_builder(&self.ident)?;

                (
                    Cow::Owned(syn::Token![:](span)),
                    Cow::Owned(ret.ty),
                    Cow::Owned(ret.builder),
                )
            }
        };

        Ok(PropsFieldImpl {
            attrs: &self.attrs,
            vis: &self.vis,
            ident: &self.ident,
            question: &self.question,
            colon,
            ty,
            builder,
        })
    }
}

fn infer_field_type_and_builder(
    ident: &syn::Ident,
) -> syn::Result<PropsFieldTypeAndBuilderExplicit> {
    let span = ident.span();
    let name = ident.to_string();
    let name = name.as_str();
    let ret = match name {
        "children" => parse_quote_spanned! {span=>
            <TNode: ::frender::react::Node>(node: Option<TNode>) -> Option<::frender::react::Children> {
                node.and_then(::frender::react::Node::into_children)
            }
        },
        "style" => {
            let ty: syn::TypePath = parse_quote_spanned!(span=>
                Option<::frender::react::CssProperties>);
            let ty = syn::Type::Path(ty);
            PropsFieldTypeAndBuilderExplicit {
                builder: PropsFieldBuilder::new(span, ty.clone()),
                ty,
            }
        }
        "id" | "class_name" | "class" => {
            let ty: syn::TypePath = parse_quote_spanned!(span=>
                Option<String>);
            let ty = syn::Type::Path(ty);
            PropsFieldTypeAndBuilderExplicit {
                builder: PropsFieldBuilder::new(span, ty.clone()),
                ty,
            }
        }
        _ => {
            return Err(syn::Error::new(
                span,
                format!("Can't infer type for prop {name}. Please specify the field type."),
            ))
        }
    };

    Ok(ret)
}
