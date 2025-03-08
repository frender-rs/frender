use web_sys::wasm_bindgen::UnwrapThrowExt;

use crate::dom_token::DomToken;

impl crate::csr::DomTokenList for web_sys::DomTokenList {
    fn set_value(&mut self, value: &str) {
        web_sys::DomTokenList::set_value(self, value)
    }

    fn add_1(&mut self, token: DomToken) {
        web_sys::DomTokenList::add_1(self, token.as_str()).unwrap_throw()
    }

    fn remove_1(&mut self, token: DomToken) {
        web_sys::DomTokenList::remove_1(self, token.as_str()).unwrap_throw()
    }

    fn replace(&mut self, old_token: DomToken, new_token: DomToken) {
        _ = web_sys::DomTokenList::replace(self, old_token.as_str(), new_token.as_str())
            .unwrap_throw()
    }
}
