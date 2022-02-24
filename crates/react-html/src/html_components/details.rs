use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "details"
    DetailsComponent(DetailsComponentProps) {
        DetailsComponentProps
        : DetailsComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlElement]
        {
            open: Option<bool>,
            // TODO: ReactEventHandler<web_sys::HtmlElement>
            on_toggle: (),
        }
    }
}
