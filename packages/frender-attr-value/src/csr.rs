pub trait ValueKind: 'static {
    type Value<'a>;
}

pub trait UpdateAttrValue {
    type Kind: ?Sized + ValueKind;

    fn set(self, value: <Self::Kind as ValueKind>::Value<'_>);

    fn remove(self);
}

/// Unlike CsrStyle and CsrDomTokens, AttrValue doesn't allow chaining.
/// Thus, [`CsrAttrValue`] doesn't have method `remove_with_state`.
pub trait CsrAttrValue<V: ?Sized + ValueKind>: Sized {
    type State;

    fn update_absent_attribute_value_into_state(
        this: Self,
        updater: impl UpdateAttrValue<Kind = V>,
    ) -> Self::State {
        Self::update_attribute_value_into_state(this, updater)
    }

    fn update_attribute_value_into_state(
        this: Self,
        updater: impl UpdateAttrValue<Kind = V>,
    ) -> Self::State;

    fn can_skip_update(this: &Self, state: &Self::State) -> bool;

    fn update_attribute_value_with_state(
        this: Self,
        updater: impl UpdateAttrValue<Kind = V>,
        state: &mut Self::State,
    ) {
        if Self::can_skip_update(&this, state) {
            return;
        }
        Self::force_update_attribute_value_with_state(this, updater, state)
    }

    fn force_update_attribute_value_with_state(
        this: Self,
        updater: impl UpdateAttrValue<Kind = V>,
        state: &mut Self::State,
    ) {
        *state = if Self::attribute_is_known_as_absent(state) {
            Self::update_absent_attribute_value_into_state(this, updater)
        } else {
            Self::update_attribute_value_into_state(this, updater)
        }
    }

    /// Returning `true` implies that the attribute is absent.
    /// Returning `false` implies that the attribute might be absent.
    ///
    /// This method is to optimize `impl CsrAttrValue for Option, Either`
    ///
    /// Always returning `false` is correct.
    fn attribute_is_known_as_absent(state: &Self::State) -> bool {
        let _ = state;
        false
    }

    fn update_attribute_value_with_option_state(
        this: Self,
        updater: impl UpdateAttrValue<Kind = V>,
        state: &mut Option<Self::State>,
    ) {
        if let Some(state) = state {
            Self::update_attribute_value_with_state(this, updater, state)
        } else {
            *state = Some(Self::update_absent_attribute_value_into_state(
                this, updater,
            ))
        }
    }
}

// always the same value
#[macro_export]
macro_rules! impl_csr_attr_value_for_unit_struct {
    (( $v:expr ) as $kind:ty) => {
        type State = ();

        fn update_attribute_value_into_state(
            Self: Self,
            updater: impl $crate::csr::UpdateAttrValue<Kind = $kind>,
        ) -> Self::State {
            updater.set($v)
        }

        fn can_skip_update(Self: &Self, (): &Self::State) -> bool {
            true
        }

        fn update_attribute_value_with_state(
            Self: Self,
            _: impl $crate::csr::UpdateAttrValue<Kind = $kind>,
            (): &mut Self::State,
        ) {
            // self is always Empty, so there is no new value
        }

        fn force_update_attribute_value_with_state(
            this: Self,
            updater: impl $crate::csr::UpdateAttrValue<Kind = $kind>,
            (): &mut Self::State,
        ) {
            <Self as $crate::csr::CsrAttrValue<$kind>>::update_attribute_value_into_state(
                this, updater,
            )
        }
    };
}

#[macro_export]
macro_rules! impl_csr_attr_value_with_cache {
    (
        kind![$kind:ty],
        $(before_set = {$($before_set:tt)*},)?
        set = |$this:ident| $set:expr,
        $(
            into_cache = $into_cache:expr,
            $(update_cache = |$update_cache_state:pat_param| $update_cache_set:expr,)?
        )?
        eq = |$eq_this:pat_param, $eq_state:pat_param| $eq:expr $(,)?
    ) => {
        fn update_attribute_value_into_state(
            $this: Self,
            updater: impl $crate::csr::UpdateAttrValue<Kind = $kind>,
        ) -> Self::State {
            $($($before_set)*)?
            updater.set($set);
            $crate::csr::__private::expand! {
                {$($into_cache)?} or ($this)
            }
        }

        fn can_skip_update($eq_this: &Self, $eq_state: &Self::State) -> bool {
            $eq
        }

        $crate::csr::__private::expand! {
            if ($($($update_cache_state)?)?) {
                fn force_update_attribute_value_with_state(
                    $this: Self,
                    updater: impl $crate::csr::UpdateAttrValue<Kind = $kind>,
                    $($($update_cache_state)?)?: &mut Self::State,
                ) {
                    updater.set($($($update_cache_set)?)?);
                }
            } else {
                fn force_update_attribute_value_with_state(
                    this: Self,
                    updater: impl $crate::csr::UpdateAttrValue<Kind = $kind>,
                    state: &mut Self::State,
                ) {
                    *state = Self::update_attribute_value_into_state(this, updater);
                }
            }
        }
    };
    (
        kind![$kind:ty],
        $(before_set = $before_set:tt,)?
        set = |$this:ident| $set:expr,
        $(
            into_cache = $into_cache:expr,
            $(update_cache = |$update_cache_state:pat_param| $update_cache_set:expr,)?
        )?
        eq = $eq:expr $(,)?
    ) => {
        $crate::impl_csr_attr_value_with_cache! {
            kind![$kind],
            $(before_set = $before_set,)?
            set = |$this| $set,
            $(
                into_cache = $into_cache,
                $(update_cache = |$update_cache_state| $update_cache_set,)?
            )?
            eq = |this, state| $eq(this, state)
        }
    };
}

#[doc(hidden)]
pub mod __private {
    pub use frender_common::expand;
}
