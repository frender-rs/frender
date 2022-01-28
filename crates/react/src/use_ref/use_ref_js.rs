use convert_js::{FromJs, ToJs};
use std::marker::PhantomData;
use wasm_bindgen::{JsCast, JsValue};

use crate::SimpleTake;

pub fn use_ref_js_value<T: ToJs + FromJs>(initial_value: T) -> MutableRefJs<T> {
    let obj = react_sys::use_ref(&initial_value.to_js());
    MutableRefJs(obj, PhantomData)
}

pub fn use_ref_js<T: ToJs + FromJs, F: FnOnce() -> T>(get_initial_value: F) -> MutableRefJs<T> {
    let mut get_initial_value =
        crate::utils::fn_once_in_runtime(move || get_initial_value().to_js());
    let obj = react_sys::use_ref_with(&mut get_initial_value);
    MutableRefJs(obj, PhantomData)
}

pub struct MutableRefJs<T: ToJs + FromJs>(react_sys::MutableRefObject, PhantomData<T>);

impl<T: ToJs + FromJs> Clone for MutableRefJs<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<T: ToJs + FromJs> AsRef<JsValue> for MutableRefJs<T> {
    fn as_ref(&self) -> &JsValue {
        self.0.as_ref()
    }
}

impl<T: ToJs + FromJs> ToJs for MutableRefJs<T> {
    fn to_js(&self) -> JsValue {
        AsRef::<JsValue>::as_ref(&self.0).clone()
    }
}

crate::impl_pass_to_js_runtime! {
    [
        borrow(this) {
            crate::PassToJsRuntimeValue {
                js_value: this.as_ref().clone(),
                to_persist: None,
            }
        }
        into(this) {
            crate::PassToJsRuntimeValue {
                js_value: this.0.dyn_into().unwrap(),
                to_persist: None,
            }
        }
    ]
    {T: ToJs + FromJs} MutableRefJs<T>
}

impl<T, W: SimpleTake<T> + ToJs + FromJs> super::ReadRef<T> for MutableRefJs<W>
where
    <W as FromJs>::Error: std::fmt::Debug,
{
    fn current(&self) -> T {
        let js_value = self.0.current();
        let w = W::from_js(js_value).unwrap();
        w.simple_take()
    }
}

impl<T: SimpleTake<W>, W: ToJs + FromJs> super::WriteRef<T> for MutableRefJs<W> {
    fn set_current(&self, v: T) {
        let w: W = v.simple_take();
        self.0.set_current(w.to_js())
    }
}