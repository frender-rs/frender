use crate::{AsHtmlTextValue, HtmlCommonSharedPropsBuilder};

crate::macros::def_intrinsic_component! {
    "button"
    ButtonComponent(ButtonComponentProps) {
        ButtonComponentProps
        : ButtonComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlButtonElement]
        {
            default_value[TValue: AsHtmlTextValue]: Option<TValue> { into? |v| AsHtmlTextValue::as_html_text_value(&v) },
            value[TValue: AsHtmlTextValue]: Option<TValue> { into? |v| AsHtmlTextValue::as_html_text_value(&v) },
            auto_focus: Option<bool>,
            disabled: Option<bool>,
            form: Option<&str>,
            form_action: Option<&str>,
            form_enc_type: Option<&str>,
            form_method: Option<&str>,
            form_no_validate: Option<bool>,
            form_target: Option<&str>,
            name: Option<&str>,
            kind@"type": Option<crate::ButtonType>,
        }
    }
}
