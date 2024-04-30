use std::task::Poll;

use async_str_iter::AsyncStrIterator;

pub struct StringsWithPredicates<'a, const N: usize> {
    strings: [&'a str; N],
    predicates: [bool; N],
    current: usize,
}

impl<'a, const N: usize> StringsWithPredicates<'a, N> {
    pub fn new(strings: [&'a str; N], predicates: [bool; N]) -> Self {
        Self {
            strings,
            predicates,
            current: 0,
        }
    }

    fn next_str(&mut self) -> Option<&'a str> {
        loop {
            if self.current >= N {
                return None;
            }
            if !self.predicates[self.current] {
                self.current += 1;
            } else {
                self.current += 1;
                return Some(self.strings[self.current]);
            }
        }
    }
}

impl<const N: usize> AsyncStrIterator for StringsWithPredicates<'_, N> {
    fn poll_next_str(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> Poll<Option<&str>> {
        Poll::Ready(self.get_mut().next_str())
    }
}
