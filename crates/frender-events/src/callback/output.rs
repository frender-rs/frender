use super::{Callback, IsCallback};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Output<V: Copy + PartialEq> {
    pub(super) value: V,
}

impl<V: Copy + PartialEq> IsCallback for Output<V> {}

impl<V: Copy + PartialEq> Callback<()> for Output<V> {
    type Output = V;

    fn emit(&self, _: ()) -> Self::Output {
        self.value
    }
}
