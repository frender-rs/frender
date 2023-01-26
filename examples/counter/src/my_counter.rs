use frender::prelude::*;
use hooks::ShareValue;

bg::builder! {
    pub struct MyCounterProps {
        initial_value[? u32],
    }
}

#[component(only_dom)]
pub fn MyCounter(ctx: _, props: &MyCounterProps) {
    let initial_value: u32 = *props.initial_value.as_some().unwrap_or(&0);
    let shared_state = hooks::use_shared_state(initial_value);

    let on_increment = {
        let shared_state = shared_state.clone();
        // clone state so that the cloned value can be moved into the following closure
        move |_: &_| {
            shared_state.replace_from_ref(|v| *v + 1);
        }
    };
    let on_decrement = {
        let shared_state = shared_state.clone();
        move |_: &_| {
            shared_state.replace_from_ref(|v| *v - 1);
        }
    };

    let state = shared_state.get();

    render!(ctx=>
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
