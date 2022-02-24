use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "base"
    BaseComponent(BaseComponentProps) {
        BaseComponentProps
        : BaseComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlBaseElement]
        {
            href: Option<&str>,
            target: Option<&str>,
        }
    }
}
