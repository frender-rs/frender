use frender_common::write::{
    attrs::{
        writable::AsyncWritableAttrValue, AsyncWritableAttrValueBooleanTrue,
        AsyncWritableAttrValueStr,
    },
    str::{AsyncWritableStr, StrWriting},
};

use crate::impl_many;

pub use value::*;

pub trait ValueUpdater<V: ?Sized> {
    fn update(self, value: &V);
    fn remove(self);
}

impl<V: ?Sized, U: FnOnce(&V), R: FnOnce()> ValueUpdater<V> for (U, R) {
    fn update(self, value: &V) {
        self.0(value)
    }

    fn remove(self) {
        self.1()
    }
}

pub enum NeverWritable {}
impl AsyncWritableAttrValue for NeverWritable {
    fn poll_write_attr_value_into<
        W: frender_common::write::attrs::write_traits::AsyncWriteAttrValue,
    >(
        &mut self,
        cx: &mut std::task::Context,
        write: W,
    ) -> std::task::Poll<std::io::Result<W::AsyncWriteAttrName>> {
        match *self {}
    }
}

impl AsyncWritableStr for NeverWritable {
    fn poll_write_str_into<W: frender_common::write::str::AsyncWriteStr>(
        &mut self,
        cx: &mut std::task::Context,
        write: &mut W,
    ) -> std::task::Poll<std::io::Result<()>> {
        match *self {}
    }
}

mod value {
    pub(super) mod sealed {
        pub enum NotSupported {}
    }

    pub trait ValueType {
        type SupportIntoChildStr;
        type SupportIntoAttrValue;
    }
}

pub trait MaybeUpdateValueWithState<V: ?Sized + ValueType> {
    type State;

    fn maybe_as(this: &Self) -> Option<&V>;

    fn initialize_state_and_update(
        this: Self,
        update: impl FnOnce(&V),
        remove: impl FnOnce(),
    ) -> Self::State;

    fn maybe_update_value_with_state(
        this: Self,
        state: &mut Self::State,
        update: impl FnOnce(&V),
        remove: impl FnOnce(),
    );

    type ChildStr: AsyncWritableStr;
    fn maybe_into_child_str(
        this: Self,
        supported: <V as ValueType>::SupportIntoChildStr,
    ) -> Option<Self::ChildStr>;

    type AttrValue: AsyncWritableAttrValue;
    fn maybe_into_attr_value(
        this: Self,
        supported: <V as ValueType>::SupportIntoAttrValue,
    ) -> Option<Self::AttrValue>;

    type UpdateWithState: Default;

    fn update_with_state(
        this: Self,
        state: &mut Self::UpdateWithState,
        updater: impl ValueUpdater<V>,
    );
}

impl<V: ?Sized + ValueType> MaybeUpdateValueWithState<V> for () {
    type State = ();

    #[inline(always)]
    fn maybe_as(_: &Self) -> Option<&V> {
        None
    }

    #[inline(always)]
    fn initialize_state_and_update(_: Self, _: impl FnOnce(&V), _: impl FnOnce()) -> Self::State {}

    #[inline(always)]
    fn maybe_update_value_with_state(
        _: Self,
        _: &mut Self::State,
        _: impl FnOnce(&V),
        _: impl FnOnce(),
    ) {
    }

    type ChildStr = NeverWritable;

    fn maybe_into_child_str(
        (): Self,
        _: <V as ValueType>::SupportIntoChildStr,
    ) -> Option<Self::ChildStr> {
        None
    }

    type AttrValue = NeverWritable;

    fn maybe_into_attr_value(
        (): Self,
        _: <V as ValueType>::SupportIntoAttrValue,
    ) -> Option<Self::AttrValue> {
        None
    }

    type UpdateWithState = ();

    fn update_with_state((): Self, (): &mut Self::UpdateWithState, _: impl ValueUpdater<V>) {}
}

impl<T: MaybeUpdateValueWithState<V>, V: ?Sized + ValueType> MaybeUpdateValueWithState<V>
    for Option<T>
{
    type State = Option<T::State>;

    fn maybe_as(this: &Self) -> Option<&V> {
        this.as_ref().and_then(T::maybe_as)
    }

    fn initialize_state_and_update(
        this: Self,
        update: impl FnOnce(&V),
        remove: impl FnOnce(),
    ) -> Self::State {
        match this {
            Some(this) => Some(T::initialize_state_and_update(this, update, remove)),
            None => {
                remove();
                None
            }
        }
    }

    fn maybe_update_value_with_state(
        this: Self,
        state: &mut Self::State,
        update: impl FnOnce(&V),
        remove: impl FnOnce(),
    ) {
        match (this, state) {
            (Some(this), Some(state)) => {
                T::maybe_update_value_with_state(this, state, update, remove)
            }
            (Some(this), state) => {
                *state = Some(T::initialize_state_and_update(this, update, remove))
            }
            (None, state) => {
                remove();
                *state = None;
            }
        }
    }

    type ChildStr = T::ChildStr;

    fn maybe_into_child_str(
        this: Self,
        supported: <V as ValueType>::SupportIntoChildStr,
    ) -> Option<Self::ChildStr> {
        this.and_then(|this| T::maybe_into_child_str(this, supported))
    }

    type AttrValue = T::AttrValue;

    fn maybe_into_attr_value(
        this: Self,
        supported: <V as ValueType>::SupportIntoAttrValue,
    ) -> Option<Self::AttrValue> {
        this.and_then(|this| T::maybe_into_attr_value(this, supported))
    }

    type UpdateWithState = T::UpdateWithState;

    fn update_with_state(
        this: Self,
        state: &mut Self::UpdateWithState,
        updater: impl ValueUpdater<V>,
    ) {
        if let Some(this) = this {
            T::update_with_state(this, state, updater)
        } else {
            updater.remove();
            *state = Default::default()
        }
    }
}

/// No cache
impl MaybeUpdateValueWithState<str> for &str {
    type State = ();

    fn maybe_as(this: &Self) -> Option<&str> {
        Some(this)
    }

    #[inline(always)]
    fn initialize_state_and_update(
        this: Self,
        update: impl FnOnce(&str),
        _: impl FnOnce(),
    ) -> Self::State {
        update(this)
    }

    #[inline(always)]
    fn maybe_update_value_with_state(
        this: Self,
        _: &mut Self::State,
        update: impl FnOnce(&str),
        _: impl FnOnce(),
    ) {
        update(this)
    }

    type ChildStr = StrWriting<Self>;

    fn maybe_into_child_str(
        this: Self,
        (): <str as ValueType>::SupportIntoChildStr,
    ) -> Option<Self::ChildStr> {
        Some(StrWriting::new(this))
    }

    type AttrValue = AsyncWritableAttrValueStr<StrWriting<Self>>;

    fn maybe_into_attr_value(
        this: Self,
        (): <str as ValueType>::SupportIntoAttrValue,
    ) -> Option<Self::AttrValue> {
        Some(AsyncWritableAttrValueStr::new_from_str(this))
    }

    type UpdateWithState = ();

    fn update_with_state(
        this: Self,
        _: &mut Self::UpdateWithState,
        updater: impl ValueUpdater<str>,
    ) {
        updater.update(this)
    }
}

/// Cache if self is [`Cow::Owned`]
impl MaybeUpdateValueWithState<str> for std::borrow::Cow<'_, str> {
    /// `None` means the previous value is borrowed and not cached.
    type State = Option<String>;

    fn maybe_as(this: &Self) -> Option<&str> {
        Some(&this)
    }

    fn initialize_state_and_update(
        this: Self,
        update: impl FnOnce(&str),
        _: impl FnOnce(),
    ) -> Self::State {
        update(&this);
        match this {
            std::borrow::Cow::Borrowed(_) => None,
            std::borrow::Cow::Owned(s) => Some(s),
        }
    }

    fn maybe_update_value_with_state(
        this: Self,
        state: &mut Self::State,
        update: impl FnOnce(&str),
        _: impl FnOnce(),
    ) {
        match (state, this) {
            (Some(state), this) if *this == **state => {}
            (state, std::borrow::Cow::Owned(this)) => {
                update(&this);
                *state = Some(this);
            }
            (state, std::borrow::Cow::Borrowed(this)) => {
                *state = None;
                update(this)
            }
        }
    }

    type ChildStr = StrWriting<Self>;

    fn maybe_into_child_str(
        this: Self,
        (): <str as ValueType>::SupportIntoChildStr,
    ) -> Option<Self::ChildStr> {
        Some(StrWriting::new(this))
    }

    type AttrValue = AsyncWritableAttrValueStr<StrWriting<Self>>;

    fn maybe_into_attr_value(
        this: Self,
        supported: <str as ValueType>::SupportIntoAttrValue,
    ) -> Option<Self::AttrValue> {
        Some(AsyncWritableAttrValueStr::new_from_str(this))
    }

    type UpdateWithState = Self::State;

    fn update_with_state(
        this: Self,
        state: &mut Self::UpdateWithState,
        updater: impl ValueUpdater<str>,
    ) {
        match (state, this) {
            (Some(state), this) if *this == **state => {}
            (state, std::borrow::Cow::Owned(this)) => {
                updater.update(&this);
                *state = Some(this);
            }
            (state, std::borrow::Cow::Borrowed(this)) => {
                *state = None;
                updater.update(this)
            }
        }
    }
}

#[cfg(feature = "StaticText")]
impl<S: frender_core::StaticStr> MaybeUpdateValueWithState<str> for frender_core::StaticText<S> {
    type State = S;

    fn maybe_as(this: &Self) -> Option<&str> {
        Some(&this)
    }

    fn maybe_into_html_attribute_value(
        this: Self,
    ) -> Option<Option<std::borrow::Cow<'static, str>>> {
        Some(Some(this.0.into()))
    }

    fn initialize_state_and_update(
        this: Self,
        update: impl FnOnce(&str),
        _: impl FnOnce(),
    ) -> Self::State {
        update(&this);
        this.0
    }

    fn maybe_update_value_with_state(
        this: Self,
        state: &mut Self::State,
        update: impl FnOnce(&str),
        _: impl FnOnce(),
    ) {
        if **state == *this.0 {
            return;
        }
        update(&this);
        *state = this.0;
    }
}

impl MaybeUpdateValueWithState<str> for String {
    type State = String;

    fn maybe_as(this: &Self) -> Option<&str> {
        Some(&this)
    }

    fn initialize_state_and_update(
        this: Self,
        update: impl FnOnce(&str),
        _: impl FnOnce(),
    ) -> Self::State {
        update(&this);
        this
    }

    fn maybe_update_value_with_state(
        this: Self,
        state: &mut Self::State,
        update: impl FnOnce(&str),
        _: impl FnOnce(),
    ) {
        if *state == this {
            return;
        }
        update(&this);
        *state = this;
    }

    type ChildStr = StrWriting<Self>;

    fn maybe_into_child_str(
        this: Self,
        (): <str as ValueType>::SupportIntoChildStr,
    ) -> Option<Self::ChildStr> {
        Some(StrWriting::new(this))
    }

    type AttrValue = AsyncWritableAttrValueStr<StrWriting<Self>>;

    fn maybe_into_attr_value(
        this: Self,
        supported: <str as ValueType>::SupportIntoAttrValue,
    ) -> Option<Self::AttrValue> {
        Some(AsyncWritableAttrValueStr::new_from_str(this))
    }

    type UpdateWithState = Option<Self::State>;

    fn update_with_state(
        this: Self,
        state: &mut Self::UpdateWithState,
        updater: impl ValueUpdater<str>,
    ) {
        if let Some(state) = state {
            if *state == this {
                return;
            }
        }

        updater.update(&this);
        *state = Some(this);
    }
}

impl_many!(
    impl<__> ValueType
        for each_of![str, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64]
    {
        type SupportIntoChildStr = ();
        type SupportIntoAttrValue = ();
    }
);

impl ValueType for bool {
    type SupportIntoChildStr = sealed::NotSupported;
    type SupportIntoAttrValue = ();
}

impl_many!(
    impl<__> MaybeUpdateValueWithState<Self>
        for each_of![i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64]
    {
        type State = Self;

        fn maybe_as(this: &Self) -> Option<&Self> {
            Some(this)
        }

        fn initialize_state_and_update(
            this: Self,
            update: impl FnOnce(&Self),
            _: impl FnOnce(),
        ) -> Self::State {
            update(&this);
            this
        }

        fn maybe_update_value_with_state(
            this: Self,
            state: &mut Self::State,
            update: impl FnOnce(&Self),
            _: impl FnOnce(),
        ) {
            if *state == this {
                return;
            }
            update(&this);
            *state = this;
        }

        type ChildStr = StrWriting<String>;

        fn maybe_into_child_str(
            this: Self,
            supported: <Self as ValueType>::SupportIntoChildStr,
        ) -> Option<Self::ChildStr> {
            String::maybe_into_child_str(this.to_string(), supported)
        }

        type AttrValue = AsyncWritableAttrValueStr<StrWriting<String>>;

        fn maybe_into_attr_value(this: Self, supported: ()) -> Option<Self::AttrValue> {
            String::maybe_into_attr_value(this.to_string(), supported)
        }

        type UpdateWithState = Option<Self::State>;

        fn update_with_state(
            this: Self,
            state: &mut Self::UpdateWithState,
            updater: impl ValueUpdater<Self>,
        ) {
            if let Some(state) = state {
                if *state == this {
                    return;
                }
            }

            updater.update(&this);
            *state = Some(this);
        }
    }
);

impl MaybeUpdateValueWithState<bool> for bool {
    type State = Self;

    fn maybe_as(this: &Self) -> Option<&Self> {
        Some(this)
    }

    fn initialize_state_and_update(
        this: Self,
        update: impl FnOnce(&Self),
        _: impl FnOnce(),
    ) -> Self::State {
        update(&this);
        this
    }

    fn maybe_update_value_with_state(
        this: Self,
        state: &mut Self::State,
        update: impl FnOnce(&Self),
        _: impl FnOnce(),
    ) {
        if *state == this {
            return;
        }
        update(&this);
        *state = this;
    }

    type ChildStr = NeverWritable;

    fn maybe_into_child_str(
        this: Self,
        supported: <bool as ValueType>::SupportIntoChildStr,
    ) -> Option<Self::ChildStr> {
        match supported {}
    }

    type AttrValue = AsyncWritableAttrValueBooleanTrue;

    fn maybe_into_attr_value(
        this: Self,
        (): <bool as ValueType>::SupportIntoAttrValue,
    ) -> Option<Self::AttrValue> {
        if this {
            Some(AsyncWritableAttrValueBooleanTrue)
        } else {
            None
        }
    }

    type UpdateWithState = Option<Self::State>;

    fn update_with_state(
        this: Self,
        state: &mut Self::UpdateWithState,
        updater: impl ValueUpdater<Self>,
    ) {
        if let Some(state) = state {
            if *state == this {
                return;
            }
        }

        updater.update(&this);
        *state = Some(this);
    }
}
