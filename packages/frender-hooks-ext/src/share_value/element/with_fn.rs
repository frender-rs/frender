#[derive(Debug, Clone, Copy)]
pub struct WithFn<F>(pub F);

#[cfg(feature = "csr")]
mod csr {
    use frender_csr::{CsrElement, RenderStateKind};

    use frender_fn_traits::{FnMut1, FnOnce1};

    use super::{
        super::{
            AsMutCsrElementWithValue, IntoAsMutCsrElementWithValue,
            impl_IntoAsMutCsrElementWithValue_with_Self,
        },
        WithFn,
    };

    impl<V, F, K> AsMutCsrElementWithValue<V> for WithFn<F>
    where
        V: ?Sized,
        F: for<'a> FnMut1<&'a V, Output: CsrElement<RenderStateKind = K>>,
        K: RenderStateKind,
    {
        type ElementWithValue<'a>
            = <F as FnOnce1<&'a V>>::Output_
        where
            Self: 'a,
            V: 'a;

        type ElementWithValueRenderStateKind = K;

        fn as_mut_csr_element_with_value<'a>(
            &'a mut self,
            value: &'a V,
        ) -> Self::ElementWithValue<'a> {
            (self.0)(value)
        }
    }

    impl<V, F, K> IntoAsMutCsrElementWithValue<V> for WithFn<F>
    where
        V: ?Sized,
        F: for<'a> FnMut1<&'a V, Output: CsrElement<RenderStateKind = K>>,
        K: RenderStateKind,
    {
        impl_IntoAsMutCsrElementWithValue_with_Self! {
            type Value = V;
        }
    }
}

#[cfg(feature = "ssr")]
mod ssr {
    use frender_fn_traits::FnMut1;
    use frender_ssr::{SsrElement, html::assert::HtmlChildren};

    use super::{super::IntoHtmlChildrenWithValue, WithFn};

    impl<V, F, HC> IntoHtmlChildrenWithValue<V> for WithFn<F>
    where
        V: ?Sized,
        F: for<'a> FnMut1<&'a V, Output: SsrElement<HtmlChildren = HC>>,
        HC: HtmlChildren,
    {
        type HtmlChildrenWithValue = HC;

        fn into_html_children_with_value(mut self, value: &V) -> Self::HtmlChildrenWithValue {
            (self.0)(value).into_html_children()
        }
    }
}
