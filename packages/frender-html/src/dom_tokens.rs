use std::marker::PhantomData;

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
pub(crate) mod ssr;

pub struct Property<PM, V> {
    _prop_marker: PhantomData<PM>,
    value: V,
}

impl<PM, V> Property<PM, V> {
    pub(crate) fn new(value: V) -> Self {
        Self { _prop_marker: PhantomData, value }
    }
}

pub(crate) mod impl_bounds {
    pub(crate) use super::Property;
    pub(crate) use frender_dom_values::dom_tokens::DomTokens as Bounds;

    #[cfg(feature = "ssr")]
    pub(crate) use super::ssr;
}
