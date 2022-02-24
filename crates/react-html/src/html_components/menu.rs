use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "menu"
    MenuComponent(MenuComponentProps) {
        MenuComponentProps
        : MenuComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlElement]
        {
            html_type@"type": Option<&str>,
        }
    }
}
