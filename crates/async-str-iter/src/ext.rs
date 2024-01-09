pub mod collect;

use crate::AsyncStrIterator;

pub trait AsyncStrIteratorExt: AsyncStrIterator {
    fn collect<R: Default + for<'a> Extend<&'a str>>(self) -> collect::Collect<Self, R>
    where
        Self: Sized,
    {
        collect::Collect::new(self)
    }
}

impl<S: AsyncStrIterator + ?Sized> AsyncStrIteratorExt for S {}
