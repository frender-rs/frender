use frender::prelude::*;

def_props! {
    #[derive(Debug)]
    pub struct MyCounterProps {
        pub initial_value?: usize,
    }
}

#[component]
pub fn MyCounter(props: &MyCounterProps) {
    let (state, state_setter) = react::use_state!(props.initial_value);

    let on_increment = {
        let state_setter = state_setter.clone();
        // clone state_setter so that the cloned value can be moved into the following closure
        move || state_setter.set_from_old(|v| **v + 1)
    };
    let on_decrement = move || state_setter.set_from_old(|v| **v - 1);

    let state = *state;

    rsx!(
        <div>
            <button on_click={on_decrement} disabled={state == 0}>
                " - "
            </button>
            " "
            {state}
            " "
            <button on_click={on_increment} disabled={state == usize::MAX}>
                " + "
            </button>
        </div>
    )
}
