use frender::{forgotten, react};
use wasm_bindgen::{prelude::*, JsCast};

use std::iter::FromIterator;

#[derive(Debug)]
pub struct CounterState {
    value: i32,
}

#[wasm_bindgen]
pub fn Timer(props: js_sys::Object) -> react::sys::Element {
    let (state, setter) = react::use_state(|| CounterState { value: 1 });

    react::use_effect_on_mounted(move || {
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

    let react_el = react::sys::create_element(
        &"h2".into(),
        &JsValue::NULL,
        &js_sys::Array::from_iter([
            JsValue::from_str("Counter value = "),
            JsValue::from(state.value.to_string()),
            JsValue::from_str(" time = "),
            JsValue::from((js_sys::Date::now() / 1000f64).round().to_string()),
        ]),
    );

    react_el
}

thread_local! {
    pub static TimerJs: JsValue =
        Closure::wrap(Box::new(Timer) as Box<dyn Fn(js_sys::Object) -> react::sys::Element>).into_js_value();
}

thread_local! {
    pub static TimerClosure: Closure<dyn Fn(js_sys::Object) -> react::sys::Element> =
        Closure::wrap(Box::new(Timer) as Box<dyn Fn(js_sys::Object) -> react::sys::Element>);
}
