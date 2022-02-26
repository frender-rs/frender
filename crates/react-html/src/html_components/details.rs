use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "details"
    DetailsComponent(DetailsComponentProps) {
        DetailsComponentProps
        : DetailsComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlElement]
        {
            open: Option<bool>,
            on_toggle: react::event::SyntheticEvent<web_sys::HtmlElement> { event_handler },
        }
    }
}
