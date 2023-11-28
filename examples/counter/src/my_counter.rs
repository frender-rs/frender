use frender::prelude::*;
use hooks::{shared_state::SharedState, ShareValue};

frender::bg::builder! {
    pub struct MyCounterProps {
        initial_value[? u32],
    }
}

#[component(bg = "frender::bg")]
pub fn MyCounter(props: MyCounterProps) {
    let initial_value: u32 = *props.initial_value.as_some().unwrap_or(&0);
    let shared_state = hooks::use_shared_state(initial_value);

    let on_increment = shared_state
        .clone()
        .into_callback(callable!(|shared_state: &SharedState<_>| {
            shared_state.replace_with(|v| *v + 1);
        }))
        .accept_anything();

    let on_decrement = shared_state
        .clone()
        .into_callback(callable!(|shared_state: &SharedState<_>| {
            shared_state.replace_with(|v| *v - 1);
        }))
        .accept_anything();

    let state = shared_state.get();

    rsx!(
        <div>
            <button on_click={on_decrement} disabled={state == 0}>
                " - "
            </button>
            " "
            {state}
            " "
            <button on_click={on_increment} disabled={state == u32::MAX}>
                " + "
            </button>
        </div>
    )
}
