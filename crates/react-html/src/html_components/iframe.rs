use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "iframe"
    IFrameComponent(IFrameComponentProps) {
        IFrameComponentProps
        : IFrameComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlIFrameElement]
        {
            allow: Option<&str>,
            allow_full_screen: Option<bool>,
            allow_transparency: Option<bool>,
            { #[deprecated] }
            frame_border['a, T: Into<crate::NumOrStr<'a>>]: Option<T> { into? |v| v.into() },
            height['a, T: Into<crate::NumOrStr<'a>>]: Option<T> { into? |v| v.into() },
            loading: Option<crate::HtmlLoading>,
            { #[deprecated] }
            margin_height: Option<f64>,
            { #[deprecated] }
            margin_width: Option<f64>,
            name: Option<&str>,
            referrer_policy: Option<crate::ReferrerPolicy>,
            sandbox: Option<&str>,
            { #[deprecated] }
            scrolling: Option<&str>,
            seamless: Option<bool>,
            src: Option<&str>,
            src_doc: Option<&str>,
            width['a, T: Into<crate::NumOrStr<'a>>]: Option<T> { into? |v| v.into() },
        }
    }
}
