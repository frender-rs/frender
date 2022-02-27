use crate::{AsHtmlTextValue, HtmlCommonSharedPropsBuilder};

crate::macros::def_intrinsic_component! {
    "meter"
    MeterComponent(MeterComponentProps) {
        MeterComponentProps
        : MeterComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlElement]
        {
            form: Option<&str>,
            high: Option<f64>,
            low: Option<f64>,
            max['a, T: Into<crate::NumOrStr<'a>>]: Option<T> { into? |v| v.into() },
            min['a, T: Into<crate::NumOrStr<'a>>]: Option<T> { into? |v| v.into() },
            optimum: Option<f64>,
            default_value[TValue: AsHtmlTextValue]: Option<TValue> { into? |v| AsHtmlTextValue::as_html_text_value(&v) },
            value[TValue: AsHtmlTextValue]: Option<TValue> { into? |v| AsHtmlTextValue::as_html_text_value(&v) },
        }
    }
}