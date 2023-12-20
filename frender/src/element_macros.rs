#[macro_export]
macro_rules! __expand_or {
    ([] $($or:tt)*) => { $($or)* };
    ([$($expand:tt)+] $($or:tt)*) => { $($expand)+ };
}

#[macro_export]
macro_rules! __expand_base_expr {
    ([] $($p:tt)*) => {
        $($p)* ()
    };
    ([$($base:tt)+] $($p:tt)*) => {
        $($p)* ::Building ( $($base)+ )
    };
}

#[macro_export]
macro_rules! __impl_element_one {
    ($lit:literal) => {
        $lit
    };
    ({$($non_builder:tt)*}) => {
        $($non_builder)*
    };
    (
        $($component_path_start:ident)?
        $(:: $component_path:ident)*
        $(($($base:tt)*))?
        $( . $method:ident $(($($method_arg:tt)*))? )*
    ) => {{
        #[allow(unused_imports)]
        use $($component_path_start)? $(:: $component_path)* ::prelude::*;
        $($component_path_start)? $(:: $component_path)* ::build_element(
            $crate::__expand_base_expr!([$($($base)*)?] $($component_path_start)? $(:: $component_path)* )
                $(
                    . $method(
                        $crate::__expand_or!([$($($method_arg)*)?] $crate::omitted::Omitted)
                    )
                )*
        )
    }};
}

#[macro_export]
macro_rules! element {
    (
        $first:tt // may be `{}` `::` `div` `$literal`
        $($component_path_start:ident)?
        $(:: $component_path:ident)*
        $(($($base:tt)*))?
        $( . $method:ident $(($($method_arg:tt)*))? )*
        $([[$($children:tt)*]])?
        $(,)?
    ) => {
        $crate::__impl_element_one! {
            $first
            $($component_path_start)?
            $(:: $component_path)*
            $(($($base)*))?
            $( . $method $(($($method_arg)*))? )*
            $(.children($crate::element![$($children)*]))?
        }
    };
    (
        $(
            $first:tt
            $($component_path_start:ident)?
            $(:: $component_path:ident)*
            $(($($base:tt)*))?
            $( . $method:ident $(($($method_arg:tt)*))? )*
            $([[$($children:tt)*]])?
        ),+ $(,)?
    ) => {
        ($(
            $crate::__impl_element_one! {
                $first
                $($component_path_start)?
                $(:: $component_path)*
                $(($($base)*))?
                $( . $method $(($($method_arg)*))? )*
                $(.children($crate::element![$($children)*]))?
            }
            ,
        )+)
    };
}

#[macro_export]
macro_rules! __impl_intrinsic_one {
    ($lit:literal) => {
        $lit
    };
    ({$($non_builder:tt)*}) => {
        $($non_builder)*
    };
    (
        $intrinsic_component:ident
        $($rest:tt)*
    ) => {
        $crate::__impl_element_one! {
            intrinsic_components::$intrinsic_component
            $($rest)*
        }
    };
}

#[macro_export]
macro_rules! intrinsic {
    (
        $first:tt // may be `{}` `div` `$literal`
        $(($($inner:tt)*))?
        $( . $method:ident $(($($method_arg:tt)*))? )*
        $([[$($children:tt)*]])?
        $(,)?
    ) => {
        $crate::__impl_intrinsic_one! {
            $first
            $(($($inner)*))?
            $( . $method $(($($method_arg)*))? )*
            $(.children($crate::intrinsic![$($children)*]))?
        }
    };
    (
        $(
            $first:tt // may be `{}` `div` `$literal`
            $(($($inner:tt)*))?
            $( . $method:ident $(($($method_arg:tt)*))? )*
            $([[$($children:tt)*]])?
        ),+ $(,)?
    ) => {
        $crate::__impl_intrinsic_many! {$({
            $crate::__impl_intrinsic_one! {
                $first
                $(($($inner)*))?
                $( . $method $(($($method_arg)*))? )*
                $(.children($crate::intrinsic![$($children)*]))?
            }
        })+}
    };
}

#[macro_export]
macro_rules! __impl_intrinsic_many {
    ({ $t0:expr } { $t1:expr } { $t2:expr } { $t3:expr } { $t4:expr } { $t5:expr } { $t6:expr } { $t7:expr } { $t8:expr } { $t9:expr } { $t10:expr } { $t11:expr } $($t:tt)+) => {
        (
            (
                $t0, $t1, $t2, $t3, $t4, $t5, $t6, $t7, $t8, $t9, $t10, $t11,
            ),
            $crate::__impl_intrinsic_many! { $($t)+ },
        )
    };
    ($({ $t0:expr })+)=>{
        ($($t0,)+)
    };
}

#[cfg(test)]
mod tests {

    mod intrinsic_components {
        pub mod num {
            pub mod prelude {}
            pub fn build_element(v: i32) -> i32 {
                v
            }

            #[allow(non_snake_case)]
            pub fn Building(v: i32) -> i32 {
                v
            }
        }

        pub fn num() -> i32 {
            0
        }
    }

    #[test]
    fn one_element() {
        let a = element!(intrinsic_components::num());
        let b = intrinsic!(num(2).wrapping_sub(2));
        assert_eq!(a, b);
    }

    #[test]
    fn elements() {
        use intrinsic_components::num as MyZero;
        let a = element!(
            //
            intrinsic_components::num().wrapping_add(99).wrapping_pow(0),
            MyZero(1),
            { "hello" },
        );
        let b = intrinsic!(
            //
            num().saturating_add(1),
            num(2).wrapping_div(2),
            { "hello" }
        );
        assert_eq!(a, b);
    }
}
