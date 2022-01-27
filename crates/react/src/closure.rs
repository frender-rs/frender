//! `Closure<F>` `Box<dyn Fn>` `Box<Fn>` `Box<dyn FnMut>` `Box<dyn FnOnce>`
//! implements [IntoJsRuntime] and [TryBorrowToJsRuntime](crate::TryBorrowToJsRuntime).
//!
//! Only `Rc<Closure<F>>`, `Rc<Fn>` implements [BorrowToJsRuntime].
//!

use crate::PassToJsRuntimeValue;
use std::{any::Any, rc::Rc};
use wasm_bindgen::closure::{Closure, WasmClosure, WasmClosureFnOnce};

// Closure<F>
crate::impl_pass_to_js_runtime! {
    [
        try_borrow: None
        into:   own_closure
    ]
    { F: 'static + ?Sized } Closure<F>
}

// Closure<F>
crate::impl_pass_to_js_runtime! {
    [
        borrow: clone
        into(this) {
            let js_value = this.as_ref().as_ref().clone();
            PassToJsRuntimeValue {
                js_value,
                to_persist: Some(this as Rc<dyn Any>),
            }
        }
    ]
    { F: 'static + ?Sized } Rc<Closure<F>>
}

// dyn FnOnce
crate::impl_pass_to_js_runtime! {
    [
        try_borrow: None
    ]
    { T0, TR } dyn FnOnce(T0) -> TR
    where Self: WasmClosureFnOnce<(T0,), TR>
}

// Box<dyn FnOnce>
crate::impl_pass_to_js_runtime! {
    [
        try_borrow: None
        into: own_fn_once
    ]
    { T0, TR } Box<dyn FnOnce(T0) -> TR>
    where Self: WasmClosureFnOnce<(T0,), TR>
}

// dyn Fn
crate::impl_pass_to_js_runtime! {
    [
        try_borrow: None
    ]
    { T0, TR } dyn Fn(T0) -> TR
    where dyn Fn(T0) -> TR: WasmClosure
}

// Box<dyn Fn>
crate::impl_pass_to_js_runtime! {
    [
        try_borrow: None
        into: own_fn
    ]
    { T0: 'static, TR: 'static } Box<dyn Fn(T0) -> TR>
    where dyn Fn(T0) -> TR: WasmClosure
}

// Rc<Fn>
crate::impl_pass_to_js_runtime! {
    [
        borrow: clone
        into: own_rc_fn
    ]
    { T0: 'static, TR: 'static } Rc<dyn Fn(T0) -> TR>
    where dyn Fn(T0) -> TR: WasmClosure
}

// dyn FnMut
crate::impl_pass_to_js_runtime! {
    [
        try_borrow: None
    ]
    { T0, TR } dyn FnMut(T0) -> TR
    where dyn FnMut(T0) -> TR: WasmClosure
}

// Box<FnMut>
crate::impl_pass_to_js_runtime! {
    [
        try_borrow: None
        into: own_fn
    ]
    { T0: 'static, TR: 'static } Box<dyn FnMut(T0) -> TR>
    where dyn FnMut(T0) -> TR: WasmClosure
}

fn own_fn_once<T: ?Sized, A, R>(v: Box<T>) -> PassToJsRuntimeValue
where
    Box<T>: WasmClosureFnOnce<A, R>,
{
    let c = Closure::once(v);
    own_closure(c)
}

fn own_rc_fn<T0: 'static, TR: 'static>(v: Rc<dyn Fn(T0) -> TR>) -> PassToJsRuntimeValue
where
    dyn Fn(T0) -> TR: WasmClosure,
{
    own_fn(Box::new(move |js| v.as_ref()(js)) as Box<dyn Fn(T0) -> TR>)
}

fn own_fn<T: 'static>(v: Box<T>) -> PassToJsRuntimeValue
where
    T: ?Sized + WasmClosure,
{
    let c = Closure::wrap(v);
    own_closure(c)
}

fn own_closure<F: 'static + ?Sized>(v: Closure<F>) -> PassToJsRuntimeValue {
    let js_value = v.as_ref().clone();
    PassToJsRuntimeValue {
        js_value,
        to_persist: Some(Rc::new(v) as Rc<dyn Any>),
    }
}
