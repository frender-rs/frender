use frender_dom::Empty;
use frender_reactive_value::{value_kind::KindOfOwned, ReactiveValue};

use crate::{
    csr::value::FormControlValue,
    value::KindOfChecked,
    values::{EitherFormControlValue, UncontrolledEmptyDefaultValue, UncontrolledWithDefaultValue},
};

use super::InputChecked;

pub trait CsrInputChecked: InputChecked {
    type IntoCsrInputChecked: FormControlValue<KindOfChecked>;
    fn into_csr_input_checked(self) -> Self::IntoCsrInputChecked;
}

impl CsrInputChecked for Empty {
    type IntoCsrInputChecked = UncontrolledEmptyDefaultValue;

    fn into_csr_input_checked(self) -> Self::IntoCsrInputChecked {
        UncontrolledEmptyDefaultValue
    }
}
impl CsrInputChecked for bool {
    type IntoCsrInputChecked = UncontrolledWithDefaultValue<bool>;

    fn into_csr_input_checked(self) -> Self::IntoCsrInputChecked {
        UncontrolledWithDefaultValue(self)
    }
}
impl<T: ReactiveValue<KindOfOwned<bool>>> CsrInputChecked for UncontrolledWithDefaultValue<T> {
    type IntoCsrInputChecked = Self;

    fn into_csr_input_checked(self) -> Self::IntoCsrInputChecked {
        self
    }
}

impl<T: CsrInputChecked> CsrInputChecked for Option<T> {
    type IntoCsrInputChecked = Option<T::IntoCsrInputChecked>;

    fn into_csr_input_checked(self) -> Self::IntoCsrInputChecked {
        self.map(T::into_csr_input_checked)
    }
}

impl<A: CsrInputChecked, B: CsrInputChecked> CsrInputChecked for EitherFormControlValue<A, B> {
    type IntoCsrInputChecked =
        EitherFormControlValue<A::IntoCsrInputChecked, B::IntoCsrInputChecked>;

    fn into_csr_input_checked(self) -> Self::IntoCsrInputChecked {
        match self {
            Self::A(this) => EitherFormControlValue::A(A::into_csr_input_checked(this)),
            Self::B(this) => EitherFormControlValue::B(B::into_csr_input_checked(this)),
        }
    }
}

#[cfg(feature = "either")]
impl<A: CsrInputChecked, B: CsrInputChecked> CsrInputChecked for either::Either<A, B> {
    type IntoCsrInputChecked =
        EitherFormControlValue<A::IntoCsrInputChecked, B::IntoCsrInputChecked>;

    fn into_csr_input_checked(self) -> Self::IntoCsrInputChecked {
        match self {
            Self::Left(this) => EitherFormControlValue::A(this),
            Self::Right(this) => EitherFormControlValue::B(this),
        }
        .into_csr_input_checked()
    }
}
