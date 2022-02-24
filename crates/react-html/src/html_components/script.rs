use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "script"
    ScriptComponent(ScriptComponentProps) {
        ScriptComponentProps
        : ScriptComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlScriptElement]
        {
            html_async@"async": Option<bool>,
            {#[deprecated]}
            char_set: Option<&str>,
            cross_origin: Option<&str>,
            defer: Option<bool>,
            integrity: Option<&str>,
            no_module: Option<bool>,
            nonce: Option<&str>,
            referrer_policy: Option<crate::ReferrerPolicy>,
            src: Option<&str>,
            html_type@"type": Option<&str>,
        }
    }
}
