use wasm_bindgen::{prelude::wasm_bindgen, JsCast};

#[wasm_bindgen]
extern "C" {
    type SetReferrerPolicyObject;

    #[wasm_bindgen(structural, method, setter, js_name = referrerPolicy)]
    fn set_referrer_policy(this: &SetReferrerPolicyObject, value: &str);

    type SetHeightWidthU32Object;
    #[wasm_bindgen(structural, method, setter, js_name = height)]
    pub fn set_height(this: &SetHeightWidthU32Object, value: u32);
    #[wasm_bindgen(structural, method, setter, js_name = width)]
    pub fn set_width(this: &SetHeightWidthU32Object, value: u32);
}

pub(crate) trait SetReferrerPolicy {
    fn set_referrer_policy(&self, value: &str);
}

impl SetReferrerPolicy for web_sys::HtmlScriptElement {
    fn set_referrer_policy(&self, value: &str) {
        self.unchecked_ref::<SetReferrerPolicyObject>()
            .set_referrer_policy(value)
    }
}

pub(crate) trait SetHeightWidthU32 {
    fn set_height(&self, value: u32);
    fn set_width(&self, value: u32);
}

impl SetHeightWidthU32 for web_sys::HtmlSourceElement {
    fn set_height(&self, value: u32) {
        self.unchecked_ref::<SetHeightWidthU32Object>()
            .set_height(value)
    }

    fn set_width(&self, value: u32) {
        self.unchecked_ref::<SetHeightWidthU32Object>()
            .set_width(value)
    }
}

pub(crate) trait SetAccept {
    fn set_accept(&self, value: &str);
}

impl SetAccept for web_sys::HtmlFormElement {
    fn set_accept(&self, value: &str) {
        self.set_attribute("accept", value).unwrap()
    }
}

pub(crate) trait SetSizes {
    fn set_sizes(&self, value: &str);
}

impl SetSizes for web_sys::HtmlLinkElement {
    fn set_sizes(&self, value: &str) {
        // TODO: unwrap behavior
        self.set_attribute("sizes", value).unwrap()
    }
}

pub(crate) trait SetHtmlFor {
    fn set_html_for(&self, value: &str);
}

impl SetHtmlFor for web_sys::HtmlOutputElement {
    fn set_html_for(&self, value: &str) {
        self.set_attribute("for", value).unwrap()
    }
}

pub(crate) trait SetBgColor {
    fn set_bg_color(&self, value: &str);
}

::frender_common::impl_many!(
    impl<__> SetBgColor
        for each_of![
            web_sys::HtmlTableSectionElement,
            web_sys::HtmlTableColElement
        ]
    {
        fn set_bg_color(&self, value: &str) {
            self.set_attribute("bgcolor", value).unwrap()
        }
    }
);

pub(crate) mod prelude {
    pub(crate) use super::{
        SetAccept as _, SetBgColor as _, SetHeightWidthU32 as _, SetHtmlFor as _,
        SetReferrerPolicy as _, SetSizes as _,
    };
}
