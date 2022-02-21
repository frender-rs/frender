use crate::HtmlBasePropsBuilder;

crate::macros::def_intrinsic_component! {
    "button"
    AnchorComponent(AnchorComponentProps) {
        AnchorComponentProps
        : AnchorComponentPropsBuilder
        : HtmlBasePropsBuilder[web_sys::HtmlAnchorElement, ()]
        {
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
