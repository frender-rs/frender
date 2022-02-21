use crate::HtmlBasePropsBuilder;

crate::macros::def_intrinsic_component! {
    "a"
    AnchorComponent(AnchorComponentProps) {
        AnchorComponentProps
        : AnchorComponentPropsBuilder
        : HtmlBasePropsBuilder[web_sys::HtmlAnchorElement, ()]
        {
            download: Option<wasm_bindgen::JsValue>,
            href: Option<&str>,
            href_lang: Option<&str>,
            media: Option<&str>,
            ping: Option<&str>,
            rel: Option<&str>,
            target: Option<crate::AnchorTarget>,
            kind@"type": Option<&str>,
            referrer_policy: Option<crate::ReferrerPolicy>,
        }
    }
}
