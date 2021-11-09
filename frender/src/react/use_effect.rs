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
       #[cfg(debug_assertions)]
       panic!("CLOSURE_NEVER_CALLED should never be called")
   })
}

/// `React.useEffect(effect, [])`
pub fn use_effect_on_mounted<F: 'static + ReactEffectFn>(effect: F) {
    let ref_effect_closure_key = react_sys::use_ref_usize(0);
    let ref_clean_closure_key = react_sys::use_ref_usize(0);

    let closure = if ref_effect_closure_key.current() != 0 {
        None
    } else {
        let closure: Closure<dyn FnMut() -> JsValue> = {
            let ref_effect_closure_key = ref_effect_closure_key.clone();
            Closure::once(move || {
                let clean = effect().into_optional_clean_fn();
                let real_clean = {
                    let ref_clean_closure_key = ref_clean_closure_key.clone();
                    Closure::once(move || {
                        if let Some(clean) = clean {
                            clean();
                        }

                        unsafe { forgotten::try_free_with_usize(ref_effect_closure_key.current()) };
                        unsafe { forgotten::try_free_with_usize(ref_clean_closure_key.current()) };
                    })
                };

                let (k, real_clean) = forgotten::forget_and_get(real_clean);

                let k = k.into_shared();
                let k = k.as_usize();
                ref_clean_closure_key.set_current(*k);

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

pub fn use_effect_one<D, C: CleanFnOrNone, RD: IntoRc<D>, F: FnOnce(&Rc<D>) -> C>(func: F, dep: D) {
    todo!()
}
