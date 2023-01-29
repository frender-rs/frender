#[inline]
pub fn area() -> Building<TypesInitial> {
    super::HtmlAreaElementProps()
}
mod reuse {
    use super::super::*;
    pub use HtmlAreaElementProps::{prelude, Building, Types, TypesInitial, ValidTypes};
}
pub use reuse::{prelude, Building, Types, TypesInitial, ValidTypes};
pub struct ComponentType;
impl crate::props::IntrinsicComponent for ComponentType {
    const INTRINSIC_TAG: &'static ::core::primitive::str = "area";
}
mod struct_data {
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub struct area<
        TypeDefs: ?::core::marker::Sized + HtmlAreaElementProps::Types,
        ComponentType: crate::props::IntrinsicComponent = super::ComponentType,
    >(pub HtmlAreaElementProps::Data<TypeDefs>, pub ComponentType);
}
pub use struct_data::area as Data;
pub type DataInitial = Data<TypesInitial>;
#[inline]
pub fn build<TypeDefs: ?::core::marker::Sized + Types>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    use super::*;
    self::Data(HtmlAreaElementProps::build(building), self::ComponentType)
}
pub use build as build_element;
#[inline]
pub fn valid<TypeDefs: ?::core::marker::Sized + ValidTypes>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    build(building)
}
