use std::borrow::Cow;

use frender_common::write::str::{AsyncWritableStr, StrWriting};

use crate::NeverWritable;

pub trait MaybeContentEditable {
    type State;

    fn initialize(this: Self, update: impl FnOnce(&str), remove: impl FnOnce()) -> Self::State;
    fn update(
        this: Self,
        update: impl FnOnce(&str),
        remove: impl FnOnce(),
        state: &mut Self::State,
    );

    type AttrValueStr: AsyncWritableStr;

    fn maybe_into_attr_value_str(this: Self) -> Option<Self::AttrValueStr>;
}

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

        type AttrValueStr = StrWriting<Self>;

        fn maybe_into_attr_value_str(this: Self) -> Option<Self::AttrValueStr> {
            Some(StrWriting::new(this))
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

    type AttrValueStr = <&'static str as MaybeContentEditable>::AttrValueStr;

    fn maybe_into_attr_value_str(this: Self) -> Option<Self::AttrValueStr> {
        <&'static str as MaybeContentEditable>::maybe_into_attr_value_str(bool_to_str(this))
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

    type AttrValueStr = V::AttrValueStr;

    fn maybe_into_attr_value_str(this: Self) -> Option<Self::AttrValueStr> {
        this.and_then(V::maybe_into_attr_value_str)
    }
}

impl MaybeContentEditable for () {
    type State = ();

    fn initialize((): Self, _: impl FnOnce(&str), _: impl FnOnce()) -> Self::State {}

    fn update((): Self, _: impl FnOnce(&str), _: impl FnOnce(), (): &mut Self::State) {}

    type AttrValueStr = NeverWritable;

    fn maybe_into_attr_value_str((): Self) -> Option<Self::AttrValueStr> {
        None
    }
}
