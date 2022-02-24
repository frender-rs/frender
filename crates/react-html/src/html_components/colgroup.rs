use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "colgroup"
    ColgroupComponent(ColgroupComponentProps) {
        ColgroupComponentProps
        : ColgroupComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlTableColElement]
        {
            span: Option<usize>,
        }
    }
}
