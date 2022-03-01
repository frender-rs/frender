//! # Design principles
//!
//! props builder methods should accept all of the following types:
//!
//! ```
//! # use std::rc::Rc;
//! # use react::{IntoPropValue, event::{ IntoJsEventHandler, MouseEvent }};
//! # struct MyPropsBuilder;
//! # impl MyPropsBuilder {
//! #   fn on_click<F: ?Sized + react::DynFn>(&self, _handler: Option<react::AnyFn<F>>)
//! #   where
//! #       react::AnyFn<F>: IntoJsEventHandler<MouseEvent> {}
//! # }
//! # let builder = MyPropsBuilder;
//! # builder.on_click(IntoPropValue::into_prop_value(
//! || {}
//! # ));
//! # builder.on_click(IntoPropValue::into_prop_value(
//! |_event| {}
//! # ));
//! # builder.on_click(IntoPropValue::into_prop_value(
//! Rc::new(|| {})
//! # ));
//! # builder.on_click(IntoPropValue::into_prop_value(
//! Rc::new(|_event| {})
//! # ));
//! # builder.on_click(IntoPropValue::into_prop_value(
//! &Rc::new(|| {})
//! # ));
//! # builder.on_click(IntoPropValue::into_prop_value(
//! &Rc::new(|_event| {})
//! # ));
//!
//! # builder.on_click(IntoPropValue::into_prop_value(
//! Some(|| {})
//! # ));
//! # builder.on_click(IntoPropValue::into_prop_value(
//! Some(Rc::new(|| {}))
//! # ));
//! # builder.on_click(IntoPropValue::into_prop_value(
//! Some(Rc::new(|_event| {}))
//! # ));
//! # builder.on_click(IntoPropValue::into_prop_value(
//! Some(&Rc::new(|| {}))
//! # ));
//! # builder.on_click(IntoPropValue::into_prop_value(
//! Some(&Rc::new(|_event| {}))
//! # ));
//! ```
use convert_js::FromJs;
use wasm_bindgen::UnwrapThrowExt;

use crate::SafeIntoJsRuntime;

pub trait IntoJsEventHandler<TEvent> {
    fn into_js_event_handler(self) -> crate::PassedToJsRuntime;
}

/// `Fn()` can be handlers for any event
impl<TEvent> IntoJsEventHandler<TEvent> for crate::AnyFn<dyn Fn()> {
    #[inline]
    fn into_js_event_handler(self) -> crate::PassedToJsRuntime {
        self.safe_into_js_runtime()
    }
}

/// `Fn(TEvent)` can be handlers for `TEvent`
impl<TEvent: 'static + FromJs> IntoJsEventHandler<TEvent> for crate::AnyFn<dyn Fn(TEvent)>
where
    TEvent::Error: std::fmt::Debug,
{
    #[inline]
    fn into_js_event_handler(self) -> crate::PassedToJsRuntime {
        crate::AnyFn::new(move |js_value: wasm_bindgen::JsValue| {
            let event = TEvent::from_js(js_value).unwrap_throw();
            (self.0.as_ref())(event);
        })
        .safe_into_js_runtime()
    }
}
