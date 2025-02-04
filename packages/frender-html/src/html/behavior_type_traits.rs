use super::{behaviors, event_types};
use crate::update_element::OnEventType;
use frender_common::convert::IdentityAs;
pub trait Node: crate::UiHandleType {
    type Node<Renderer: ?Sized + super::RenderHtml>: behaviors::Node<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait Element:
    crate::UiHandleType
    + Node
    + OnEventType<event_types::on_cancel>
    + OnEventType<event_types::on_error>
    + OnEventType<event_types::on_scroll>
    + OnEventType<event_types::on_security_policy_violation>
    + OnEventType<event_types::on_select>
    + OnEventType<event_types::on_wheel>
    + OnEventType<event_types::on_copy>
    + OnEventType<event_types::on_cut>
    + OnEventType<event_types::on_paste>
    + OnEventType<event_types::on_composition_end>
    + OnEventType<event_types::on_composition_start>
    + OnEventType<event_types::on_composition_update>
    + OnEventType<event_types::on_blur>
    + OnEventType<event_types::on_focus>
    + OnEventType<event_types::on_focus_in>
    + OnEventType<event_types::on_focus_out>
    + OnEventType<event_types::on_fullscreen_change>
    + OnEventType<event_types::on_fullscreen_error>
    + OnEventType<event_types::on_key_down>
    + OnEventType<event_types::on_key_up>
    + OnEventType<event_types::on_aux_click>
    + OnEventType<event_types::on_click>
    + OnEventType<event_types::on_context_menu>
    + OnEventType<event_types::on_double_click>
    + OnEventType<event_types::on_mouse_down>
    + OnEventType<event_types::on_mouse_enter>
    + OnEventType<event_types::on_mouse_leave>
    + OnEventType<event_types::on_mouse_move>
    + OnEventType<event_types::on_mouse_out>
    + OnEventType<event_types::on_mouse_over>
    + OnEventType<event_types::on_mouse_up>
    + OnEventType<event_types::on_touch_cancel>
    + OnEventType<event_types::on_touch_end>
    + OnEventType<event_types::on_touch_move>
    + OnEventType<event_types::on_touch_start>
{
    type Element<Renderer: ?Sized + super::RenderHtml>: behaviors::Element<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithHrefAttribute: crate::UiHandleType + Element {
    type ElementWithHrefAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithHrefAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithTargetAttribute: crate::UiHandleType + Element {
    type ElementWithTargetAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithTargetAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithTypeAttribute: crate::UiHandleType + Element {
    type ElementWithTypeAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithTypeAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithCiteAttribute: crate::UiHandleType + Element {
    type ElementWithCiteAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithCiteAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithPlaceHolderAttribute: crate::UiHandleType + Element {
    type ElementWithPlaceHolderAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithPlaceHolderAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithMaxMinLengthAttributes: crate::UiHandleType + Element {
    type ElementWithMaxMinLengthAttributes<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithMaxMinLengthAttributes<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithHeightWidthStrAttributes: crate::UiHandleType + Element {
    type ElementWithHeightWidthStrAttributes<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithHeightWidthStrAttributes<Renderer>
        + IdentityAs<Self::OfBehaviorType<Renderer>>
        + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithHeightWidthU32Attributes: crate::UiHandleType + Element {
    type ElementWithHeightWidthU32Attributes<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithHeightWidthU32Attributes<Renderer>
        + IdentityAs<Self::OfBehaviorType<Renderer>>
        + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithMaxF64Attribute: crate::UiHandleType + Element {
    type ElementWithMaxF64Attribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithMaxF64Attribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithValueF64Attribute: crate::UiHandleType + Element {
    type ElementWithValueF64Attribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithValueF64Attribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithValueStrAttribute: crate::UiHandleType + Element {
    type ElementWithValueStrAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithValueStrAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithOpenAttribute: crate::UiHandleType + Element {
    type ElementWithOpenAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithOpenAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithNameAttribute: crate::UiHandleType + Element {
    type ElementWithNameAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithNameAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithDisabledAttribute: crate::UiHandleType + Element {
    type ElementWithDisabledAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithDisabledAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithCrossOriginAttribute: crate::UiHandleType + Element {
    type ElementWithCrossOriginAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithCrossOriginAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithRelAttribute: crate::UiHandleType + Element {
    type ElementWithRelAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithRelAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithReferrerPolicyAttribute: crate::UiHandleType + Element {
    type ElementWithReferrerPolicyAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithReferrerPolicyAttribute<Renderer>
        + IdentityAs<Self::OfBehaviorType<Renderer>>
        + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithAltAttribute: crate::UiHandleType + Element {
    type ElementWithAltAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithAltAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithLoadingAttribute: crate::UiHandleType + Element {
    type ElementWithLoadingAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithLoadingAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithAcceptAttribute: crate::UiHandleType + Element {
    type ElementWithAcceptAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithAcceptAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithAutoCompleteAttribute: crate::UiHandleType + Element {
    type ElementWithAutoCompleteAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithAutoCompleteAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithAutoCorrectAttribute: crate::UiHandleType + Element {
    type ElementWithAutoCorrectAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithAutoCorrectAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithFormAttribute: crate::UiHandleType + Element {
    type ElementWithFormAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithFormAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithFormAttributes: crate::UiHandleType + Element + ElementWithFormAttribute {
    type ElementWithFormAttributes<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithFormAttributes<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithFetchPriorityAttribute: crate::UiHandleType + Element {
    type ElementWithFetchPriorityAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithFetchPriorityAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithHrefLangAttribute: crate::UiHandleType + Element + ElementWithHrefAttribute {
    type ElementWithHrefLangAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithHrefLangAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithSizesAttribute: crate::UiHandleType + Element {
    type ElementWithSizesAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithSizesAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithUseMapAttribute: crate::UiHandleType + Element {
    type ElementWithUseMapAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithUseMapAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithLabelAttribute: crate::UiHandleType + Element {
    type ElementWithLabelAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithLabelAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithForAttribute: crate::UiHandleType + Element {
    type ElementWithForAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithForAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithIntegrityAttribute: crate::UiHandleType + Element {
    type ElementWithIntegrityAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithIntegrityAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithBlockingAttribute: crate::UiHandleType + Element {
    type ElementWithBlockingAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithBlockingAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithMultipleAttribute: crate::UiHandleType + Element {
    type ElementWithMultipleAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithMultipleAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithRequiredAttribute: crate::UiHandleType + Element {
    type ElementWithRequiredAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithRequiredAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithSizeU32Attribute: crate::UiHandleType + Element {
    type ElementWithSizeU32Attribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithSizeU32Attribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithSrcAttribute: crate::UiHandleType + Element {
    type ElementWithSrcAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithSrcAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithSrcsetAttribute: crate::UiHandleType + Element + ElementWithSrcAttribute {
    type ElementWithSrcsetAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithSrcsetAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithBgColorAttribute: crate::UiHandleType + Element {
    type ElementWithBgColorAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithBgColorAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithAlignAttribute: crate::UiHandleType + Element {
    type ElementWithAlignAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithAlignAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithMediaAttribute: crate::UiHandleType + Element {
    type ElementWithMediaAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithMediaAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithReadOnlyAttribute: crate::UiHandleType + Element {
    type ElementWithReadOnlyAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithReadOnlyAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithDateTimeAttribute: crate::UiHandleType + Element {
    type ElementWithDateTimeAttribute<Renderer: ?Sized + super::RenderHtml>: behaviors::ElementWithDateTimeAttribute<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlElement:
    crate::UiHandleType
    + Element
    + OnEventType<event_types::on_invalid>
    + OnEventType<event_types::on_animation_cancel>
    + OnEventType<event_types::on_animation_end>
    + OnEventType<event_types::on_animation_iteration>
    + OnEventType<event_types::on_animation_start>
    + OnEventType<event_types::on_before_input>
    + OnEventType<event_types::on_input>
    + OnEventType<event_types::on_change>
    + OnEventType<event_types::on_got_pointer_capture>
    + OnEventType<event_types::on_lost_pointer_capture>
    + OnEventType<event_types::on_pointer_cancel>
    + OnEventType<event_types::on_pointer_down>
    + OnEventType<event_types::on_pointer_enter>
    + OnEventType<event_types::on_pointer_leave>
    + OnEventType<event_types::on_pointer_move>
    + OnEventType<event_types::on_pointer_out>
    + OnEventType<event_types::on_pointer_over>
    + OnEventType<event_types::on_pointer_up>
    + OnEventType<event_types::on_transition_cancel>
    + OnEventType<event_types::on_transition_end>
    + OnEventType<event_types::on_transition_run>
    + OnEventType<event_types::on_transition_start>
    + OnEventType<event_types::on_drag>
    + OnEventType<event_types::on_drag_end>
    + OnEventType<event_types::on_drag_enter>
    + OnEventType<event_types::on_drag_leave>
    + OnEventType<event_types::on_drag_over>
    + OnEventType<event_types::on_drag_start>
    + OnEventType<event_types::on_drop>
{
    type HtmlElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlDataListElement: crate::UiHandleType + HtmlElement {
    type HtmlDataListElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlDataListElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlDivElement: crate::UiHandleType + HtmlElement {
    type HtmlDivElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlDivElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlDListElement: crate::UiHandleType + HtmlElement {
    type HtmlDListElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlDListElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlHeadingElement: crate::UiHandleType + HtmlElement {
    type HtmlHeadingElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlHeadingElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlHeadElement: crate::UiHandleType + HtmlElement {
    type HtmlHeadElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlHeadElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlHrElement: crate::UiHandleType + HtmlElement {
    type HtmlHrElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlHrElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlLegendElement: crate::UiHandleType + HtmlElement {
    type HtmlLegendElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlLegendElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlMenuElement: crate::UiHandleType + HtmlElement {
    type HtmlMenuElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlMenuElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlParagraphElement: crate::UiHandleType + HtmlElement {
    type HtmlParagraphElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlParagraphElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlPictureElement: crate::UiHandleType + HtmlElement {
    type HtmlPictureElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlPictureElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlPreElement: crate::UiHandleType + HtmlElement {
    type HtmlPreElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlPreElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlSpanElement: crate::UiHandleType + HtmlElement {
    type HtmlSpanElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlSpanElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlTemplateElement: crate::UiHandleType + HtmlElement {
    type HtmlTemplateElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlTemplateElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlTitleElement: crate::UiHandleType + HtmlElement {
    type HtmlTitleElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlTitleElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlElementWithHref: crate::UiHandleType + HtmlElement + ElementWithHrefAttribute + ElementWithTargetAttribute + ElementWithReferrerPolicyAttribute + ElementWithRelAttribute {
    type HtmlElementWithHref<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlElementWithHref<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlAnchorElement: crate::UiHandleType + HtmlElement + HtmlElementWithHref + ElementWithTypeAttribute + ElementWithHrefLangAttribute {
    type HtmlAnchorElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlAnchorElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlAreaElement: crate::UiHandleType + HtmlElement + HtmlElementWithHref + ElementWithAltAttribute {
    type HtmlAreaElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlAreaElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlMediaElement:
    crate::UiHandleType
    + HtmlElement
    + ElementWithSrcAttribute
    + ElementWithCrossOriginAttribute
    + OnEventType<event_types::on_abort>
    + OnEventType<event_types::on_can_play>
    + OnEventType<event_types::on_can_play_through>
    + OnEventType<event_types::on_duration_change>
    + OnEventType<event_types::on_emptied>
    + OnEventType<event_types::on_ended>
    + OnEventType<event_types::on_loaded_data>
    + OnEventType<event_types::on_loaded_metadata>
    + OnEventType<event_types::on_load_start>
    + OnEventType<event_types::on_pause>
    + OnEventType<event_types::on_play>
    + OnEventType<event_types::on_playing>
    + OnEventType<event_types::on_progress>
    + OnEventType<event_types::on_rate_change>
    + OnEventType<event_types::on_resize>
    + OnEventType<event_types::on_seeked>
    + OnEventType<event_types::on_seeking>
    + OnEventType<event_types::on_stalled>
    + OnEventType<event_types::on_suspend>
    + OnEventType<event_types::on_time_update>
    + OnEventType<event_types::on_volume_change>
    + OnEventType<event_types::on_waiting>
{
    type HtmlMediaElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlMediaElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlBaseElement: crate::UiHandleType + HtmlElement + ElementWithHrefAttribute + ElementWithTargetAttribute {
    type HtmlBaseElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlBaseElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlQuoteElement: crate::UiHandleType + HtmlElement + ElementWithCiteAttribute {
    type HtmlQuoteElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlQuoteElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlBodyElement: crate::UiHandleType + HtmlElement {
    type HtmlBodyElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlBodyElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlBrElement: crate::UiHandleType + HtmlElement {
    type HtmlBrElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlBrElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlButtonElement: crate::UiHandleType + HtmlElement + ElementWithTypeAttribute + ElementWithFormAttributes + ElementWithDisabledAttribute + ElementWithNameAttribute + ElementWithValueStrAttribute {
    type HtmlButtonElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlButtonElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlCanvasElement: crate::UiHandleType + HtmlElement + ElementWithHeightWidthU32Attributes {
    type HtmlCanvasElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlCanvasElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlTableCaptionElement: crate::UiHandleType + HtmlElement + ElementWithAlignAttribute {
    type HtmlTableCaptionElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlTableCaptionElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlDataElement: crate::UiHandleType + HtmlElement + ElementWithValueStrAttribute {
    type HtmlDataElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlDataElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlModElement: crate::UiHandleType + HtmlElement + ElementWithCiteAttribute + ElementWithDateTimeAttribute {
    type HtmlModElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlModElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlDetailsElement: crate::UiHandleType + HtmlElement + ElementWithOpenAttribute {
    type HtmlDetailsElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlDetailsElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlDialogElement: crate::UiHandleType + HtmlElement + ElementWithOpenAttribute {
    type HtmlDialogElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlDialogElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlEmbedElement: crate::UiHandleType + HtmlElement + ElementWithTypeAttribute + ElementWithSrcAttribute + ElementWithHeightWidthStrAttributes {
    type HtmlEmbedElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlEmbedElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlFieldSetElement: crate::UiHandleType + HtmlElement + ElementWithFormAttribute + ElementWithDisabledAttribute + ElementWithNameAttribute {
    type HtmlFieldSetElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlFieldSetElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlFormElement:
    crate::UiHandleType
    + HtmlElement
    + ElementWithTargetAttribute
    + ElementWithAutoCompleteAttribute
    + ElementWithAcceptAttribute
    + ElementWithRelAttribute
    + ElementWithNameAttribute
    + OnEventType<event_types::on_form_data>
    + OnEventType<event_types::on_reset>
    + OnEventType<event_types::on_submit>
{
    type HtmlFormElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlFormElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlHtmlElement: crate::UiHandleType + HtmlElement {
    type HtmlHtmlElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlHtmlElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlIFrameElement:
    crate::UiHandleType
    + HtmlElement
    + ElementWithSrcAttribute
    + ElementWithFetchPriorityAttribute
    + ElementWithLoadingAttribute
    + ElementWithReferrerPolicyAttribute
    + ElementWithNameAttribute
    + ElementWithHeightWidthStrAttributes
{
    type HtmlIFrameElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlIFrameElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlImageElement:
    crate::UiHandleType
    + HtmlElement
    + ElementWithSrcsetAttribute
    + ElementWithUseMapAttribute
    + ElementWithSizesAttribute
    + ElementWithLoadingAttribute
    + ElementWithAltAttribute
    + ElementWithReferrerPolicyAttribute
    + ElementWithCrossOriginAttribute
    + ElementWithHeightWidthU32Attributes
{
    type HtmlImageElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlImageElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlInputElement:
    crate::UiHandleType
    + HtmlElement
    + ElementWithReadOnlyAttribute
    + ElementWithPlaceHolderAttribute
    + ElementWithMaxMinLengthAttributes
    + ElementWithSrcAttribute
    + ElementWithSizeU32Attribute
    + ElementWithRequiredAttribute
    + ElementWithMultipleAttribute
    + ElementWithFormAttributes
    + ElementWithAutoCompleteAttribute
    + ElementWithAutoCorrectAttribute
    + ElementWithAcceptAttribute
    + ElementWithAltAttribute
    + ElementWithDisabledAttribute
    + ElementWithNameAttribute
    + ElementWithHeightWidthU32Attributes
{
    type HtmlInputElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlInputElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlLabelElement: crate::UiHandleType + HtmlElement + ElementWithForAttribute {
    type HtmlLabelElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlLabelElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlLiElement: crate::UiHandleType + HtmlElement {
    type HtmlLiElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlLiElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlLinkElement:
    crate::UiHandleType
    + HtmlElement
    + ElementWithHrefAttribute
    + ElementWithTypeAttribute
    + ElementWithMediaAttribute
    + ElementWithBlockingAttribute
    + ElementWithIntegrityAttribute
    + ElementWithSizesAttribute
    + ElementWithHrefLangAttribute
    + ElementWithFetchPriorityAttribute
    + ElementWithReferrerPolicyAttribute
    + ElementWithRelAttribute
    + ElementWithCrossOriginAttribute
{
    type HtmlLinkElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlLinkElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlMapElement: crate::UiHandleType + HtmlElement + ElementWithNameAttribute {
    type HtmlMapElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlMapElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlMetaElement: crate::UiHandleType + HtmlElement + ElementWithNameAttribute {
    type HtmlMetaElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlMetaElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlMeterElement: crate::UiHandleType + HtmlElement + ElementWithMaxF64Attribute + ElementWithValueF64Attribute {
    type HtmlMeterElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlMeterElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlObjectElement: crate::UiHandleType + HtmlElement + ElementWithTypeAttribute + ElementWithUseMapAttribute + ElementWithFormAttribute + ElementWithNameAttribute + ElementWithHeightWidthStrAttributes {
    type HtmlObjectElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlObjectElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlOListElement: crate::UiHandleType + HtmlElement + ElementWithTypeAttribute {
    type HtmlOListElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlOListElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlOptGroupElement: crate::UiHandleType + HtmlElement + ElementWithLabelAttribute + ElementWithDisabledAttribute {
    type HtmlOptGroupElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlOptGroupElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlOptionElement: crate::UiHandleType + HtmlElement + ElementWithLabelAttribute + ElementWithDisabledAttribute + ElementWithValueStrAttribute {
    type HtmlOptionElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlOptionElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlOutputElement: crate::UiHandleType + HtmlElement + ElementWithForAttribute + ElementWithFormAttribute + ElementWithNameAttribute {
    type HtmlOutputElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlOutputElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlProgressElement: crate::UiHandleType + HtmlElement + ElementWithMaxF64Attribute + ElementWithValueF64Attribute {
    type HtmlProgressElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlProgressElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlScriptElement:
    crate::UiHandleType
    + HtmlElement
    + ElementWithTypeAttribute
    + ElementWithSrcAttribute
    + ElementWithBlockingAttribute
    + ElementWithIntegrityAttribute
    + ElementWithFetchPriorityAttribute
    + ElementWithReferrerPolicyAttribute
    + ElementWithCrossOriginAttribute
{
    type HtmlScriptElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlScriptElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlSelectElement:
    crate::UiHandleType
    + HtmlElement
    + ElementWithSizeU32Attribute
    + ElementWithRequiredAttribute
    + ElementWithMultipleAttribute
    + ElementWithFormAttribute
    + ElementWithAutoCompleteAttribute
    + ElementWithDisabledAttribute
    + ElementWithNameAttribute
{
    type HtmlSelectElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlSelectElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlSlotElement: crate::UiHandleType + HtmlElement + ElementWithNameAttribute {
    type HtmlSlotElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlSlotElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlSourceElement:
    crate::UiHandleType + HtmlElement + ElementWithTypeAttribute + ElementWithMediaAttribute + ElementWithSrcsetAttribute + ElementWithSizesAttribute + ElementWithHeightWidthU32Attributes
{
    type HtmlSourceElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlSourceElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlStyleElement: crate::UiHandleType + HtmlElement + ElementWithTypeAttribute + ElementWithMediaAttribute + ElementWithBlockingAttribute {
    type HtmlStyleElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlStyleElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlTableElement: crate::UiHandleType + HtmlElement + ElementWithAlignAttribute + ElementWithBgColorAttribute {
    type HtmlTableElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlTableElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlTableChildElement: crate::UiHandleType + HtmlElement + ElementWithAlignAttribute + ElementWithBgColorAttribute {
    type HtmlTableChildElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlTableChildElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlTableSectionElement: crate::UiHandleType + HtmlElement + HtmlTableChildElement {
    type HtmlTableSectionElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlTableSectionElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlTableRowElement: crate::UiHandleType + HtmlElement + HtmlTableChildElement {
    type HtmlTableRowElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlTableRowElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlTableColElement: crate::UiHandleType + HtmlElement + HtmlTableChildElement {
    type HtmlTableColElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlTableColElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlTableCellElement: crate::UiHandleType + HtmlElement + ElementWithHeightWidthStrAttributes + HtmlTableChildElement {
    type HtmlTableCellElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlTableCellElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlTextAreaElement:
    crate::UiHandleType
    + HtmlElement
    + ElementWithReadOnlyAttribute
    + ElementWithPlaceHolderAttribute
    + ElementWithMaxMinLengthAttributes
    + ElementWithRequiredAttribute
    + ElementWithFormAttribute
    + ElementWithAutoCompleteAttribute
    + ElementWithAutoCorrectAttribute
    + ElementWithDisabledAttribute
    + ElementWithNameAttribute
{
    type HtmlTextAreaElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlTextAreaElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlTimeElement: crate::UiHandleType + HtmlElement + ElementWithDateTimeAttribute {
    type HtmlTimeElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlTimeElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlTrackElement: crate::UiHandleType + HtmlElement + ElementWithSrcAttribute + ElementWithLabelAttribute {
    type HtmlTrackElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlTrackElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlUListElement: crate::UiHandleType + HtmlElement + ElementWithTypeAttribute {
    type HtmlUListElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlUListElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlAudioElement: crate::UiHandleType + HtmlMediaElement {
    type HtmlAudioElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlAudioElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlVideoElement: crate::UiHandleType + HtmlMediaElement + ElementWithHeightWidthU32Attributes {
    type HtmlVideoElement<Renderer: ?Sized + super::RenderHtml>: behaviors::HtmlVideoElement<Renderer> + IdentityAs<Self::OfBehaviorType<Renderer>> + IdentityAs<Self::UiHandle<Renderer>>;
}
