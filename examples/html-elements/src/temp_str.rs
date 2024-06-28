use frender::prelude::*;

fn use_effect_every_second(
    effect: impl FnMut() + 'static,
) -> hooks::use_effect<(), impl hooks::effect::EffectFor<()>> {
    hooks::use_effect(
        move |(): &()| {
            let timer = gloo::timers::callback::Interval::new(1_000, effect);

            move || drop(timer)
        },
        (),
    )
}

hooks::hook_fn!(
    fn use_time_string() -> &'hook str {
        let (state, set_state) = h![hooks::use_shared_set_with(
            || "<current date time>".to_string()
        )];

        h![use_effect_every_second({
            let set_state = set_state.clone();
            move || set_state.set(js_sys::Date::new_0().to_string().into())
        })];

        state
    }
);

#[cfg(todo)]
#[component]
fn temp_str() {
    let s = h![use_time_string()];
    frender::TempStr(s)
}

fn temp_str_with_inline_component_fn() -> impl Element {
    // TODO: frender::TempStr<&str> is also allowed in this case
    component_fn!(move || -> frender::TempStr<&'hook str> {
        let s = h![use_time_string()];

        frender::TempStr(s)
    })
}

fn temp_str_with_inline_component_fn_with_generics<E: Element + Copy>(el: E) -> impl Element {
    // TODO: frender::TempStr<&str> is also allowed in this case
    component_fn!(
        for<E: Element + Copy> move || -> (E, frender::TempStr<&'hook str>) {
            let s = h![use_time_string()];

            (el, frender::TempStr(s))
        }
    )
}

pub fn main() -> impl Element {
    (
        temp_str_with_inline_component_fn(),
        temp_str_with_inline_component_fn_with_generics("This is an example with generics:"),
    )
}
