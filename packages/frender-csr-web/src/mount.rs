#[cfg(feature = "csr")]
pub use self::csr::mount_to_dom_element;
#[cfg(feature = "csr")]
#[cfg(feature = "spawn")]
pub use self::csr::spawn_mount_to_dom_element;

#[cfg(feature = "csr")]
mod csr;

pub trait GetDomElement {
    fn get_dom_element(self, document: &web_sys::Document) -> web_sys::Element;
}

impl GetDomElement for &str {
    fn get_dom_element(self, document: &web_sys::Document) -> web_sys::Element {
        document
            .get_element_by_id(self)
            .expect("document should have an element with the specified id")
    }
}

impl<F> GetDomElement for F
where
    F: FnOnce(&web_sys::Document) -> web_sys::Element,
{
    #[inline(always)]
    fn get_dom_element(self, document: &web_sys::Document) -> web_sys::Element {
        self(document)
    }
}
