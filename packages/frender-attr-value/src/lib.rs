pub mod csr;
pub mod ssr;

mod kinds;

#[cfg(feature = "html")]
pub mod html;

mod string;

/// A trait alias.
pub trait MaybeAttrValue<VK: ?Sized + crate::csr::ValueKind>:
    ssr::MaybeIntoHtmlAttributeValue<VK> + crate::csr::MaybeValue<VK>
{
}

impl<
        T: ?Sized + ssr::MaybeIntoHtmlAttributeValue<VK> + crate::csr::MaybeValue<VK>,
        VK: ?Sized + crate::csr::ValueKind,
    > MaybeAttrValue<VK> for T
{
}
