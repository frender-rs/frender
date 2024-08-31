use core::array::IntoIter;
use std::borrow::{Borrow, BorrowMut};

use crate::{ConstUsize, IsConstUsize};

mod sealed {
    pub trait IsArray {}
    pub trait IsArrayIntoIter {}
}

pub trait IsArray:
    sealed::IsArray
    + IntoIterator<IntoIter = Self::_IntoIter>
    + AsRef<[Self::Item]>
    + AsMut<[Self::Item]>
    + Borrow<[Self::Item]>
    + BorrowMut<[Self::Item]>
{
    type Len: IsConstUsize<Array<Self::Item> = Self>;

    type _IntoIter: IsArrayIntoIter<Len = Self::Len, Item = Self::Item>;
}

pub trait IsArrayIntoIter: sealed::IsArrayIntoIter + Iterator {
    type Len: IsConstUsize<ArrayIntoIter<Self::Item> = Self>;
}

impl<T, const N: usize> sealed::IsArray for [T; N] {}
impl<T, const N: usize> IsArray for [T; N] {
    type Len = ConstUsize<N>;

    type _IntoIter = Self::IntoIter;
}

impl<T, const N: usize> sealed::IsArrayIntoIter for IntoIter<T, N> {}
impl<T, const N: usize> IsArrayIntoIter for IntoIter<T, N> {
    type Len = ConstUsize<N>;
}
