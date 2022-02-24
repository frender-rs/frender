use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "ol"
    OListComponent(OListComponentProps) {
        OListComponentProps
        : OListComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlOListElement]
        {
            reversed: Option<bool>,
            start: Option<f64>,
            html_type@"type": Option<crate::OListType>,
        }
    }
}
