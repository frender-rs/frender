use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_props_trait! {
    [TElement] HtmlTableCellSharedProps [TElement] : HtmlTableCellSharedPropsBuilder
    : HtmlCommonSharedPropsBuilder[TElement]
    {
        align: Option<crate::TableCellAlign>,
        col_span: Option<usize>,
        headers: Option<&str>,
        row_span: Option<f64>,
        scope: Option<&str>,
        abbr: Option<&str>,
    }
}
