use convert_js::ToJs;
use wasm_bindgen::JsValue;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CssProperties {
    custom: js_sys::Object,
}

impl CssProperties {
    pub fn new() -> Self {
        Self {
            custom: js_sys::Object::new(),
        }
    }

    pub fn with_key_value<V: ToJs>(self, key: &str, value: V) -> Self {
        let _ = js_sys::Reflect::set(
            self.custom.as_ref(),
            &JsValue::from_str(key),
            &value.to_js(),
        );

        self
    }
}

impl ToJs for CssProperties {
    fn to_js(&self) -> JsValue {
        JsValue::from(&self.custom)
    }
}

/// ```no_run
/// let style: react::CssProperties = style! {
///     "margin": "10px auto",
///     "paddingTop": 8,
/// };
/// ```
///
/// Currently there is no static type checking
/// with this macro.
#[macro_export]
macro_rules! style {
    ($($key:literal : $value:expr),* $(,)? ) => {{
        $crate::CssProperties::new()
            $(.with_key_value($key, $value))*
    }};
}
