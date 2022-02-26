use std::rc::Rc;
use wasm_bindgen::{prelude::Closure, UnwrapThrowExt};
use wasm_bindgen::{JsCast, JsValue};

use wasm_bindgen::prelude::wasm_bindgen;

use super::{IntoOptionalRc, IntoRc};

thread_local! {
    static CLOSURE_FREE_WITH_USIZE: Closure<dyn FnMut(usize)> =
        Closure::wrap(Box::new(|n| { unsafe { forgotten::try_free_with_usize(n) }; }) as Box<dyn FnMut(usize)>);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone, PartialEq)]
    type MutableRefUsizeSlice;
    #[wasm_bindgen(structural, method, getter)]
    fn current(this: &MutableRefUsizeSlice) -> Box<[usize]>;
    #[wasm_bindgen(structural, method, setter)]
    fn set_current(this: &MutableRefUsizeSlice, val: Box<[usize]>);

    type MySetter;
    #[wasm_bindgen(structural, method, js_name = "set_state")]
    fn set_state_with(this: &MySetter, get_value_from_old: &Closure<dyn Fn(usize) -> usize>);
}

impl MutableRefUsizeSlice {
    #[inline]
    fn current_as_vec(&self) -> Vec<usize> {
        self.current().into_vec()
    }
    #[inline]
    fn set_current_vec(&self, vec: Vec<usize>) {
        self.set_current(vec.into_boxed_slice())
    }
    #[inline]
    fn push_into_current(&self, state: usize) {
        let mut arr = self.current_as_vec();
        arr.push(state);
        self.set_current_vec(arr);
    }
}

#[derive(Debug)]
pub struct StateSetter<T: ?Sized> {
    js_setter: react_sys::UseStateUsizeObjectSetter,
    ref_state_updaters: MutableRefUsizeSlice,
    _data: std::marker::PhantomData<T>,
}

impl<T: ?Sized> PartialEq for StateSetter<T> {
    fn eq(&self, other: &Self) -> bool {
        self.ref_state_updaters == other.ref_state_updaters
    }
}

impl<T: ?Sized> Eq for StateSetter<T> {}

impl<T: ?Sized> Clone for StateSetter<T> {
    fn clone(&self) -> Self {
        Self {
            js_setter: self.js_setter.clone(),
            ref_state_updaters: self.ref_state_updaters.clone(),
            _data: std::marker::PhantomData,
        }
    }
}

impl<T: 'static + ?Sized> StateSetter<T> {
    fn new(
        js_setter: react_sys::UseStateUsizeObjectSetter,
        ref_state_updaters: MutableRefUsizeSlice,
    ) -> Self {
        Self {
            js_setter,
            ref_state_updaters,
            _data: std::marker::PhantomData,
        }
    }

    pub fn set<V: IntoRc<T>>(&self, v: V) {
        let v: Rc<T> = v.into_rc();
        let k = forgotten::forget(v).into_shared();
        let k = k.as_usize();
        self.js_setter.set_state(*k)
    }

    pub fn set_from_old<R: IntoRc<T>, F: 'static + Fn(&Rc<T>) -> R>(&self, dispatch: F) {
        self.set_optional_from_old(move |old| Some((&dispatch)(old).into_rc()))
    }

    pub fn set_optional_from_old<R: IntoOptionalRc<T>, F: 'static + Fn(&Rc<T>) -> R>(
        &self,
        dispatch: F,
    ) {
        let ref_state_updaters = self.ref_state_updaters.clone();
        let closure = move |old_key| {
            let old = unsafe { forgotten::try_get_with_usize::<Rc<T>>(&old_key) };
            let old = old.unwrap_throw();

            let new_state: Option<Rc<T>> = dispatch(&old).into_optional_rc();

            if let Some(new_state) = new_state {
                let new_k = forgotten::forget(new_state).into_shared();
                let new_k = *new_k.as_usize();
                ref_state_updaters.push_into_current(new_k);
                new_k
            } else {
                old_key
            }
        };

        let closure = Closure::wrap(Box::new(closure) as Box<dyn Fn(usize) -> usize>);

        let (k, closure) = forgotten::forget_and_get(closure);
        let k = k.into_shared();
        let k = k.as_usize();

        self.ref_state_updaters.push_into_current(*k);

        let setter: &MySetter = self.js_setter.unchecked_ref();
        setter.set_state_with(closure.as_ref());
    }
}

pub fn use_state_value<T: 'static + ?Sized>(initial_value: Rc<T>) -> (Rc<T>, StateSetter<T>) {
    use_state(move || Rc::clone(&initial_value))
}

pub fn use_state<T: 'static + ?Sized, F: Fn() -> Rc<T>>(
    get_initial_value: F,
) -> (Rc<T>, StateSetter<T>) {
    let ref_all_persisted = use_ref_all_persisted();

    let ret = react_sys::use_state_usize_with(&mut || {
        let state = get_initial_value();
        let k = forgotten::forget(state);
        let k = k.into_shared();
        let k = *k.as_usize();
        let mut arr = ref_all_persisted.current().into_vec();
        arr.push(k);
        ref_all_persisted.set_current(arr.into_boxed_slice());
        k
    });

    let state = ret.value();

    use_clean_outdated_persisted(state, ref_all_persisted.clone());

    let state = unsafe { forgotten::try_get_with_usize::<Rc<T>>(&state) };
    let state = state.unwrap_throw();
    let state = Rc::clone(state.as_ref());

    let setter = ret.setter();

    (state, StateSetter::new(setter, ref_all_persisted))
}

fn use_ref_all_persisted() -> MutableRefUsizeSlice {
    let ref_all_persisted = react_sys::use_ref(&JsValue::UNDEFINED);
    let not_initialized = ref_all_persisted.current().is_falsy();
    let ref_all_persisted: MutableRefUsizeSlice = ref_all_persisted.unchecked_into();
    if not_initialized {
        ref_all_persisted.set_current_vec(Vec::<usize>::with_capacity(4));
    }
    ref_all_persisted
}

fn use_clean_outdated_persisted(state: usize, ref_all_persisted: MutableRefUsizeSlice) {
    crate::use_effect!(
        (state, ref_all_persisted) => {
            let state = state.as_ref();
            let state_updaters = ref_all_persisted.current();
            if state_updaters.len()>0 {
                for k in state_updaters.iter() {
                    if k == state {
                        continue;
                    }
                    #[cfg(debug_assertions)]
                    {
                        let cleaned = unsafe { forgotten::try_free_with_usize(*k) };
                        if !cleaned {
                            web_sys::console::error_2(
                                &"use_state*: forgotten key invalid or already dropped. forgotten::Key = "
                                    .into(),
                                &(*k).into(),
                            )
                        }
                    }

                    #[cfg(not(debug_assertions))]
                    unsafe {
                        forgotten::try_free_with_usize(k)
                    };
                }
                let mut state_updaters = Vec::with_capacity(4);
                state_updaters.push(*state);
                ref_all_persisted.set_current_vec(state_updaters);
            }
        }
    );
}

#[macro_export]
macro_rules! use_state {
    ($e:expr) => {
        $crate::use_state_value($crate::auto_wrap_rc!($e))
    };
    (() => $e:expr) => {
        $crate::use_state(move || $crate::auto_wrap_rc!($e))
    };
}
