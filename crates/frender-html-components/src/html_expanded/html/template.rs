#[inline(always)]
pub fn template() -> Building<TypesInitial> {
    super::HtmlElementProps()
}
pub use super::abbr::{prelude, Building, Types, TypesInitial, ValidTypes};
pub struct ComponentType;
impl crate::imports::frender_html::props::IntrinsicComponent for ComponentType {
    const INTRINSIC_TAG: &'static ::core::primitive::str = "template";
}
pub type Data<TypeDefs> = super::abbr::Data<TypeDefs, ComponentType>;
pub type DataInitial = Data<TypesInitial>;
#[inline(always)]
pub fn build<TypeDefs: ?::core::marker::Sized + Types>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    use super::*;
    super::abbr::Data(HtmlElementProps::build(building), self::ComponentType)
}
pub use build as build_element;
#[inline(always)]
pub fn valid<TypeDefs: ?::core::marker::Sized + ValidTypes>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    build(building)
}
