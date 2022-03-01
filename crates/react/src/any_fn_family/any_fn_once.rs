use std::any::Any;
use wasm_bindgen::closure::{Closure, WasmClosureFnOnce};

use crate::{DynFnOnce, FnOnceOfArgs};

pub struct AnyFnOnce<F>(pub Box<F>)
where
    F: ?Sized + DynFnOnce;

impl<F: ?Sized + DynFnOnce> AnyFnOnce<F> {
    #[inline]
    pub fn inner(&self) -> &Box<F> {
        &self.0
    }

    #[inline]
    pub fn into_inner(self) -> Box<F> {
        self.0
    }

    #[inline]
    pub fn new<
        TFunc: 'static + ?Sized + FnOnceOfArgs<F::ArgsTuple, DynFnOnce = F>,
        BF: crate::IntoBoxed<TFunc>,
    >(
        func: BF,
    ) -> Self {
        let func: Box<TFunc> = func.into_boxed();
        let func = func.into_box_dyn_fn_once();
        Self(func)
    }
}

impl<F: ?Sized + DynFnOnce> std::ops::Deref for AnyFnOnce<F> {
    type Target = Box<F>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F: ?Sized + DynFnOnce> AsRef<Box<F>> for AnyFnOnce<F> {
    #[inline]
    fn as_ref(&self) -> &Box<F> {
        &self.0
    }
}

impl<F: ?Sized + DynFnOnce> std::fmt::Debug for AnyFnOnce<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AnyFnOnce<{}>", std::any::type_name::<F>())
    }
}

impl<F: ?Sized + DynFnOnce> crate::SafeIntoJsRuntime for AnyFnOnce<F>
where
    F: 'static,
    Box<F>: WasmClosureFnOnce<F::ArgsTuple, F::Output>,
{
    fn safe_into_js_runtime(self) -> crate::PassedToJsRuntime {
        let closure = Closure::once(self.0);
        let js_value = closure.as_ref().clone();
        crate::PassedToJsRuntime {
            js_value,
            to_persist: Some(Box::new(closure) as Box<dyn Any>),
        }
    }
}
