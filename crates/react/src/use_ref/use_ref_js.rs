use convert_js::{FromJs, ToJs};
use std::marker::PhantomData;
use wasm_bindgen::JsValue;

use super::IntoRefValue;

pub fn use_ref_js<T: ToJs + FromJs>(initial_value: T) -> MutableRefJs<T> {
    let obj = react_sys::use_ref(&initial_value.to_js());
    MutableRefJs(obj, PhantomData)
}

pub fn use_ref_js_with<T: ToJs + FromJs, F: FnOnce() -> T>(
    get_initial_value: F,
) -> MutableRefJs<T> {
    let ref_initialized = react_sys::use_ref_bool(false);
    let obj = react_sys::use_ref(&JsValue::UNDEFINED);
    if !ref_initialized.current() {
        obj.set_current(get_initial_value().to_js())
    }
    MutableRefJs(obj, PhantomData)
}

pub fn use_ref_js_set_as<T: ToJs + FromJs>(value: T) -> MutableRefJs<T> {
    let obj = react_sys::use_ref(&JsValue::UNDEFINED);
    obj.set_current(value.to_js());
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

impl<T: ToJs + FromJs> crate::SafeIntoJsRuntime for MutableRefJs<T> {
    fn safe_into_js_runtime(self) -> crate::PassedToJsRuntime {
        crate::PassedToJsRuntime {
            js_value: self.0.into(),
            to_persist: None,
        }
    }
}

impl<T, W: IntoRefValue<T> + ToJs + FromJs> super::ReadRef<T> for MutableRefJs<W>
where
    <W as FromJs>::Error: std::fmt::Debug,
{
    fn current(&self) -> T {
        let js_value = self.0.current();
        let w = W::from_js(js_value).unwrap();
        w.into_ref_value()
    }
}

impl<T: IntoRefValue<W>, W: ToJs + FromJs> super::WriteRef<T> for MutableRefJs<W> {
    fn set_current(&self, v: T) {
        let w: W = v.into_ref_value();
        self.0.set_current(w.to_js())
    }
}
