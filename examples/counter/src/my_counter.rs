use frender::prelude::*;
use hooks::{shared_state::SharedState, ShareValue};

pub struct MyCounterProps {
    pub initial_value: Option<u32>,
}

impl MyCounterProps {
    pub fn initial_value(mut self, v: impl Into<Option<u32>>) -> Self {
        self.initial_value = v.into();
        self
    }
}

#[allow(non_snake_case)]
pub mod MyCounter {
    pub mod prelude {}

    pub use super::MyCounterImpl as build_element;
}

#[allow(non_snake_case)]
pub fn MyCounter() -> MyCounterProps {
    MyCounterProps {
        initial_value: None,
    }
}

#[component]
pub fn MyCounterImpl(props: MyCounterProps) {
    let initial_value: u32 = props.initial_value.unwrap_or(0);
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
