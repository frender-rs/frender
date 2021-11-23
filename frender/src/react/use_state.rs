use std::{any::Any, rc::Rc};
use wasm_bindgen::{prelude::Closure, UnwrapThrowExt};

use super::{IntoOptionalRc, IntoRc};

thread_local! {
    static CLOSURE_FREE_WITH_USIZE: Closure<dyn FnMut(usize)> =
        Closure::wrap(Box::new(|n| { unsafe { forgotten::try_free_with_usize(n) }; }) as Box<dyn FnMut(usize)>);
}

#[derive(Debug)]
pub struct StateSetter<T: Any> {
    js_setter: react_sys::UseStateUsizeObjectSetter,
    _data: std::marker::PhantomData<T>,
}

impl<T: Any> Clone for StateSetter<T> {
    fn clone(&self) -> Self {
        Self {
            js_setter: self.js_setter.clone(),
            _data: std::marker::PhantomData,
        }
    }
}

impl<T: Any> StateSetter<T> {
    fn new(js_setter: react_sys::UseStateUsizeObjectSetter) -> Self {
        Self {
            js_setter,
            _data: std::marker::PhantomData,
        }
    }

    pub fn set<V: IntoRc<T>>(&self, v: V) {
        let v: Rc<T> = v.into_rc();
        let k = forgotten::forget_rc(v);
        let k = k.into_shared();
        let k = k.as_usize();
        self.js_setter.set_state(*k)
    }

    pub fn set_from_old<R: IntoRc<T>, F: FnOnce(&Rc<T>) -> R>(&self, dispatch: F) {
        self.set_optional_from_old(|old| Some(dispatch(old).into_rc()))
    }

    pub fn set_optional_from_old<R: IntoOptionalRc<T>, F: FnOnce(&Rc<T>) -> R>(&self, dispatch: F) {
        let mut dispatch = Some(dispatch);

        self.js_setter.set_state_with(&mut |old_key| {
            let dispatch = dispatch
                .take()
                .expect_throw("React.setState(fn) fn should only be called once");

            let k = unsafe { forgotten::SharedForgottenKey::<T>::from_usize(old_key) };

            let old = forgotten::try_get(&k).unwrap();

            let new_state: Option<Rc<T>> = dispatch(&old).into_optional_rc();

            if let Some(new_state) = new_state {
                let new_k = forgotten::forget_rc(new_state).into_shared();
                *new_k.as_usize()
            } else {
                old_key
            }
        });
    }
}

pub fn use_state_value<T: Any, V: IntoRc<T>>(initial_value: V) -> (Rc<T>, StateSetter<T>) {
    use_state(move || initial_value)
}

pub fn use_state<T: Any, R: IntoRc<T>, F: FnOnce() -> R>(
    get_initial_value: F,
) -> (Rc<T>, StateSetter<T>) {
    let mut func = Some(get_initial_value);

    let ret = CLOSURE_FREE_WITH_USIZE.with(|closure| {
        react_sys::use_state_usize_auto_clean_with(
            &mut || -> usize {
                let func = func
                    .take()
                    .expect_throw("React.useState get_initial_value should only be called once");

                let state: Rc<T> = func().into_rc();

                let k = forgotten::forget_rc(state).into_shared();

                *k.as_usize()
            },
            closure,
        )
    });

    let state = ret.value();

    let state = unsafe { forgotten::SharedForgottenKey::<T>::from_usize(state) };
    let state = forgotten::try_get::<T>(&state).unwrap();

    let setter = ret.setter();

    (state, StateSetter::new(setter))
}
