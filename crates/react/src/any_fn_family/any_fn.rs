use std::{any::Any, rc::Rc};
use wasm_bindgen::closure::{Closure, WasmClosure};

use crate::{DynFn, FnOfArgs, IntoPropValue};

pub struct AnyFn<F>(pub Rc<F>)
where
    F: ?Sized + DynFn;

impl<F: ?Sized + DynFn> AnyFn<F> {
    #[inline]
    pub fn inner(&self) -> &Rc<F> {
        &self.0
    }

    #[inline]
    pub fn into_inner(self) -> Rc<F> {
        self.0
    }

    #[inline]
    pub fn new<
        TFunc: 'static + ?Sized + FnOfArgs<F::ArgsTuple, DynFn = F>,
        RF: crate::IntoRc<TFunc>,
    >(
        func: RF,
    ) -> Self {
        let func: Rc<TFunc> = func.into_rc();
        let func = func.into_rc_dyn_fn();
        Self(func)
    }
}

impl<F: ?Sized + DynFn> std::ops::Deref for AnyFn<F> {
    type Target = Rc<F>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F: ?Sized + DynFn> AsRef<Rc<F>> for AnyFn<F> {
    #[inline]
    fn as_ref(&self) -> &Rc<F> {
        &self.0
    }
}

impl<F: ?Sized + DynFn> Clone for AnyFn<F> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<F: ?Sized + DynFn> std::fmt::Debug for AnyFn<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AnyFn<{}>", std::any::type_name::<F>())
    }
}

impl<F: ?Sized + DynFn> crate::SafeIntoJsRuntime for AnyFn<F>
where
    F: 'static + WasmClosure,
{
    fn safe_into_js_runtime(self) -> crate::PassedToJsRuntime {
        let boxed = self.0.rc_into_box_dyn_fn();

        let closure = Closure::wrap(boxed);
        let js_value = closure.as_ref().clone();
        crate::PassedToJsRuntime {
            js_value,
            to_persist: Some(Box::new(closure) as Box<dyn Any>),
        }
    }
}

impl<F: ?Sized + DynFn, TFunc: 'static + ?Sized + FnOfArgs<F::ArgsTuple, DynFn = F>>
    IntoPropValue<AnyFn<F>> for Rc<TFunc>
{
    fn into_prop_value(self) -> AnyFn<F> {
        AnyFn(self.into_rc_dyn_fn())
    }
}

impl<F: ?Sized + DynFn, TFunc: 'static + ?Sized + FnOfArgs<F::ArgsTuple, DynFn = F>>
    IntoPropValue<Option<AnyFn<F>>> for Rc<TFunc>
{
    fn into_prop_value(self) -> Option<AnyFn<F>> {
        Some(self.into_prop_value())
    }
}

impl<F: ?Sized + DynFn, TFunc: 'static + ?Sized + FnOfArgs<F::ArgsTuple, DynFn = F>>
    IntoPropValue<Option<AnyFn<F>>> for Option<Rc<TFunc>>
{
    fn into_prop_value(self) -> Option<AnyFn<F>> {
        self.map(IntoPropValue::into_prop_value)
    }
}

impl<F: ?Sized + DynFn, TFunc: 'static + ?Sized + FnOfArgs<F::ArgsTuple, DynFn = F>>
    IntoPropValue<AnyFn<F>> for &Rc<TFunc>
{
    fn into_prop_value(self) -> AnyFn<F> {
        AnyFn(Rc::clone(self).into_rc_dyn_fn())
    }
}

impl<F: ?Sized + DynFn, TFunc: 'static + ?Sized + FnOfArgs<F::ArgsTuple, DynFn = F>>
    IntoPropValue<Option<AnyFn<F>>> for &Rc<TFunc>
{
    fn into_prop_value(self) -> Option<AnyFn<F>> {
        Some(self.into_prop_value())
    }
}

impl<F: ?Sized + DynFn, TFunc: 'static + ?Sized + FnOfArgs<F::ArgsTuple, DynFn = F>>
    IntoPropValue<Option<AnyFn<F>>> for Option<&Rc<TFunc>>
{
    fn into_prop_value(self) -> Option<AnyFn<F>> {
        self.map(IntoPropValue::into_prop_value)
    }
}
