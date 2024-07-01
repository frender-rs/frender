use std::{cell::RefCell, iter::FusedIterator, ops::Deref};

struct SyncedDrain<'a> {
    all_states: &'a mut RefCell<super::AllStates>,
    drain_range: std::ops::Range<usize>,
}

impl<'a> Drop for SyncedDrain<'a> {
    fn drop(&mut self) {
        use super::StatesCommon;

        self.all_states.get_mut().drain(self.drain_range.clone());
    }
}

pub struct Drain<'a, T> {
    vec_drain: std::vec::Drain<'a, T>,
    #[allow(dead_code)]
    synced: SyncedDrain<'a>,
}

impl<'a, T> Drain<'a, T> {
    pub(super) fn new(
        all_states: &'a mut RefCell<super::AllStates>,
        vec_drain: std::vec::Drain<'a, T>,
        drain_range: std::ops::Range<usize>,
    ) -> Self {
        Self {
            vec_drain,
            synced: SyncedDrain {
                all_states,
                drain_range,
            },
        }
    }
}

impl<'a, T> Deref for Drain<'a, T> {
    type Target = std::vec::Drain<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.vec_drain
    }
}

impl<'a, T: std::fmt::Debug> std::fmt::Debug for Drain<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.vec_drain.fmt(f)
    }
}

impl<'a, T> Iterator for Drain<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.vec_drain.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.vec_drain.size_hint()
    }
}

impl<'a, T> DoubleEndedIterator for Drain<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.vec_drain.next_back()
    }
}

impl<'a, T> ExactSizeIterator for Drain<'a, T> {
    fn len(&self) -> usize {
        self.vec_drain.len()
    }
}

impl<'a, T> FusedIterator for Drain<'a, T> {}
