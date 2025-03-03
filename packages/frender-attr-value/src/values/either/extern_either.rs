use either::Either;

use crate::{AttrValue, AttrValueKind};

impl<A: AttrValue<AK>, B: AttrValue<AK>, AK: AttrValueKind> AttrValue<AK> for Either<A, B> {}

#[cfg(feature = "ssr")]
mod ssr {
    use either::Either;

    use crate::{ssr::SsrAttrValue, AttrValueKind};

    impl<L: SsrAttrValue<AK>, R: SsrAttrValue<AK>, AK: AttrValueKind> SsrAttrValue<AK>
        for Either<L, R>
    {
        type HtmlAttributeValue =
            async_str_iter::either::IterEither<L::HtmlAttributeValue, R::HtmlAttributeValue>;

        fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
            match this {
                Either::Left(this) => SsrAttrValue::maybe_into_html_attribute_value(this)
                    .map(async_str_iter::either::IterEither::Left),
                Either::Right(this) => SsrAttrValue::maybe_into_html_attribute_value(this)
                    .map(async_str_iter::either::IterEither::Right),
            }
        }
    }
}

#[cfg(feature = "csr")]
mod csr {
    use either::Either;

    use crate::{
        csr::{CsrAttrValue, UpdateAttrValue},
        values::EitherAttrValue,
        AttrValueKind,
    };

    fn map_either<L, R>(e: Either<L, R>) -> EitherAttrValue<L, R> {
        match e {
            Either::Left(e) => EitherAttrValue::A(e),
            Either::Right(e) => EitherAttrValue::B(e),
        }
    }

    impl<AK: AttrValueKind, L: CsrAttrValue<AK>, R: CsrAttrValue<AK>> CsrAttrValue<AK>
        for Either<L, R>
    {
        type State = Result<L::State, R::State>;

        fn update_absent_attribute_value_into_state(
            this: Self,
            updater: impl UpdateAttrValue<Kind = AK>,
        ) -> Self::State {
            <_>::update_absent_attribute_value_into_state(map_either(this), updater)
        }

        fn update_attribute_value_into_state(
            this: Self,
            updater: impl UpdateAttrValue<Kind = AK>,
        ) -> Self::State {
            <_>::update_attribute_value_into_state(map_either(this), updater)
        }

        fn can_skip_update(this: &Self, state: &Self::State) -> bool {
            match (this, state) {
                (Self::Left(this), Ok(state)) => L::can_skip_update(this, state),
                (Self::Right(this), Err(state)) => R::can_skip_update(this, state),
                _ => false,
            }
        }

        fn update_attribute_value_with_state(
            this: Self,
            updater: impl UpdateAttrValue<Kind = AK>,
            state: &mut Self::State,
        ) {
            <_>::update_attribute_value_with_state(map_either(this), updater, state)
        }

        fn force_update_attribute_value_with_state(
            this: Self,
            updater: impl UpdateAttrValue<Kind = AK>,
            state: &mut Self::State,
        ) {
            <_>::force_update_attribute_value_with_state(map_either(this), updater, state)
        }

        fn attribute_is_known_as_absent(state: &Self::State) -> bool {
            <EitherAttrValue<L, R>>::attribute_is_known_as_absent(state)
        }
    }
}
