mod debug;
pub use debug::*;

pub mod intrinsic_components;

pub use forgotten;
pub use react;
pub use react::{AsKey, Children};
pub use react_html as html;

pub use frender_macros::{component, def_props, rsx};
pub use react::Element;

pub mod prelude {
    pub use super::html;
    pub use super::intrinsic_components;
    pub use super::react;
    pub use super::{component, def_props, rsx};

    pub mod rsx_runtime {
        pub use super::super::impl_rsx_prop;
        pub use react::Fragment;

        #[inline]
        pub fn init_props_builder<TComp: react::Component>(
        ) -> <TComp::Props as react::Props>::InitialBuilder {
            <TComp::Props as react::Props>::init_builder()
        }

        #[inline]
        pub fn impl_rsx_static<
            TComp: react::ComponentStatic,
            TBuilder: react::PropsBuilder<TComp::Props>,
        >(
            props_builder: TBuilder,
            key: Option<react::Key>,
        ) -> TComp::Element {
            let props = react::PropsBuilder::build(props_builder);
            TComp::create_element(props, key)
        }

        #[inline]
        pub fn impl_rsx<TComp: react::Component, TBuilder: react::PropsBuilder<TComp::Props>>(
            component: TComp,
            props_builder: TBuilder,
            key: Option<react::Key>,
        ) -> TComp::Element {
            let props = react::PropsBuilder::build(props_builder);
            component.create_element(props, key)
        }
    }
}

#[macro_export]
macro_rules! impl_rsx_prop {
    ($v:ident . $prop:ident ( $value:expr )) => {
        $v.$prop($value)
    };
}
