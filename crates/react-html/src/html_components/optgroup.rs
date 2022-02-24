use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "optgroup"
    OptGroupComponent(OptGroupComponentProps) {
        OptGroupComponentProps
        : OptGroupComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlOptGroupElement]
        {
            disabled: Option<bool>,
            label: Option<&str>,
        }
    }
}
