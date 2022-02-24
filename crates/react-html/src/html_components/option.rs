use crate::{AsHtmlTextValue, HtmlCommonSharedPropsBuilder};

crate::macros::def_intrinsic_component! {
    "option"
    OptionComponent(OptionComponentProps) {
        OptionComponentProps
        : OptionComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlOptionElement]
        {
            disabled: Option<bool>,
            label: Option<&str>,
            selected: Option<bool>,
            default_value[TValue: AsHtmlTextValue]: Option<TValue> { into? |v| AsHtmlTextValue::as_html_text_value(&v) },
            value[TValue: AsHtmlTextValue]: Option<TValue> { into? |v| AsHtmlTextValue::as_html_text_value(&v) },
        }
    }
}
