use frender::prelude::*;
use hooks::prelude::*;

#[component]
pub fn main() {
    // non-reactive
    let state = hooks::use_mut_with(|| hooks::SharedSignal::new(false));

    cs::div.children((
        cs::button
            .children("toggle")
            .on_click(state.to_callback_toggle()),
        {
            let state = state.clone();
            component_fn!(move || {
                // make this element reactive over state
                h![state.use_signal()];
                state.get().then_some(cs::div.children("a"))
            })
        },
    ))
}
