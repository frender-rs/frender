pub trait MaybeHandleEvent<E: ?Sized>:
    callable::gat::MaybeHandleEvent<E, Callable = Self::StaticCloneCallable>
{
    type StaticCloneCallable: 'static + Clone + for<'e> callable::Callable<(&'e E,), Output = ()>;
}

impl<C, E: ?Sized> MaybeHandleEvent<E> for C
where
    C: callable::gat::MaybeHandleEvent<E>,
    C::Callable: 'static + Clone + for<'e> callable::Callable<(&'e E,), Output = ()>,
{
    type StaticCloneCallable = C::Callable;
}
