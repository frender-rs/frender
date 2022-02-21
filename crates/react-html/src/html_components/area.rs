use crate::HtmlBasePropsBuilder;

crate::macros::def_intrinsic_component! {
    "area"
    AreaComponent(AreaComponentProps) {
        AreaComponentProps
        : AreaComponentPropsBuilder
        : HtmlBasePropsBuilder[web_sys::HtmlAreaElement, ()]
        {
            alt: Option<&str>,
            coords: Option<&str>,
            download: Option<wasm_bindgen::JsValue>,
            href: Option<&str>,
            href_lang: Option<&str>,
            media: Option<&str>,
            referrer_policy: crate::ReferrerPolicy,
            rel: Option<&str>,
            shape: Option<&str>,
            target: Option<&str>,
        }
    }
}
