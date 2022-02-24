use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "ins"
    InsComponent(InsComponentProps) {
        InsComponentProps
        : InsComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlModElement]
        {
            cite: Option<&str>,
            date_time: Option<&str>,
        }
    }
}
