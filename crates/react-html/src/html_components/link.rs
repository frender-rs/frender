use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "link"
    LinkComponent(LinkComponentProps) {
        LinkComponentProps
        : LinkComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlLinkElement]
        {
            html_as@"as": Option<&str>,
            cross_origin: Option<&str>,
            href: Option<&str>,
            href_lang: Option<&str>,
            integrity: Option<&str>,
            media: Option<&str>,
            referrer_policy: Option<crate::ReferrerPolicy>,
            rel: Option<&str>,
            sizes: Option<&str>,
            html_type@"type": Option<&str>,
            char_set: Option<&str>,
        }
    }
}
