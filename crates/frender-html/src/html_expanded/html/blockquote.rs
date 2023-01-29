#[inline]
pub fn blockquote() -> Building<TypesInitial> {
    super::HtmlQuoteElementProps()
}
mod reuse {
    use super::super::*;
    pub use HtmlQuoteElementProps::{prelude, Building, Types, TypesInitial, ValidTypes};
}
pub use reuse::{prelude, Building, Types, TypesInitial, ValidTypes};
pub struct ComponentType;
impl crate::props::IntrinsicComponent for ComponentType {
    const INTRINSIC_TAG: &'static ::core::primitive::str = "blockquote";
}
mod struct_data {
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub struct blockquote<
        TypeDefs: ?::core::marker::Sized + HtmlQuoteElementProps::Types,
        ComponentType: crate::props::IntrinsicComponent = super::ComponentType,
    >(pub HtmlQuoteElementProps::Data<TypeDefs>, pub ComponentType);
}
pub use struct_data::blockquote as Data;
pub type DataInitial = Data<TypesInitial>;
#[inline]
pub fn build<TypeDefs: ?::core::marker::Sized + Types>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    use super::*;
    self::Data(HtmlQuoteElementProps::build(building), self::ComponentType)
}
pub use build as build_element;
#[inline]
pub fn valid<TypeDefs: ?::core::marker::Sized + ValidTypes>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    build(building)
}
