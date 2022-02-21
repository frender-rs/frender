/// A NewType style struct which marks the inner element is created with
///
/// `Keyed<T>` doesn't need to impl [`Node`](crate::Node)
/// as it's not necessary to assign keys for direct child nodes.
///
/// `Vec<Keyed<T>>` and `Vec<Option<Keyed<T>>>` (and also corresponding slices and arrays)
/// impl [`Node`](crate::Node)
/// as long as `T` impl [`IntoElement`],
/// so that list of keyed elements can be used as child nodes.
#[derive(Debug, Clone)]
pub struct Keyed<T>(pub T);

impl<T> Keyed<T> {
    #[inline]
    pub fn inner(&self) -> &T {
        &self.0
    }
    #[inline]
    pub fn into_inner(self) -> T {
        self.0
    }
}

// TODO: Do we need to impl IntoElement and Node for Keyed<T>

// impl<T: IntoElement> IntoElement for Keyed<T> {
//     fn into_element(self) -> crate::Element {
//         self.0.into_element()
//     }
// }
