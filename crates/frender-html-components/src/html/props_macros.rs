macro_rules! def_props_fully_and_simply {
    (
        $vis_full:vis   full($full_mod:ident   $full_feature:literal)
        $vis_simple:vis simple($simple_mod:ident $simple_feature:literal)
        $($rest:tt)*
    ) => {
        #[cfg(feature = $full_feature)]
        $vis_full mod $full_mod {
            #[allow(unused_imports)]
            use super::*;

            ::frender_macros::def_intrinsic_component_props! {
                $($rest)*
            }
        }

        #[cfg(feature = $simple_feature)]
        $vis_simple mod $simple_mod {
            #[allow(unused_imports)]
            use super::*;

            ::frender_macros::def_intrinsic_component_props! {
                ~simply_typed
                $($rest)*
            }
        }
    };
}

pub(super) use def_props_fully_and_simply;
