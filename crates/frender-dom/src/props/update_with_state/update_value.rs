use std::borrow::Cow;

use frender_core::{StaticStr, StaticText};

pub trait UpdateValueWithState<V: ?Sized> {
    type State: Default;

    fn update_value_with_state(this: Self, state: &mut Self::State, update: impl FnOnce(&V));
}

/// No cache
impl UpdateValueWithState<str> for &str {
    type State = ();

    #[inline(always)]
    fn update_value_with_state(this: Self, _: &mut Self::State, update: impl FnOnce(&str)) {
        update(this)
    }
}

/// Cache if self is [`Cow::Owned`]
impl UpdateValueWithState<str> for Cow<'_, str> {
    type State = Option<String>;

    fn update_value_with_state(this: Self, cache: &mut Self::State, update: impl FnOnce(&str)) {
        if cache.as_deref() == Some(&*this) {
            return;
        }
        update(&this);
        *cache = match this {
            Cow::Borrowed(_) => None,
            Cow::Owned(v) => Some(v),
        }
    }
}

impl<S: StaticStr> UpdateValueWithState<str> for StaticText<S> {
    type State = Option<S>;

    fn update_value_with_state(this: Self, cache: &mut Self::State, update: impl FnOnce(&str)) {
        if cache.as_deref() == Some(&*this) {
            return;
        }

        update(&*this);
        *cache = Some(this.0);
    }
}

impl UpdateValueWithState<str> for String {
    type State = Option<String>;

    fn update_value_with_state(this: Self, cache: &mut Self::State, update: impl FnOnce(&str)) {
        if cache.as_ref() == Some(&this) {
            return;
        }
        update(&this);
        *cache = Some(this);
    }
}

impl UpdateValueWithState<bool> for bool {
    type State = bool;

    fn update_value_with_state(this: Self, state: &mut Self::State, update: impl FnOnce(&bool)) {
        if *state == this {
            return;
        }
        update(&this);
        *state = this;
    }
}
