pub(crate) mod rel_list {
    use web_sys::{DomTokenList, HtmlAnchorElement, HtmlAreaElement, HtmlLinkElement};

    pub(crate) trait RelList {
        fn rel_list(&self) -> DomTokenList;
    }

    macro_rules! impl_many {
        (
            impl<__> $trait_name:ident for each_of![$($ty:ty),* $(,)?]
            $block:tt
        ) => {$(
            impl $trait_name for $ty
            $block
        )*};
    }

    impl_many!(
        impl<__> RelList
            for each_of![
                HtmlAnchorElement,
                HtmlAreaElement,
                HtmlLinkElement,
                // SvgaElement
            ]
        {
            #[inline(always)]
            fn rel_list(&self) -> DomTokenList {
                self.rel_list()
            }
        }
    );
}
