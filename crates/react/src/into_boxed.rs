pub trait IntoBoxed<T: ?Sized> {
    fn into_boxed(self) -> Box<T>;
}

impl<T> IntoBoxed<T> for T {
    #[inline]
    fn into_boxed(self) -> Box<T> {
        Box::new(self)
    }
}

impl<T: ?Sized> IntoBoxed<T> for Box<T> {
    #[inline]
    fn into_boxed(self) -> Box<T> {
        self
    }
}
