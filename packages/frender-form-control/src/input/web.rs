use web_sys::wasm_bindgen::{self, prelude::*};

#[wasm_bindgen(module = "/js/input.js")]
extern "C" {
    #[wasm_bindgen(js_name = "numberAsInputValue")]
    pub(crate) fn number_as_input_value(input_type: &str, value: f64) -> String;

    #[wasm_bindgen(js_name = "setDefaultValueAsNumber")]
    pub(crate) fn set_default_value(input: &web_sys::HtmlInputElement, defaultValue: f64);
}
