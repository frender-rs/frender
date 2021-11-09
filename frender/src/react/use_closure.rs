use wasm_bindgen::{closure::WasmClosure, prelude::*};

pub trait FrenderClosure {
    type WasmClosureType: ?Sized + WasmClosure;

    fn into_wasm_closure(self) -> Closure<Self::WasmClosureType>;
}

impl<R> FrenderClosure for &'static dyn FnOnce() -> R {
    type WasmClosureType = dyn FnOnce();
}

macro_rules! impl_into_react_closure {
    ($(
        ($($var:ident)*)
    )*) => ($(
        unsafe impl<$($var,)* R> WasmClosure for dyn Fn($($var),*) -> R + 'static
            where $($var: FromWasmAbi + 'static,)*
                  R: ReturnWasmAbi + 'static,
        {
            fn describe() {
                #[allow(non_snake_case)]
                unsafe extern "C" fn invoke<$($var: FromWasmAbi,)* R: ReturnWasmAbi>(
                    a: usize,
                    b: usize,
                    $($var: <$var as FromWasmAbi>::Abi),*
                ) -> <R as ReturnWasmAbi>::Abi {
                    if a == 0 {
                        throw_str("closure invoked recursively or destroyed already");
                    }
                    // Make sure all stack variables are converted before we
                    // convert `ret` as it may throw (for `Result`, for
                    // example)
                    let ret = {
                        let f: *const dyn Fn($($var),*) -> R =
                            FatPtr { fields: (a, b) }.ptr;
                        $(
                            let $var = <$var as FromWasmAbi>::from_abi($var);
                        )*
                        (*f)($($var),*)
                    };
                    ret.return_abi()
                }

                inform(invoke::<$($var,)* R> as u32);

                unsafe extern fn destroy<$($var: FromWasmAbi,)* R: ReturnWasmAbi>(
                    a: usize,
                    b: usize,
                ) {
                    // This can be called by the JS glue in erroneous situations
                    // such as when the closure has already been destroyed. If
                    // that's the case let's not make things worse by
                    // segfaulting and/or asserting, so just ignore null
                    // pointers.
                    if a == 0 {
                        return;
                    }
                    drop(Box::from_raw(FatPtr::<dyn Fn($($var,)*) -> R> {
                        fields: (a, b)
                    }.ptr));
                }
                inform(destroy::<$($var,)* R> as u32);

                <&Self>::describe();
            }
        }

        unsafe impl<$($var,)* R> WasmClosure for dyn FnMut($($var),*) -> R + 'static
            where $($var: FromWasmAbi + 'static,)*
                  R: ReturnWasmAbi + 'static,
        {
            fn describe() {
                #[allow(non_snake_case)]
                unsafe extern "C" fn invoke<$($var: FromWasmAbi,)* R: ReturnWasmAbi>(
                    a: usize,
                    b: usize,
                    $($var: <$var as FromWasmAbi>::Abi),*
                ) -> <R as ReturnWasmAbi>::Abi {
                    if a == 0 {
                        throw_str("closure invoked recursively or destroyed already");
                    }
                    // Make sure all stack variables are converted before we
                    // convert `ret` as it may throw (for `Result`, for
                    // example)
                    let ret = {
                        let f: *const dyn FnMut($($var),*) -> R =
                            FatPtr { fields: (a, b) }.ptr;
                        let f = f as *mut dyn FnMut($($var),*) -> R;
                        $(
                            let $var = <$var as FromWasmAbi>::from_abi($var);
                        )*
                        (*f)($($var),*)
                    };
                    ret.return_abi()
                }

                inform(invoke::<$($var,)* R> as u32);

                unsafe extern fn destroy<$($var: FromWasmAbi,)* R: ReturnWasmAbi>(
                    a: usize,
                    b: usize,
                ) {
                    // See `Fn()` above for why we simply return
                    if a == 0 {
                        return;
                    }
                    drop(Box::from_raw(FatPtr::<dyn FnMut($($var,)*) -> R> {
                        fields: (a, b)
                    }.ptr));
                }
                inform(destroy::<$($var,)* R> as u32);

                <&mut Self>::describe();
            }
        }

        #[allow(non_snake_case, unused_parens)]
        impl<T, $($var,)* R> WasmClosureFnOnce<($($var),*), R> for T
            where T: 'static + FnOnce($($var),*) -> R,
                  $($var: FromWasmAbi + 'static,)*
                  R: ReturnWasmAbi + 'static
        {
            type FnMut = dyn FnMut($($var),*) -> R;

            fn into_fn_mut(self) -> Box<Self::FnMut> {
                let mut me = Some(self);
                Box::new(move |$($var),*| {
                    let me = me.take().expect_throw("FnOnce called more than once");
                    me($($var),*)
                })
            }

            fn into_js_function(self) -> JsValue {
                use std::rc::Rc;
                use crate::__rt::WasmRefCell;

                let mut me = Some(self);

                let rc1 = Rc::new(WasmRefCell::new(None));
                let rc2 = rc1.clone();

                let closure = Closure::wrap(Box::new(move |$($var),*| {
                    // Invoke ourself and get the result.
                    let me = me.take().expect_throw("FnOnce called more than once");
                    let result = me($($var),*);

                    // And then drop the `Rc` holding this function's `Closure`
                    // alive.
                    debug_assert_eq!(Rc::strong_count(&rc2), 1);
                    let option_closure = rc2.borrow_mut().take();
                    debug_assert!(option_closure.is_some());
                    drop(option_closure);

                    result
                }) as Box<dyn FnMut($($var),*) -> R>);

                let js_val = closure.as_ref().clone();

                *rc1.borrow_mut() = Some(closure);
                debug_assert_eq!(Rc::strong_count(&rc1), 2);
                drop(rc1);

                js_val
            }
        }
    )*)
}

impl_into_react_closure! {
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

pub fn use_closure<F: IntoReactClosure>() -> Closure {}

#[cfg(test)]
mod tests {
    fn test_use_closure_fn_once() {}
    fn test_use_closure_fn_mut() {}
    fn test_use_closure_fn() {}
}
