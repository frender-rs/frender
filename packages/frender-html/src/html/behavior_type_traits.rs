use crate::update_element::OnEventType;
pub trait Node: crate::UiHandleType {
    type Node<Renderer: ?Sized + super::RenderHtml>: super::behaviors::Node<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait Element:
    crate::UiHandleType
    + Node
    + OnEventType<super::event_types::on_cancel>
    + OnEventType<super::event_types::on_error>
    + OnEventType<super::event_types::on_scroll>
    + OnEventType<super::event_types::on_security_policy_violation>
    + OnEventType<super::event_types::on_select>
    + OnEventType<super::event_types::on_wheel>
    + OnEventType<super::event_types::on_copy>
    + OnEventType<super::event_types::on_cut>
    + OnEventType<super::event_types::on_paste>
    + OnEventType<super::event_types::on_composition_end>
    + OnEventType<super::event_types::on_composition_start>
    + OnEventType<super::event_types::on_composition_update>
    + OnEventType<super::event_types::on_blur>
    + OnEventType<super::event_types::on_focus>
    + OnEventType<super::event_types::on_focus_in>
    + OnEventType<super::event_types::on_focus_out>
    + OnEventType<super::event_types::on_fullscreen_change>
    + OnEventType<super::event_types::on_fullscreen_error>
    + OnEventType<super::event_types::on_key_down>
    + OnEventType<super::event_types::on_key_up>
    + OnEventType<super::event_types::on_aux_click>
    + OnEventType<super::event_types::on_click>
    + OnEventType<super::event_types::on_context_menu>
    + OnEventType<super::event_types::on_double_click>
    + OnEventType<super::event_types::on_mouse_down>
    + OnEventType<super::event_types::on_mouse_enter>
    + OnEventType<super::event_types::on_mouse_leave>
    + OnEventType<super::event_types::on_mouse_move>
    + OnEventType<super::event_types::on_mouse_out>
    + OnEventType<super::event_types::on_mouse_over>
    + OnEventType<super::event_types::on_mouse_up>
    + OnEventType<super::event_types::on_touch_cancel>
    + OnEventType<super::event_types::on_touch_end>
    + OnEventType<super::event_types::on_touch_move>
    + OnEventType<super::event_types::on_touch_start>
{
    type Element<Renderer: ?Sized + super::RenderHtml>: super::behaviors::Element<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithHrefAttribute: crate::UiHandleType + Element {
    type ElementWithHrefAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithHrefAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithTargetAttribute: crate::UiHandleType + Element {
    type ElementWithTargetAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithTargetAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithTypeAttribute: crate::UiHandleType + Element {
    type ElementWithTypeAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithTypeAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithCiteAttribute: crate::UiHandleType + Element {
    type ElementWithCiteAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithCiteAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithPlaceHolderAttribute: crate::UiHandleType + Element {
    type ElementWithPlaceHolderAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithPlaceHolderAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithMaxMinLengthAttributes: crate::UiHandleType + Element {
    type ElementWithMaxMinLengthAttributes<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithMaxMinLengthAttributes<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithHeightWidthStrAttributes: crate::UiHandleType + Element {
    type ElementWithHeightWidthStrAttributes<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithHeightWidthStrAttributes<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithHeightWidthU32Attributes: crate::UiHandleType + Element {
    type ElementWithHeightWidthU32Attributes<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithHeightWidthU32Attributes<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithMaxF64Attribute: crate::UiHandleType + Element {
    type ElementWithMaxF64Attribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithMaxF64Attribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithValueF64Attribute: crate::UiHandleType + Element {
    type ElementWithValueF64Attribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithValueF64Attribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithValueStrAttribute: crate::UiHandleType + Element {
    type ElementWithValueStrAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithValueStrAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithOpenAttribute: crate::UiHandleType + Element {
    type ElementWithOpenAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithOpenAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithNameAttribute: crate::UiHandleType + Element {
    type ElementWithNameAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithNameAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithDisabledAttribute: crate::UiHandleType + Element {
    type ElementWithDisabledAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithDisabledAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithCrossOriginAttribute: crate::UiHandleType + Element {
    type ElementWithCrossOriginAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithCrossOriginAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithRelAttribute: crate::UiHandleType + Element {
    type ElementWithRelAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithRelAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithReferrerPolicyAttribute: crate::UiHandleType + Element {
    type ElementWithReferrerPolicyAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithReferrerPolicyAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithAltAttribute: crate::UiHandleType + Element {
    type ElementWithAltAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithAltAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithLoadingAttribute: crate::UiHandleType + Element {
    type ElementWithLoadingAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithLoadingAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithAcceptAttribute: crate::UiHandleType + Element {
    type ElementWithAcceptAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithAcceptAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithAutoCompleteAttribute: crate::UiHandleType + Element {
    type ElementWithAutoCompleteAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithAutoCompleteAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithAutoCorrectAttribute: crate::UiHandleType + Element {
    type ElementWithAutoCorrectAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithAutoCorrectAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithFormAttribute: crate::UiHandleType + Element {
    type ElementWithFormAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithFormAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithFormAttributes: crate::UiHandleType + Element + ElementWithFormAttribute {
    type ElementWithFormAttributes<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithFormAttributes<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithFetchPriorityAttribute: crate::UiHandleType + Element {
    type ElementWithFetchPriorityAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithFetchPriorityAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithHrefLangAttribute: crate::UiHandleType + Element + ElementWithHrefAttribute {
    type ElementWithHrefLangAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithHrefLangAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithSizesAttribute: crate::UiHandleType + Element {
    type ElementWithSizesAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithSizesAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithUseMapAttribute: crate::UiHandleType + Element {
    type ElementWithUseMapAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithUseMapAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithLabelAttribute: crate::UiHandleType + Element {
    type ElementWithLabelAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithLabelAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithForAttribute: crate::UiHandleType + Element {
    type ElementWithForAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithForAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithIntegrityAttribute: crate::UiHandleType + Element {
    type ElementWithIntegrityAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithIntegrityAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithBlockingAttribute: crate::UiHandleType + Element {
    type ElementWithBlockingAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithBlockingAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithMultipleAttribute: crate::UiHandleType + Element {
    type ElementWithMultipleAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithMultipleAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithRequiredAttribute: crate::UiHandleType + Element {
    type ElementWithRequiredAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithRequiredAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithSizeU32Attribute: crate::UiHandleType + Element {
    type ElementWithSizeU32Attribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithSizeU32Attribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithSrcAttribute: crate::UiHandleType + Element {
    type ElementWithSrcAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithSrcAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithSrcsetAttribute: crate::UiHandleType + Element + ElementWithSrcAttribute {
    type ElementWithSrcsetAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithSrcsetAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithBgColorAttribute: crate::UiHandleType + Element {
    type ElementWithBgColorAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithBgColorAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithAlignAttribute: crate::UiHandleType + Element {
    type ElementWithAlignAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithAlignAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithMediaAttribute: crate::UiHandleType + Element {
    type ElementWithMediaAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithMediaAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithReadOnlyAttribute: crate::UiHandleType + Element {
    type ElementWithReadOnlyAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithReadOnlyAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait ElementWithDateTimeAttribute: crate::UiHandleType + Element {
    type ElementWithDateTimeAttribute<Renderer: ?Sized + super::RenderHtml>: super::behaviors::ElementWithDateTimeAttribute<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlElement:
    crate::UiHandleType
    + Element
    + OnEventType<super::event_types::on_invalid>
    + OnEventType<super::event_types::on_animation_cancel>
    + OnEventType<super::event_types::on_animation_end>
    + OnEventType<super::event_types::on_animation_iteration>
    + OnEventType<super::event_types::on_animation_start>
    + OnEventType<super::event_types::on_before_input>
    + OnEventType<super::event_types::on_input>
    + OnEventType<super::event_types::on_change>
    + OnEventType<super::event_types::on_got_pointer_capture>
    + OnEventType<super::event_types::on_lost_pointer_capture>
    + OnEventType<super::event_types::on_pointer_cancel>
    + OnEventType<super::event_types::on_pointer_down>
    + OnEventType<super::event_types::on_pointer_enter>
    + OnEventType<super::event_types::on_pointer_leave>
    + OnEventType<super::event_types::on_pointer_move>
    + OnEventType<super::event_types::on_pointer_out>
    + OnEventType<super::event_types::on_pointer_over>
    + OnEventType<super::event_types::on_pointer_up>
    + OnEventType<super::event_types::on_transition_cancel>
    + OnEventType<super::event_types::on_transition_end>
    + OnEventType<super::event_types::on_transition_run>
    + OnEventType<super::event_types::on_transition_start>
    + OnEventType<super::event_types::on_drag>
    + OnEventType<super::event_types::on_drag_end>
    + OnEventType<super::event_types::on_drag_enter>
    + OnEventType<super::event_types::on_drag_leave>
    + OnEventType<super::event_types::on_drag_over>
    + OnEventType<super::event_types::on_drag_start>
    + OnEventType<super::event_types::on_drop>
{
    type HtmlElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlDataListElement: crate::UiHandleType + HtmlElement {
    type HtmlDataListElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlDataListElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlDivElement: crate::UiHandleType + HtmlElement {
    type HtmlDivElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlDivElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlDListElement: crate::UiHandleType + HtmlElement {
    type HtmlDListElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlDListElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlHeadingElement: crate::UiHandleType + HtmlElement {
    type HtmlHeadingElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlHeadingElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlHeadElement: crate::UiHandleType + HtmlElement {
    type HtmlHeadElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlHeadElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlHrElement: crate::UiHandleType + HtmlElement {
    type HtmlHrElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlHrElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlLegendElement: crate::UiHandleType + HtmlElement {
    type HtmlLegendElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlLegendElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlMenuElement: crate::UiHandleType + HtmlElement {
    type HtmlMenuElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlMenuElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlParagraphElement: crate::UiHandleType + HtmlElement {
    type HtmlParagraphElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlParagraphElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlPictureElement: crate::UiHandleType + HtmlElement {
    type HtmlPictureElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlPictureElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlPreElement: crate::UiHandleType + HtmlElement {
    type HtmlPreElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlPreElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlSpanElement: crate::UiHandleType + HtmlElement {
    type HtmlSpanElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlSpanElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlTemplateElement: crate::UiHandleType + HtmlElement {
    type HtmlTemplateElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlTemplateElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlTitleElement: crate::UiHandleType + HtmlElement {
    type HtmlTitleElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlTitleElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlElementWithHref: crate::UiHandleType + HtmlElement + ElementWithHrefAttribute + ElementWithTargetAttribute + ElementWithReferrerPolicyAttribute + ElementWithRelAttribute {
    type HtmlElementWithHref<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlElementWithHref<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlAnchorElement: crate::UiHandleType + HtmlElement + HtmlElementWithHref + ElementWithTypeAttribute + ElementWithHrefLangAttribute {
    type HtmlAnchorElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlAnchorElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlAreaElement: crate::UiHandleType + HtmlElement + HtmlElementWithHref + ElementWithAltAttribute {
    type HtmlAreaElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlAreaElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlMediaElement:
    crate::UiHandleType
    + HtmlElement
    + ElementWithSrcAttribute
    + ElementWithCrossOriginAttribute
    + OnEventType<super::event_types::on_abort>
    + OnEventType<super::event_types::on_can_play>
    + OnEventType<super::event_types::on_can_play_through>
    + OnEventType<super::event_types::on_duration_change>
    + OnEventType<super::event_types::on_emptied>
    + OnEventType<super::event_types::on_ended>
    + OnEventType<super::event_types::on_loaded_data>
    + OnEventType<super::event_types::on_loaded_metadata>
    + OnEventType<super::event_types::on_load_start>
    + OnEventType<super::event_types::on_pause>
    + OnEventType<super::event_types::on_play>
    + OnEventType<super::event_types::on_playing>
    + OnEventType<super::event_types::on_progress>
    + OnEventType<super::event_types::on_rate_change>
    + OnEventType<super::event_types::on_resize>
    + OnEventType<super::event_types::on_seeked>
    + OnEventType<super::event_types::on_seeking>
    + OnEventType<super::event_types::on_stalled>
    + OnEventType<super::event_types::on_suspend>
    + OnEventType<super::event_types::on_time_update>
    + OnEventType<super::event_types::on_volume_change>
    + OnEventType<super::event_types::on_waiting>
{
    type HtmlMediaElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlMediaElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlBaseElement: crate::UiHandleType + HtmlElement + ElementWithHrefAttribute + ElementWithTargetAttribute {
    type HtmlBaseElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlBaseElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlQuoteElement: crate::UiHandleType + HtmlElement + ElementWithCiteAttribute {
    type HtmlQuoteElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlQuoteElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlBodyElement: crate::UiHandleType + HtmlElement {
    type HtmlBodyElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlBodyElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlBrElement: crate::UiHandleType + HtmlElement {
    type HtmlBrElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlBrElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlButtonElement: crate::UiHandleType + HtmlElement + ElementWithTypeAttribute + ElementWithFormAttributes + ElementWithDisabledAttribute + ElementWithNameAttribute + ElementWithValueStrAttribute {
    type HtmlButtonElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlButtonElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlCanvasElement: crate::UiHandleType + HtmlElement + ElementWithHeightWidthU32Attributes {
    type HtmlCanvasElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlCanvasElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlTableCaptionElement: crate::UiHandleType + HtmlElement + ElementWithAlignAttribute {
    type HtmlTableCaptionElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlTableCaptionElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlDataElement: crate::UiHandleType + HtmlElement + ElementWithValueStrAttribute {
    type HtmlDataElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlDataElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlModElement: crate::UiHandleType + HtmlElement + ElementWithCiteAttribute + ElementWithDateTimeAttribute {
    type HtmlModElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlModElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlDetailsElement: crate::UiHandleType + HtmlElement + ElementWithOpenAttribute {
    type HtmlDetailsElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlDetailsElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlDialogElement: crate::UiHandleType + HtmlElement + ElementWithOpenAttribute {
    type HtmlDialogElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlDialogElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlEmbedElement: crate::UiHandleType + HtmlElement + ElementWithTypeAttribute + ElementWithSrcAttribute + ElementWithHeightWidthStrAttributes {
    type HtmlEmbedElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlEmbedElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlFieldSetElement: crate::UiHandleType + HtmlElement + ElementWithFormAttribute + ElementWithDisabledAttribute + ElementWithNameAttribute {
    type HtmlFieldSetElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlFieldSetElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlFormElement:
    crate::UiHandleType
    + HtmlElement
    + ElementWithTargetAttribute
    + ElementWithAutoCompleteAttribute
    + ElementWithAcceptAttribute
    + ElementWithRelAttribute
    + ElementWithNameAttribute
    + OnEventType<super::event_types::on_form_data>
    + OnEventType<super::event_types::on_reset>
    + OnEventType<super::event_types::on_submit>
{
    type HtmlFormElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlFormElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlHtmlElement: crate::UiHandleType + HtmlElement {
    type HtmlHtmlElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlHtmlElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
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
    type HtmlIFrameElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlIFrameElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
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
    type HtmlImageElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlImageElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
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
    type HtmlInputElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlInputElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlLabelElement: crate::UiHandleType + HtmlElement + ElementWithForAttribute {
    type HtmlLabelElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlLabelElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlLiElement: crate::UiHandleType + HtmlElement {
    type HtmlLiElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlLiElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
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
    type HtmlLinkElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlLinkElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlMapElement: crate::UiHandleType + HtmlElement + ElementWithNameAttribute {
    type HtmlMapElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlMapElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlMetaElement: crate::UiHandleType + HtmlElement + ElementWithNameAttribute {
    type HtmlMetaElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlMetaElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlMeterElement: crate::UiHandleType + HtmlElement + ElementWithMaxF64Attribute + ElementWithValueF64Attribute {
    type HtmlMeterElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlMeterElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlObjectElement: crate::UiHandleType + HtmlElement + ElementWithTypeAttribute + ElementWithUseMapAttribute + ElementWithFormAttribute + ElementWithNameAttribute + ElementWithHeightWidthStrAttributes {
    type HtmlObjectElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlObjectElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlOListElement: crate::UiHandleType + HtmlElement + ElementWithTypeAttribute {
    type HtmlOListElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlOListElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlOptGroupElement: crate::UiHandleType + HtmlElement + ElementWithLabelAttribute + ElementWithDisabledAttribute {
    type HtmlOptGroupElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlOptGroupElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlOptionElement: crate::UiHandleType + HtmlElement + ElementWithLabelAttribute + ElementWithDisabledAttribute + ElementWithValueStrAttribute {
    type HtmlOptionElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlOptionElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlOutputElement: crate::UiHandleType + HtmlElement + ElementWithForAttribute + ElementWithFormAttribute + ElementWithNameAttribute {
    type HtmlOutputElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlOutputElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlProgressElement: crate::UiHandleType + HtmlElement + ElementWithMaxF64Attribute + ElementWithValueF64Attribute {
    type HtmlProgressElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlProgressElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
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
    type HtmlScriptElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlScriptElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
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
    type HtmlSelectElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlSelectElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlSlotElement: crate::UiHandleType + HtmlElement + ElementWithNameAttribute {
    type HtmlSlotElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlSlotElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlSourceElement:
    crate::UiHandleType + HtmlElement + ElementWithTypeAttribute + ElementWithMediaAttribute + ElementWithSrcsetAttribute + ElementWithSizesAttribute + ElementWithHeightWidthU32Attributes
{
    type HtmlSourceElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlSourceElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlStyleElement: crate::UiHandleType + HtmlElement + ElementWithTypeAttribute + ElementWithMediaAttribute + ElementWithBlockingAttribute {
    type HtmlStyleElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlStyleElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlTableElement: crate::UiHandleType + HtmlElement + ElementWithAlignAttribute + ElementWithBgColorAttribute {
    type HtmlTableElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlTableElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlTableChildElement: crate::UiHandleType + HtmlElement + ElementWithAlignAttribute + ElementWithBgColorAttribute {
    type HtmlTableChildElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlTableChildElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlTableSectionElement: crate::UiHandleType + HtmlElement + HtmlTableChildElement {
    type HtmlTableSectionElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlTableSectionElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlTableRowElement: crate::UiHandleType + HtmlElement + HtmlTableChildElement {
    type HtmlTableRowElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlTableRowElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlTableColElement: crate::UiHandleType + HtmlElement + HtmlTableChildElement {
    type HtmlTableColElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlTableColElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlTableCellElement: crate::UiHandleType + HtmlElement + ElementWithHeightWidthStrAttributes + HtmlTableChildElement {
    type HtmlTableCellElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlTableCellElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
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
    type HtmlTextAreaElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlTextAreaElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlTimeElement: crate::UiHandleType + HtmlElement + ElementWithDateTimeAttribute {
    type HtmlTimeElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlTimeElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlTrackElement: crate::UiHandleType + HtmlElement + ElementWithSrcAttribute + ElementWithLabelAttribute {
    type HtmlTrackElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlTrackElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlUListElement: crate::UiHandleType + HtmlElement + ElementWithTypeAttribute {
    type HtmlUListElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlUListElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlAudioElement: crate::UiHandleType + HtmlMediaElement {
    type HtmlAudioElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlAudioElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
pub trait HtmlVideoElement: crate::UiHandleType + HtmlMediaElement + ElementWithHeightWidthU32Attributes {
    type HtmlVideoElement<Renderer: ?Sized + super::RenderHtml>: super::behaviors::HtmlVideoElement<Renderer>
        + ::frender_common::convert::IdentityAs<Self::OfBehaviorType<Renderer>>
        + ::frender_common::convert::IdentityAs<Self::UiHandle<Renderer>>;
}
