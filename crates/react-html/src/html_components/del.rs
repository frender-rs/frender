use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "del"
    DelComponent(DelComponentProps) {
        DelComponentProps
        : DelComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlElement]
        {
            cite: Option<&str>,
            date_time: Option<&str>,
        }
    }
}
