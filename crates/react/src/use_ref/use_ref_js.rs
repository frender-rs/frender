use convert_js::{FromJs, ToJs};
use std::marker::PhantomData;
use wasm_bindgen::JsValue;

use super::SimpleInto;

pub fn use_ref_js<T: ToJs + FromJs>(initial_value: T) -> MutableRefJs<T> {
    let obj = react_sys::use_ref(&initial_value.to_js());
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

impl<T, W: SimpleInto<T> + ToJs + FromJs> super::ReadRef<T> for MutableRefJs<W>
where
    <W as FromJs>::Error: std::fmt::Debug,
{
    fn current(&self) -> T {
        let js_value = self.0.current();
        let w = W::from_js(js_value).unwrap();
        w.simple_into()
    }
}

impl<T: SimpleInto<W>, W: ToJs + FromJs> super::WriteRef<T> for MutableRefJs<W> {
    fn set_current(&self, v: T) {
        let w: W = v.simple_into();
        self.0.set_current(w.to_js())
    }
}
