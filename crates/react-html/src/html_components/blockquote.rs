use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "blockquote"
    BlockquoteComponent(BlockquoteComponentProps) {
        BlockquoteComponentProps
        : BlockquoteComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlElement]
        {
            cite: Option<&str>,
        }
    }
}
