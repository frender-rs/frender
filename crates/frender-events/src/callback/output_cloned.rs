use super::{Callback, IsCallback};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutputCloned<V: Clone + PartialEq> {
    pub(super) value: V,
}

impl<V: Clone + PartialEq> IsCallback for OutputCloned<V> {}

impl<V: Clone + PartialEq> Callback<()> for OutputCloned<V> {
    type Output = V;

    fn emit(&self, _: ()) -> Self::Output {
        self.value.clone()
    }
}
