use convert_js::ToJs;
use js_sys::JsString;
use wasm_bindgen::JsValue;

#[derive(Debug, Clone, ToJs)]
pub struct JsStringsArray(js_sys::Array);

pub trait AsJsStringsArray {
    fn as_js_strings_array(&self) -> JsStringsArray;
}

impl AsJsStringsArray for [&str] {
    fn as_js_strings_array(&self) -> JsStringsArray {
        JsStringsArray(self.iter().map(|v| JsValue::from_str(v)).collect())
    }
}

impl AsJsStringsArray for [String] {
    fn as_js_strings_array(&self) -> JsStringsArray {
        JsStringsArray(self.iter().map(|v| JsValue::from_str(v)).collect())
    }
}

impl AsJsStringsArray for [JsString] {
    fn as_js_strings_array(&self) -> JsStringsArray {
        JsStringsArray(self.iter().map(|v| JsValue::from(v)).collect())
    }
}

impl AsJsStringsArray for JsStringsArray {
    fn as_js_strings_array(&self) -> JsStringsArray {
        self.clone()
    }
}

impl<T: AsJsStringsArray> AsJsStringsArray for &T {
    fn as_js_strings_array(&self) -> JsStringsArray {
        (*self).as_js_strings_array()
    }
}

#[derive(Debug, Clone, ToJs)]
#[convert_js(union)]
pub enum HtmlTextValue {
    String(JsString),
    Number(js_sys::Number),
    Strings(JsStringsArray),
}

pub trait AsHtmlTextValue {
    fn as_html_text_value(&self) -> HtmlTextValue;
}

impl AsHtmlTextValue for str {
    fn as_html_text_value(&self) -> HtmlTextValue {
        HtmlTextValue::String(JsString::from(self))
    }
}

impl AsHtmlTextValue for String {
    fn as_html_text_value(&self) -> HtmlTextValue {
        HtmlTextValue::String(JsString::from(self.as_str()))
    }
}

impl AsHtmlTextValue for JsString {
    fn as_html_text_value(&self) -> HtmlTextValue {
        HtmlTextValue::String(self.clone())
    }
}

impl AsHtmlTextValue for f64 {
    fn as_html_text_value(&self) -> HtmlTextValue {
        HtmlTextValue::Number(js_sys::Number::from(*self))
    }
}

impl<T: AsHtmlTextValue> AsHtmlTextValue for &T {
    fn as_html_text_value(&self) -> HtmlTextValue {
        (*self).as_html_text_value()
    }
}

impl AsHtmlTextValue for [&str] {
    fn as_html_text_value(&self) -> HtmlTextValue {
        HtmlTextValue::Strings(self.as_js_strings_array())
    }
}

impl AsHtmlTextValue for [String] {
    fn as_html_text_value(&self) -> HtmlTextValue {
        HtmlTextValue::Strings(self.as_js_strings_array())
    }
}

impl AsHtmlTextValue for [JsString] {
    fn as_html_text_value(&self) -> HtmlTextValue {
        HtmlTextValue::Strings(self.as_js_strings_array())
    }
}
