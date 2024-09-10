pub mod csr;
pub mod ssr;

mod kinds;

pub mod values;

#[cfg(feature = "html")]
pub mod html;

/// A trait alias.
pub trait AttrValue<VK: ?Sized + crate::csr::ValueKind>:
    ssr::SsrAttrValue<VK> + crate::csr::CsrAttrValue<VK>
{
}

impl<
        T: ?Sized + ssr::SsrAttrValue<VK> + crate::csr::CsrAttrValue<VK>,
        VK: ?Sized + crate::csr::ValueKind,
    > AttrValue<VK> for T
{
}
