use frender::prelude::*;
use hooks::prelude::*;

component_fn!(
    #[component(csr)]
    #[inline]
    fn Counter(initial_value: u32) {
        let state = h!(use_shared_state(initial_value));
        let increment = {
            let state = state.clone();
            move |_: &_| {
                state.replace_with(|v| *v + 1);
            }
        };
        let decrement = {
            let state = state.clone();
            move |_: &_| {
                state.replace_with(|v| *v - 1);
            }
        };

        let state = state.get();

        intrinsic!(div.children(intrinsic!(
            button
                .on_click(decrement)
                .disabled(state == 0)
                .children("-"),
            " ",
            { state },
            " ",
            button
                .on_click(increment)
                .disabled(state == u32::MAX)
                .children("+")
        )))
    }
);

component_fn!(
    #[component(csr)]
    pub fn MyTimer(initial_interval: u32) {
        // store the initial_interval value,
        // so that the value never changes in the component life.
        let ref_initial_interval = h![hooks::use_mut_default::<Option<u32>>()];
        let mut initial_interval = *ref_initial_interval.get_or_insert(initial_interval);
        if initial_interval == 0 {
            initial_interval = 1000;
        }

        let (state, state_updater) = h![hooks::use_state(0usize)];
        let (stopped, stopped_setter) = h![hooks::use_state(false)];

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
                            state_updater.replace_with_fn_pointer(|v| v.overflowing_add(1).0)
                        });

                    // return a cleanup function which will clear the interval
                    Some(move || drop(interval))
                }
            },
            stopped,
        )];

        let state = *state;
        let stopped_setter = stopped_setter.clone();
        let toggle_stopped = move |_: &_| stopped_setter.replace_with_fn_pointer(|v| !*v);

        intrinsic!(div.children(intrinsic!(
            "Timer(initial_interval=",
            { initial_interval },
            "): ",
            { state },
            " ",
            button
                .on_click(toggle_stopped)
                .children(if stopped { " RESUME " } else { "  STOP  " })
        )))
    }
);

#[allow(non_snake_case)]
fn DivCode(code: Element![csr], children: Element![csr]) -> Element![csr] {
    intrinsic!(div.children(intrinsic!(code.children(code), { children })))
}

#[allow(non_snake_case)]
fn Main() -> Element![csr] {
    intrinsic!(div
        .style(
            r#"margin: auto;
padding: 16px;
max-width: 768px;
"#
        )
        .children(intrinsic!(
            h1.children(intrinsic!(
                "Counter & Timer (without proc-macro) - ",
                i.children(intrinsic!(a
                    .href("https://github.com/frender-rs/frender")
                    .target("_blank")
                    .children(intrinsic!(b.children("f"), "render"))))
            )),
            main.children((
                DivCode("Counter(0)", Counter(0)),
                DivCode("Counter(3)", Counter(3)),
                DivCode("MyTimer(1000)", MyTimer(1000)),
                DivCode("MyTimer(500)", MyTimer(500)),
            ))
        )))
}

// TODO: macro
fn main() {
    frender::hook_element::frender_dom::spawn_mount_to_dom_element(|| Main(), "frender-root")
}
