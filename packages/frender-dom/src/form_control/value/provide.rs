use super::FormControlValueKind;

pub trait MaybeProvideFormControlValue<VK: ?Sized + FormControlValueKind> {
    type ProvideFormControlValue: ProvideFormControlValue<VK>;

    fn maybe_into_provide_form_control_value(this: Self) -> Option<Self::ProvideFormControlValue>;
}

pub trait ProvideFormControlValue<VK: ?Sized + FormControlValueKind>:
    MaybeProvideFormControlValue<VK, ProvideFormControlValue = Self>
{
    fn provide_form_control_value<R>(&self, receive: impl FnOnce(&VK) -> R) -> R;
}

macro_rules! impl_maybe_provide_with_some {
    () => {
        type ProvideFormControlValue = Self;

        fn maybe_into_provide_form_control_value(
            this: Self,
        ) -> Option<Self::ProvideFormControlValue> {
            Some(this)
        }
    };
}

pub struct BorrowToProvideFormControlValue<V>(pub V);

impl<T: std::borrow::Borrow<VK>, VK: ?Sized + FormControlValueKind> MaybeProvideFormControlValue<VK>
    for BorrowToProvideFormControlValue<T>
{
    impl_maybe_provide_with_some! {}
}

impl<T: std::borrow::Borrow<VK>, VK: ?Sized + FormControlValueKind> ProvideFormControlValue<VK>
    for BorrowToProvideFormControlValue<T>
{
    fn provide_form_control_value<R>(&self, receive: impl FnOnce(&VK) -> R) -> R {
        receive(self.0.borrow())
    }
}

pub enum NeverProvideFormControlValue {}

impl<VK: ?Sized + FormControlValueKind> MaybeProvideFormControlValue<VK>
    for NeverProvideFormControlValue
{
    impl_maybe_provide_with_some! {}
}

impl<VK: ?Sized + FormControlValueKind> ProvideFormControlValue<VK>
    for NeverProvideFormControlValue
{
    fn provide_form_control_value<R>(&self, _: impl FnOnce(&VK) -> R) -> R {
        match *self {}
    }
}

macro_rules! provide_self {
    ($($ty:ty),* $(,)?) => {
        $(
            impl MaybeProvideFormControlValue<$ty> for $ty {
                impl_maybe_provide_with_some! {}
            }

            impl ProvideFormControlValue<$ty> for $ty {
                fn provide_form_control_value<R>(&self, receive: impl FnOnce(&$ty) -> R) -> R {
                    receive(self)
                }
            }
        )*
    };
}

provide_self!(f64, bool);

frender_common::impl_many!(
    impl<__> MaybeProvideFormControlValue<str>
        for each_of![
            //
            &str,
            String,
            std::borrow::Cow<'_, str>,
            std::rc::Rc<str>,
            std::sync::Arc<str>
        ]
    {
        impl_maybe_provide_with_some! {}
    }
);

frender_common::impl_many!(
    impl<__> ProvideFormControlValue<str>
        for each_of![
            //
            &str,
            String,
            std::borrow::Cow<'_, str>,
            std::rc::Rc<str>,
            std::sync::Arc<str>,
        ]
    {
        fn provide_form_control_value<R>(&self, receive: impl FnOnce(&str) -> R) -> R {
            receive(self)
        }
    }
);

// nothing
impl<VK: ?Sized + FormControlValueKind> MaybeProvideFormControlValue<VK> for () {
    type ProvideFormControlValue = NeverProvideFormControlValue;

    fn maybe_into_provide_form_control_value((): Self) -> Option<Self::ProvideFormControlValue> {
        None
    }
}

// option
impl<T: MaybeProvideFormControlValue<VK>, VK: ?Sized + FormControlValueKind>
    MaybeProvideFormControlValue<VK> for Option<T>
{
    type ProvideFormControlValue = T::ProvideFormControlValue;

    fn maybe_into_provide_form_control_value(this: Self) -> Option<Self::ProvideFormControlValue> {
        this.and_then(T::maybe_into_provide_form_control_value)
    }
}

// either
#[cfg(feature = "either")]
impl<
        //
        VK: ?Sized + FormControlValueKind,
        A: ProvideFormControlValue<VK>,
        B: ProvideFormControlValue<VK>,
    > ProvideFormControlValue<VK> for either::Either<A, B>
{
    fn provide_form_control_value<R>(&self, receive: impl FnOnce(&VK) -> R) -> R {
        either::for_both!(self, this => this.provide_form_control_value(receive))
    }
}

#[cfg(feature = "either")]
impl<
        L: MaybeProvideFormControlValue<VK>,
        R: MaybeProvideFormControlValue<VK>,
        VK: ?Sized + FormControlValueKind,
    > MaybeProvideFormControlValue<VK> for either::Either<L, R>
{
    type ProvideFormControlValue =
        either::Either<L::ProvideFormControlValue, R::ProvideFormControlValue>;

    fn maybe_into_provide_form_control_value(this: Self) -> Option<Self::ProvideFormControlValue> {
        match this {
            either::Either::Left(this) => {
                L::maybe_into_provide_form_control_value(this).map(either::Either::Left)
            }
            either::Either::Right(this) => {
                R::maybe_into_provide_form_control_value(this).map(either::Either::Right)
            }
        }
    }
}
