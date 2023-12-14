use std::borrow::Cow;

use frender::{
    dom::value::{Controlled, OneWayBinding, UncontrolledWithDefaultValue},
    prelude::*,
    ScriptInnerTextWronglyEncoded,
};
use hooks::{
    shared_state::{SharedState, SharedStateEq},
    ShareValue,
};

fn input() -> impl Element {
    intrinsic!(
        div[[
            //
            code[[r##"input.type_("text")"##]],
            input.type_("text"),
        ]],
    )
}

fn script() -> impl Element {
    intrinsic!(
        div[[
            code[["1</script/"]],
            " is ",
            span.id("my_script_result")[["running..."]]
        ]],
        script[[{
            ScriptInnerTextWronglyEncoded(
                "document.getElementById('my_script_result').innerText = String(1</script/)",
            )
        }]],
    )
}

fn style() -> impl Element {
    intrinsic!(
        div[[div.id("my_styled_div")[["Style"]]]],
        style[[r#"#my_styled_div {
color: blue;
}"#]],
    )
}

#[component]
fn textarea() {
    let state_text = hooks::use_shared_state_eq_with(|| "default value".to_string());

    intrinsic!(
        {
            intrinsic!(
                "One way binding (thus readonly)",
                textarea.value(OneWayBinding(state_text.get_cloned())),
                "Controlled with a callback",
                textarea.value(Controlled(
                    state_text.get_cloned(),
                    state_text.clone().into_setter(),
                )),
                "Uncontrolled",
                textarea,
                "Uncontrolled with default value",
                textarea.value(UncontrolledWithDefaultValue(state_text.get_cloned())),
                "Uncontrolled, but update state on input",
                textarea.on_input(state_text.clone().into_setter_form_control_value()),
                "Uncontrolled, but update state on change",
                textarea.on_change(state_text.clone().into_setter_form_control_value())
            )
        },
        button.on_click(
            state_text
                .clone()
                .into_setter()
                .provide_first_argument_with(callable![|| {
                    js_sys::Date::new_0().to_string().into()
                }])
                .accept_anything()
        )[["Set state to current date string"]]
    )
}

#[component(main(get_dom_element = "frender-root"))]
fn Main() {
    (input(), script(), style(), textarea())
}
