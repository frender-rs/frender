use frender::{
    form_control::UncontrolledWithDefaultValue, prelude::*, Empty, ScriptInnerTextWronglyEncoded,
};
use hooks::{IntoEq, ShareValue, ToOwnedShareValue};

fn input() -> impl Element {
    component_fn!(move || {
        let state_text =
            h![hooks::use_shared_signal_with(|| "default value".to_string())].into_eq();

        let checked = h![hooks::use_shared_signal(false)].into_eq();

        elements!(
            cs::code().children(r##"input.type_("text")"##),
            "uncontrolled, value and checked are absent",
            cs::input(),
            cs::input().type_("text"),
            "uncontrolled, value and checked are Empty",
            cs::input().type_("text").value(Empty).checked(Empty),
            "uncontrolled, with defaultValue and defaultChecked",
            cs::input().value(state_text.get_cloned()),
            "Uncontrolled, with defaultChecked and value",
            cs::input()
                .type_("checkbox")
                .value(state_text.get_cloned())
                .checked(checked.get()),
            "Uncontrolled explicitly",
            cs::input().value(UncontrolledWithDefaultValue(state_text.get_cloned())),
            cs::br(),
            "Controlled with a shared state",
            cs::input().value(state_text.to_controlled()),
            "Controlled checkbox with a shared state",
            cs::input()
                .r#type("checkbox")
                .value(state_text.get_cloned())
                .checked(checked.to_controlled()),
            "Uncontrolled, but update state on input",
            cs::input().on_input(state_text.to_set_form_control_value()),
            "Uncontrolled, but update state on change",
            cs::input().on_change(state_text.to_set_form_control_value()),
            cs::button()
                .on_click({
                    let state_text = state_text.to_owned_share_value();
                    move |_: &_| state_text.set(current_date_string().into())
                })
                .children("Set state to current date string"),
        )
    })
}

fn script() -> impl Element {
    (
        cs::div().children((
            cs::code().children("1</script/"),
            " is ",
            cs::span().id("my_script_result").children("running..."),
        )),
        cs::script().children(ScriptInnerTextWronglyEncoded(
            "document.getElementById('my_script_result').innerText = String(1</script/)",
        )),
    )
}

fn style() -> impl Element {
    (
        cs::div().children(cs::div().id("my_styled_div").children("Style")),
        cs::style().children(
            r#"#my_styled_div {
color: blue;
}"#,
        ),
    )
}

#[component]
fn textarea() {
    let state_text = hooks::use_shared_signal_with(|| "default value".to_string()).into_eq();

    elements!(
        // "One way binding (thus readonly)",
        // cs::textarea.value(OneWayBinding(state_text.get_cloned())),
        // "Controlled with a callback",
        // cs::textarea.value(Controlled(state_text.get_cloned(), {
        //     let state_text = state_text.clone();
        //     // move |v| state_text.set(v)
        //     move |v| {}
        // })),
        "Controlled with a shared state",
        cs::textarea().value(state_text.to_controlled()),
        "Controlled with a shared state",
        cs::textarea().value(state_text.to_controlled()),
        "Uncontrolled",
        cs::textarea(),
        "Uncontrolled with default value (implicitly)",
        cs::textarea().value(state_text.get_cloned()),
        "Uncontrolled with default value (explicitly)",
        cs::textarea().value(UncontrolledWithDefaultValue(state_text.get_cloned())),
        "Uncontrolled, but update state on input",
        cs::textarea().on_input(state_text.to_set_form_control_value()),
        "Uncontrolled, but update state on change",
        cs::textarea().on_change(state_text.to_set_form_control_value()),
        cs::button()
            .on_click({
                let state_text = state_text.to_owned_share_value();
                move |_: &_| state_text.set(current_date_string().into())
            })
            .children("Set state to current date string"),
    )
}

fn current_date_string() -> String {
    js_sys::Date::new_0().to_string().into()
}

mod option;
mod temp_str;

#[component(main(get_dom_element = "frender-root"))]
fn Main() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    (
        temp_str::main(),
        input(),
        script(),
        style(),
        textarea(),
        option::main(),
    )
}
