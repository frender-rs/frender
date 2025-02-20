#[cfg(feature = "csr")]
use ref_cast::{ref_cast_custom, RefCastCustom};

#[cfg_attr(feature = "csr", derive(RefCastCustom))]
#[repr(transparent)]
pub struct ElementProxyAttrs<E: ?Sized>(pub E);

#[cfg(feature = "csr")]
impl<E: ?Sized> ElementProxyAttrs<E> {
    #[ref_cast_custom]
    fn ref_cast_mut(inner: &mut E) -> &mut Self;
}

#[cfg(feature = "csr")]
mod ui_handle;

#[cfg(feature = "csr")]
pub mod csr;
