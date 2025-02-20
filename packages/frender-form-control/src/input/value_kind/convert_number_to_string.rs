/// https://html.spec.whatwg.org/multipage/input.html#concept-input-value-number-string
pub(super) fn convert_number_to_string(input_type: &str, value: f64) -> String {
    if value.is_nan() {
        return String::new();
    }

    #[cfg(all(feature = "web", target_arch = "wasm32"))]
    return crate::input::web::number_as_input_value(input_type, value);

    #[cfg(not(all(feature = "web", target_arch = "wasm32")))]
    #[cfg(feature = "chrono")]
    return with_chrono::convert_non_nan_number_to_string(input_type, value);

    #[cfg(not(all(feature = "web", target_arch = "wasm32")))]
    #[cfg(not(feature = "chrono"))]
    return match input_type {
        "number" | "range" => value.to_string(),
        // TODO: we just ignore the default value in ssr if
        // there is no implementation to convert number to string
        _ => String::new(),
    };
}

#[cfg(feature = "chrono")]
#[cfg(not(all(feature = "web", target_arch = "wasm32")))]
mod with_chrono;
