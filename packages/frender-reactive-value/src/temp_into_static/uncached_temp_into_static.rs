use crate::static_or_temp_ref::StaticOrTempRef;

use super::{IntoStatic, ToStatic};

pub trait UncachedTempIntoStatic<V: ?Sized + 'static>: IntoStatic<V> {
    fn uncached_as_ref_temp(&self) -> &V;
}

impl<T: ?Sized + ToStatic<V>, V: ?Sized + 'static> UncachedTempIntoStatic<V> for &T {
    fn uncached_as_ref_temp(&self) -> &V {
        self.borrow()
    }
}
impl<V: ?Sized + 'static + ToOwned> UncachedTempIntoStatic<V> for StaticOrTempRef<'_, V> {
    fn uncached_as_ref_temp(&self) -> &V {
        self.to_ref()
    }
}
