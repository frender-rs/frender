use hooks_frender::{component, element, Dom};

use bg::{builder, Maybe};

builder! {
    struct CounterWithInitialValueProps {
        initial_value[? i32],
    }
}

#[component]
pub fn MyComp(ctx: _, props: CounterWithInitialValueProps) {
    ctx.render(props.initial_value.as_some().map(ToString::to_string))
}

#[component]
pub fn CounterWithInitialValue(ctx: _, props: &CounterWithInitialValueProps) {
    let (state, updater) =
        hooks::use_state_with(|| props.initial_value.as_some().copied().unwrap_or(4));

    let updater = updater.clone();

    web_sys::console::log_1(&"render".into());

    ctx.render((
        if *state % 3 == 0 { None } else { Some("help ") },
        element!(hooks_frender::button()
            .on_click(move |_: &_| {
                web_sys::console::log_1(&"on_click".into());
                updater.replace_with_fn_pointer(|v| *v + 1);
            })
            .children(format!("state = {}", state))),
        if *state % 2 == 0 {
            Some("state is even")
        } else {
            None
        },
        if *state % 2 == 1 {
            Some(format!("{state} is odd"))
        } else {
            None
        },
        " Last",
    ))
}

#[component]
pub fn Counter(ctx: _) {
    let (state, updater) = hooks::use_state(0);

    let updater = updater.clone();

    web_sys::console::log_1(&"render".into());

    ctx.render((
        if *state % 3 == 0 { None } else { Some("help ") },
        element!(hooks_frender::button()
            .on_click(move |_: &_| {
                web_sys::console::log_1(&"on_click".into());
                updater.replace_with_fn_pointer(|v| *v + 1);
            })
            .children(format!("state = {}", state))),
        if *state % 2 == 0 {
            Some("state is even")
        } else {
            None
        },
        if *state % 2 == 1 {
            Some(format!("{state} is odd"))
        } else {
            None
        },
        " Last",
    ))
}

fn main() {
    wasm_bindgen_futures::spawn_local(async {
        let window = web_sys::window().unwrap();

        let document = window.document().unwrap();
        let current_parent = document.get_element_by_id("main").unwrap();

        Dom::new(document, current_parent)
            .render_get_element(
                || {
                    (
                        element!(CounterWithInitialValue().initial_value(8)),
                        element!(Counter()),
                        element!(CounterWithInitialValue().initial_value(-8)),
                    )
                },
                std::future::pending(),
            )
            .await;
    })
}
