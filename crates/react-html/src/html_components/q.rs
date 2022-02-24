use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "q"
    QuoteComponent(QuoteComponentProps) {
        QuoteComponentProps
        : QuoteComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlQuoteElement]
        {
            cite: Option<&str>,
        }
    }
}
