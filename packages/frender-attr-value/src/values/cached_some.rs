use std::marker::PhantomData;

pub struct CachedSome<T, VK: ?Sized>(pub(crate) T, PhantomData<VK>);

impl<T, VK: ?Sized> CachedSome<T, VK> {
    pub(crate) const fn new(v: T) -> Self {
        Self(v, PhantomData)
    }
}
