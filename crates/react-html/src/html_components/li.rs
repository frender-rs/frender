use crate::{AsHtmlTextValue, HtmlCommonSharedPropsBuilder};

crate::macros::def_intrinsic_component! {
    "li"
    LiComponent(LiComponentProps) {
        LiComponentProps
        : LiComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlLiElement]
        {
            default_value[TValue: AsHtmlTextValue]: Option<TValue> { into? |v| AsHtmlTextValue::as_html_text_value(&v) },
            value[TValue: AsHtmlTextValue]: Option<TValue> { into? |v| AsHtmlTextValue::as_html_text_value(&v) },
        }
    }
}
