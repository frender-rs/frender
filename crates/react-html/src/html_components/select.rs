use crate::{AsHtmlTextValue, HtmlCommonSharedPropsBuilder};

crate::macros::def_intrinsic_component! {
    "select"
    SelectComponent(SelectComponentProps) {
        SelectComponentProps
        : SelectComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlSelectElement]
        {
            auto_complete: Option<&str>,
            auto_focus: Option<bool>,
            disabled: Option<bool>,
            form: Option<&str>,
            multiple: Option<bool>,
            name: Option<&str>,
            required: Option<bool>,
            size: Option<f64>,
            default_value[TValue: AsHtmlTextValue]: Option<TValue> { into? |v| AsHtmlTextValue::as_html_text_value(&v) },
            value[TValue: AsHtmlTextValue]: Option<TValue> { into? |v| AsHtmlTextValue::as_html_text_value(&v) },

            on_change: react::event::ChangeEvent<web_sys::HtmlSelectElement> { event_handler },
        }
    }
}
