use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "source"
    SourceComponent(SourceComponentProps) {
        SourceComponentProps
        : SourceComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlSourceElement]
        {
            height['a, T: Into<crate::NumOrStr<'a>>]: Option<T> { into? |v| v.into() },
            media: Option<&str>,
            sizes: Option<&str>,
            src: Option<&str>,
            src_set: Option<&str>,
            html_type@"type": Option<&str>,
            width['a, T: Into<crate::NumOrStr<'a>>]: Option<T> { into? |v| v.into() },
        }
    }
}
