// #![no_std]
#![cfg_attr(not(test), no_std)]

use const_array_vec::ArrayVec;

/// [`String`]
#[derive(Clone, Copy)]
pub struct ArrayString<const CAP: usize> {
    vec: ArrayVec<u8, CAP>,
}

impl<const CAP: usize> core::fmt::Debug for ArrayString<CAP> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <str>::fmt(self.as_str(), f)
    }
}

impl<const CAP: usize> ArrayString<CAP> {
    /// [`String::new`]
    pub const fn new() -> Self {
        Self {
            vec: ArrayVec::new(),
        }
    }

    /// [`str::len`]
    /// [`String::len`]
    pub const fn len(&self) -> usize {
        self.vec.len()
    }

    pub const fn as_str(&self) -> &str {
        match ::core::str::from_utf8(self.as_bytes()) {
            Ok(v) => v,
            Err(_) => unreachable!(),
        }
    }

    /// [`str::as_bytes`]
    pub const fn as_bytes(&self) -> &[u8] {
        self.vec.as_slice()
    }

    /// [`String::push_str`]
    pub const fn push_str(&mut self, string: &str) {
        self.vec.extend_from_slice(string.as_bytes())
    }
}
