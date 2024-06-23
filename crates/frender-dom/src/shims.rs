use wasm_bindgen::{prelude::wasm_bindgen, JsCast};

pub(crate) trait RelList {
    fn rel_list(&self) -> web_sys::DomTokenList;
}

impl RelList for web_sys::HtmlFormElement {
    fn rel_list(&self) -> web_sys::DomTokenList {
        // https://html.spec.whatwg.org/multipage/forms.html#dom-form-rellist
        self.unchecked_ref::<RelListObject>().rel_list()
    }
}

#[wasm_bindgen]
extern "C" {
    type RelListObject;
    #[wasm_bindgen(structural, method, getter, js_name = relList)]
    fn rel_list(this: &RelListObject) -> web_sys::DomTokenList;
}
