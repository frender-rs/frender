use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "embed"
    EmbedComponent(EmbedComponentProps) {
        EmbedComponentProps
        : EmbedComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlEmbedElement]
        {
            width['a, T: Into<crate::NumOrStr<'a>>]: Option<T> { into? |v| v.into() },
            height['a, T: Into<crate::NumOrStr<'a>>]: Option<T> { into? |v| v.into() },
            src: Option<&str>,
            html_type@"type": Option<&str>,
        }
    }
}
