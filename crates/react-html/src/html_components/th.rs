use crate::{HtmlCommonSharedPropsBuilder, HtmlTableCellSharedPropsBuilder};

crate::macros::def_intrinsic_component! {
    "th"
    TableHeaderCellComponent(TableHeaderCellComponentProps) {
        TableHeaderCellComponentProps
        : TableHeaderCellComponentPropsBuilder
        : HtmlTableCellSharedPropsBuilder[web_sys::HtmlTableCellElement]
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlTableCellElement]
        {
        }
    }
}
