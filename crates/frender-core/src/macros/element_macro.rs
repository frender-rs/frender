#[macro_export]
macro_rules! element {
    (
        $($t:tt)*
    ) => {
        $crate::bg::finish_builder_with!(
            [build_element]
            $($t)*
        )
    };
}
