use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "style"
    StyleComponent(StyleComponentProps) {
        StyleComponentProps
        : StyleComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlStyleElement]
        {
            media: Option<&str>,
            nonce: Option<&str>,
            scoped: Option<bool>,
            html_type@"type": Option<&str>,
        }
    }
}
