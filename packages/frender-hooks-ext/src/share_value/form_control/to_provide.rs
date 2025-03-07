use frender_common::impl_many;
use frender_form_control::{
    values::EitherFormControlValue, FormControlValueKind, KindOfChecked, KindOfValue,
    KindOfValueAsNumber,
};

pub trait ToProvideFormControlValue<VK: FormControlValueKind> {
    fn to_provide_form_control_value<Out>(&self, receive: impl FnOnce(VK::Value<'_>) -> Out)
        -> Out;
}

pub trait ToProvideFormControlValueWithKind:
    ToProvideFormControlValue<Self::ToProvideFormControlValueKind>
{
    type ToProvideFormControlValueKind: FormControlValueKind;
}

impl ToProvideFormControlValue<KindOfValueAsNumber> for f64 {
    fn to_provide_form_control_value<Out>(&self, receive: impl FnOnce(Self) -> Out) -> Out {
        receive(*self)
    }
}
impl ToProvideFormControlValueWithKind for f64 {
    type ToProvideFormControlValueKind = KindOfValueAsNumber;
}

impl ToProvideFormControlValue<KindOfChecked> for bool {
    fn to_provide_form_control_value<Out>(&self, receive: impl FnOnce(Self) -> Out) -> Out {
        receive(*self)
    }
}
impl ToProvideFormControlValueWithKind for bool {
    type ToProvideFormControlValueKind = KindOfChecked;
}

// region: str
trait KnownStr: AsRef<str> {}
impl_many!(
    impl<__> KnownStr
        for each_of![
            //
            str,
            &str,
            String,
            std::borrow::Cow<'_, str>,
            std::rc::Rc<str>,
            std::sync::Arc<str>,
        ]
    {
    }
);
impl<S: ?Sized + KnownStr> ToProvideFormControlValue<KindOfValue> for S {
    fn to_provide_form_control_value<Out>(&self, receive: impl FnOnce(&str) -> Out) -> Out {
        receive(self.as_ref())
    }
}
impl<S: ?Sized + KnownStr> ToProvideFormControlValueWithKind for S {
    type ToProvideFormControlValueKind = KindOfValue;
}
// endregion
// region: either
impl<
        VK: FormControlValueKind,
        A: ToProvideFormControlValue<VK>,
        B: ToProvideFormControlValue<VK>,
    > ToProvideFormControlValue<VK> for EitherFormControlValue<A, B>
{
    fn to_provide_form_control_value<Out>(
        &self,
        receive: impl FnOnce(<VK as FormControlValueKind>::Value<'_>) -> Out,
    ) -> Out {
        match self {
            EitherFormControlValue::A(this) => this.to_provide_form_control_value(receive),
            EitherFormControlValue::B(this) => this.to_provide_form_control_value(receive),
        }
    }
}
impl<
        VK: FormControlValueKind,
        A: ToProvideFormControlValueWithKind<ToProvideFormControlValueKind = VK>,
        B: ToProvideFormControlValueWithKind<ToProvideFormControlValueKind = VK>,
    > ToProvideFormControlValueWithKind for EitherFormControlValue<A, B>
{
    type ToProvideFormControlValueKind = VK;
}

#[cfg(feature = "either")]
mod extern_either {
    use either::Either;
    use frender_form_control::FormControlValueKind;

    use super::{ToProvideFormControlValue, ToProvideFormControlValueWithKind};

    impl<
            VK: FormControlValueKind,
            L: ToProvideFormControlValue<VK>,
            R: ToProvideFormControlValue<VK>,
        > ToProvideFormControlValue<VK> for Either<L, R>
    {
        fn to_provide_form_control_value<Out>(
            &self,
            receive: impl FnOnce(<VK as FormControlValueKind>::Value<'_>) -> Out,
        ) -> Out {
            either::for_both!(self, this => this.to_provide_form_control_value(receive))
        }
    }

    impl<
            VK: FormControlValueKind,
            L: ToProvideFormControlValueWithKind<ToProvideFormControlValueKind = VK>,
            R: ToProvideFormControlValueWithKind<ToProvideFormControlValueKind = VK>,
        > ToProvideFormControlValueWithKind for Either<L, R>
    {
        type ToProvideFormControlValueKind = VK;
    }
}
// endregion
