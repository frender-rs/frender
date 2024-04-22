use wasm_bindgen::UnwrapThrowExt;

// TODO: remove
pub struct DomTokenList(pub web_sys::DomTokenList);

impl frender_html_common::dom_token::DomTokenList for DomTokenList {
    fn set_value(&mut self, value: &str) {
        self.0.set_value(value)
    }

    fn add_1(&mut self, token: &str) {
        self.0.add_1(token).unwrap_throw()
    }

    fn remove_1(&mut self, token: &str) {
        self.0.remove_1(token).unwrap_throw()
    }

    fn replace(&mut self, old_token: &str, new_token: &str) {
        _ = self.0.replace(old_token, new_token).unwrap_throw()
    }
}
