pub struct Iter<'a, T: 'a> {
    slice: &'a [T],
}

impl<'a, T: 'a> Clone for Iter<'a, T> {
    fn clone(&self) -> Self {
        self.const_clone()
    }
}

impl<'a, T: 'a> Iter<'a, T> {
    #[must_use]
    #[inline]
    pub const fn new(slice: &'a [T]) -> Self {
        Self { slice }
    }

    #[must_use]
    #[inline]
    pub const fn as_slice(&self) -> &'a [T] {
        self.slice
    }

    pub const fn const_clone(&self) -> Self {
        Self { slice: self.slice }
    }
}

impl<'a, T: 'a> IntoIterator for Iter<'a, T> {
    type Item = &'a T;

    type IntoIter = ::core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}

impl<'a, T: 'a> Iter<'a, T> {
    pub const fn next(&mut self) -> Option<&'a T> {
        if let Some((first, rest)) = self.slice.split_first() {
            self.slice = rest;
            Some(first)
        } else {
            None
        }
    }
}
