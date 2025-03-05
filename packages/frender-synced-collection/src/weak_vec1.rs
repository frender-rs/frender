use std::rc::{Rc, Weak};

// Option<Weak> has the same size as Weak
pub(super) struct WeakVec1<T: ?Sized>(pub(super) Option<Weak<T>>, pub(super) Vec<Option<Weak<T>>>);

impl<T: ?Sized> std::fmt::Debug for WeakVec1<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(Some(&self.0).into_iter().chain(self.1.iter()))
            .finish()
    }
}

impl<T: ?Sized> Default for WeakVec1<T> {
    fn default() -> Self {
        Self::DEFAULT
    }
}

fn upgrade_or_set_none<T: ?Sized>(weak: &mut Option<Weak<T>>) -> Option<Rc<T>> {
    if let Some(v) = weak {
        if let Some(v) = v.upgrade() {
            Some(v)
        } else {
            *weak = None;
            None
        }
    } else {
        None
    }
}

impl<T: ?Sized> WeakVec1<T> {
    pub(super) const DEFAULT: Self = Self(None, Vec::new());

    pub(super) fn for_each_alive(&mut self, mut f: impl FnMut(Rc<T>)) {
        if let Some(v) = upgrade_or_set_none(&mut self.0) {
            f(v);
        }

        let continuous_empty_count = self.1.iter_mut().fold(0usize, |n, weak| {
            if let Some(v) = upgrade_or_set_none(weak) {
                f(v);
                0
            } else {
                n + 1
            }
        });

        self.1.truncate(self.1.len() - continuous_empty_count);
    }
}
