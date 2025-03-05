use std::rc::{Rc, Weak};

use super::weak_is_of_rc;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) struct Key(usize);

impl Key {
    pub(super) const STACK: Self = Self(usize::MAX);
}

pub(super) struct RcWithKey<T: ?Sized> {
    pub(super) rc: Rc<T>,
    pub(super) key: Key,
}

// Option<Weak> has the same size as Weak
pub(super) struct WeakVec1<T: ?Sized>(Option<Weak<T>>, Vec<Option<Weak<T>>>);

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

fn weak_is_empty<T: ?Sized>(v: &Weak<T>) -> bool {
    v.strong_count() == 0
}

/// Returns `true` if `v` is available.
fn put_into_available_weak<T: ?Sized, R: ?Sized>(
    v: &mut Option<Weak<T>>,
    rc: &R,
    weak_has_same_addr_of: impl FnOnce(&Weak<T>, &R) -> bool,
    to_weak: impl FnOnce(&R) -> Weak<T>,
) -> bool {
    match v {
        None => {}
        Some(v) if weak_is_empty(v) => {}
        Some(v) if weak_has_same_addr_of(v, rc) => {
            // the weak matched the rc so it doesn't need to be updated
            return true;
        }
        _ => {
            // the weak is alive and doesn't match the rc
            return false;
        }
    }

    *v = Some(to_weak(rc));

    true
}

impl<T: ?Sized> WeakVec1<T> {
    pub(super) const DEFAULT: Self = Self(None, Vec::new());
    pub(super) fn contains<U: ?Sized>(&self, v: &RcWithKey<U>) -> bool {
        let weak = match v.key {
            Key::STACK => self.0.as_ref(),
            Key(i) => self.1.get(i).and_then(Option::as_ref),
        };
        weak.map_or(false, |weak| weak_is_of_rc(weak, &v.rc))
    }

    pub(crate) fn put_into_old_available_or_append<R: ?Sized>(
        &mut self,
        old_key: Key,
        rc: &R,
        // We require Copy because we can.
        weak_has_same_addr_of: impl Copy + FnOnce(&Weak<T>, &R) -> bool,
        to_weak: impl Copy + FnOnce(&R) -> Weak<T>,
    ) -> Key {
        let stack = &mut self.0;

        match old_key {
            Key::STACK => {}
            Key(index) => match self.1.get_mut(index) {
                Some(weak) => {
                    if put_into_available_weak(weak, rc, weak_has_same_addr_of, to_weak) {
                        return old_key;
                    }
                }
                None => {}
            },
        }

        if put_into_available_weak(stack, rc, weak_has_same_addr_of, to_weak) {
            return Key::STACK;
        }

        let i = self.1.len();
        assert_ne!(i, Key::STACK.0);
        self.1.push(Some(to_weak(rc)));
        Key(i)
    }

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
