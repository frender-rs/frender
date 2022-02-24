use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "output"
    OutputComponent(OutputComponentProps) {
        OutputComponentProps
        : OutputComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlElement]
        {
            form: Option<&str>,
            html_for: Option<&str>,
            name: Option<&str>,
        }
    }
}
