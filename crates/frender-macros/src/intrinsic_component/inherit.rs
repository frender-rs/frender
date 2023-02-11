use darling::ToTokens;
use proc_macro2::TokenStream;
use quote::quote;

use super::Field;

#[derive(Clone)]
pub struct FieldDeclarationInherit {
    pub from_path: syn::Path,
    pub dom_element_ty: syn::Type,
    pub fields: Vec<Field>,
}

impl FieldDeclarationInherit {
    pub fn field_names(&self) -> impl Iterator<Item = &syn::Ident> {
        self.fields.iter().map(|f| &f.name)
    }

    pub fn make_overwrite_type_items<'a>(
        from_path: &'a syn::Path,
        inherited_fields: &'a Vec<Field>,
        inherit_on_field: &'a syn::Ident,
    ) -> impl Iterator<Item = TokenStream> + 'a {
        inherited_fields.iter().map(move |field| {
            let sub_fields = field.declaration.as_inherit().map(|f| {
                Self::make_overwrite_type_items(from_path, &f.fields, inherit_on_field)
                    .collect::<TokenStream>()
            });

            let field_name = &field.name;
            quote! {
                pub type #field_name <TypeDefs, Value> = self:: #inherit_on_field <
                    TypeDefs,
                    super::super:: #from_path ::overwrite:: #field_name <
                        <TypeDefs as super::Types>:: #inherit_on_field,
                        Value,
                    >,
                >;
                #sub_fields
            }
        })
    }

    pub fn make_builder_fns(
        crate_path: &impl ToTokens,
        from_path: &syn::Path,
        inherited_fields: &Vec<Field>,
        this_field_name: &syn::Ident,
        all_fields: &Vec<&syn::Ident>,
    ) -> TokenStream {
        // }

        // pub fn to_builder_fns(
        //     &self,
        //     crate_path: &impl ToTokens,
        //     this_field_name: &syn::Ident,
        //     all_fields: &Vec<&syn::Ident>,
        // ) -> TokenStream {
        //     let FieldDeclarationInherit {
        //         from_path,
        //         dom_element_ty,
        //         fields,
        //     } = self;

        inherited_fields
            .iter()
            .map(|field| {
                if let Some(inherit) = field.declaration.as_inherit() {
                    return Self::make_builder_fns(
                        crate_path,
                        from_path,
                        &inherit.fields,
                        this_field_name,
                        all_fields,
                    );
                }

                let name = &field.name;
                let field_bounds = field
                    .declaration
                    .bounds(crate_path)
                    .map(|bounds| quote!(: #bounds));

                let doc = format!(
                    "See [`{}::{}`]",
                    from_path.to_token_stream().to_string(),
                    name,
                );

                let new_value = all_fields.iter().map(|field| {
                    if *field == this_field_name {
                        quote! {
                            #field: #from_path ::build(
                                #from_path ::Building(self.0. #field). #name(#name),
                            )
                        }
                    } else {
                        quote!(#field: self.0.#field)
                    }
                });

                quote! {
                    #[doc = #doc]
                    #[inline(always)]
                    pub const fn #name <V #field_bounds>(
                        self,
                        #name: V,
                    ) -> super::Building<
                        super::overwrite::#name<TypeDefs, V>,
                    > {
                        super::Building(
                            super::Data {
                                #(#new_value),*
                            }
                        )
                    }
                }
            })
            .collect()
    }
}
