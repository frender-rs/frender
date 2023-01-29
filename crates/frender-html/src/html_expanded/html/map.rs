#[inline]
pub fn map() -> Building<TypesInitial> {
    super::HtmlMapElementProps()
}
mod reuse {
    use super::super::*;
    pub use HtmlMapElementProps::{prelude, Building, Types, TypesInitial, ValidTypes};
}
pub use reuse::{prelude, Building, Types, TypesInitial, ValidTypes};
pub struct ComponentType;
impl crate::props::IntrinsicComponent for ComponentType {
    const INTRINSIC_TAG: &'static ::core::primitive::str = "map";
}
mod struct_data {
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub struct map<
        TypeDefs: ?::core::marker::Sized + HtmlMapElementProps::Types,
        ComponentType: crate::props::IntrinsicComponent = super::ComponentType,
    >(pub HtmlMapElementProps::Data<TypeDefs>, pub ComponentType);
}
pub use struct_data::map as Data;
pub type DataInitial = Data<TypesInitial>;
#[inline]
pub fn build<TypeDefs: ?::core::marker::Sized + Types>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    use super::*;
    self::Data(HtmlMapElementProps::build(building), self::ComponentType)
}
pub use build as build_element;
#[inline]
pub fn valid<TypeDefs: ?::core::marker::Sized + ValidTypes>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    build(building)
}
