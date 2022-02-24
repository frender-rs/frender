use crate::{AsHtmlTextValue, HtmlCommonSharedPropsBuilder};

crate::macros::def_intrinsic_component! {
    "data"
    DataComponent(DataComponentProps) {
        DataComponentProps
        : DataComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlDataElement]
        {
            default_value[TValue: AsHtmlTextValue]: Option<TValue> { into? |v| AsHtmlTextValue::as_html_text_value(&v) },
            value[TValue: AsHtmlTextValue]: Option<TValue> { into? |v| AsHtmlTextValue::as_html_text_value(&v) },
        }
    }
}
