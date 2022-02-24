use crate::{AsHtmlTextValue, HtmlCommonSharedPropsBuilder};

crate::macros::def_intrinsic_component! {
    "param"
    ParamComponent(ParamComponentProps) {
        ParamComponentProps
        : ParamComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlParamElement]
        {
            name: Option<&str>,
            default_value[TValue: AsHtmlTextValue]: Option<TValue> { into? |v| AsHtmlTextValue::as_html_text_value(&v) },
            value[TValue: AsHtmlTextValue]: Option<TValue> { into? |v| AsHtmlTextValue::as_html_text_value(&v) },
        }
    }
}
