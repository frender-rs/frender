use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "label"
    LabelComponent(LabelComponentProps) {
        LabelComponentProps
        : LabelComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlLabelElement]
        {
            form: Option<&str>,
            html_for: Option<&str>,
        }
    }
}
