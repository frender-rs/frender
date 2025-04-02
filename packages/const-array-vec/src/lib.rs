#![no_std]

#[cfg(any(doc, test))]
extern crate std;
#[cfg(any(doc, test))]
use std::prelude::rust_2021::*;

#[derive(Clone, Copy)]
pub struct ArrayVec<T, const CAP: usize> {
    len: usize,
    buf: [T; CAP],
}

impl<T, const CAP: usize> ArrayVec<T, CAP> {
    pub const fn new() -> Self
    where
        T: ConstDummyValue,
    {
        Self {
            len: 0,
            buf: [T::DUMMY_VALUE; CAP],
        }
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// [`Vec::capacity`]
    #[inline(always)]
    pub const fn capacity(&self) -> usize {
        CAP
    }

    /// [`ArrayVec::is_full`](https://docs.rs/arrayvec/latest/arrayvec/struct.ArrayVec.html#method.is_full)
    pub const fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }

    /// [`Vec::as_slice`]
    pub const fn as_slice(&self) -> &[T] {
        let (s, _) = self.buf.split_at(self.len);
        s
    }

    /// [`Vec::extend_from_slice`]
    ///
    /// Panics if `CAP <= self.len()`
    pub const fn push(&mut self, item: T)
    where
        T: Copy, // Sufficient and unnecessary for T: ~const core::marker::Destruct
    {
        let (_, padding) = self.buf.split_at_mut(self.len);

        padding[0] = item;

        self.len += 1;
    }

    /// [`Vec::extend_from_slice`]
    ///
    /// Panics if `self.len() + other.len() > CAP`
    pub const fn extend_from_slice(&mut self, other: &[T])
    where
        T: Copy,
    {
        let (_, padding) = self.buf.split_at_mut(self.len);

        // let (this, _) = rest.split_at_mut(other.len());
        // this.copy_from_slice(other);

        let mut i = 0usize;
        while i < other.len() {
            padding[i] = other[i];
            i += 1;
        }

        self.len += other.len();
    }

    /// Panics if `NEW_CAP < self.len`
    pub const fn into_with_capacity<const NEW_CAP: usize>(self) -> ArrayVec<T, NEW_CAP>
    where
        T: Copy,
        T: ConstDummyValue,
    {
        assert!(NEW_CAP >= self.len());
        let mut res = ArrayVec::new();
        res.extend_from_slice(self.as_slice());
        res
    }
}

/// The implementation should have the same value as [`Default::default()`].
pub trait ConstDefault: Default {
    const DEFAULT: Self;
}

pub trait ConstDummyValue {
    const DUMMY_VALUE: Self;
}

impl<T: ConstDefault> ConstDummyValue for T {
    const DUMMY_VALUE: Self = T::DEFAULT;
}

impl ConstDefault for u8 {
    const DEFAULT: Self = 0;
}

impl ConstDefault for char {
    const DEFAULT: Self = '\x00';
}
