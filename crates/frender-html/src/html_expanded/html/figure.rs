#[inline]
pub const fn figure() -> Building<TypesInitial> {
    super::HtmlElementProps()
}
pub use super::abbr::{prelude, Building, Types, TypesInitial, ValidTypes};
pub struct ComponentType;
impl crate::props::IntrinsicComponent for ComponentType {
    const INTRINSIC_TAG: &'static ::core::primitive::str = "figure";
}
pub type Data<TypeDefs> = super::abbr::Data<TypeDefs, ComponentType>;
pub type DataInitial = Data<TypesInitial>;
#[inline]
pub const fn build<TypeDefs: ?::core::marker::Sized + Types>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    use super::*;
    super::abbr::Data(HtmlElementProps::build(building), self::ComponentType)
}
pub use build as build_element;
#[inline]
pub fn valid<TypeDefs: ?::core::marker::Sized + ValidTypes>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    build(building)
}
