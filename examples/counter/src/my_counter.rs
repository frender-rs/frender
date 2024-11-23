use frender::prelude::*;
use hooks::ShareValue;

pub struct MyCounter {
    pub initial_value: Option<u32>,
}

impl MyCounter {
    pub fn initial_value(mut self, v: impl Into<Option<u32>>) -> Self {
        self.initial_value = v.into();
        self
    }
}

const MY_COUNTER: MyCounter = MyCounter {
    initial_value: None,
};

#[allow(non_snake_case)]
pub const fn MyCounter() -> MyCounter {
    MY_COUNTER
}

impl MyCounter {
    #[component]
    pub fn into_element(self) {
        let initial_value: u32 = self.initial_value.unwrap_or(0);
        let shared_state = hooks::use_shared_signal(initial_value);

        let on_increment = {
            let shared_state = shared_state.clone();
            move |_: &_| _ = shared_state.replace_with(|v| *v + 1)
        };

        let on_decrement = {
            let shared_state = shared_state.clone();
            move |_: &_| _ = shared_state.replace_with(|v| *v - 1)
        };

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
}
