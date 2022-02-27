use frender::prelude::*;
use wasm_bindgen::{closure::Closure, JsCast, JsValue, UnwrapThrowExt};

def_props! {
    #[derive(Debug)]
    pub struct MyTimerProps {
        pub initial_interval?: i32,
    }
}

#[component]
pub fn MyTimer(props: &MyTimerProps) {
    let initial_interval = &props.initial_interval;
    let initial_interval = if initial_interval > &0 {
        *initial_interval
    } else {
        1000
    };

    // use_ref of the initial_interval value,
    // so that the value never changes in the component life.
    let ref_initial_interval = react::use_ref!(initial_interval);
    let initial_interval = *ref_initial_interval.current();

    let (state, state_setter) = react::use_state!(0usize);
    let (stopped_rc, stopped_setter) = react::use_state!(false);

    let stopped = *stopped_rc;

    // use_ref of our closure,
    // so that the closure is persisted in the component life.
    let ref_handler = react::use_ref!(None::<Closure<dyn Fn()>>);

    // ref_handler should also be listed as an explicit dependency,
    // even though it will always be equal to the previous value.
    react::use_effect!((stopped_rc) => {
        let stopped = *stopped_rc;
        let initial_interval = *ref_initial_interval.current();

        web_sys::console::log_1(&JsValue::from(
            format!("Timer(initial_interval={}) stopped changed to {}", initial_interval, stopped)
        ));
        if stopped {
            ref_handler.set_current(None);
            return None;
        }
        let window = web_sys::window().unwrap();
        let closure = Closure::wrap(Box::new(move || {
            state_setter.set_from_old(|v| {
                let v = **v;
                v.overflowing_add(1).0
            })
        }) as Box<dyn Fn()>);
        let js_func: &JsValue = closure.as_ref().as_ref();

        let handle = window
            .set_interval_with_callback_and_timeout_and_arguments_0(
                js_func.unchecked_ref(),
                initial_interval,
            )
            .unwrap_throw();

        ref_handler.set_current(Some(closure));

        // return a cleanup function which will clear the interval
        Some(move || {
            window.clear_interval_with_handle(handle);
        })
    });

    let state = *state;
    let toggle_stopped = move || stopped_setter.set_from_old(|v| !**v);

    rsx!(
        <div>
            "Timer(initial_interval="{initial_interval}"): "
            {state}
            " "
            <button on_click={toggle_stopped}>
                {if stopped { " RESUME " } else { "  STOP  " }}
            </button>
        </div>
    )
}
