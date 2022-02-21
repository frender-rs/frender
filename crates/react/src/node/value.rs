use wasm_bindgen::JsValue;

/// A `number`, `string`, `boolean` or `Array<ReactNode>`
/// that are not elements
/// but valid `ReactNode`
#[derive(Debug, Clone)]
pub struct AnyNodeValue {
    inner: JsValue,
}

impl AnyNodeValue {
    #[inline]
    pub(crate) fn unsafe_from_js_react_node(js_value: JsValue) -> Self {
        Self { inner: js_value }
    }

    /// ## Safety:
    /// Please make sure the js value is a valid `ReactNode`
    #[inline]
    pub unsafe fn from_js_react_node(js_value: JsValue) -> Self {
        Self::unsafe_from_js_react_node(js_value)
    }
}

impl AsRef<JsValue> for AnyNodeValue {
    #[inline]
    fn as_ref(&self) -> &JsValue {
        &self.inner
    }
}

impl Into<JsValue> for AnyNodeValue {
    fn into(self) -> JsValue {
        self.inner
    }
}
