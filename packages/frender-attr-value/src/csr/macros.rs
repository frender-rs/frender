// always the same value
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

pub(crate) use {impl_csr_attr_value_for_unit_struct, impl_csr_attr_value_with_cache};
