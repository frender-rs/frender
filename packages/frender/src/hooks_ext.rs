use hooks::{ShareValue, ToOwnedShareValue};

use crate::fn_traits::{FnMut1, FnMut2};

pub mod callback;
#[cfg(todo)]
pub mod element;
pub mod form_control;
pub mod setter;

/// `to_*` methods require [`ToOwnedShareValue`] instead of [`Clone`], so that:
///  - for [`&SharedSignal`](hooks::SharedSignal), the value is cloned as expected
///  - for [`SignalEq<&SharedSignal>`](hooks::SignalEq), the inner value is cloned,
///    so the value becomes `SignalEq<SharedSignal>`
///    (rather than the reference is copied)
///  - for `GenSignal` and `SignalEq<GenSignal>`, the value is copied
pub trait ShareValueExt: ShareValue {
    fn into_controlled(self) -> form_control::SignalIntoControlledValue<Self>
    where
        Self: Sized,
    {
        form_control::SignalIntoControlledValue(self)
    }

    fn to_controlled(&self) -> form_control::SignalIntoControlledValue<Self::OwnedShareValue>
    where
        Self: ToOwnedShareValue,
    {
        self.to_owned_share_value().into_controlled()
    }

    fn into_set_form_control_value(self) -> setter::SetEventTargetFormControlValue<Self>
    where
        Self: Sized,
        for<'a> std::borrow::Cow<'a, str>: Into<Self::Value>,
    {
        setter::SetEventTargetFormControlValue(self)
    }

    fn to_set_form_control_value(
        &self,
    ) -> setter::SetEventTargetFormControlValue<Self::OwnedShareValue>
    where
        Self: ToOwnedShareValue,
        for<'a> std::borrow::Cow<'a, str>: Into<Self::Value>, // TODO: better constraints
    {
        self.to_owned_share_value().into_set_form_control_value()
    }

    #[cfg(todo)]
    fn into_element(self) -> element::SignalIntoElement<Self, element::WithToElement>
    where
        Self: Sized,
        Self::Value: crate::ToElement, // TODO: relax to ToSsrElement
    {
        element::SignalIntoElement(self, element::WithToElement)
    }

    #[cfg(todo)]
    fn to_element(
        &self,
    ) -> element::SignalIntoElement<Self::OwnedShareValue, element::WithToElement>
    where
        Self: Sized + ToOwnedShareValue,
        Self::Value: crate::ToElement, // TODO: relax to ToSsrElement
    {
        self.to_owned_share_value().into_element()
    }

    #[cfg(todo)]
    fn into_element_with_fn<F>(self, f: F) -> element::SignalIntoElement<Self, element::WithFn<F>>
    where
        Self: Sized,
        F: for<'a> FnMut1<&'a Self::Value>,
    {
        element::SignalIntoElement(self, element::WithFn(f))
    }

    #[cfg(todo)]
    fn to_element_with_fn<F>(
        &self,
        f: F,
    ) -> element::SignalIntoElement<Self::OwnedShareValue, element::WithFn<F>>
    where
        Self: Sized + ToOwnedShareValue,
        F: for<'a> FnMut1<&'a Self::Value>,
    {
        self.to_owned_share_value().into_element_with_fn(f)
    }

    #[cfg(todo)]
    fn into_element_with_memo<F, Dep>(
        self,
        f: F,
        dep: Dep,
    ) -> element::SignalIntoElement<Self, crate::Memo<F, Dep>>
    where
        Self: Sized,
        F: for<'a, 'b> FnMut2<&'a Self::Value, &'b Dep>,
        Dep: PartialEq,
    {
        element::SignalIntoElement(self, crate::Memo(f, dep))
    }

    #[cfg(todo)]
    fn to_element_with_memo<F, Dep>(
        &self,
        f: F,
        dep: Dep,
    ) -> element::SignalIntoElement<Self::OwnedShareValue, crate::Memo<F, Dep>>
    where
        Self: Sized + ToOwnedShareValue,
        F: for<'a, 'b> FnMut2<&'a Self::Value, &'b Dep>,
        Dep: PartialEq,
    {
        self.to_owned_share_value().into_element_with_memo(f, dep)
    }

    fn into_callback_toggle(self) -> callback::Toggle<Self>
    where
        Self: Sized + ShareValue<Value = bool>,
    {
        callback::Toggle(self)
    }

    fn to_callback_toggle(&self) -> callback::Toggle<Self::OwnedShareValue>
    where
        Self: ShareValue<Value = bool> + ToOwnedShareValue,
    {
        self.to_owned_share_value().into_callback_toggle()
    }
}

impl<S: ?Sized> ShareValueExt for S where S: ShareValue {}
