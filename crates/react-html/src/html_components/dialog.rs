use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "dialog"
    DialogComponent(DialogComponentProps) {
        DialogComponentProps
        : DialogComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlDialogElement]
        {
            open: Option<bool>,
        }
    }
}
