use frender::{prelude::*, ScriptInnerTextWronglyEncoded};

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

#[component(main(get_dom_element = "frender-root"))]
fn Main() {
    (input(), script())
}
