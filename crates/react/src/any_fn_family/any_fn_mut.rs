use std::any::Any;
use wasm_bindgen::closure::{Closure, WasmClosure};

use crate::{DynFnMut, FnMutOfArgs};

pub struct AnyFnMut<F>(pub Box<F>)
where
    F: ?Sized + DynFnMut;

impl<F: ?Sized + DynFnMut> AnyFnMut<F> {
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
        TFunc: 'static + ?Sized + FnMutOfArgs<F::ArgsTuple, DynFnMut = F>,
        BF: crate::IntoBoxed<TFunc>,
    >(
        func: BF,
    ) -> Self {
        let func: Box<TFunc> = func.into_boxed();
        let func = func.into_box_dyn_fn_mut();
        Self(func)
    }
}

impl<F: ?Sized + DynFnMut> std::ops::Deref for AnyFnMut<F> {
    type Target = Box<F>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F: ?Sized + DynFnMut> AsRef<Box<F>> for AnyFnMut<F> {
    #[inline]
    fn as_ref(&self) -> &Box<F> {
        &self.0
    }
}

impl<F: ?Sized + DynFnMut> std::fmt::Debug for AnyFnMut<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AnyFnMut<{}>", std::any::type_name::<F>())
    }
}

impl<F: ?Sized + DynFnMut> crate::SafeIntoJsRuntime for AnyFnMut<F>
where
    F: 'static + WasmClosure,
{
    fn safe_into_js_runtime(self) -> crate::PassedToJsRuntime {
        let closure = Closure::wrap(self.0);
        let js_value = closure.as_ref().clone();
        crate::PassedToJsRuntime {
            js_value,
            to_persist: Some(Box::new(closure) as Box<dyn Any>),
        }
    }
}
