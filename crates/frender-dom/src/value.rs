use std::{
    borrow::{Borrow, Cow},
    cell::{Cell, RefCell},
    rc::Rc,
};

use frender_events::callable::Callable;
use frender_html_common::{
    maybe_str::{IntoOneStringOrEmpty, MaybeStr},
    MaybeUpdateValueWithState,
};
use frender_ssr::html::assert::OneStringOrEmpty;

pub trait FormControlValue {
    type State<ForceValue, OnValueChangeEventListener>: Default;

    fn update_with_state<C: FormControlController>(
        this: Self,
        state: &mut Self::State<C::ForceValue, C::OnValueChangeEventListener>,
        controller: C,
    );

    type OneStringOrEmpty: OneStringOrEmpty;

    fn into_one_string_or_empty(this: Self) -> Self::OneStringOrEmpty;
}

pub trait FormControlController {
    fn set_default_value(&mut self, value: &str);
    fn set_value(&mut self, value: &str);

    /// When dropped, the form control's value should not be forced.
    type ForceValue;

    fn force_value<V: TempAsRef<str> + 'static>(&mut self, value: V) -> Self::ForceValue;

    type OnValueChangeEventListener;

    fn on_value_change(
        &mut self,
        f: impl FnMut(Cow<'_, str>) + 'static,
    ) -> Self::OnValueChangeEventListener;
}

pub trait TempAsRef<T: ?Sized> {
    fn temp_as_ref<U: FnOnce(&T) -> R, R>(&self, use_value: U) -> R;
}

impl<V: TempAsRef<T>, T: ?Sized> TempAsRef<T> for Rc<V> {
    fn temp_as_ref<U: FnOnce(&T) -> R, R>(&self, use_value: U) -> R {
        V::temp_as_ref(&*self, use_value)
    }
}

/// Currently the implementation is
///
/// For `input` and `textarea`,
///
/// ```js
/// e.target.defaultValue = value;
/// e.target.value = value;
///
/// // only register once for the same `value`
/// element.addEventListener("beforeinput", (e) => {
///     e.preventDefault()
/// })
/// ```
///
/// For `select`,
///
/// ```js
/// e.target.defaultValue = value;
/// e.target.value = value;
///
/// // only register once for the same `value`
/// element.addEventListener("input", (e) => {
///     e.target.defaultValue = value;
///     e.target.value = value;
/// })
/// ```
pub struct OneWayBinding<V: Value<str>>(pub V);

pub trait Value<T: ?Sized>: Borrow<T> + IntoOneStringOrEmpty {
    type InteriorMutable: InteriorMutableValue<T, OwnedRef = Self> + 'static;

    // TODO: change signature to this: Self
    fn into_interior_mutable(self) -> Self::InteriorMutable;
}

impl Value<str> for &'static str {
    type InteriorMutable = Cell<Self>;

    fn into_interior_mutable(self) -> Self::InteriorMutable {
        Cell::new(self)
    }
}

impl Value<str> for String {
    type InteriorMutable = RefCell<Self>;

    fn into_interior_mutable(self) -> Self::InteriorMutable {
        RefCell::new(self)
    }
}

impl Value<str> for Cow<'static, str> {
    type InteriorMutable = RefCell<Self>;

    fn into_interior_mutable(self) -> Self::InteriorMutable {
        RefCell::new(self)
    }
}

pub trait InteriorMutableValue<T: ?Sized>: TempAsRef<T> {
    type OwnedRef: Borrow<T>;
    /// return true if replaced.
    fn replace_if_and_then<R>(
        &self,
        new_value: Self::OwnedRef,
        predicate: impl FnOnce(&T, &T) -> bool,
        and_then: impl FnOnce(&T) -> R,
    ) -> Option<R>;
}

impl<V: Copy + Borrow<T>, T: ?Sized> TempAsRef<T> for Cell<V> {
    fn temp_as_ref<U: FnOnce(&T) -> R, R>(&self, use_value: U) -> R {
        use_value(self.get().borrow())
    }
}
impl<V: Copy + Borrow<T>, T: ?Sized> InteriorMutableValue<T> for Cell<V> {
    type OwnedRef = V;

    fn replace_if_and_then<R>(
        &self,
        new_value: Self::OwnedRef,
        predicate: impl FnOnce(&T, &T) -> bool,
        and_then: impl FnOnce(&T) -> R,
    ) -> Option<R> {
        let old_value = self.get();
        let old_value = old_value.borrow();
        let v = new_value.borrow();
        if predicate(old_value, v) {
            let res = and_then(v);
            self.set(new_value);
            Some(res)
        } else {
            None
        }
    }
}

impl<V: Borrow<T>, T: ?Sized> TempAsRef<T> for RefCell<V> {
    fn temp_as_ref<U: FnOnce(&T) -> R, R>(&self, use_value: U) -> R {
        let v = self.borrow();
        let v = V::borrow(&v);
        use_value(v)
    }
}

impl<V: Borrow<T>, T: ?Sized> InteriorMutableValue<T> for RefCell<V> {
    type OwnedRef = V;

    fn replace_if_and_then<R>(
        &self,
        new_value: Self::OwnedRef,
        predicate: impl FnOnce(&T, &T) -> bool,
        and_then: impl FnOnce(&T) -> R,
    ) -> Option<R> {
        let old_value = &mut *self.borrow_mut();
        let v = new_value.borrow();
        if predicate((*old_value).borrow(), v) {
            let res = and_then(v);
            *old_value = new_value;
            Some(res)
        } else {
            None
        }
    }
}

pub struct OneWayBindingState<V, F> {
    value: Rc<V>,
    #[allow(dead_code)]
    force_value: F,
}

impl<V: Value<str>> FormControlValue for OneWayBinding<V> {
    type State<ForceValue, OnValueChangeEventListener> =
        Option<OneWayBindingState<V::InteriorMutable, ForceValue>>;

    fn update_with_state<C: FormControlController>(
        this: Self,
        state: &mut Self::State<C::ForceValue, C::OnValueChangeEventListener>,
        mut controller: C,
    ) {
        let owned_value = this.0;
        let value = owned_value.borrow();
        if let Some(state) = state {
            _ = state
                .value
                .replace_if_and_then(owned_value, str::ne, |value| {
                    controller.set_default_value(value);
                    controller.set_value(value);
                });
        } else {
            controller.set_default_value(&value);
            controller.set_value(&value);

            let v = Rc::new(V::into_interior_mutable(owned_value));
            *state = Some(OneWayBindingState {
                value: Rc::clone(&v),
                force_value: controller.force_value(v),
            })
        }
    }

    type OneStringOrEmpty = V::OneStringOrEmpty;

    fn into_one_string_or_empty(this: Self) -> Self::OneStringOrEmpty {
        V::into_one_string_or_empty(this.0)
    }
}

pub struct Controlled<V: Value<str>, C: Callable<(String,), Output = ()> + 'static>(pub V, pub C);

pub struct ControlledState<V, Cbk, EL> {
    value: V,
    callback: Cbk,
    on_value_change_event_listener: EL,
}

impl<V: Value<str>, Cbk: Callable<(String,), Output = ()> + PartialEq + Clone> FormControlValue
    for Controlled<V, Cbk>
{
    type State<ForceValue, OnValueChangeEventListener> =
        Option<ControlledState<V, Cbk, OnValueChangeEventListener>>;

    fn update_with_state<C: FormControlController>(
        this: Self,
        state: &mut Self::State<C::ForceValue, C::OnValueChangeEventListener>,
        mut controller: C,
    ) {
        if let Some(state) = state {
            let Self(value, cbk) = this;
            let v = value.borrow();
            if state.value.borrow() != v {
                controller.set_default_value(v);
                controller.set_value(v);
                state.value = value;
            }

            if state.callback != cbk {
                state.callback = cbk.clone();
                state.on_value_change_event_listener =
                    controller.on_value_change(move |v| cbk.call_fn((v.into_owned(),)));
            }
        } else {
            let Self(value, cbk) = this;

            {
                let v = value.borrow();
                controller.set_default_value(v);
                controller.set_value(v);
            }

            *state = Some(ControlledState {
                callback: Cbk::clone(&cbk),
                on_value_change_event_listener: controller
                    .on_value_change(move |v| cbk.call_fn((v.into_owned(),))),
                value,
            })
        }
    }

    type OneStringOrEmpty = V::OneStringOrEmpty;

    fn into_one_string_or_empty(this: Self) -> Self::OneStringOrEmpty {
        V::into_one_string_or_empty(this.0)
    }
}

/// Uncontrolled form control value (no default value).
impl FormControlValue for () {
    type State<ForceValue, OnValueChangeEventListener> = ();

    fn update_with_state<C: FormControlController>(
        (): Self,
        (): &mut Self::State<C::ForceValue, C::OnValueChangeEventListener>,
        _: C,
    ) {
    }

    type OneStringOrEmpty = async_str_iter::empty::Empty;

    fn into_one_string_or_empty((): Self) -> Self::OneStringOrEmpty {
        async_str_iter::empty::Empty
    }
}

pub struct UncontrolledWithDefaultValue<V: MaybeStr>(pub V);

impl<V: MaybeStr> FormControlValue for UncontrolledWithDefaultValue<V> {
    type State<ForceValue, OnValueChangeEventListener> = V::UpdateWithState;

    fn update_with_state<C: FormControlController>(
        this: Self,
        state: &mut Self::State<C::ForceValue, C::OnValueChangeEventListener>,
        controller: C,
    ) {
        struct UpdateDefaultValue<C: FormControlController>(C);

        impl<C: FormControlController> frender_html_common::ValueUpdater<str> for UpdateDefaultValue<C> {
            fn update(mut self, value: &str) {
                self.0.set_default_value(value)
            }

            fn remove(mut self) {
                self.0.set_default_value("")
            }
        }
        V::update_with_state(this.0, state, UpdateDefaultValue(controller))
    }

    type OneStringOrEmpty = V::OneStringOrEmpty;

    fn into_one_string_or_empty(this: Self) -> Self::OneStringOrEmpty {
        V::into_one_string_or_empty(this.0)
    }
}
