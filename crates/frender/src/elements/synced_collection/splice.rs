use std::{cell::RefCell, ops::Range};

pub struct SpliceReplaceWith<'a, I: Iterator> {
    pub(super) iter: I,
    pub(super) all_states: &'a mut RefCell<super::AllStates>,
    pub(super) range: Range<usize>,
    pub(super) count: usize,
}

impl<'a, I: ExactSizeIterator> ExactSizeIterator for SpliceReplaceWith<'a, I> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}
impl<'a, I: Iterator> Iterator for SpliceReplaceWith<'a, I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next();

        if next.is_some() {
            self.count += 1;
        } else {
            use super::StatesCommon;
            self.all_states
                .get_mut()
                .splice(self.range.clone(), self.count)
        }

        next
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
