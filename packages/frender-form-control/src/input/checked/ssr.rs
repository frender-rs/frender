use frender_dom::Empty;

use crate::{
    provide::{MaybeProvideFormControlValue, NeverProvideFormControlValue},
    value::KindOfChecked,
    values::{EitherFormControlValue, UncontrolledWithDefaultValue},
};

use super::InputChecked;

pub trait SsrInputChecked: InputChecked {
    type IntoSsrInputChecked: MaybeProvideFormControlValue<KindOfChecked>;
    fn into_ssr_input_checked(self) -> Self::IntoSsrInputChecked;
}

impl SsrInputChecked for Empty {
    type IntoSsrInputChecked = Option<NeverProvideFormControlValue>;

    fn into_ssr_input_checked(self) -> Self::IntoSsrInputChecked {
        None
    }
}

impl SsrInputChecked for bool {
    type IntoSsrInputChecked = Self;

    fn into_ssr_input_checked(self) -> Self::IntoSsrInputChecked {
        self
    }
}

impl<T: MaybeProvideFormControlValue<KindOfChecked>> SsrInputChecked
    for UncontrolledWithDefaultValue<T>
{
    type IntoSsrInputChecked = T;

    fn into_ssr_input_checked(self) -> Self::IntoSsrInputChecked {
        self.0
    }
}

impl<T: SsrInputChecked> SsrInputChecked for Option<T> {
    type IntoSsrInputChecked = Option<T::IntoSsrInputChecked>;

    fn into_ssr_input_checked(self) -> Self::IntoSsrInputChecked {
        self.map(T::into_ssr_input_checked)
    }
}

impl<A: SsrInputChecked, B: SsrInputChecked> SsrInputChecked for EitherFormControlValue<A, B> {
    type IntoSsrInputChecked =
        EitherFormControlValue<A::IntoSsrInputChecked, B::IntoSsrInputChecked>;

    fn into_ssr_input_checked(self) -> Self::IntoSsrInputChecked {
        match self {
            EitherFormControlValue::A(this) => {
                EitherFormControlValue::A(A::into_ssr_input_checked(this))
            }
            EitherFormControlValue::B(this) => {
                EitherFormControlValue::B(B::into_ssr_input_checked(this))
            }
        }
    }
}

#[cfg(feature = "either")]
impl<A: SsrInputChecked, B: SsrInputChecked> SsrInputChecked for either::Either<A, B> {
    type IntoSsrInputChecked =
        EitherFormControlValue<A::IntoSsrInputChecked, B::IntoSsrInputChecked>;

    fn into_ssr_input_checked(self) -> Self::IntoSsrInputChecked {
        EitherFormControlValue::from_either(self).into_ssr_input_checked()
    }
}
