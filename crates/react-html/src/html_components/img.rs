use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "img"
    ImageComponent(ImageComponentProps) {
        ImageComponentProps
        : ImageComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlImageElement]
        {
            alt: Option<&str>,
            cross_origin: Option<crate::CrossOrigin>,
            decoding: Option<crate::Decoding>,
            height['a, T: Into<crate::NumOrStr<'a>>]: Option<T> { into? |v| v.into() },
            loading: Option<crate::Loading>,
            referrer_policy: Option<crate::ReferrerPolicy>,
            sizes: Option<&str>,
            src: Option<&str>,
            src_set: Option<&str>,
            use_map: Option<&str>,
            width['a, T: Into<crate::NumOrStr<'a>>]: Option<T> { into? |v| v.into() },
        }
    }
}
