use crate::{HtmlCommonSharedPropsBuilder, HtmlTableCellSharedPropsBuilder};

crate::macros::def_intrinsic_component! {
    "td"
    TableDataCellComponent(TableDataCellComponentProps) {
        TableDataCellComponentProps
        : TableDataCellComponentPropsBuilder
        : HtmlTableCellSharedPropsBuilder[web_sys::HtmlTableCellElement]
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlTableCellElement]
        {
            height['a, T: Into<crate::NumOrStr<'a>>]: Option<T> { into? |v| v.into() },
            width['a, T: Into<crate::NumOrStr<'a>>]: Option<T> { into? |v| v.into() },
            valign: Option<crate::TableCellVAlign>,
        }
    }
}
