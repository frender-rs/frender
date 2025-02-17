#[derive(Debug, Clone, Copy)]
pub struct WithFn<F>(pub F);

mod csr {
    use frender_html::{CsrElement, RenderStateKind};

    use crate::fn_traits::{FnMut1, FnOnce1};

    use super::{
        super::{
            impl_IntoAsMutCsrElementWithValue_with_Self, AsMutCsrElementWithValue,
            IntoAsMutCsrElementWithValue,
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

mod ssr {
    use frender_ssr::html::assert::HtmlChildren;

    use crate::{fn_traits::FnMut1, SsrElement};

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
