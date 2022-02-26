use crate::{AsHtmlTextValue, HtmlCommonSharedPropsBuilder};

crate::macros::def_intrinsic_component! {
    "textarea"
    TextAreaComponent(TextAreaComponentProps) {
        TextAreaComponentProps
        : TextAreaComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlTextAreaElement]
        {
            auto_complete: Option<&str>,
            auto_focus: Option<bool>,
            cols: Option<usize>,
            dir_name: Option<&str>,
            disabled: Option<bool>,
            form: Option<&str>,
            max_length: Option<usize>,
            min_length: Option<usize>,
            name: Option<&str>,
            placeholder: Option<&str>,
            read_only: Option<bool>,
            required: Option<bool>,
            rows: Option<usize>,
            default_value[TValue: AsHtmlTextValue]: Option<TValue> { into? |v| AsHtmlTextValue::as_html_text_value(&v) },
            value[TValue: AsHtmlTextValue]: Option<TValue> { into? |v| AsHtmlTextValue::as_html_text_value(&v) },
            wrap: Option<&str>,

            on_change: react::event::ChangeEvent<web_sys::HtmlTextAreaElement> { event_handler },
        }
    }
}
