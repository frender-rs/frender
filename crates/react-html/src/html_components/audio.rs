use crate::{HtmlCommonSharedPropsBuilder, HtmlMediaSharedPropsBuilder};

crate::macros::def_intrinsic_component! {
    "audio"
    AudioComponent(AudioComponentProps) {
        AudioComponentProps
        : AudioComponentPropsBuilder
        : HtmlMediaSharedPropsBuilder[web_sys::HtmlAudioElement]
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlAudioElement]
        {
        }
    }
}
