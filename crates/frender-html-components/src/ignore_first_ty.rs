pub trait IgnoreFirstTypeOutput<A: ?Sized, B: ?Sized> {
    type Output: ?Sized;
}

pub enum IgnoreFirstType {}

impl<A: ?Sized, B: ?Sized> IgnoreFirstTypeOutput<A, B> for IgnoreFirstType {
    type Output = B;
}

macro_rules! ignore_first_ty {
    ($ty1:ty, $ty2:ty $(,)?) => {
        <$crate::ignore_first_ty::IgnoreFirstType as $crate::ignore_first_ty::IgnoreFirstTypeOutput<$ty1, $ty2>>::Output
    };
}

pub(crate) use ignore_first_ty;
