#[derive(Debug, Clone, Copy)]
pub enum EitherAttrValue<A, B> {
    A(A),
    B(B),
}

mod ssr {
    use crate::ssr::SsrAttrValue;

    use super::EitherAttrValue;

    impl<A: SsrAttrValue<AttributeType>, B: SsrAttrValue<AttributeType>, AttributeType: ?Sized>
        SsrAttrValue<AttributeType> for EitherAttrValue<A, B>
    {
        type HtmlAttributeValue =
            async_str_iter::either::IterEither<A::HtmlAttributeValue, B::HtmlAttributeValue>;

        fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
            match this {
                Self::A(this) => SsrAttrValue::maybe_into_html_attribute_value(this)
                    .map(async_str_iter::either::IterEither::Left),
                Self::B(this) => SsrAttrValue::maybe_into_html_attribute_value(this)
                    .map(async_str_iter::either::IterEither::Right),
            }
        }
    }
}

mod csr {
    use crate::csr::{CsrAttrValue, UpdateAttrValue, ValueKind};

    use super::EitherAttrValue;

    impl<V: ?Sized + ValueKind, A: CsrAttrValue<V>, B: CsrAttrValue<V>> CsrAttrValue<V>
        for EitherAttrValue<A, B>
    {
        type State = Result<A::State, B::State>;

        fn update_absent_attribute_value_into_state(
            this: Self,
            updater: impl UpdateAttrValue<Kind = V>,
        ) -> Self::State {
            match this {
                EitherAttrValue::A(this) => {
                    Ok(A::update_absent_attribute_value_into_state(this, updater))
                }
                EitherAttrValue::B(this) => {
                    Err(B::update_absent_attribute_value_into_state(this, updater))
                }
            }
        }

        fn update_attribute_value_into_state(
            this: Self,
            updater: impl UpdateAttrValue<Kind = V>,
        ) -> Self::State {
            match this {
                EitherAttrValue::A(this) => Ok(A::update_attribute_value_into_state(this, updater)),
                EitherAttrValue::B(this) => {
                    Err(B::update_attribute_value_into_state(this, updater))
                }
            }
        }

        fn can_skip_update(this: &Self, state: &Self::State) -> bool {
            match (this, state) {
                (EitherAttrValue::A(this), Ok(state)) => A::can_skip_update(this, state),
                (EitherAttrValue::B(this), Err(state)) => B::can_skip_update(this, state),
                _ => false,
            }
        }

        fn update_attribute_value_with_state(
            this: Self,
            updater: impl UpdateAttrValue<Kind = V>,
            state: &mut Self::State,
        ) {
            match (this, &mut *state) {
                (EitherAttrValue::A(this), Ok(state)) => {
                    A::update_attribute_value_with_state(this, updater, state)
                }
                (EitherAttrValue::B(this), Err(state)) => {
                    B::update_attribute_value_with_state(this, updater, state)
                }
                (EitherAttrValue::A(this), Err(old_state)) => {
                    *state = Ok(if B::attribute_is_known_as_absent(old_state) {
                        A::update_absent_attribute_value_into_state(this, updater)
                    } else {
                        A::update_attribute_value_into_state(this, updater)
                    })
                }
                (EitherAttrValue::B(this), Ok(old_state)) => {
                    *state = Err(if A::attribute_is_known_as_absent(old_state) {
                        B::update_absent_attribute_value_into_state(this, updater)
                    } else {
                        B::update_attribute_value_into_state(this, updater)
                    })
                }
            }
        }

        fn force_update_attribute_value_with_state(
            this: Self,
            updater: impl UpdateAttrValue<Kind = V>,
            state: &mut Self::State,
        ) {
            match (this, &mut *state) {
                (EitherAttrValue::A(this), Ok(state)) => {
                    A::force_update_attribute_value_with_state(this, updater, state)
                }
                (EitherAttrValue::B(this), Err(state)) => {
                    B::force_update_attribute_value_with_state(this, updater, state)
                }
                (EitherAttrValue::A(this), Err(old_state)) => {
                    *state = Ok(if B::attribute_is_known_as_absent(old_state) {
                        A::update_absent_attribute_value_into_state(this, updater)
                    } else {
                        A::update_attribute_value_into_state(this, updater)
                    })
                }
                (EitherAttrValue::B(this), Ok(old_state)) => {
                    *state = Err(if A::attribute_is_known_as_absent(old_state) {
                        B::update_absent_attribute_value_into_state(this, updater)
                    } else {
                        B::update_attribute_value_into_state(this, updater)
                    })
                }
            }
        }

        fn attribute_is_known_as_absent(state: &Self::State) -> bool {
            match state {
                Ok(state) => A::attribute_is_known_as_absent(state),
                Err(state) => B::attribute_is_known_as_absent(state),
            }
        }
    }
}

#[cfg(feature = "either")]
mod extern_either {
    mod ssr {
        #[cfg(feature = "either")]
        mod either {
            use either::Either;

            use crate::ssr::SsrAttrValue;

            impl<
                    L: SsrAttrValue<AttributeType>,
                    R: SsrAttrValue<AttributeType>,
                    AttributeType: ?Sized,
                > SsrAttrValue<AttributeType> for Either<L, R>
            {
                type HtmlAttributeValue = async_str_iter::either::IterEither<
                    L::HtmlAttributeValue,
                    R::HtmlAttributeValue,
                >;

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
    }

    mod csr {
        use either::Either;

        use crate::{
            csr::{CsrAttrValue, UpdateAttrValue, ValueKind},
            values::EitherAttrValue,
        };

        fn map_either<L, R>(e: Either<L, R>) -> EitherAttrValue<L, R> {
            match e {
                Either::Left(e) => EitherAttrValue::A(e),
                Either::Right(e) => EitherAttrValue::B(e),
            }
        }

        impl<V: ?Sized + ValueKind, L: CsrAttrValue<V>, R: CsrAttrValue<V>> CsrAttrValue<V>
            for Either<L, R>
        {
            type State = Result<L::State, R::State>;

            fn update_absent_attribute_value_into_state(
                this: Self,
                updater: impl UpdateAttrValue<Kind = V>,
            ) -> Self::State {
                <_>::update_absent_attribute_value_into_state(map_either(this), updater)
            }

            fn update_attribute_value_into_state(
                this: Self,
                updater: impl UpdateAttrValue<Kind = V>,
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
                updater: impl UpdateAttrValue<Kind = V>,
                state: &mut Self::State,
            ) {
                <_>::update_attribute_value_with_state(map_either(this), updater, state)
            }

            fn force_update_attribute_value_with_state(
                this: Self,
                updater: impl UpdateAttrValue<Kind = V>,
                state: &mut Self::State,
            ) {
                <_>::force_update_attribute_value_with_state(map_either(this), updater, state)
            }

            fn attribute_is_known_as_absent(state: &Self::State) -> bool {
                <EitherAttrValue<L, R>>::attribute_is_known_as_absent(state)
            }
        }
    }
}
