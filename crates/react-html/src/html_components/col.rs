use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "col"
    ColComponent(ColComponentProps) {
        ColComponentProps
        : ColComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlTableColElement]
        {
            span: Option<usize>,
            width['a, T: Into<crate::NumOrStr<'a>>]: Option<T> { into? |v| v.into() },
        }
    }
}
