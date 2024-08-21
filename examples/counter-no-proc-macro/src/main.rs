use frender::prelude::*;
use hooks::prelude::*;

component_fn!(
    #[inline]
    fn Counter(initial_value: u32) {
        let state = h!(use_shared_signal(initial_value));

        let increment = {
            let state = state.clone();
            move |_: &_| _ = state.replace_with(|v| *v + 1)
        };

        let decrement = {
            let state = state.clone();
            move |_: &_| _ = state.replace_with(|v| *v - 1)
        };

        let state = state.get();

        cs::div.children((
            cs::button
                .on_click(decrement)
                .disabled(state == 0)
                .children("-"),
            " ",
            state,
            " ",
            cs::button
                .on_click(increment)
                .disabled(state == u32::MAX)
                .children("+"),
        ))
    }
);

component_fn!(
    #[component]
    pub fn MyTimer(initial_interval: u32) {
        // store the initial_interval value,
        // so that the value never changes in the component life.
        let ref_initial_interval = h![hooks::use_mut_default::<Option<u32>>()];
        let mut initial_interval = *ref_initial_interval.get_or_insert(initial_interval);
        if initial_interval == 0 {
            initial_interval = 1000;
        }

        let (state, state_updater) =
            h![hooks::use_shared_call(0usize, |v| *v = v.saturating_add(1))];
        let (stopped, stopped_setter) = h![hooks::use_shared_toggle(false)];

        let stopped = *stopped;

        let state_updater = state_updater.clone();

        h![hooks::use_effect(
            move |stopped: &_| {
                let stopped = *stopped;
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
        )];

        let state = *state;

        let toggle_stopped = {
            let stopped_setter = stopped_setter.clone();

            move |_: &_| stopped_setter.toggle()
        };

        cs::div.children((
            "Timer(initial_interval=",
            initial_interval,
            "): ",
            state,
            " ",
            cs::button.on_click(toggle_stopped).children(if stopped {
                " RESUME "
            } else {
                "  STOP  "
            }),
        ))
    }
);

#[allow(non_snake_case)]
fn DivCode(code: impl Element, children: impl Element) -> impl Element {
    cs::div.children((cs::code.children(code), { children }))
}

component_fn!(
    #[component(main)]
    fn Main() {
        cs::div
            .id("a")
            .style(style!(
                r#"margin: auto;
padding: 16px;
max-width: 768px;
"#
            ))
            .children((
                cs::h1.children((
                    "Counter & Timer (without proc-macro) - ",
                    cs::div.children(
                        cs::a
                            .href("https://github.com/frender-rs/frender")
                            .target("_blank")
                            .children((
                                //
                                cs::b.children("f"),
                                "render",
                            )),
                    ),
                )),
                cs::main.children((
                    DivCode("Counter(0)", Counter(0)),
                    DivCode("Counter(3)", Counter(3)),
                    DivCode("MyTimer(1000)", MyTimer(1000)),
                    DivCode("MyTimer(500)", MyTimer(500)),
                )),
            ))
    }
);
