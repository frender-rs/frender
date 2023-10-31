pub trait FromMut<T: ?Sized> {
    fn from_mut(v: &mut T) -> &mut Self;
}

impl<T: ?Sized> FromMut<T> for T {
    fn from_mut(v: &mut T) -> &mut Self {
        v
    }
}

pub trait IntoMut<T: ?Sized> {
    fn into_mut(&mut self) -> &mut T;
}

impl<T: ?Sized, R: ?Sized + FromMut<T>> IntoMut<R> for T {
    fn into_mut(&mut self) -> &mut R {
        R::from_mut(self)
    }
}

pub trait IdentityAs<T>: From<T> + Into<T> + FromMut<T> + IntoMut<T> {}

impl<T, R: From<T> + Into<T> + FromMut<T> + IntoMut<T>> IdentityAs<T> for R {}
