use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "meta"
    MetaComponent(MetaComponentProps) {
        MetaComponentProps
        : MetaComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlMetaElement]
        {
            char_set: Option<&str>,
            content: Option<&str>,
            http_equiv: Option<&str>,
            name: Option<&str>,
            media: Option<&str>,
        }
    }
}
