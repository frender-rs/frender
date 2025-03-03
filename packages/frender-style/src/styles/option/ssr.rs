use crate::ssr::SsrStyle;

impl<T: SsrStyle> SsrStyle for Option<T> {
    type IntoSsrDeclarationList = Option<T::IntoSsrDeclarationList>;

    fn into_ssr_declaration_list(this: Self) -> Self::IntoSsrDeclarationList {
        this.map(T::into_ssr_declaration_list)
    }
}
