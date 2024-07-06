use frender_dom_tokens::DomToken;
use web_sys::wasm_bindgen::UnwrapThrowExt;

pub struct DomTokenList(pub web_sys::DomTokenList);

impl From<web_sys::DomTokenList> for DomTokenList {
    fn from(value: web_sys::DomTokenList) -> Self {
        Self(value)
    }
}

impl Into<web_sys::DomTokenList> for DomTokenList {
    fn into(self) -> web_sys::DomTokenList {
        self.0
    }
}

impl std::ops::Deref for DomTokenList {
    type Target = web_sys::DomTokenList;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl frender_dom_tokens::DomTokenList for DomTokenList {
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
