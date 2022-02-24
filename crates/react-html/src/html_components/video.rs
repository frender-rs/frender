use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "video"
    VideoComponent(VideoComponentProps) {
        VideoComponentProps
        : VideoComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlVideoElement]
        {
            height['a, T: Into<crate::NumOrStr<'a>>]: Option<T> { into? |v| v.into() },
            plays_inline: Option<bool>,
            poster: Option<&str>,
            width['a, T: Into<crate::NumOrStr<'a>>]: Option<T> { into? |v| v.into() },
            disable_picture_in_picture: Option<bool>,
            disable_remote_playback: Option<bool>,
        }
    }
}
