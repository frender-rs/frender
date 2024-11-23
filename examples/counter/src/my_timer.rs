use frender::prelude::*;

pub struct MyTimer {
    pub initial_interval: u32,
}

impl MyTimer {
    pub fn initial_interval(mut self, v: u32) -> Self {
        self.initial_interval = v;
        self
    }
}

const MY_TIMER: MyTimer = MyTimer {
    initial_interval: 0,
};

#[allow(non_snake_case)]
pub const fn MyTimer() -> MyTimer {
    MY_TIMER
}

impl MyTimer {
    #[component]
    pub fn into_element(self) {
        // store the initial_interval value,
        // so that the value never changes in the component life.
        let ref_initial_interval = hooks::use_mut_default::<Option<u32>>();
        let mut initial_interval = *ref_initial_interval.get_or_insert(self.initial_interval);
        if initial_interval == 0 {
            initial_interval = 1000;
        }

        let (state, state_updater) = hooks::use_shared_call(0usize, |v| *v = v.saturating_add(1));
        let (stopped, stopped_setter) = hooks::use_shared_toggle(false);

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
                    let interval =
                        gloo::timers::callback::Interval::new(initial_interval, move || {
                            state_updater.call()
                        });

                    // return a cleanup function which will clear the interval
                    Some(move || drop(interval))
                }
            },
            stopped,
        );

        let state = *state;
        let stopped_setter = stopped_setter.clone();
        let toggle_stopped = {
            let stopped_setter = stopped_setter.clone();
            move |_: &_| stopped_setter.toggle()
        };

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
}
