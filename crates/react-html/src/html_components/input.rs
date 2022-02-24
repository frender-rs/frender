use crate::{AsHtmlTextValue, HtmlCommonSharedPropsBuilder};

crate::macros::def_intrinsic_component! {
    "input"
    InputComponent(InputComponentProps) {
        InputComponentProps
        : InputComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlInputElement]
        {
            accept: Option<&str>,
            alt: Option<&str>,
            auto_complete: Option<&str>,
            auto_focus: Option<bool>,
            {
                /// https://www.w3.org/TR/html-media-capture/#the-capture-attribute
            }
            capture: Option<crate::BoolOrStr<'_>>,
            checked: Option<bool>,
            cross_origin: Option<&str>,
            disabled: Option<bool>,
            enter_key_hint: Option<crate::EnterKeyHint>,
            form: Option<&str>,
            form_action: Option<&str>,
            form_enc_type: Option<&str>,
            form_method: Option<&str>,
            form_no_validate: Option<bool>,
            form_target: Option<&str>,
            height['a, T: Into<crate::NumOrStr<'a>>]: Option<T> { into? |v| v.into() },
            list: Option<&str>,
            max['a, T: Into<crate::NumOrStr<'a>>]: Option<T> { into? |v| v.into() },
            max_length: Option<f64>,
            min['a, T: Into<crate::NumOrStr<'a>>]: Option<T> { into? |v| v.into() },
            min_length: Option<f64>,
            multiple: Option<bool>,
            name: Option<&str>,
            pattern: Option<&str>,
            placeholder: Option<&str>,
            read_only: Option<bool>,
            required: Option<bool>,
            size: Option<f64>,
            src: Option<&str>,
            step['a, T: Into<crate::NumOrStr<'a>>]: Option<T> { into? |v| v.into() },
            html_type@"type": Option<crate::InputType>,
            default_value[TValue: AsHtmlTextValue]: Option<TValue> { into? |v| AsHtmlTextValue::as_html_text_value(&v) },
            value[TValue: AsHtmlTextValue]: Option<TValue> { into? |v| AsHtmlTextValue::as_html_text_value(&v) },
            width['a, T: Into<crate::NumOrStr<'a>>]: Option<T> { into? |v| v.into() },

            // TODO: ChangeEventHandler<web_sys::HtmlInputElement>
            on_change: (),
        }
    }
}
