use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "html"
    HtmlComponent(HtmlComponentProps) {
        HtmlComponentProps
        : HtmlComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlHtmlElement]
        {
            manifest: Option<&str>,
        }
    }
}
