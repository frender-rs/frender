use std::marker::PhantomData;
use wasm_bindgen::JsValue;

/// An element that is building,
/// which might accept children later.
pub trait ElementBuilding {
    type ComponentType: react::Component;
    type Builder;

    fn unwrap_building_data(self) -> ElementBuildingData<Self::ComponentType, Self::Builder>;
}

pub struct ElementBuildingData<TComp: react::Component, TBuilder> {
    _comp: PhantomData<TComp>,
    pub props_builder: TBuilder,
    pub key: Option<JsValue>,
}

impl<TComp: react::Component, TBuilder> ElementBuildingData<TComp, TBuilder> {
    pub fn new(props_builder: TBuilder, key: Option<JsValue>) -> Self {
        Self {
            _comp: PhantomData,
            props_builder,
            key,
        }
    }
}

impl<TComp: react::Component, TBuilder> ElementBuilding for ElementBuildingData<TComp, TBuilder> {
    type ComponentType = TComp;
    type Builder = TBuilder;

    #[inline]
    fn unwrap_building_data(self) -> ElementBuildingData<Self::ComponentType, Self::Builder> {
        self
    }
}
