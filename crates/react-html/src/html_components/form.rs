use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "form"
    FormComponent(FormComponentProps) {
        FormComponentProps
        : FormComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlFormElement]
        {
            accept_charset: Option<&str>,
            action: Option<&str>,
            auto_complete: Option<&str>,
            enc_type: Option<&str>,
            method: Option<&str>,
            name: Option<&str>,
            no_validate: Option<bool>,
            target: Option<&str>,
        }
    }
}
