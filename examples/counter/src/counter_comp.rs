use frender::{forgotten, react, react_sys};
use wasm_bindgen::{prelude::*, JsCast};

use std::{
    borrow::Borrow,
    cell::{Ref, RefCell},
    collections::HashMap,
    iter::FromIterator,
    mem::ManuallyDrop,
    rc::Rc,
};

#[derive(Debug)]
pub struct CounterState {
    value: i32,
}

#[wasm_bindgen]
pub fn Counter() -> react_sys::Element {
    let (state, setter) = react::use_state(|| CounterState { value: 1 });

    web_sys::console::log_1(&JsValue::from(format!(
        "[rust] Counter render: state.value = {:?}",
        state.value,
    )));

    react::use_effect_on_mounted(move || {
        web_sys::console::log_1(&"Counter use_effect_once".into());
        let window = web_sys::window().unwrap();

        let (k, handler) = forgotten::forget_and_get(Closure::wrap(Box::new(move || {
            setter.set_from_old(|old| CounterState {
                value: old.as_ref().value + 1,
            });
        }) as Box<dyn FnMut()>));

        let handle = window
            .set_interval_with_callback_and_timeout_and_arguments_0(
                handler.as_ref().as_ref().dyn_ref().unwrap(),
                1000,
            )
            .unwrap();

        move || {
            forgotten::free(k);
            window.clear_interval_with_handle(handle);
        }
    });

    let react_el = react_sys::create_element(
        &"h2".into(),
        JsValue::NULL,
        js_sys::Array::from_iter([
            JsValue::from_str("Counter value = "),
            JsValue::from(state.value.to_string()),
            JsValue::from_str(" time = "),
            JsValue::from((js_sys::Date::now() / 1000f64).to_string()),
        ]),
    );

    react_el
}

thread_local! {
    pub static CounterJs: JsValue =
       Closure::wrap(Box::new(Counter) as Box<dyn Fn() -> react_sys::Element>).into_js_value();
}
