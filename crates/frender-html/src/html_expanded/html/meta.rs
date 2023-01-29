#[inline]
pub fn meta() -> Building<TypesInitial> {
    super::HtmlMetaElementProps()
}
mod reuse {
    use super::super::*;
    pub use HtmlMetaElementProps::{prelude, Building, Types, TypesInitial, ValidTypes};
}
pub use reuse::{prelude, Building, Types, TypesInitial, ValidTypes};
pub struct ComponentType;
impl crate::props::IntrinsicComponent for ComponentType {
    const INTRINSIC_TAG: &'static ::core::primitive::str = "meta";
}
mod struct_data {
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub struct meta<
        TypeDefs: ?::core::marker::Sized + HtmlMetaElementProps::Types,
        ComponentType: crate::props::IntrinsicComponent = super::ComponentType,
    >(pub HtmlMetaElementProps::Data<TypeDefs>, pub ComponentType);
}
pub use struct_data::meta as Data;
pub type DataInitial = Data<TypesInitial>;
#[inline]
pub fn build<TypeDefs: ?::core::marker::Sized + Types>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    use super::*;
    self::Data(HtmlMetaElementProps::build(building), self::ComponentType)
}
pub use build as build_element;
#[inline]
pub fn valid<TypeDefs: ?::core::marker::Sized + ValidTypes>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    build(building)
}
