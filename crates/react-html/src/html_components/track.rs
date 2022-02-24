use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "track"
    TrackComponent(TrackComponentProps) {
        TrackComponentProps
        : TrackComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlTrackElement]
        {
            default: Option<bool>,
            kind: Option<&str>,
            label: Option<&str>,
            src: Option<&str>,
            src_lang: Option<&str>,
        }
    }
}
