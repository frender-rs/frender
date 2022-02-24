use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "table"
    TableComponent(TableComponentProps) {
        TableComponentProps
        : TableComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlTableElement]
        {
            cell_padding['a, T: Into<crate::NumOrStr<'a>>]: Option<T> { into? |v| v.into() },
            cell_spacing['a, T: Into<crate::NumOrStr<'a>>]: Option<T> { into? |v| v.into() },
            summary: Option<&str>,
            width['a, T: Into<crate::NumOrStr<'a>>]: Option<T> { into? |v| v.into() },
        }
    }
}
