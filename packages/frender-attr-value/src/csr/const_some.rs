use super::CsrAttrValueState;

pub struct State;

impl CsrAttrValueState for State {}

// always the same value
macro_rules! impl_csr_attr_value_for_const_some {
    (( $v:expr ) as $kind:ty) => {
        type State = crate::csr::const_some::State;

        fn render_init_on_absent_attribute(
            this: Self,
            updater: impl $crate::csr::UpdateAttrValue<Kind = $kind>,
        ) -> Self::State {
            <Self as $crate::csr::CsrAttrValue<$kind>>::render_init(this, updater)
        }

        fn render_init(
            _: Self,
            updater: impl $crate::csr::UpdateAttrValue<Kind = $kind>,
        ) -> Self::State {
            updater.set($v);
            crate::csr::const_some::State
        }

        fn render_init_by_reusing_on_absent_attribute(
            this: Self,
            updater: impl $crate::csr::UpdateAttrValue<Kind = $kind>,
            state: &mut Self::State,
        ) {
            *state = <Self as $crate::csr::CsrAttrValue<$kind>>::render_init_on_absent_attribute(
                this, updater,
            )
        }

        fn render_init_by_reusing(
            this: Self,
            updater: impl $crate::csr::UpdateAttrValue<Kind = $kind>,
            state: &mut Self::State,
        ) {
            *state = <Self as $crate::csr::CsrAttrValue<$kind>>::render_init(this, updater)
        }

        fn render_update(
            _: Self,
            _: impl $crate::csr::UpdateAttrValue<Kind = $kind>,
            crate::csr::const_some::State: &mut Self::State,
        ) {
            // the value is always $v, so there is no new value
        }
    };
}

pub(crate) use impl_csr_attr_value_for_const_some;
