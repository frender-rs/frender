use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "time"
    TimeComponent(TimeComponentProps) {
        TimeComponentProps
        : TimeComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlElement]
        {
            date_time: Option<String>,
        }
    }
}
