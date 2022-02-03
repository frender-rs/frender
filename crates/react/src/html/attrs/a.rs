use crate::{html::common::extend_html_props, IntoJsAdapterComponentProps, Props};

// pub struct ComponentProps();

extend_html_props! {
    struct { ComponentProps : crate::html::common::ComponentProps<web_sys::HtmlAnchorElement, ()> }
    trait  { AsPropsBuilder : crate::html::common::AsPropsBuilder<web_sys::HtmlAnchorElement, ()> }
    attrs {
        download: Option<wasm_bindgen::JsValue>,
        href: Option<&str>,
        href_lang: Option<&str>,
        media: Option<&str>,
        ping: Option<&str>,
        rel: Option<&str>,
        target: Option<crate::html::AnchorTarget>,
        kind@"type": Option<&str>,
        referrer_policy: Option<crate::html::ReferrerPolicy>,
    }
}

pub type Component = crate::html::common::Component<ComponentProps>;
