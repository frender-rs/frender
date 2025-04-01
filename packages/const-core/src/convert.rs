pub trait MarkerOfConstValue<T> {
    const VALUE: T;
}

/// [`From`]
pub trait ConstFrom<T>: Sized {
    /// [`From::from`]
    type From<Value: ?Sized + MarkerOfConstValue<T>>: ?Sized + MarkerOfConstValue<Self>;
}

impl<T> ConstFrom<T> for T {
    type From<Value: ?Sized + MarkerOfConstValue<T>> = Value;
}

/// [`Into`]
pub trait ConstInto<T>: Sized {
    type Into<This: ?Sized + MarkerOfConstValue<Self>>: ?Sized + MarkerOfConstValue<T>;
}

impl<T, U> ConstInto<U> for T
where
    U: ConstFrom<T>,
{
    type Into<This: ?Sized + MarkerOfConstValue<Self>> = U::From<This>;
}

pub const fn const_into<T: ConstInto<U>, U, Value: ?Sized + MarkerOfConstValue<T>>() -> U {
    <T::Into<Value>>::VALUE
}
