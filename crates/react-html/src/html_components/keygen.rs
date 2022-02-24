use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "keygen"
    KeygenComponent(KeygenComponentProps) {
        KeygenComponentProps
        : KeygenComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlElement]
        {
            auto_focus: Option<bool>,
            challenge: Option<&str>,
            disabled: Option<bool>,
            form: Option<&str>,
            key_type: Option<&str>,
            key_params: Option<&str>,
            name: Option<&str>,
        }
    }
}
