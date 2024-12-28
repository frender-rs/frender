#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
pub mod conflicted_names {
    #![allow(non_camel_case_types)]
    pub enum value {}
    pub enum height {}
    pub enum width {}
    pub enum max {}
    pub enum min {}
}
use crate::intrinsic::{AllowAttributeName, Intrinsic, PropertyValue};
pub mod Node {}
pub mod Element {
    pub use super::Node::*;
    pub enum children {}
    pub enum ref_element {}
    pub enum class {}
    pub enum id {}
    pub enum part {}
    pub enum on_cancel {}
    pub enum on_error {}
    pub enum on_scroll {}
    pub enum on_security_policy_violation {}
    pub enum on_select {}
    pub enum on_wheel {}
    pub enum on_copy {}
    pub enum on_cut {}
    pub enum on_paste {}
    pub enum on_composition_end {}
    pub enum on_composition_start {}
    pub enum on_composition_update {}
    pub enum on_blur {}
    pub enum on_focus {}
    pub enum on_focus_in {}
    pub enum on_focus_out {}
    pub enum on_fullscreen_change {}
    pub enum on_fullscreen_error {}
    pub enum on_key_down {}
    pub enum on_key_up {}
    pub enum on_aux_click {}
    pub enum on_click {}
    pub enum on_context_menu {}
    pub enum on_double_click {}
    pub enum on_mouse_down {}
    pub enum on_mouse_enter {}
    pub enum on_mouse_leave {}
    pub enum on_mouse_move {}
    pub enum on_mouse_out {}
    pub enum on_mouse_over {}
    pub enum on_mouse_up {}
    pub enum on_touch_cancel {}
    pub enum on_touch_end {}
    pub enum on_touch_move {}
    pub enum on_touch_start {}
}
pub mod ElementWithHrefAttribute {
    pub use super::Element::*;
    pub enum href {}
}
pub mod ElementWithTargetAttribute {
    pub use super::Element::*;
    pub enum target {}
}
pub mod ElementWithTypeAttribute {
    pub use super::Element::*;
    pub enum r#type {}
}
pub mod ElementWithCiteAttribute {
    pub use super::Element::*;
    pub enum cite {}
}
pub mod ElementWithPlaceHolderAttribute {
    pub use super::Element::*;
    pub enum placeholder {}
}
pub mod ElementWithMaxMinLengthAttributes {
    pub use super::Element::*;
    pub enum max_length {}
    pub enum min_length {}
}
pub mod ElementWithHeightWidthStrAttributes {
    pub use super::Element::*;
    pub enum height {}
    pub enum width {}
}
pub mod ElementWithHeightWidthU32Attributes {
    pub use super::Element::*;
    pub enum height {}
    pub enum width {}
}
pub mod ElementWithMaxF64Attribute {
    pub use super::Element::*;
    pub enum max {}
}
pub mod ElementWithValueF64Attribute {
    pub use super::Element::*;
    pub enum value {}
}
pub mod ElementWithValueStrAttribute {
    pub use super::Element::*;
    pub enum value {}
}
pub mod ElementWithOpenAttribute {
    pub use super::Element::*;
    pub enum open {}
}
pub mod ElementWithNameAttribute {
    pub use super::Element::*;
    pub enum name {}
}
pub mod ElementWithDisabledAttribute {
    pub use super::Element::*;
    pub enum disabled {}
}
pub mod ElementWithCrossOriginAttribute {
    pub use super::Element::*;
    pub enum cross_origin {}
}
pub mod ElementWithRelAttribute {
    pub use super::Element::*;
    pub enum rel {}
}
pub mod ElementWithReferrerPolicyAttribute {
    pub use super::Element::*;
    pub enum referrer_policy {}
}
pub mod ElementWithAltAttribute {
    pub use super::Element::*;
    pub enum alt {}
}
pub mod ElementWithLoadingAttribute {
    pub use super::Element::*;
    pub enum loading {}
}
pub mod ElementWithAcceptAttribute {
    pub use super::Element::*;
    pub enum accept {}
}
pub mod ElementWithAutoCompleteAttribute {
    pub use super::Element::*;
    pub enum auto_complete {}
}
pub mod ElementWithAutoCorrectAttribute {
    pub use super::Element::*;
    pub enum auto_correct {}
}
pub mod ElementWithFormAttribute {
    pub use super::Element::*;
    pub enum form {}
}
pub mod ElementWithFormAttributes {
    pub use super::Element::*;
    pub use super::ElementWithFormAttribute::*;
    pub enum form_action {}
    pub enum form_enc_type {}
    pub enum form_method {}
    pub enum form_no_validate {}
    pub enum form_target {}
}
pub mod ElementWithFetchPriorityAttribute {
    pub use super::Element::*;
    pub enum fetch_priority {}
}
pub mod ElementWithHrefLangAttribute {
    pub use super::Element::*;
    pub use super::ElementWithHrefAttribute::*;
    pub enum href_lang {}
}
pub mod ElementWithSizesAttribute {
    pub use super::Element::*;
    pub enum sizes {}
}
pub mod ElementWithUseMapAttribute {
    pub use super::Element::*;
    pub enum use_map {}
}
pub mod ElementWithLabelAttribute {
    pub use super::Element::*;
    pub enum label {}
}
pub mod ElementWithForAttribute {
    pub use super::Element::*;
    pub enum r#for {}
}
pub mod ElementWithIntegrityAttribute {
    pub use super::Element::*;
    pub enum integrity {}
}
pub mod ElementWithBlockingAttribute {
    pub use super::Element::*;
    pub enum blocking {}
}
pub mod ElementWithMultipleAttribute {
    pub use super::Element::*;
    pub enum multiple {}
}
pub mod ElementWithRequiredAttribute {
    pub use super::Element::*;
    pub enum required {}
}
pub mod ElementWithSizeU32Attribute {
    pub use super::Element::*;
    pub enum size {}
}
pub mod ElementWithSrcAttribute {
    pub use super::Element::*;
    pub enum src {}
}
pub mod ElementWithSrcsetAttribute {
    pub use super::Element::*;
    pub use super::ElementWithSrcAttribute::*;
    pub enum srcset {}
}
pub mod ElementWithBgColorAttribute {
    pub use super::Element::*;
    pub enum bg_color {}
}
pub mod ElementWithAlignAttribute {
    pub use super::Element::*;
    pub enum align {}
}
pub mod ElementWithMediaAttribute {
    pub use super::Element::*;
    pub enum media {}
}
pub mod ElementWithReadOnlyAttribute {
    pub use super::Element::*;
    pub enum read_only {}
}
pub mod ElementWithDateTimeAttribute {
    pub use super::Element::*;
    pub enum date_time {}
}
pub mod HtmlElement {
    pub use super::Element::*;
    pub enum ref_html_element {}
    pub enum access_key {}
    pub enum auto_capitalize {}
    pub enum auto_focus {}
    pub enum content_editable {}
    pub enum context_menu {}
    pub enum dir {}
    pub enum draggable {}
    pub enum enter_key_hint {}
    pub enum hidden {}
    pub enum inert {}
    pub enum input_mode {}
    pub enum is {}
    pub enum item_id {}
    pub enum item_prop {}
    pub enum item_ref {}
    pub enum item_scope {}
    pub enum item_type {}
    pub enum lang {}
    pub enum nonce {}
    pub enum role {}
    pub enum slot {}
    pub enum spellcheck {}
    pub enum style {}
    pub enum tab_index {}
    pub enum title {}
    pub enum translate {}
    pub enum virtual_keyboard_policy {}
    pub enum on_invalid {}
    pub enum on_animation_cancel {}
    pub enum on_animation_end {}
    pub enum on_animation_iteration {}
    pub enum on_animation_start {}
    pub enum on_before_input {}
    pub enum on_input {}
    pub enum on_change {}
    pub enum on_got_pointer_capture {}
    pub enum on_lost_pointer_capture {}
    pub enum on_pointer_cancel {}
    pub enum on_pointer_down {}
    pub enum on_pointer_enter {}
    pub enum on_pointer_leave {}
    pub enum on_pointer_move {}
    pub enum on_pointer_out {}
    pub enum on_pointer_over {}
    pub enum on_pointer_up {}
    pub enum on_transition_cancel {}
    pub enum on_transition_end {}
    pub enum on_transition_run {}
    pub enum on_transition_start {}
    pub enum on_drag {}
    pub enum on_drag_end {}
    pub enum on_drag_enter {}
    pub enum on_drag_leave {}
    pub enum on_drag_over {}
    pub enum on_drag_start {}
    pub enum on_drop {}
}
pub mod HtmlDataListElement {
    pub use super::HtmlElement::*;
}
pub mod HtmlDivElement {
    pub use super::HtmlElement::*;
}
pub mod HtmlDListElement {
    pub use super::HtmlElement::*;
}
pub mod HtmlHeadingElement {
    pub use super::HtmlElement::*;
}
pub mod HtmlHeadElement {
    pub use super::HtmlElement::*;
}
pub mod HtmlHrElement {
    pub use super::HtmlElement::*;
}
pub mod HtmlLegendElement {
    pub use super::HtmlElement::*;
}
pub mod HtmlMenuElement {
    pub use super::HtmlElement::*;
}
pub mod HtmlParagraphElement {
    pub use super::HtmlElement::*;
}
pub mod HtmlPictureElement {
    pub use super::HtmlElement::*;
}
pub mod HtmlPreElement {
    pub use super::HtmlElement::*;
}
pub mod HtmlSpanElement {
    pub use super::HtmlElement::*;
}
pub mod HtmlTemplateElement {
    pub use super::HtmlElement::*;
}
pub mod HtmlTitleElement {
    pub use super::HtmlElement::*;
}
pub mod HtmlElementWithHref {
    pub use super::ElementWithHrefAttribute::*;
    pub use super::ElementWithReferrerPolicyAttribute::*;
    pub use super::ElementWithRelAttribute::*;
    pub use super::ElementWithTargetAttribute::*;
    pub use super::HtmlElement::*;
    pub enum download {}
    pub enum ping {}
}
pub mod HtmlAnchorElement {
    pub use super::ElementWithHrefLangAttribute::*;
    pub use super::ElementWithTypeAttribute::*;
    pub use super::HtmlElement::*;
    pub use super::HtmlElementWithHref::*;
}
pub mod HtmlAreaElement {
    pub use super::ElementWithAltAttribute::*;
    pub use super::HtmlElement::*;
    pub use super::HtmlElementWithHref::*;
    pub enum coords {}
    pub enum shape {}
}
pub mod HtmlMediaElement {
    pub use super::ElementWithCrossOriginAttribute::*;
    pub use super::ElementWithSrcAttribute::*;
    pub use super::HtmlElement::*;
    pub enum auto_play {}
    pub enum controls {}
    pub enum r#loop {}
    pub enum muted {}
    pub enum preload {}
    pub enum on_abort {}
    pub enum on_can_play {}
    pub enum on_can_play_through {}
    pub enum on_duration_change {}
    pub enum on_emptied {}
    pub enum on_ended {}
    pub enum on_loaded_data {}
    pub enum on_loaded_metadata {}
    pub enum on_load_start {}
    pub enum on_pause {}
    pub enum on_play {}
    pub enum on_playing {}
    pub enum on_progress {}
    pub enum on_rate_change {}
    pub enum on_resize {}
    pub enum on_seeked {}
    pub enum on_seeking {}
    pub enum on_stalled {}
    pub enum on_suspend {}
    pub enum on_time_update {}
    pub enum on_volume_change {}
    pub enum on_waiting {}
}
pub mod HtmlBaseElement {
    pub use super::ElementWithHrefAttribute::*;
    pub use super::ElementWithTargetAttribute::*;
    pub use super::HtmlElement::*;
}
pub mod HtmlQuoteElement {
    pub use super::ElementWithCiteAttribute::*;
    pub use super::HtmlElement::*;
}
pub mod HtmlBodyElement {
    pub use super::HtmlElement::*;
    pub enum alink {}
}
pub mod HtmlBrElement {
    pub use super::HtmlElement::*;
    pub enum clear {}
}
pub mod HtmlButtonElement {
    pub use super::ElementWithDisabledAttribute::*;
    pub use super::ElementWithFormAttributes::*;
    pub use super::ElementWithNameAttribute::*;
    pub use super::ElementWithTypeAttribute::*;
    pub use super::ElementWithValueStrAttribute::*;
    pub use super::HtmlElement::*;
}
pub mod HtmlCanvasElement {
    pub use super::ElementWithHeightWidthU32Attributes::*;
    pub use super::HtmlElement::*;
}
pub mod HtmlTableCaptionElement {
    pub use super::ElementWithAlignAttribute::*;
    pub use super::HtmlElement::*;
}
pub mod HtmlDataElement {
    pub use super::ElementWithValueStrAttribute::*;
    pub use super::HtmlElement::*;
}
pub mod HtmlModElement {
    pub use super::ElementWithCiteAttribute::*;
    pub use super::ElementWithDateTimeAttribute::*;
    pub use super::HtmlElement::*;
}
pub mod HtmlDetailsElement {
    pub use super::ElementWithOpenAttribute::*;
    pub use super::HtmlElement::*;
}
pub mod HtmlDialogElement {
    pub use super::ElementWithOpenAttribute::*;
    pub use super::HtmlElement::*;
}
pub mod HtmlEmbedElement {
    pub use super::ElementWithHeightWidthStrAttributes::*;
    pub use super::ElementWithSrcAttribute::*;
    pub use super::ElementWithTypeAttribute::*;
    pub use super::HtmlElement::*;
}
pub mod HtmlFieldSetElement {
    pub use super::ElementWithDisabledAttribute::*;
    pub use super::ElementWithFormAttribute::*;
    pub use super::ElementWithNameAttribute::*;
    pub use super::HtmlElement::*;
}
pub mod HtmlFormElement {
    pub use super::ElementWithAcceptAttribute::*;
    pub use super::ElementWithAutoCompleteAttribute::*;
    pub use super::ElementWithNameAttribute::*;
    pub use super::ElementWithRelAttribute::*;
    pub use super::ElementWithTargetAttribute::*;
    pub use super::HtmlElement::*;
    pub enum accept_charset {}
    pub enum action {}
    pub enum enc_type {}
    pub enum method {}
    pub enum no_validate {}
    pub enum on_form_data {}
    pub enum on_reset {}
    pub enum on_submit {}
}
pub mod HtmlHtmlElement {
    pub use super::HtmlElement::*;
    pub enum xmlns {}
}
pub mod HtmlIFrameElement {
    pub use super::ElementWithFetchPriorityAttribute::*;
    pub use super::ElementWithHeightWidthStrAttributes::*;
    pub use super::ElementWithLoadingAttribute::*;
    pub use super::ElementWithNameAttribute::*;
    pub use super::ElementWithReferrerPolicyAttribute::*;
    pub use super::ElementWithSrcAttribute::*;
    pub use super::HtmlElement::*;
    pub enum allow {}
    pub enum allow_fullscreen {}
    pub enum allow_payment_request {}
    pub enum csp {}
    pub enum sandbox {}
    pub enum src_doc {}
}
pub mod HtmlImageElement {
    pub use super::ElementWithAltAttribute::*;
    pub use super::ElementWithCrossOriginAttribute::*;
    pub use super::ElementWithHeightWidthU32Attributes::*;
    pub use super::ElementWithLoadingAttribute::*;
    pub use super::ElementWithReferrerPolicyAttribute::*;
    pub use super::ElementWithSizesAttribute::*;
    pub use super::ElementWithSrcsetAttribute::*;
    pub use super::ElementWithUseMapAttribute::*;
    pub use super::HtmlElement::*;
    pub enum decoding {}
    pub enum element_timing {}
    pub enum is_map {}
}
pub mod HtmlInputElement {
    pub use super::ElementWithAcceptAttribute::*;
    pub use super::ElementWithAltAttribute::*;
    pub use super::ElementWithAutoCompleteAttribute::*;
    pub use super::ElementWithAutoCorrectAttribute::*;
    pub use super::ElementWithDisabledAttribute::*;
    pub use super::ElementWithFormAttributes::*;
    pub use super::ElementWithHeightWidthU32Attributes::*;
    pub use super::ElementWithMaxMinLengthAttributes::*;
    pub use super::ElementWithMultipleAttribute::*;
    pub use super::ElementWithNameAttribute::*;
    pub use super::ElementWithPlaceHolderAttribute::*;
    pub use super::ElementWithReadOnlyAttribute::*;
    pub use super::ElementWithRequiredAttribute::*;
    pub use super::ElementWithSizeU32Attribute::*;
    pub use super::ElementWithSrcAttribute::*;
    pub use super::HtmlElement::*;
    pub enum capture {}
    pub enum dirname {}
    pub enum list {}
    pub enum max {}
    pub enum min {}
    pub enum pattern {}
    pub enum step {}
}
pub mod HtmlLabelElement {
    pub use super::ElementWithForAttribute::*;
    pub use super::HtmlElement::*;
}
pub mod HtmlLiElement {
    pub use super::HtmlElement::*;
    pub enum value {}
}
pub mod HtmlLinkElement {
    pub use super::ElementWithBlockingAttribute::*;
    pub use super::ElementWithCrossOriginAttribute::*;
    pub use super::ElementWithFetchPriorityAttribute::*;
    pub use super::ElementWithHrefAttribute::*;
    pub use super::ElementWithHrefLangAttribute::*;
    pub use super::ElementWithIntegrityAttribute::*;
    pub use super::ElementWithMediaAttribute::*;
    pub use super::ElementWithReferrerPolicyAttribute::*;
    pub use super::ElementWithRelAttribute::*;
    pub use super::ElementWithSizesAttribute::*;
    pub use super::ElementWithTypeAttribute::*;
    pub use super::HtmlElement::*;
    pub enum r#as {}
    pub enum image_sizes {}
    pub enum image_src_set {}
    pub enum prefetch {}
}
pub mod HtmlMapElement {
    pub use super::ElementWithNameAttribute::*;
    pub use super::HtmlElement::*;
}
pub mod HtmlMetaElement {
    pub use super::ElementWithNameAttribute::*;
    pub use super::HtmlElement::*;
    pub enum charset {}
    pub enum content {}
    pub enum http_equiv {}
}
pub mod HtmlMeterElement {
    pub use super::ElementWithMaxF64Attribute::*;
    pub use super::ElementWithValueF64Attribute::*;
    pub use super::HtmlElement::*;
    pub enum min {}
    pub enum low {}
    pub enum high {}
    pub enum optimum {}
}
pub mod HtmlObjectElement {
    pub use super::ElementWithFormAttribute::*;
    pub use super::ElementWithHeightWidthStrAttributes::*;
    pub use super::ElementWithNameAttribute::*;
    pub use super::ElementWithTypeAttribute::*;
    pub use super::ElementWithUseMapAttribute::*;
    pub use super::HtmlElement::*;
    pub enum data {}
}
pub mod HtmlOListElement {
    pub use super::ElementWithTypeAttribute::*;
    pub use super::HtmlElement::*;
    pub enum reversed {}
    pub enum start {}
}
pub mod HtmlOptGroupElement {
    pub use super::ElementWithDisabledAttribute::*;
    pub use super::ElementWithLabelAttribute::*;
    pub use super::HtmlElement::*;
}
pub mod HtmlOptionElement {
    pub use super::ElementWithDisabledAttribute::*;
    pub use super::ElementWithLabelAttribute::*;
    pub use super::ElementWithValueStrAttribute::*;
    pub use super::HtmlElement::*;
    pub enum selected {}
}
pub mod HtmlOutputElement {
    pub use super::ElementWithForAttribute::*;
    pub use super::ElementWithFormAttribute::*;
    pub use super::ElementWithNameAttribute::*;
    pub use super::HtmlElement::*;
}
pub mod HtmlProgressElement {
    pub use super::ElementWithMaxF64Attribute::*;
    pub use super::ElementWithValueF64Attribute::*;
    pub use super::HtmlElement::*;
}
pub mod HtmlScriptElement {
    pub use super::ElementWithBlockingAttribute::*;
    pub use super::ElementWithCrossOriginAttribute::*;
    pub use super::ElementWithFetchPriorityAttribute::*;
    pub use super::ElementWithIntegrityAttribute::*;
    pub use super::ElementWithReferrerPolicyAttribute::*;
    pub use super::ElementWithSrcAttribute::*;
    pub use super::ElementWithTypeAttribute::*;
    pub use super::HtmlElement::*;
    pub enum children {}
    pub enum r#async {}
    pub enum defer {}
    pub enum no_module {}
}
pub mod HtmlSelectElement {
    pub use super::ElementWithAutoCompleteAttribute::*;
    pub use super::ElementWithDisabledAttribute::*;
    pub use super::ElementWithFormAttribute::*;
    pub use super::ElementWithMultipleAttribute::*;
    pub use super::ElementWithNameAttribute::*;
    pub use super::ElementWithRequiredAttribute::*;
    pub use super::ElementWithSizeU32Attribute::*;
    pub use super::HtmlElement::*;
}
pub mod HtmlSlotElement {
    pub use super::ElementWithNameAttribute::*;
    pub use super::HtmlElement::*;
}
pub mod HtmlSourceElement {
    pub use super::ElementWithHeightWidthU32Attributes::*;
    pub use super::ElementWithMediaAttribute::*;
    pub use super::ElementWithSizesAttribute::*;
    pub use super::ElementWithSrcsetAttribute::*;
    pub use super::ElementWithTypeAttribute::*;
    pub use super::HtmlElement::*;
}
pub mod HtmlStyleElement {
    pub use super::ElementWithBlockingAttribute::*;
    pub use super::ElementWithMediaAttribute::*;
    pub use super::ElementWithTypeAttribute::*;
    pub use super::HtmlElement::*;
}
pub mod HtmlTableElement {
    pub use super::ElementWithAlignAttribute::*;
    pub use super::ElementWithBgColorAttribute::*;
    pub use super::HtmlElement::*;
    pub enum border {}
    pub enum cell_padding {}
    pub enum cell_spacing {}
    pub enum frame {}
    pub enum rules {}
    pub enum summary {}
    pub enum width {}
}
pub mod HtmlTableChildElement {
    pub use super::ElementWithAlignAttribute::*;
    pub use super::ElementWithBgColorAttribute::*;
    pub use super::HtmlElement::*;
    pub enum char {}
    pub enum char_off {}
    pub enum v_align {}
}
pub mod HtmlTableSectionElement {
    pub use super::HtmlElement::*;
    pub use super::HtmlTableChildElement::*;
}
pub mod HtmlTableRowElement {
    pub use super::HtmlElement::*;
    pub use super::HtmlTableChildElement::*;
}
pub mod HtmlTableColElement {
    pub use super::HtmlElement::*;
    pub use super::HtmlTableChildElement::*;
    pub enum span {}
    pub enum width {}
}
pub mod HtmlTableCellElement {
    pub use super::ElementWithHeightWidthStrAttributes::*;
    pub use super::HtmlElement::*;
    pub use super::HtmlTableChildElement::*;
    pub enum col_span {}
    pub enum headers {}
    pub enum row_span {}
    pub enum abbr {}
    pub enum axis {}
    pub enum scope {}
}
pub mod HtmlTextAreaElement {
    pub use super::ElementWithAutoCompleteAttribute::*;
    pub use super::ElementWithAutoCorrectAttribute::*;
    pub use super::ElementWithDisabledAttribute::*;
    pub use super::ElementWithFormAttribute::*;
    pub use super::ElementWithMaxMinLengthAttributes::*;
    pub use super::ElementWithNameAttribute::*;
    pub use super::ElementWithPlaceHolderAttribute::*;
    pub use super::ElementWithReadOnlyAttribute::*;
    pub use super::ElementWithRequiredAttribute::*;
    pub use super::HtmlElement::*;
    pub enum children {}
    pub enum cols {}
    pub enum rows {}
    pub enum wrap {}
}
pub mod HtmlTimeElement {
    pub use super::ElementWithDateTimeAttribute::*;
    pub use super::HtmlElement::*;
}
pub mod HtmlTrackElement {
    pub use super::ElementWithLabelAttribute::*;
    pub use super::ElementWithSrcAttribute::*;
    pub use super::HtmlElement::*;
    pub enum default {}
    pub enum kind {}
    pub enum src_lang {}
}
pub mod HtmlUListElement {
    pub use super::ElementWithTypeAttribute::*;
    pub use super::HtmlElement::*;
    pub enum compact {}
}
pub mod HtmlAudioElement {
    pub use super::HtmlMediaElement::*;
}
pub mod HtmlVideoElement {
    pub use super::ElementWithHeightWidthU32Attributes::*;
    pub use super::HtmlMediaElement::*;
    pub enum plays_inline {}
    pub enum poster {}
}
impl<M: AllowAttributeName<self::conflicted_names::value>, C, A, P> Intrinsic<M, C, A, P> {
    pub fn value<T: PropertyValue<M::AttributeMarker>>(self, value: T) -> Intrinsic<M, C, (A, T::Property), P> {
        Self::with_attribute_appended(self, T::wrapped_into_property(value))
    }
}
impl<M: AllowAttributeName<self::conflicted_names::height>, C, A, P> Intrinsic<M, C, A, P> {
    pub fn height<T: PropertyValue<M::AttributeMarker>>(self, value: T) -> Intrinsic<M, C, (A, T::Property), P> {
        Self::with_attribute_appended(self, T::wrapped_into_property(value))
    }
}
impl<M: AllowAttributeName<self::conflicted_names::width>, C, A, P> Intrinsic<M, C, A, P> {
    pub fn width<T: PropertyValue<M::AttributeMarker>>(self, value: T) -> Intrinsic<M, C, (A, T::Property), P> {
        Self::with_attribute_appended(self, T::wrapped_into_property(value))
    }
}
impl<M: AllowAttributeName<self::conflicted_names::max>, C, A, P> Intrinsic<M, C, A, P> {
    pub fn max<T: PropertyValue<M::AttributeMarker>>(self, value: T) -> Intrinsic<M, C, (A, T::Property), P> {
        Self::with_attribute_appended(self, T::wrapped_into_property(value))
    }
}
impl<M: AllowAttributeName<self::conflicted_names::min>, C, A, P> Intrinsic<M, C, A, P> {
    pub fn min<T: PropertyValue<M::AttributeMarker>>(self, value: T) -> Intrinsic<M, C, (A, T::Property), P> {
        Self::with_attribute_appended(self, T::wrapped_into_property(value))
    }
}
macro_rules! expand_if_conflicted_name_or_else {
    (value $_if:tt $_else:tt) => {
        frender_common::expand! { $_if }
    };
    (height $_if:tt $_else:tt) => {
        frender_common::expand! { $_if }
    };
    (width $_if:tt $_else:tt) => {
        frender_common::expand! { $_if }
    };
    (max $_if:tt $_else:tt) => {
        frender_common::expand! { $_if }
    };
    (min $_if:tt $_else:tt) => {
        frender_common::expand! { $_if }
    };
    ($not_conflicted_name:ident $_if:tt $_else:tt) => {
        frender_common::expand! { $_else }
    };
}
pub(super) use expand_if_conflicted_name_or_else;
