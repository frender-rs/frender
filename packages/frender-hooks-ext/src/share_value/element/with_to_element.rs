use frender_to_element::ToElement;

use super::bound::MapValueToElement;

#[derive(Debug, Clone, Copy)]
pub struct WithToElement;

impl<V: ?Sized + ToElement> MapValueToElement<V> for WithToElement {}

#[cfg(feature = "ssr")]
mod ssr {
    use frender_ssr::{html::assert::HtmlChildren, SsrElement};

    use frender_to_element::ToElement;

    use super::{super::bound::ssr::MapValueToSsrElement, WithToElement};

    impl<
            V: ?Sized + for<'a> ToElement<ToElement<'a>: SsrElement<HtmlChildren = HC>>,
            HC: HtmlChildren,
        > MapValueToSsrElement<V> for WithToElement
    {
        type HtmlChildrenWithValue = HC;

        fn into_html_children_with_value(self, value: &V) -> Self::HtmlChildrenWithValue {
            use frender_ssr::SsrElement as _;
            value.to_element().into_html_children()
        }
    }
}

#[cfg(feature = "csr")]
mod csr {
    use frender_csr::{CsrElement, RenderStateKind};

    use frender_to_element::ToElement;

    use super::{
        super::bound::csr::{
            impl_MapValueToCsrElement_with_Self, AsMutCsrElementWithValue, MapValueToCsrElement,
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
        > MapValueToCsrElement<V> for WithToElement
    {
        impl_MapValueToCsrElement_with_Self! {
            type Value = V;
        }
    }
}
