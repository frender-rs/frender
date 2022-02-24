use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "canvas"
    CanvasComponent(CanvasComponentProps) {
        CanvasComponentProps
        : CanvasComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlCanvasElement]
        {
            width['a, T: Into<crate::NumOrStr<'a>>]: Option<T> { into? |v| v.into() },
            height['a, T: Into<crate::NumOrStr<'a>>]: Option<T> { into? |v| v.into() },
        }
    }
}
