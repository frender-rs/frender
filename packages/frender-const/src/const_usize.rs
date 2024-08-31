use crate::array::{IsArray, IsArrayIntoIter};

mod sealed {
    pub trait IsConstUsize {}
}

pub trait IsConstUsize: sealed::IsConstUsize + 'static {
    type Array<T>: IsArray<Item = T, Len = Self>;
    type ArrayIntoIter<T>: IsArrayIntoIter<Item = T, Len = Self>;
}

pub struct ConstUsize<const N: usize>;

impl<const N: usize> sealed::IsConstUsize for ConstUsize<N> {}
impl<const N: usize> IsConstUsize for ConstUsize<N> {
    type Array<T> = [T; N];
    type ArrayIntoIter<T> = core::array::IntoIter<T, N>;
}
