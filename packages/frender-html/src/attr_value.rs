use std::marker::PhantomData;

use frender_attr_value::AttrValueKind;

#[cfg(feature = "csr")]
pub(crate) mod csr;
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

pub trait HasAttrValueKind {
    type AttrValueKind: ?Sized + AttrValueKind;
}
