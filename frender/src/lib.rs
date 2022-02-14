mod debug;
pub use debug::*;

pub mod intrinsic_components;

pub mod into_prop_value;
pub use into_prop_value::*;

pub use forgotten;
pub use react;
// pub use react::{children, AsKey, Children, Component, Node, Props, PropsBuilder};

pub use frender_macros::{component, def_props, rsx};

pub mod prelude {
    pub use super::intrinsic_components;
    pub use super::react;
    pub use super::{component, def_props, rsx, rsx_runtime_impl_rsx_prop};

    pub mod rsx_runtime {
        pub use react::Fragment;

        #[inline]
        pub fn init_props_builder<TComp: react::Component>(
        ) -> <TComp::Props as react::Props>::InitialBuilder {
            <TComp::Props as react::Props>::init_builder()
        }

        #[inline]
        pub fn impl_rsx<TComp: react::Component, TBuilder: react::PropsBuilder<TComp::Props>>(
            props_builder: TBuilder,
            key: Option<&wasm_bindgen::JsValue>,
        ) -> react::sys::Element {
            let props = react::PropsBuilder::build(props_builder);
            let comp = TComp::new_with_props(props);
            comp.call_create_element(key)
        }
    }
}

#[macro_export]
macro_rules! rsx_runtime_impl_rsx_prop {
    ($v:ident . $prop:ident ( $value:expr )) => {
        $v.$prop($value)
    };
}
