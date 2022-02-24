use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "slot"
    SlotComponent(SlotComponentProps) {
        SlotComponentProps
        : SlotComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlSlotElement]
        {
            name: Option<&str>,
        }
    }
}
