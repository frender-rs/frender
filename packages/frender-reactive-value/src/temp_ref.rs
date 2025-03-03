#[derive(Debug)]
pub struct TempRef<'a, T: ?Sized>(pub &'a T);

impl<'a, T: ?Sized> Copy for TempRef<'a, T> {}
impl<'a, T: ?Sized> Clone for TempRef<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T: ?Sized> AsRef<T> for TempRef<'a, T> {
    fn as_ref(&self) -> &T {
        self.0
    }
}
