//! [`Closure<F>`]
//! [`WrapFn<F: Fn>`]
//! [`WrapFnMut<F: FnMut>`]
//! [`WrapFnOnce<F: FnOnce>`]
//! [`WrapDynFnOrFnMut<dyn Fn>`]
//! [`WrapDynFnOrFnMut<dyn FnMut>`]
//! implements [`SafeIntoJsRuntime`].

use super::{PassedToJsRuntime, SafeIntoJsRuntime};
use std::{any::Any, marker::PhantomData, rc::Rc};
use wasm_bindgen::closure::{Closure, WasmClosure, WasmClosureFnOnce};

pub struct WrapFn<F: ?Sized, TArgs>(pub Rc<F>, PhantomData<TArgs>);
pub struct WrapFnMut<F, TArgs>(pub F, PhantomData<TArgs>);
pub struct WrapFnOnce<F, TArgs, TReturn>(pub F, PhantomData<(TArgs, TReturn)>);
pub struct WrapDynFnOrFnMut<F: ?Sized>(pub Box<F>);

impl<F: ?Sized> WrapFn<F, ()> {
    #[inline]
    pub fn new<A, RF: crate::IntoRc<F>>(func: RF) -> WrapFn<F, A>
    where
        WrapFn<F, A>: SafeIntoJsRuntime,
    {
        let func: Rc<F> = func.into_rc();
        WrapFn(func, PhantomData)
    }
}

impl<F> WrapFnMut<F, ()> {
    #[inline]
    pub fn new<A>(func: F) -> WrapFnMut<F, A>
    where
        WrapFnMut<F, A>: SafeIntoJsRuntime,
    {
        WrapFnMut(func, PhantomData)
    }
}

impl<F: ?Sized> WrapDynFnOrFnMut<F> {
    #[inline]
    pub fn new(func: Box<F>) -> Self {
        Self(func)
    }
}

impl<F> WrapFnOnce<F, (), ()> {
    #[inline]
    pub fn new<A, R>(func: F) -> WrapFnOnce<F, A, R>
    where
        F: WasmClosureFnOnce<A, R>,
    {
        WrapFnOnce(func, PhantomData)
    }
}

/// Closure<F>
impl<F: 'static + ?Sized> SafeIntoJsRuntime for Closure<F> {
    #[inline]
    fn safe_into_js_runtime(self) -> PassedToJsRuntime {
        own_closure(self)
    }
}

impl<F: 'static + ?Sized + WasmClosure> SafeIntoJsRuntime for WrapDynFnOrFnMut<F> {
    #[inline]
    fn safe_into_js_runtime(self) -> PassedToJsRuntime {
        own_fn(self.0)
    }
}

impl<F: 'static + WasmClosureFnOnce<A, R>, A, R> SafeIntoJsRuntime for WrapFnOnce<F, A, R> {
    #[inline]
    fn safe_into_js_runtime(self) -> PassedToJsRuntime {
        own_fn_once(self.0)
    }
}

impl<F: ?Sized + WasmClosure> crate::IntoPropValue<WrapDynFnOrFnMut<F>> for Box<F> {
    fn into_prop_value(self) -> WrapDynFnOrFnMut<F> {
        WrapDynFnOrFnMut(self)
    }
}

fn own_fn_once<F: 'static + WasmClosureFnOnce<A, R>, A, R>(func: F) -> PassedToJsRuntime {
    let c = Closure::once(func);
    own_closure(c)
}

fn own_fn<T: 'static>(v: Box<T>) -> PassedToJsRuntime
where
    T: ?Sized + WasmClosure,
{
    let c = Closure::wrap(v);
    own_closure(c)
}

fn own_closure<F: 'static + ?Sized>(v: Closure<F>) -> PassedToJsRuntime {
    let js_value = v.as_ref().clone();
    PassedToJsRuntime {
        js_value,
        to_persist: Some(Box::new(v) as Box<dyn Any>),
    }
}

macro_rules! doit {
    ($(
        ($($var:ident)*)
    )*) => ($(
        #[doc = concat!("WrapFn<TFunc: Fn(", $(stringify!($var), ", ",)* ") -> TR>")]
        impl<TFunc, $($var,)* R> SafeIntoJsRuntime for WrapFn<TFunc, ($($var,)*)>
            where $($var: 'static,)*
                  R: 'static,
                  TFunc: Fn($($var),*) -> R + 'static,
                  dyn Fn($($var),*) -> R: WasmClosure,
        {
            #[inline]
            fn safe_into_js_runtime(self) -> PassedToJsRuntime {
                #[allow(non_snake_case)]
                own_fn(Box::new(move |$($var,)*| (self.0)($($var,)*) ) as Box<dyn Fn($($var),*) -> R>)
            }
        }

        #[doc = concat!("WrapFnMut<TFunc: FnMut(", $(stringify!($var), ", ",)* ") -> TR>")]
        impl<TFunc, $($var,)* R> SafeIntoJsRuntime for WrapFnMut<TFunc, ($($var,)*)>
            where $($var: 'static,)*
                  R: 'static,
                  TFunc: Fn($($var),*) -> R + 'static,
                  dyn FnMut($($var),*) -> R: WasmClosure,
        {
            #[inline]
            fn safe_into_js_runtime(self) -> PassedToJsRuntime {
                own_fn(Box::new(self.0) as Box<dyn FnMut($($var),*) -> R>)
            }
        }

        impl<TFunc, $($var,)* R> $crate::IntoPropValue<WrapFn<TFunc, ($($var,)*)>> for TFunc
            where TFunc: Fn($($var),*) -> R
        {
            #[inline]
            fn into_prop_value(self) -> WrapFn<TFunc, ($($var,)*)> {
                WrapFn(Rc::new(self), PhantomData)
            }
        }

        impl<TFunc: ?Sized, $($var,)* R> $crate::IntoPropValue<WrapFn<TFunc, ($($var,)*)>> for Rc<TFunc>
        where TFunc: Fn($($var),*) -> R
        {
            #[inline]
            fn into_prop_value(self) -> WrapFn<TFunc, ($($var,)*)> {
                WrapFn(self, PhantomData)
            }
        }

        impl<TFunc: ?Sized, $($var,)* R> $crate::IntoPropValue<WrapFn<TFunc, ($($var,)*)>> for &Rc<TFunc>
        where TFunc: Fn($($var),*) -> R
        {
            #[inline]
            fn into_prop_value(self) -> WrapFn<TFunc, ($($var,)*)> {
                WrapFn(Rc::clone(self), PhantomData)
            }
        }

        impl<TFunc, $($var,)* R> $crate::IntoPropValue<Option<WrapFn<TFunc, ($($var,)*)>>> for TFunc
            where TFunc: Fn($($var),*) -> R
        {
            #[inline]
            fn into_prop_value(self) -> Option<WrapFn<TFunc, ($($var,)*)>> {
                Some(WrapFn(Rc::new(self), PhantomData))
            }
        }

        impl<TFunc, $($var,)* R> $crate::IntoPropValue<Option<WrapFn<TFunc, ($($var,)*)>>> for Option<TFunc>
        where TFunc: Fn($($var),*) -> R
        {
            #[inline]
            fn into_prop_value(self) -> Option<WrapFn<TFunc, ($($var,)*)>> {
                self.map(crate::IntoPropValue::into_prop_value)
            }
        }

        impl<TFunc: ?Sized, $($var,)* R> $crate::IntoPropValue<Option<WrapFn<TFunc, ($($var,)*)>>> for Rc<TFunc>
        where TFunc: Fn($($var),*) -> R
        {
            #[inline]
            fn into_prop_value(self) -> Option<WrapFn<TFunc, ($($var,)*)>> {
                Some(WrapFn(self, PhantomData))
            }
        }

        impl<TFunc: ?Sized, $($var,)* R> $crate::IntoPropValue<Option<WrapFn<TFunc, ($($var,)*)>>> for Option<Rc<TFunc>>
        where TFunc: Fn($($var),*) -> R
        {
            #[inline]
            fn into_prop_value(self) -> Option<WrapFn<TFunc, ($($var,)*)>> {
                self.map(crate::IntoPropValue::into_prop_value)
            }
        }

        impl<TFunc: ?Sized, $($var,)* R> $crate::IntoPropValue<Option<WrapFn<TFunc, ($($var,)*)>>> for &Rc<TFunc>
        where TFunc: Fn($($var),*) -> R
        {
            #[inline]
            fn into_prop_value(self) -> Option<WrapFn<TFunc, ($($var,)*)>> {
                Some(WrapFn(Rc::clone(self), PhantomData))
            }
        }

        impl<TFunc: ?Sized, $($var,)* R> $crate::IntoPropValue<Option<WrapFn<TFunc, ($($var,)*)>>> for Option<&Rc<TFunc>>
        where TFunc: Fn($($var),*) -> R
        {
            #[inline]
            fn into_prop_value(self) -> Option<WrapFn<TFunc, ($($var,)*)>> {
                self.map(crate::IntoPropValue::into_prop_value)
            }
        }

        impl<TFunc, $($var,)* R> $crate::IntoPropValue<WrapFnMut<TFunc, ($($var,)*)>> for TFunc
            where TFunc: FnMut($($var),*) -> R
        {
            fn into_prop_value(self) -> WrapFnMut<TFunc, ($($var,)*)> {
                WrapFnMut(self, PhantomData)
            }
        }

        impl<TFunc, $($var,)* R> $crate::IntoPropValue<Option<WrapFnMut<TFunc, ($($var,)*)>>> for TFunc
            where TFunc: FnMut($($var),*) -> R
        {
            fn into_prop_value(self) -> Option<WrapFnMut<TFunc, ($($var,)*)>> {
                Some(WrapFnMut(self, PhantomData))
            }
        }

        impl<TFunc, $($var,)* R> $crate::IntoPropValue<Option<WrapFnMut<TFunc, ($($var,)*)>>> for Option<TFunc>
            where TFunc: FnMut($($var),*) -> R
        {
            fn into_prop_value(self) -> Option<WrapFnMut<TFunc, ($($var,)*)>> {
                self.map(crate::IntoPropValue::into_prop_value)
            }
        }

        impl<TFunc, $($var,)* R> $crate::IntoPropValue<WrapFnOnce<TFunc, ($($var,)*), R>> for TFunc
            where TFunc: FnOnce($($var),*) -> R
        {
            fn into_prop_value(self) -> WrapFnOnce<TFunc, ($($var,)*), R> {
                WrapFnOnce(self, PhantomData)
            }
        }

        impl<TFunc, $($var,)* R> $crate::IntoPropValue<Option<WrapFnOnce<TFunc, ($($var,)*), R>>> for TFunc
            where TFunc: FnOnce($($var),*) -> R
        {
            fn into_prop_value(self) -> Option<WrapFnOnce<TFunc, ($($var,)*), R>> {
                Some(WrapFnOnce(self, PhantomData))
            }
        }

        impl<TFunc, $($var,)* R> $crate::IntoPropValue<Option<WrapFnOnce<TFunc, ($($var,)*), R>>> for Option<TFunc>
            where TFunc: FnOnce($($var),*) -> R
        {
            fn into_prop_value(self) -> Option<WrapFnOnce<TFunc, ($($var,)*), R>> {
                self.map(crate::IntoPropValue::into_prop_value)
            }
        }
    )*)
}

doit! {
    ()
    (A)
    (A B)
    (A B C)
    (A B C D)
    (A B C D E)
    (A B C D E F)
    (A B C D E F G)
    (A B C D E F G H)
}
