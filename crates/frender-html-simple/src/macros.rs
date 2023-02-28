#[macro_export]
macro_rules! def_props {
    (
        $($name:ident),* $(,)?
    ) => {$(
        #[derive(Debug, Clone, Copy)]
        pub struct $name<V>(pub V);
        impl<V> ::core::marker::Unpin for $name<V> {}
    )*};
}

#[macro_export]
macro_rules! inherit_props_from {
    (
        $($p:tt)+
    ) => {
        mod __inherited_props {
            use super::super::super::*;

            pub use $($p)+::props;
        }

        pub use __inherited_props::props::*;
    };
}
