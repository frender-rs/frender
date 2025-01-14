macro_rules! Node {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use Node;
macro_rules! Element {
    ($commands:tt) => {
        ::frender_common::expand! { { on_cancel on_error on_scroll
        on_security_policy_violation on_select on_wheel on_copy on_cut on_paste
        on_composition_end on_composition_start on_composition_update on_blur on_focus
        on_focus_in on_focus_out on_fullscreen_change on_fullscreen_error on_key_down
        on_key_up on_aux_click on_click on_context_menu on_double_click on_mouse_down
        on_mouse_enter on_mouse_leave on_mouse_move on_mouse_out on_mouse_over
        on_mouse_up on_touch_cancel on_touch_end on_touch_move on_touch_start } do
        $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { { on_cancel on_error on_scroll
        on_security_policy_violation on_select on_wheel on_copy on_cut on_paste
        on_composition_end on_composition_start on_composition_update on_blur on_focus
        on_focus_in on_focus_out on_fullscreen_change on_fullscreen_error on_key_down
        on_key_up on_aux_click on_click on_context_menu on_double_click on_mouse_down
        on_mouse_enter on_mouse_leave on_mouse_move on_mouse_out on_mouse_over
        on_mouse_up on_touch_cancel on_touch_end on_touch_move on_touch_start } $args do
        $commands }
    };
}
pub(crate) use Element;
macro_rules! ElementWithHrefAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithHrefAttribute;
macro_rules! ElementWithTargetAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithTargetAttribute;
macro_rules! ElementWithTypeAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithTypeAttribute;
macro_rules! ElementWithCiteAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithCiteAttribute;
macro_rules! ElementWithPlaceHolderAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithPlaceHolderAttribute;
macro_rules! ElementWithMaxMinLengthAttributes {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithMaxMinLengthAttributes;
macro_rules! ElementWithHeightWidthStrAttributes {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithHeightWidthStrAttributes;
macro_rules! ElementWithHeightWidthU32Attributes {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithHeightWidthU32Attributes;
macro_rules! ElementWithMaxF64Attribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithMaxF64Attribute;
macro_rules! ElementWithValueF64Attribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithValueF64Attribute;
macro_rules! ElementWithValueStrAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithValueStrAttribute;
macro_rules! ElementWithOpenAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithOpenAttribute;
macro_rules! ElementWithNameAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithNameAttribute;
macro_rules! ElementWithDisabledAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithDisabledAttribute;
macro_rules! ElementWithCrossOriginAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithCrossOriginAttribute;
macro_rules! ElementWithRelAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithRelAttribute;
macro_rules! ElementWithReferrerPolicyAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithReferrerPolicyAttribute;
macro_rules! ElementWithAltAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithAltAttribute;
macro_rules! ElementWithLoadingAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithLoadingAttribute;
macro_rules! ElementWithAcceptAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithAcceptAttribute;
macro_rules! ElementWithAutoCompleteAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithAutoCompleteAttribute;
macro_rules! ElementWithAutoCorrectAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithAutoCorrectAttribute;
macro_rules! ElementWithFormAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithFormAttribute;
macro_rules! ElementWithFormAttributes {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithFormAttributes;
macro_rules! ElementWithFetchPriorityAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithFetchPriorityAttribute;
macro_rules! ElementWithHrefLangAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithHrefLangAttribute;
macro_rules! ElementWithSizesAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithSizesAttribute;
macro_rules! ElementWithUseMapAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithUseMapAttribute;
macro_rules! ElementWithLabelAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithLabelAttribute;
macro_rules! ElementWithForAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithForAttribute;
macro_rules! ElementWithIntegrityAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithIntegrityAttribute;
macro_rules! ElementWithBlockingAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithBlockingAttribute;
macro_rules! ElementWithMultipleAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithMultipleAttribute;
macro_rules! ElementWithRequiredAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithRequiredAttribute;
macro_rules! ElementWithSizeU32Attribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithSizeU32Attribute;
macro_rules! ElementWithSrcAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithSrcAttribute;
macro_rules! ElementWithSrcsetAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithSrcsetAttribute;
macro_rules! ElementWithBgColorAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithBgColorAttribute;
macro_rules! ElementWithAlignAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithAlignAttribute;
macro_rules! ElementWithMediaAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithMediaAttribute;
macro_rules! ElementWithReadOnlyAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithReadOnlyAttribute;
macro_rules! ElementWithDateTimeAttribute {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use ElementWithDateTimeAttribute;
macro_rules! HtmlElement {
    ($commands:tt) => {
        ::frender_common::expand! { { on_invalid on_animation_cancel on_animation_end
        on_animation_iteration on_animation_start on_before_input on_input on_change
        on_got_pointer_capture on_lost_pointer_capture on_pointer_cancel on_pointer_down
        on_pointer_enter on_pointer_leave on_pointer_move on_pointer_out on_pointer_over
        on_pointer_up on_transition_cancel on_transition_end on_transition_run
        on_transition_start on_drag on_drag_end on_drag_enter on_drag_leave on_drag_over
        on_drag_start on_drop } do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { { on_invalid on_animation_cancel
        on_animation_end on_animation_iteration on_animation_start on_before_input
        on_input on_change on_got_pointer_capture on_lost_pointer_capture
        on_pointer_cancel on_pointer_down on_pointer_enter on_pointer_leave
        on_pointer_move on_pointer_out on_pointer_over on_pointer_up on_transition_cancel
        on_transition_end on_transition_run on_transition_start on_drag on_drag_end
        on_drag_enter on_drag_leave on_drag_over on_drag_start on_drop } $args do
        $commands }
    };
}
pub(crate) use HtmlElement;
macro_rules! HtmlDataListElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlDataListElement;
macro_rules! HtmlDivElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlDivElement;
macro_rules! HtmlDListElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlDListElement;
macro_rules! HtmlHeadingElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlHeadingElement;
macro_rules! HtmlHeadElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlHeadElement;
macro_rules! HtmlHrElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlHrElement;
macro_rules! HtmlLegendElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlLegendElement;
macro_rules! HtmlMenuElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlMenuElement;
macro_rules! HtmlParagraphElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlParagraphElement;
macro_rules! HtmlPictureElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlPictureElement;
macro_rules! HtmlPreElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlPreElement;
macro_rules! HtmlSpanElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlSpanElement;
macro_rules! HtmlTemplateElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlTemplateElement;
macro_rules! HtmlTitleElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlTitleElement;
macro_rules! HtmlElementWithHref {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlElementWithHref;
macro_rules! HtmlAnchorElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlAnchorElement;
macro_rules! HtmlAreaElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlAreaElement;
macro_rules! HtmlMediaElement {
    ($commands:tt) => {
        ::frender_common::expand! { { on_abort on_can_play on_can_play_through
        on_duration_change on_emptied on_ended on_loaded_data on_loaded_metadata
        on_load_start on_pause on_play on_playing on_progress on_rate_change on_resize
        on_seeked on_seeking on_stalled on_suspend on_time_update on_volume_change
        on_waiting } do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { { on_abort on_can_play
        on_can_play_through on_duration_change on_emptied on_ended on_loaded_data
        on_loaded_metadata on_load_start on_pause on_play on_playing on_progress
        on_rate_change on_resize on_seeked on_seeking on_stalled on_suspend
        on_time_update on_volume_change on_waiting } $args do $commands }
    };
}
pub(crate) use HtmlMediaElement;
macro_rules! HtmlBaseElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlBaseElement;
macro_rules! HtmlQuoteElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlQuoteElement;
macro_rules! HtmlBodyElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlBodyElement;
macro_rules! HtmlBrElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlBrElement;
macro_rules! HtmlButtonElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlButtonElement;
macro_rules! HtmlCanvasElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlCanvasElement;
macro_rules! HtmlTableCaptionElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlTableCaptionElement;
macro_rules! HtmlDataElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlDataElement;
macro_rules! HtmlModElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlModElement;
macro_rules! HtmlDetailsElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlDetailsElement;
macro_rules! HtmlDialogElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlDialogElement;
macro_rules! HtmlEmbedElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlEmbedElement;
macro_rules! HtmlFieldSetElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlFieldSetElement;
macro_rules! HtmlFormElement {
    ($commands:tt) => {
        ::frender_common::expand! { { on_form_data on_reset on_submit } do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { { on_form_data on_reset on_submit }
        $args do $commands }
    };
}
pub(crate) use HtmlFormElement;
macro_rules! HtmlHtmlElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlHtmlElement;
macro_rules! HtmlIFrameElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlIFrameElement;
macro_rules! HtmlImageElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlImageElement;
macro_rules! HtmlInputElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlInputElement;
macro_rules! HtmlLabelElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlLabelElement;
macro_rules! HtmlLiElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlLiElement;
macro_rules! HtmlLinkElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlLinkElement;
macro_rules! HtmlMapElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlMapElement;
macro_rules! HtmlMetaElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlMetaElement;
macro_rules! HtmlMeterElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlMeterElement;
macro_rules! HtmlObjectElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlObjectElement;
macro_rules! HtmlOListElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlOListElement;
macro_rules! HtmlOptGroupElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlOptGroupElement;
macro_rules! HtmlOptionElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlOptionElement;
macro_rules! HtmlOutputElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlOutputElement;
macro_rules! HtmlProgressElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlProgressElement;
macro_rules! HtmlScriptElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlScriptElement;
macro_rules! HtmlSelectElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlSelectElement;
macro_rules! HtmlSlotElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlSlotElement;
macro_rules! HtmlSourceElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlSourceElement;
macro_rules! HtmlStyleElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlStyleElement;
macro_rules! HtmlTableElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlTableElement;
macro_rules! HtmlTableChildElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlTableChildElement;
macro_rules! HtmlTableSectionElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlTableSectionElement;
macro_rules! HtmlTableRowElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlTableRowElement;
macro_rules! HtmlTableColElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlTableColElement;
macro_rules! HtmlTableCellElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlTableCellElement;
macro_rules! HtmlTextAreaElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlTextAreaElement;
macro_rules! HtmlTimeElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlTimeElement;
macro_rules! HtmlTrackElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlTrackElement;
macro_rules! HtmlUListElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlUListElement;
macro_rules! HtmlAudioElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlAudioElement;
macro_rules! HtmlVideoElement {
    ($commands:tt) => {
        ::frender_common::expand! { {} do $commands }
    };
    (args $args:tt do $commands:tt) => {
        crate::macros::event_names::expand_impl! { {} $args do $commands }
    };
}
pub(crate) use HtmlVideoElement;
