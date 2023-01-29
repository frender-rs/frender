#[inline]
pub fn link() -> Building<TypesInitial> {
    super::HtmlLinkElementProps()
}
mod reuse {
    use super::super::*;
    pub use HtmlLinkElementProps::{prelude, Building, Types, TypesInitial, ValidTypes};
}
pub use reuse::{prelude, Building, Types, TypesInitial, ValidTypes};
pub struct ComponentType;
impl crate::props::IntrinsicComponent for ComponentType {
    const INTRINSIC_TAG: &'static ::core::primitive::str = "link";
}
mod struct_data {
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub struct link<
        TypeDefs: ?::core::marker::Sized + HtmlLinkElementProps::Types,
        ComponentType: crate::props::IntrinsicComponent = super::ComponentType,
    >(pub HtmlLinkElementProps::Data<TypeDefs>, pub ComponentType);
}
pub use struct_data::link as Data;
pub type DataInitial = Data<TypesInitial>;
#[inline]
pub fn build<TypeDefs: ?::core::marker::Sized + Types>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    use super::*;
    self::Data(HtmlLinkElementProps::build(building), self::ComponentType)
}
pub use build as build_element;
#[inline]
pub fn valid<TypeDefs: ?::core::marker::Sized + ValidTypes>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    build(building)
}
