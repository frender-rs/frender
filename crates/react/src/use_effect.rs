use once_cell::unsync::OnceCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;

use super::IntoRc;

pub trait CleanFnOrNone: Sized {
    type CleanFn: 'static + FnOnce();
    fn into_optional_clean_fn(self) -> Option<Self::CleanFn>;
}

impl<T> CleanFnOrNone for T
where
    T: 'static + FnOnce(),
{
    type CleanFn = T;
    fn into_optional_clean_fn(self) -> Option<T> {
        Some(self)
    }
}

impl<T> CleanFnOrNone for Option<T>
where
    T: 'static + FnOnce(),
{
    type CleanFn = T;
    fn into_optional_clean_fn(self) -> Option<T> {
        self
    }
}

impl CleanFnOrNone for () {
    type CleanFn = &'static dyn Fn();

    fn into_optional_clean_fn(self) -> Option<Self::CleanFn> {
        None
    }
}

pub trait ReactEffectFn: FnOnce() -> Self::Clean {
    type Clean: CleanFnOrNone;
}
impl<F: FnOnce() -> R, R: CleanFnOrNone> ReactEffectFn for F {
    type Clean = R;
}

thread_local! {
   static CLOSURE_NEVER_CALLED: Closure<dyn FnMut() -> JsValue> = Closure::once(|| {
       panic!("CLOSURE_NEVER_CALLED should never be called")
   })
}

/// `React.useEffect(effect, [])`
pub fn use_effect_on_mounted<F: 'static + ReactEffectFn>(effect: F) {
    let ref_effect_closure_key = react_sys::use_ref_usize(0);

    let closure = if ref_effect_closure_key.current() != 0 {
        None
    } else {
        let closure: Closure<dyn FnMut() -> JsValue> = {
            let ref_effect_closure_key = ref_effect_closure_key.clone();
            Closure::once(move || {
                let clean = effect().into_optional_clean_fn();

                let cell_clean_closure_key = OnceCell::<usize>::new();
                let real_clean = {
                    let cell_clean_closure_key = cell_clean_closure_key.clone();
                    Closure::once(move || {
                        if let Some(clean) = clean {
                            clean();
                        }

                        let effect_closure_key = ref_effect_closure_key.current();
                        let clean_closure_key = *cell_clean_closure_key.get().unwrap();
                        unsafe { forgotten::try_free_with_usize(effect_closure_key) };
                        unsafe { forgotten::try_free_with_usize(clean_closure_key) };
                    })
                };

                let (k, real_clean) = forgotten::forget_and_get(real_clean);

                let k = k.into_shared();
                let k = k.as_usize();
                cell_clean_closure_key.set(*k).unwrap();

                let v = real_clean.as_ref().as_ref();

                v.clone()
            })
        };

        let (k, closure) = forgotten::forget_and_get(closure);

        let k = k.into_shared();
        let k = k.as_usize();
        ref_effect_closure_key.set_current(*k);

        Some(closure)
    };

    // let closure = closure.as_ref();
    // let closure: &js_sys::Function = closure.dyn_ref().unwrap();

    CLOSURE_NEVER_CALLED.with(|never_called| {
        react_sys::use_effect_with_clean(
            closure.as_ref().map(|v| v.as_ref()).unwrap_or(never_called),
            Some(Box::new([])),
        );
    });
}

pub fn use_effect_one<
    D: 'static + PartialEq,
    C: CleanFnOrNone,
    RD: IntoRc<D>,
    F: 'static + FnOnce(Rc<D>) -> C,
>(
    func: F,
    dep: D,
) {
    let mut dep_changed = false;
    let memo_effect_key = super::use_memo_one(
        |dep| {
            dep_changed = true;

            let dep = Rc::clone(dep);

            let cell_effect_closure_key = OnceCell::<usize>::new();

            let effect_key = {
                let cell_effect_closure_key = cell_effect_closure_key.clone();

                forgotten::forget(Closure::once(move || {
                    let clean = func(dep).into_optional_clean_fn();
                    let cell_clean_closure_key = OnceCell::<usize>::new();

                    let real_clean = {
                        let cell_clean_closure_key = cell_clean_closure_key.clone();
                        Closure::once(move || {
                            if let Some(clean) = clean {
                                clean();
                            }

                            let effect_closure_key = cell_effect_closure_key.get().unwrap();
                            let clean_closure_key = cell_clean_closure_key.get().unwrap();

                            unsafe { forgotten::try_free_with_usize(*effect_closure_key) };
                            unsafe { forgotten::try_free_with_usize(*clean_closure_key) };
                        })
                    };

                    let (clean_key, real_clean) = forgotten::forget_and_get(real_clean);

                    let clean_key = clean_key.into_shared();
                    let clean_key = clean_key.as_usize();
                    cell_clean_closure_key.set(*clean_key).unwrap();

                    let v = real_clean.as_ref().as_ref();

                    v.clone()
                }))
            };

            let effect_key = effect_key.into_shared();

            cell_effect_closure_key.set(*effect_key.as_usize()).unwrap();
            effect_key
        },
        dep,
    );

    let effect_key_usize = *memo_effect_key.as_usize();

    CLOSURE_NEVER_CALLED.with(|never_called| {
        let new_closure = if dep_changed {
            let closure = forgotten::try_get(&memo_effect_key).unwrap();
            Some(closure)
        } else {
            None
        };
        react_sys::use_effect_with_clean(
            new_closure.as_ref().map_or(never_called, |c| c.as_ref()),
            Some(Box::new([effect_key_usize.into()])),
        );
    });
}