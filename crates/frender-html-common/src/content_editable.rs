use std::borrow::Cow;

use async_str_iter::{AsyncStrIterator, IntoAsyncStrIterator};

pub trait MaybeContentEditable {
    type State;

    fn initialize(this: Self, update: impl FnOnce(&str), remove: impl FnOnce()) -> Self::State;
    fn update(
        this: Self,
        update: impl FnOnce(&str),
        remove: impl FnOnce(),
        state: &mut Self::State,
    );

    type UpdateWithState: Default;

    fn update_with_state(
        this: Self,
        updater: impl crate::ValueUpdater<str>,
        state: &mut Self::UpdateWithState,
    );

    type ContentEditableIntoAsyncStrIter: AsyncStrIterator;

    fn content_editable_maybe_into_async_str_iter(
        this: Self,
    ) -> Option<Self::ContentEditableIntoAsyncStrIter>;
}

// TODO: only static string is implemented
// impl for static string
crate::impl_many!(
    impl<__> MaybeContentEditable for each_of![&'static str, String, Cow<'static, str>] {
        type State = Self;

        fn initialize(this: Self, update: impl FnOnce(&str), _: impl FnOnce()) -> Self::State {
            update(&this);
            this
        }

        fn update(
            this: Self,
            update: impl FnOnce(&str),
            remove: impl FnOnce(),
            state: &mut Self::State,
        ) {
            if *state != this {
                *state = Self::initialize(this, update, remove);
            }
        }

        type UpdateWithState = Option<Self>;

        fn update_with_state(
            this: Self,
            updater: impl crate::ValueUpdater<str>,
            state: &mut Self::UpdateWithState,
        ) {
            match state {
                Some(state) if *state == this => {}
                _ => {
                    updater.update(&this);
                    *state = Some(this);
                }
            }
        }

        type ContentEditableIntoAsyncStrIter = <Self as IntoAsyncStrIterator>::IntoAsyncStrIterator;

        fn content_editable_maybe_into_async_str_iter(
            this: Self,
        ) -> Option<Self::ContentEditableIntoAsyncStrIter> {
            Some(this.into_async_str_iterator())
        }
    }
);

fn bool_to_str(this: bool) -> &'static str {
    if this {
        "true"
    } else {
        "false"
    }
}

impl MaybeContentEditable for bool {
    type State = Self;

    fn initialize(this: Self, update: impl FnOnce(&str), _: impl FnOnce()) -> Self::State {
        update(bool_to_str(this));
        this
    }

    fn update(
        this: Self,
        update: impl FnOnce(&str),
        remove: impl FnOnce(),
        state: &mut Self::State,
    ) {
        if *state != this {
            *state = Self::initialize(this, update, remove)
        }
    }

    type UpdateWithState = Option<Self>;

    fn update_with_state(
        this: Self,
        updater: impl crate::ValueUpdater<str>,
        state: &mut Self::UpdateWithState,
    ) {
        if *state == Some(this) {
            return;
        }
        *state = Some(this);
        updater.update(bool_to_str(this));
    }

    type ContentEditableIntoAsyncStrIter = &'static str;

    fn content_editable_maybe_into_async_str_iter(
        this: Self,
    ) -> Option<Self::ContentEditableIntoAsyncStrIter> {
        Some(bool_to_str(this))
    }
}

impl<V: MaybeContentEditable> MaybeContentEditable for Option<V> {
    type State = Option<V::State>;

    fn initialize(this: Self, update: impl FnOnce(&str), remove: impl FnOnce()) -> Self::State {
        if let Some(this) = this {
            Some(V::initialize(this, update, remove))
        } else {
            remove();
            None
        }
    }

    fn update(
        this: Self,
        update: impl FnOnce(&str),
        remove: impl FnOnce(),
        state: &mut Self::State,
    ) {
        match (this, state) {
            (None, None) => {}
            (None, state @ Some(_)) => {
                remove();
                *state = None;
            }
            (Some(this), state @ None) => {
                *state = Some(V::initialize(this, update, remove));
            }
            (Some(this), Some(state)) => V::update(this, update, remove, state),
        }
    }

    type UpdateWithState = V::UpdateWithState;

    fn update_with_state(
        this: Self,
        updater: impl crate::ValueUpdater<str>,
        state: &mut Self::UpdateWithState,
    ) {
        if let Some(this) = this {
            V::update_with_state(this, updater, state);
        } else {
            *state = Default::default();
            updater.remove();
        }
    }

    type ContentEditableIntoAsyncStrIter = V::ContentEditableIntoAsyncStrIter;

    fn content_editable_maybe_into_async_str_iter(
        this: Self,
    ) -> Option<Self::ContentEditableIntoAsyncStrIter> {
        this.and_then(V::content_editable_maybe_into_async_str_iter)
    }
}

impl MaybeContentEditable for () {
    type State = ();

    fn initialize((): Self, _: impl FnOnce(&str), _: impl FnOnce()) -> Self::State {}

    fn update((): Self, _: impl FnOnce(&str), _: impl FnOnce(), (): &mut Self::State) {}

    type UpdateWithState = ();

    fn update_with_state(
        (): Self,
        _: impl crate::ValueUpdater<str>,
        (): &mut Self::UpdateWithState,
    ) {
    }

    type ContentEditableIntoAsyncStrIter = async_str_iter::never::Never;

    fn content_editable_maybe_into_async_str_iter(
        (): Self,
    ) -> Option<Self::ContentEditableIntoAsyncStrIter> {
        None
    }
}
