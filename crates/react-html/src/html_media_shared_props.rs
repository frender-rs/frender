use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_props_trait! {
    [TElement] HtmlMediaSharedProps [TElement] : HtmlMediaSharedPropsBuilder
    : HtmlCommonSharedPropsBuilder[TElement]
    {
        auto_play: Option<bool>,
        controls: Option<bool>,
        controls_list: Option<&str>,
        cross_origin: Option<&str>,
        {
        /// `loop` in html media props
        }
        html_loop@"loop": Option<bool>,
        media_group: Option<&str>,
        muted: Option<bool>,
        plays_inline: Option<bool>,
        preload: Option<&str>,
        src: Option<&str>,
    }
}
