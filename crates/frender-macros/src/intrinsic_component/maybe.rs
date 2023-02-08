use crate::utils::grouped::Braced;

use super::FieldDeclarationMaybeDetails;

#[derive(Clone)]
pub struct FieldDeclarationMaybe {
    pub question_token: syn::Token![?],
    pub by_ref: Option<syn::Token![&]>,
    pub ty: syn::Type,
    pub details: Option<Braced<FieldDeclarationMaybeDetails>>,
}
