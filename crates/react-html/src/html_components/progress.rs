use crate::{AsHtmlTextValue, HtmlCommonSharedPropsBuilder};

crate::macros::def_intrinsic_component! {
    "progress"
    ProgressComponent(ProgressComponentProps) {
        ProgressComponentProps
        : ProgressComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlProgressElement]
        {
            max['a, T: Into<crate::NumOrStr<'a>>]: Option<T> { into? |v| v.into() },
            default_value[TValue: AsHtmlTextValue]: Option<TValue> { into? |v| AsHtmlTextValue::as_html_text_value(&v) },
            value[TValue: AsHtmlTextValue]: Option<TValue> { into? |v| AsHtmlTextValue::as_html_text_value(&v) },
        }
    }
}
