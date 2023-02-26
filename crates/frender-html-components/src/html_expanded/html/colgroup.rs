#[inline(always)]
pub fn colgroup() -> Building<TypesInitial> {
    super::HtmlTableColElementProps()
}
pub use super::col::{prelude, Building, Types, TypesInitial, ValidTypes};
pub struct ComponentType;
impl crate::imports::frender_html::props::IntrinsicComponent for ComponentType {
    const INTRINSIC_TAG: &'static ::core::primitive::str = "colgroup";
}
pub type Data<TypeDefs> = super::col::Data<TypeDefs, ComponentType>;
pub type DataInitial = Data<TypesInitial>;
#[inline(always)]
pub fn build<TypeDefs: ?::core::marker::Sized + Types>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    use super::*;
    super::col::Data(
        HtmlTableColElementProps::build(building),
        self::ComponentType,
    )
}
pub use build as build_element;
#[inline(always)]
pub fn valid<TypeDefs: ?::core::marker::Sized + ValidTypes>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    build(building)
}
