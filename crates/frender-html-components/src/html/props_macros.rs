macro_rules! def_props_fully_and_simply {
    (
        $vis:vis $fully_name:ident $simply_name:ident
        $($rest:tt)*
    ) => {
        $vis mod $fully_name {
            frender_macros::def_intrinsic_component_props! {
                $($rest:tt)*
            }
        }
        $vis mod $simply_name {
            frender_macros::def_intrinsic_component_props! {
                ~simply_typed
                $($rest:tt)*
            }
        }
    };
}
