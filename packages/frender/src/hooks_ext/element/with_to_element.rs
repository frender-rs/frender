#[derive(Debug, Clone, Copy)]
pub struct WithToElement;

mod ssr {
    use frender_ssr::{html::assert::HtmlChildren, SsrElement};

    use crate::ToElement;

    use super::{super::IntoHtmlChildrenWithValue, WithToElement};

    impl<
            V: ?Sized + for<'a> ToElement<ToElement<'a>: SsrElement<HtmlChildren = HC>>,
            HC: HtmlChildren,
        > IntoHtmlChildrenWithValue<V> for WithToElement
    {
        type HtmlChildrenWithValue = HC;

        fn into_html_children_with_value(self, value: &V) -> Self::HtmlChildrenWithValue {
            use frender_ssr::SsrElement as _;
            value.to_element().into_html_children()
        }
    }
}

mod csr {
    use frender_html::{CsrElement, RenderStateKind};

    use crate::ToElement;

    use super::{
        super::{
            impl_IntoAsMutCsrElementWithValue_with_Self, AsMutCsrElementWithValue,
            IntoAsMutCsrElementWithValue,
        },
        WithToElement,
    };

    impl<
            V: ?Sized + for<'a> ToElement<ToElement<'a>: CsrElement<RenderStateKind = K>>,
            K: RenderStateKind,
        > AsMutCsrElementWithValue<V> for WithToElement
    {
        type ElementWithValue<'a>
            = V::ToElement<'a>
        where
            V: 'a;

        type ElementWithValueRenderStateKind = K;

        fn as_mut_csr_element_with_value<'a>(&'a mut self, v: &'a V) -> Self::ElementWithValue<'a> {
            v.to_element()
        }
    }

    impl<
            V: ?Sized + for<'a> ToElement<ToElement<'a>: CsrElement<RenderStateKind = K>>,
            K: RenderStateKind,
        > IntoAsMutCsrElementWithValue<V> for WithToElement
    {
        impl_IntoAsMutCsrElementWithValue_with_Self! {
            type Value = V;
        }
    }
}
