use crate::SsrElement;

frender_common::impl_many!(
    impl<__> SsrElement
        for each_of![
            i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64, char,
        ]
    {
        type HtmlChildren = frender_ssr_html::scalar::Scalar;

        fn into_html_children(self) -> Self::HtmlChildren {
            Self::HtmlChildren::new(self)
        }
    }
);
