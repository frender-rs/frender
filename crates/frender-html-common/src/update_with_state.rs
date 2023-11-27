use async_str_iter::{
    any_str::{AnyStr, IterAnyStr},
    AsyncStrIterator, IntoAsyncStrIterator,
};

use frender_ssr_html::{assert::HtmlAttributeEqValueOrEmpty, attr_value::AttrEqValue};

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

    type HtmlAttributeEqValueOrEmpty: HtmlAttributeEqValueOrEmpty;

    /// `None` indicates this attributes is not present
    fn maybe_into_html_attribute_eq_value_or_empty(
        this: Self,
    ) -> Option<Self::HtmlAttributeEqValueOrEmpty>;

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

    type UpdateWithState = ();

    fn update_with_state((): Self, (): &mut Self::UpdateWithState, _: impl ValueUpdater<V>) {}

    type HtmlAttributeEqValueOrEmpty = async_str_iter::never::Never;

    fn maybe_into_html_attribute_eq_value_or_empty(
        (): Self,
    ) -> Option<Self::HtmlAttributeEqValueOrEmpty> {
        None
    }
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

    type HtmlAttributeEqValueOrEmpty = T::HtmlAttributeEqValueOrEmpty;

    fn maybe_into_html_attribute_eq_value_or_empty(
        this: Self,
    ) -> Option<Self::HtmlAttributeEqValueOrEmpty> {
        this.and_then(T::maybe_into_html_attribute_eq_value_or_empty)
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

    type UpdateWithState = ();

    fn update_with_state(
        this: Self,
        _: &mut Self::UpdateWithState,
        updater: impl ValueUpdater<str>,
    ) {
        updater.update(this)
    }

    type HtmlAttributeEqValueOrEmpty = AttrEqValue<Self>;

    fn maybe_into_html_attribute_eq_value_or_empty(
        this: Self,
    ) -> Option<Self::HtmlAttributeEqValueOrEmpty> {
        Some(AttrEqValue(this))
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

    type HtmlAttributeEqValueOrEmpty = AttrEqValue<IterAnyStr<Self>>;

    fn maybe_into_html_attribute_eq_value_or_empty(
        this: Self,
    ) -> Option<Self::HtmlAttributeEqValueOrEmpty> {
        Some(AttrEqValue(AnyStr(this).into_async_str_iterator()))
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

    type HtmlAttributeEqValueOrEmpty = AttrEqValue<IterAnyStr<Self>>;

    fn maybe_into_html_attribute_eq_value_or_empty(
        this: Self,
    ) -> Option<Self::HtmlAttributeEqValueOrEmpty> {
        Some(AttrEqValue(AnyStr(this).into_async_str_iterator()))
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

        type HtmlAttributeEqValueOrEmpty = AttrEqValue<IterAnyStr<String>>;

        fn maybe_into_html_attribute_eq_value_or_empty(
            this: Self,
        ) -> Option<Self::HtmlAttributeEqValueOrEmpty> {
            String::maybe_into_html_attribute_eq_value_or_empty(this.to_string())
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

    type HtmlAttributeEqValueOrEmpty = async_str_iter::empty::Empty;

    fn maybe_into_html_attribute_eq_value_or_empty(
        this: Self,
    ) -> Option<Self::HtmlAttributeEqValueOrEmpty> {
        if this {
            Some(async_str_iter::empty::Empty)
        } else {
            None
        }
    }
}

pub struct BooleanTrue;

impl Unpin for BooleanTrue {}

impl AsyncStrIterator for BooleanTrue {
    fn poll_next_str(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<&str>> {
        std::task::Poll::Ready(None)
    }
}
