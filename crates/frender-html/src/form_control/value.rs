use std::{
    borrow::{Borrow, Cow},
    cell::{Cell, RefCell},
    rc::Rc,
};

use frender_common::PrimarilyBorrow;
use frender_dom::{render_state::non_reactive::NonReactiveRenderState, RenderState};
use frender_html_common::{IntoOneStringOrEmpty, MaybeValue};

use super::element::FormControlElement;

pub trait Value {
    /// The value, reference, or `Cow` passed between functions.
    type Passed<'a>
    where
        Self: 'a;
}

pub trait OfValue: PrimarilyBorrow<Borrowed = Self::Value> {
    type Value: ?Sized + Value;

    /// `passed_value` might be cloned in this method.
    fn of_value(passed_value: <Self::Value as Value>::Passed<'_>) -> Self;
}

pub trait HandleValue<V: ?Sized + Value> {
    fn handle_value(&mut self, v: V::Passed<'_>);
}

impl<V: ?Sized + Value, F: for<'v> FnMut(V::Passed<'v>)> HandleValue<V> for F {
    fn handle_value(&mut self, v: <V as Value>::Passed<'_>) {
        self(v)
    }
}

impl Value for str {
    type Passed<'a> = Cow<'a, str>;
}

impl<T: Copy> Value for T {
    type Passed<'a> = T where T: 'a;
}

impl OfValue for String {
    type Value = str;

    fn of_value(passed_value: <Self::Value as Value>::Passed<'_>) -> Self {
        passed_value.into()
    }
}

impl OfValue for Cow<'_, str> {
    type Value = str;

    fn of_value(passed_value: <Self::Value as Value>::Passed<'_>) -> Self {
        passed_value.into_owned().into()
    }
}

impl OfValue for std::rc::Rc<str> {
    type Value = str;

    fn of_value(passed_value: <Self::Value as Value>::Passed<'_>) -> Self {
        passed_value.into()
    }
}

impl OfValue for bool {
    type Value = bool;

    fn of_value(passed_value: <Self::Value as Value>::Passed<'_>) -> Self {
        passed_value
    }
}

pub trait FormControlValue<V: ?Sized + Value> {
    type State<E: FormControlElement<V, R> + ?Sized, R: ?Sized>: Default + RenderState<E, R> + Unpin;

    fn update_with_state<E: FormControlElement<V, R> + ?Sized, R: ?Sized>(this: Self, state: &mut Self::State<E, R>, element: &mut E, renderer: &mut R);
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
pub struct OneWayBinding<V: OfValue>(pub V);

impl<V: OfValue<Value = str>> IntoOneStringOrEmpty for OneWayBinding<V> {
    type OneStringOrEmpty = async_str_iter::borrow_str::IterBorrowStr<V>;

    fn into_one_string_or_empty(this: Self) -> Self::OneStringOrEmpty {
        Self::OneStringOrEmpty::new(this.0)
    }
}

pub trait InteriorMutableValue<T: ?Sized>: TempAsRef<T> {
    type OwnedRef: Borrow<T>;
    /// return true if replaced.
    fn replace_if_and_then<R>(&self, new_value: Self::OwnedRef, predicate: impl FnOnce(&T, &T) -> bool, and_then: impl FnOnce(&T) -> R) -> Option<R>;
}

impl<V: Copy + Borrow<T>, T: ?Sized> TempAsRef<T> for Cell<V> {
    fn temp_as_ref<U: FnOnce(&T) -> R, R>(&self, use_value: U) -> R {
        use_value(self.get().borrow())
    }
}
impl<V: Copy + Borrow<T>, T: ?Sized> InteriorMutableValue<T> for Cell<V> {
    type OwnedRef = V;

    fn replace_if_and_then<R>(&self, new_value: Self::OwnedRef, predicate: impl FnOnce(&T, &T) -> bool, and_then: impl FnOnce(&T) -> R) -> Option<R> {
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

    fn replace_if_and_then<R>(&self, new_value: Self::OwnedRef, predicate: impl FnOnce(&T, &T) -> bool, and_then: impl FnOnce(&T) -> R) -> Option<R> {
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

impl<Val: OfValue<Value = V> + 'static, V: ?Sized + Value + PartialEq> FormControlValue<V> for OneWayBinding<Val> {
    type State<E: FormControlElement<V, R> + ?Sized, R: ?Sized> = NonReactiveRenderState<Option<OneWayBindingState<RefCell<Val>, E::ForceValue>>>;

    fn update_with_state<E: FormControlElement<V, R> + ?Sized, R: ?Sized>(this: Self, state: &mut Self::State<E, R>, element: &mut E, renderer: &mut R) {
        let state = &mut state.0;
        let owned_value = this.0;
        let value = owned_value.borrow();
        if let Some(state) = state {
            _ = state.value.replace_if_and_then(owned_value, V::ne, |value| {
                element.set_default_value(renderer, value);
                element.set_value(renderer, value);
            });
        } else {
            element.set_default_value(renderer, &value);
            element.set_value(renderer, &value);

            let v = Rc::new(RefCell::new(owned_value));
            *state = Some(OneWayBindingState {
                value: Rc::clone(&v),
                force_value: element.force_value(renderer, v),
            })
        }
    }
}

pub struct Controlled<
    //
    V: OfValue,
    C: FnMut(V) + 'static,
>(pub V, pub C);

impl<
        //
        V: OfValue<Value = str>,
        C: FnMut(V) + 'static,
    > IntoOneStringOrEmpty for Controlled<V, C>
{
    type OneStringOrEmpty = async_str_iter::borrow_str::IterBorrowStr<V>;

    fn into_one_string_or_empty(this: Self) -> Self::OneStringOrEmpty {
        Self::OneStringOrEmpty::new(this.0)
    }
}

pub struct HandleOfValue<Val, F: FnMut(Val)> {
    f: F,
    _v: std::marker::PhantomData<Val>,
}

impl<Val, F: FnMut(Val)> HandleOfValue<Val, F> {
    fn new(f: F) -> Self {
        Self { f, _v: std::marker::PhantomData }
    }
}

impl<V: ?Sized + Value, Val: OfValue<Value = V>, F: FnMut(Val)> HandleValue<V> for HandleOfValue<Val, F> {
    fn handle_value(&mut self, v: <V as Value>::Passed<'_>) {
        (self.f)(Val::of_value(v))
    }
}

impl<
        //
        V: ?Sized + Value + PartialEq,
        Val: OfValue<Value = V> + 'static,
        Cbk: FnMut(Val) + 'static,
    > FormControlValue<V> for Controlled<Val, Cbk>
{
    type State<E: FormControlElement<V, R> + ?Sized, R: ?Sized> = NonReactiveRenderState<(Option<Val>, E::OnValueChangeEventListener<HandleOfValue<Val, Cbk>>)>;

    fn update_with_state<E: FormControlElement<V, R> + ?Sized, R: ?Sized>(this: Self, state: &mut Self::State<E, R>, element: &mut E, renderer: &mut R) {
        let (state, event_listener) = &mut state.0;
        let Self(value, f) = this;

        element.on_value_change(renderer, event_listener, HandleOfValue::new(f));

        let v = value.borrow();
        if let Some(state) = state {
            if Borrow::<V>::borrow(state) == v {
                return;
            }
        }

        element.set_default_value(renderer, v);
        element.set_value(renderer, v);

        *state = Some(value);
    }
}

/// Uncontrolled form control value (no default value).
impl<V: ?Sized + Value> FormControlValue<V> for () {
    type State<E: FormControlElement<V, R> + ?Sized, R: ?Sized> = ();

    fn update_with_state<E: FormControlElement<V, R> + ?Sized, R: ?Sized>((): Self, (): &mut Self::State<E, R>, _: &mut E, _: &mut R) {}
}

pub struct UncontrolledWithDefaultValue<V: MaybeValue<str> + IntoOneStringOrEmpty>(pub V);

impl<V: MaybeValue<str> + IntoOneStringOrEmpty> IntoOneStringOrEmpty for UncontrolledWithDefaultValue<V> {
    type OneStringOrEmpty = V::OneStringOrEmpty;

    fn into_one_string_or_empty(this: Self) -> Self::OneStringOrEmpty {
        V::into_one_string_or_empty(this.0)
    }
}

impl<V: MaybeValue<str> + IntoOneStringOrEmpty> FormControlValue<str> for UncontrolledWithDefaultValue<V> {
    type State<E: FormControlElement<str, R> + ?Sized, R: ?Sized> = NonReactiveRenderState<V::UpdateWithState>;

    fn update_with_state<E: FormControlElement<str, R> + ?Sized, R: ?Sized>(this: Self, state: &mut Self::State<E, R>, element: &mut E, renderer: &mut R) {
        let state = &mut state.0;

        // TODO: refactor
        struct UpdateDefaultValue<'a, E: ?Sized + FormControlElement<str, R>, R: ?Sized> {
            element: &'a mut E,
            renderer: &'a mut R,
        }

        impl<'a, E: ?Sized + FormControlElement<str, R>, R: ?Sized> frender_html_common::ValueUpdater<str> for UpdateDefaultValue<'a, E, R> {
            fn update(self, value: &str) {
                self.element.set_default_value(self.renderer, value)
            }

            fn remove(self) {
                self.element.set_default_value(self.renderer, "")
            }
        }

        V::update_with_state(this.0, state, UpdateDefaultValue { element, renderer })
    }
}
