use std::fmt::Debug;

#[derive(Clone, Copy)]
pub(crate) struct LeadingVec<T, const CAP: usize> {
    len: usize,
    /// ```compile_fail
    /// if let Some(whole_or_leading) = self.whole_or_leading {
    ///     if CAP >= self.len { whole_items + padding_dummy_items } else { leading_items }
    /// } else {
    ///     unknown
    /// }
    /// ```
    whole_or_leading: Option<[T; CAP]>,
}

impl<T: Debug, const CAP: usize> Debug for LeadingVec<T, CAP> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const NAME: &str = "LeadingVec";
        if let Some(whole_or_leading) = &self.whole_or_leading {
            if CAP >= self.len {
                let (all, _) = whole_or_leading.split_at(self.len);
                f.debug_tuple(NAME).field(&all).finish()
            } else {
                f.debug_struct(NAME)
                    .field("len", &self.len)
                    .field("leading_items", &whole_or_leading)
                    .finish()
            }
        } else if self.len == 0 {
            let empty: &[T] = &[];
            f.debug_tuple(NAME).field(&empty).finish()
        } else {
            // unknown
            f.debug_struct(NAME)
                .field("len", &self.len)
                .finish_non_exhaustive()
        }
    }
}
impl<T, const CAP: usize> LeadingVec<T, CAP> {
    pub const fn new() -> Self {
        Self {
            len: 0,
            whole_or_leading: None,
        }
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub const fn try_as_slice(&self) -> Option<&[T]> {
        if let Some(whole_or_leading) = &self.whole_or_leading {
            if CAP >= self.len {
                let (whole, _) = whole_or_leading.split_at(self.len);
                Some(whole)
            } else {
                None
            }
        } else if self.len == 0 {
            Some(&[])
        } else {
            None
        }
    }

    /// Panics if self doesn't contain all items.
    pub const fn as_slice(&self) -> &[T] {
        match self.try_as_slice() {
            Some(whole) => whole,
            None => panic!("LeadingVec doesn't contain all items"),
        }
    }

    /// [`Vec::push`]
    pub(crate) const fn push(&mut self, item: T)
    where
        T: Copy, // ~const Destruct
    {
        self.make_empty_as_known(item);
        if let Some(whole_or_leading) = &mut self.whole_or_leading {
            if CAP > self.len {
                let (_, padding) = whole_or_leading.split_at_mut(self.len);
                padding[0] = item;
            } else {
                // leading_items
                // skip pushing
            }
        } else {
            // unknown
        }

        self.len += 1;
    }

    const fn make_empty_as_known(&mut self, dummy_value: T)
    where
        T: Copy,
    {
        if self.len == 0 && self.whole_or_leading.is_none() {
            // actually items are known
            self.whole_or_leading = Some([dummy_value; CAP])
        }
    }
}
