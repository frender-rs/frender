use frender::prelude::*;

pub struct MyTimerProps {
    pub initial_interval: u32,
}

impl MyTimerProps {
    pub fn initial_interval(mut self, v: u32) -> Self {
        self.initial_interval = v;
        self
    }
}

#[allow(non_snake_case)]
pub mod MyTimer {
    pub mod prelude {}

    pub use super::MyTimerImpl as build_element;
}

#[allow(non_snake_case)]
pub fn MyTimer() -> MyTimerProps {
    MyTimerProps {
        initial_interval: 0,
    }
}

#[component]
pub fn MyTimerImpl(props: MyTimerProps) {
    // store the initial_interval value,
    // so that the value never changes in the component life.
    let ref_initial_interval = hooks::use_mut_default::<Option<u32>>();
    let mut initial_interval = *ref_initial_interval.get_or_insert(props.initial_interval);
    if initial_interval == 0 {
        initial_interval = 1000;
    }

    let (state, state_updater) = hooks::use_state(0usize);
    let (stopped, stopped_setter) = hooks::use_state(false);

    let stopped = *stopped;

    let state_updater = state_updater.clone();

    hooks::use_effect(
        move |stopped: &_| {
            let stopped = *stopped;
            gloo::console::log!(format!(
                "Timer(initial_interval={initial_interval}) stopped changed to {stopped}"
            ));
            if stopped {
                None
            } else {
                let interval = gloo::timers::callback::Interval::new(initial_interval, move || {
                    state_updater.replace_with_fn_pointer(|v| v.overflowing_add(1).0)
                });

                // return a cleanup function which will clear the interval
                Some(move || drop(interval))
            }
        },
        stopped,
    );

    let state = *state;
    let stopped_setter = stopped_setter.clone();
    let toggle_stopped = callable!(
        || stopped_setter.replace_with_fn_pointer(|v| !*v),
        stopped_setter = stopped_setter.clone(),
    )
    .accept_anything();

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
