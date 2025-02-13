use super::{event_types, *};
use frender_dom::OnEvent;
pub trait Node<Renderer: ?Sized>: frender_dom::behaviors::Node<Renderer> + UiHandle<Renderer> {}
pub trait Element<Renderer: ?Sized>:
    Node<Renderer>
    + OnEvent<Renderer, event_types::on_cancel>
    + OnEvent<Renderer, event_types::on_error>
    + OnEvent<Renderer, event_types::on_scroll>
    + OnEvent<Renderer, event_types::on_security_policy_violation>
    + OnEvent<Renderer, event_types::on_select>
    + OnEvent<Renderer, event_types::on_wheel>
    + OnEvent<Renderer, event_types::on_copy>
    + OnEvent<Renderer, event_types::on_cut>
    + OnEvent<Renderer, event_types::on_paste>
    + OnEvent<Renderer, event_types::on_composition_end>
    + OnEvent<Renderer, event_types::on_composition_start>
    + OnEvent<Renderer, event_types::on_composition_update>
    + OnEvent<Renderer, event_types::on_blur>
    + OnEvent<Renderer, event_types::on_focus>
    + OnEvent<Renderer, event_types::on_focus_in>
    + OnEvent<Renderer, event_types::on_focus_out>
    + OnEvent<Renderer, event_types::on_fullscreen_change>
    + OnEvent<Renderer, event_types::on_fullscreen_error>
    + OnEvent<Renderer, event_types::on_key_down>
    + OnEvent<Renderer, event_types::on_key_up>
    + OnEvent<Renderer, event_types::on_aux_click>
    + OnEvent<Renderer, event_types::on_click>
    + OnEvent<Renderer, event_types::on_context_menu>
    + OnEvent<Renderer, event_types::on_double_click>
    + OnEvent<Renderer, event_types::on_mouse_down>
    + OnEvent<Renderer, event_types::on_mouse_enter>
    + OnEvent<Renderer, event_types::on_mouse_leave>
    + OnEvent<Renderer, event_types::on_mouse_move>
    + OnEvent<Renderer, event_types::on_mouse_out>
    + OnEvent<Renderer, event_types::on_mouse_over>
    + OnEvent<Renderer, event_types::on_mouse_up>
    + OnEvent<Renderer, event_types::on_touch_cancel>
    + OnEvent<Renderer, event_types::on_touch_end>
    + OnEvent<Renderer, event_types::on_touch_move>
    + OnEvent<Renderer, event_types::on_touch_start>
    + frender_dom::ui_handle::UiHandle<Renderer>
    + frender_dom::behaviors::Element<Renderer>
    + frender_dom::behaviors::ElementWithChildren<Renderer>
    + frender_dom::behaviors::ElementWithClassList<Renderer>
{
    fn set_id(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait ElementWithHrefAttribute<Renderer: ?Sized>: Element<Renderer> {
    fn set_href(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait ElementWithTargetAttribute<Renderer: ?Sized>: Element<Renderer> {
    fn set_target(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait ElementWithTypeAttribute<Renderer: ?Sized>: Element<Renderer> {
    fn set_type(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait ElementWithCiteAttribute<Renderer: ?Sized>: Element<Renderer> {
    fn set_cite(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait ElementWithPlaceHolderAttribute<Renderer: ?Sized>: Element<Renderer> {
    fn set_placeholder(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait ElementWithMaxMinLengthAttributes<Renderer: ?Sized>: Element<Renderer> {
    fn set_max_length(&mut self, renderer: &mut Renderer, value: i32);
    fn set_min_length(&mut self, renderer: &mut Renderer, value: i32);
}
pub trait ElementWithHeightWidthStrAttributes<Renderer: ?Sized>: Element<Renderer> {
    fn set_height(&mut self, renderer: &mut Renderer, value: &str);
    fn set_width(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait ElementWithHeightWidthU32Attributes<Renderer: ?Sized>: Element<Renderer> {
    fn set_height(&mut self, renderer: &mut Renderer, value: u32);
    fn set_width(&mut self, renderer: &mut Renderer, value: u32);
}
pub trait ElementWithMaxF64Attribute<Renderer: ?Sized>: Element<Renderer> {
    fn set_max(&mut self, renderer: &mut Renderer, value: f64);
}
pub trait ElementWithValueF64Attribute<Renderer: ?Sized>: Element<Renderer> {
    fn set_value(&mut self, renderer: &mut Renderer, value: f64);
}
pub trait ElementWithValueStrAttribute<Renderer: ?Sized>: Element<Renderer> {
    fn set_value(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait ElementWithOpenAttribute<Renderer: ?Sized>: Element<Renderer> {
    fn set_open(&mut self, renderer: &mut Renderer, value: bool);
}
pub trait ElementWithNameAttribute<Renderer: ?Sized>: Element<Renderer> {
    fn set_name(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait ElementWithDisabledAttribute<Renderer: ?Sized>: Element<Renderer> {
    fn set_disabled(&mut self, renderer: &mut Renderer, value: bool);
}
pub trait ElementWithCrossOriginAttribute<Renderer: ?Sized>: Element<Renderer> {
    fn set_cross_origin(&mut self, renderer: &mut Renderer, value: Option<&str>);
}
pub trait ElementWithRelAttribute<Renderer: ?Sized>: Element<Renderer> + frender_dom::behaviors::ElementWithRelList<Renderer> {}
pub trait ElementWithReferrerPolicyAttribute<Renderer: ?Sized>: Element<Renderer> {
    fn set_referrer_policy(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait ElementWithAltAttribute<Renderer: ?Sized>: Element<Renderer> {
    fn set_alt(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait ElementWithLoadingAttribute<Renderer: ?Sized>: Element<Renderer> {}
pub trait ElementWithAcceptAttribute<Renderer: ?Sized>: Element<Renderer> {
    fn set_accept(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait ElementWithAutoCompleteAttribute<Renderer: ?Sized>: Element<Renderer> {
    fn set_auto_complete(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait ElementWithAutoCorrectAttribute<Renderer: ?Sized>: Element<Renderer> {}
pub trait ElementWithFormAttribute<Renderer: ?Sized>: Element<Renderer> {}
pub trait ElementWithFormAttributes<Renderer: ?Sized>: Element<Renderer> + ElementWithFormAttribute<Renderer> {
    fn set_form_action(&mut self, renderer: &mut Renderer, value: &str);
    fn set_form_enctype(&mut self, renderer: &mut Renderer, value: &str);
    fn set_form_method(&mut self, renderer: &mut Renderer, value: &str);
    fn set_form_no_validate(&mut self, renderer: &mut Renderer, value: bool);
    fn set_form_target(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait ElementWithFetchPriorityAttribute<Renderer: ?Sized>: Element<Renderer> {}
pub trait ElementWithHrefLangAttribute<Renderer: ?Sized>: Element<Renderer> + ElementWithHrefAttribute<Renderer> {
    fn set_href_lang(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait ElementWithSizesAttribute<Renderer: ?Sized>: Element<Renderer> {
    fn set_sizes(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait ElementWithUseMapAttribute<Renderer: ?Sized>: Element<Renderer> {
    fn set_use_map(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait ElementWithLabelAttribute<Renderer: ?Sized>: Element<Renderer> {
    fn set_label(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait ElementWithForAttribute<Renderer: ?Sized>: Element<Renderer> {
    fn set_html_for(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait ElementWithIntegrityAttribute<Renderer: ?Sized>: Element<Renderer> {
    fn set_integrity(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait ElementWithBlockingAttribute<Renderer: ?Sized>: Element<Renderer> {}
pub trait ElementWithMultipleAttribute<Renderer: ?Sized>: Element<Renderer> {
    fn set_multiple(&mut self, renderer: &mut Renderer, value: bool);
}
pub trait ElementWithRequiredAttribute<Renderer: ?Sized>: Element<Renderer> {
    fn set_required(&mut self, renderer: &mut Renderer, value: bool);
}
pub trait ElementWithSizeU32Attribute<Renderer: ?Sized>: Element<Renderer> {
    fn set_size(&mut self, renderer: &mut Renderer, value: u32);
}
pub trait ElementWithSrcAttribute<Renderer: ?Sized>: Element<Renderer> {
    fn set_src(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait ElementWithSrcsetAttribute<Renderer: ?Sized>: Element<Renderer> + ElementWithSrcAttribute<Renderer> {
    fn set_srcset(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait ElementWithBgColorAttribute<Renderer: ?Sized>: Element<Renderer> {
    fn set_bg_color(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait ElementWithAlignAttribute<Renderer: ?Sized>: Element<Renderer> {
    fn set_align(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait ElementWithMediaAttribute<Renderer: ?Sized>: Element<Renderer> {
    fn set_media(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait ElementWithReadOnlyAttribute<Renderer: ?Sized>: Element<Renderer> {
    fn set_read_only(&mut self, renderer: &mut Renderer, value: bool);
}
pub trait ElementWithDateTimeAttribute<Renderer: ?Sized>: Element<Renderer> {
    fn set_date_time(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait HtmlElement<Renderer: ?Sized>:
    Element<Renderer>
    + OnEvent<Renderer, event_types::on_invalid>
    + OnEvent<Renderer, event_types::on_animation_cancel>
    + OnEvent<Renderer, event_types::on_animation_end>
    + OnEvent<Renderer, event_types::on_animation_iteration>
    + OnEvent<Renderer, event_types::on_animation_start>
    + OnEvent<Renderer, event_types::on_before_input>
    + OnEvent<Renderer, event_types::on_input>
    + OnEvent<Renderer, event_types::on_change>
    + OnEvent<Renderer, event_types::on_got_pointer_capture>
    + OnEvent<Renderer, event_types::on_lost_pointer_capture>
    + OnEvent<Renderer, event_types::on_pointer_cancel>
    + OnEvent<Renderer, event_types::on_pointer_down>
    + OnEvent<Renderer, event_types::on_pointer_enter>
    + OnEvent<Renderer, event_types::on_pointer_leave>
    + OnEvent<Renderer, event_types::on_pointer_move>
    + OnEvent<Renderer, event_types::on_pointer_out>
    + OnEvent<Renderer, event_types::on_pointer_over>
    + OnEvent<Renderer, event_types::on_pointer_up>
    + OnEvent<Renderer, event_types::on_transition_cancel>
    + OnEvent<Renderer, event_types::on_transition_end>
    + OnEvent<Renderer, event_types::on_transition_run>
    + OnEvent<Renderer, event_types::on_transition_start>
    + OnEvent<Renderer, event_types::on_drag>
    + OnEvent<Renderer, event_types::on_drag_end>
    + OnEvent<Renderer, event_types::on_drag_enter>
    + OnEvent<Renderer, event_types::on_drag_leave>
    + OnEvent<Renderer, event_types::on_drag_over>
    + OnEvent<Renderer, event_types::on_drag_start>
    + OnEvent<Renderer, event_types::on_drop>
    + frender_dom::behaviors::HtmlElement<Renderer>
    + frender_dom::behaviors::ElementWithStyle<Renderer>
{
    fn set_access_key(&mut self, renderer: &mut Renderer, value: &str);
    fn set_content_editable(&mut self, renderer: &mut Renderer, value: &str);
    fn set_dir(&mut self, renderer: &mut Renderer, value: &str);
    fn set_draggable(&mut self, renderer: &mut Renderer, value: bool);
    fn set_hidden(&mut self, renderer: &mut Renderer, value: bool);
    fn set_lang(&mut self, renderer: &mut Renderer, value: &str);
    fn set_spellcheck(&mut self, renderer: &mut Renderer, value: bool);
    fn set_tab_index(&mut self, renderer: &mut Renderer, value: i32);
    fn set_title(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait HtmlDataListElement<Renderer: ?Sized>: HtmlElement<Renderer> {}
pub trait HtmlDivElement<Renderer: ?Sized>: HtmlElement<Renderer> {}
pub trait HtmlDListElement<Renderer: ?Sized>: HtmlElement<Renderer> {}
pub trait HtmlHeadingElement<Renderer: ?Sized>: HtmlElement<Renderer> {}
pub trait HtmlHeadElement<Renderer: ?Sized>: HtmlElement<Renderer> {}
pub trait HtmlHrElement<Renderer: ?Sized>: HtmlElement<Renderer> {}
pub trait HtmlLegendElement<Renderer: ?Sized>: HtmlElement<Renderer> {}
pub trait HtmlMenuElement<Renderer: ?Sized>: HtmlElement<Renderer> {}
pub trait HtmlParagraphElement<Renderer: ?Sized>: HtmlElement<Renderer> {}
pub trait HtmlPictureElement<Renderer: ?Sized>: HtmlElement<Renderer> {}
pub trait HtmlPreElement<Renderer: ?Sized>: HtmlElement<Renderer> {}
pub trait HtmlSpanElement<Renderer: ?Sized>: HtmlElement<Renderer> {}
pub trait HtmlTemplateElement<Renderer: ?Sized>: HtmlElement<Renderer> {}
pub trait HtmlTitleElement<Renderer: ?Sized>: HtmlElement<Renderer> {}
pub trait HtmlElementWithHref<Renderer: ?Sized>:
    HtmlElement<Renderer> + ElementWithHrefAttribute<Renderer> + ElementWithTargetAttribute<Renderer> + ElementWithReferrerPolicyAttribute<Renderer> + ElementWithRelAttribute<Renderer>
{
    fn set_download(&mut self, renderer: &mut Renderer, value: &str);
    fn set_ping(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait HtmlAnchorElement<Renderer: ?Sized>: HtmlElement<Renderer> + HtmlElementWithHref<Renderer> + ElementWithTypeAttribute<Renderer> + ElementWithHrefLangAttribute<Renderer> {}
pub trait HtmlAreaElement<Renderer: ?Sized>: HtmlElement<Renderer> + HtmlElementWithHref<Renderer> + ElementWithAltAttribute<Renderer> {
    fn set_coords(&mut self, renderer: &mut Renderer, value: &str);
    fn set_shape(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait HtmlMediaElement<Renderer: ?Sized>:
    HtmlElement<Renderer>
    + ElementWithSrcAttribute<Renderer>
    + ElementWithCrossOriginAttribute<Renderer>
    + OnEvent<Renderer, event_types::on_abort>
    + OnEvent<Renderer, event_types::on_can_play>
    + OnEvent<Renderer, event_types::on_can_play_through>
    + OnEvent<Renderer, event_types::on_duration_change>
    + OnEvent<Renderer, event_types::on_emptied>
    + OnEvent<Renderer, event_types::on_ended>
    + OnEvent<Renderer, event_types::on_loaded_data>
    + OnEvent<Renderer, event_types::on_loaded_metadata>
    + OnEvent<Renderer, event_types::on_load_start>
    + OnEvent<Renderer, event_types::on_pause>
    + OnEvent<Renderer, event_types::on_play>
    + OnEvent<Renderer, event_types::on_playing>
    + OnEvent<Renderer, event_types::on_progress>
    + OnEvent<Renderer, event_types::on_rate_change>
    + OnEvent<Renderer, event_types::on_resize>
    + OnEvent<Renderer, event_types::on_seeked>
    + OnEvent<Renderer, event_types::on_seeking>
    + OnEvent<Renderer, event_types::on_stalled>
    + OnEvent<Renderer, event_types::on_suspend>
    + OnEvent<Renderer, event_types::on_time_update>
    + OnEvent<Renderer, event_types::on_volume_change>
    + OnEvent<Renderer, event_types::on_waiting>
{
    fn set_auto_play(&mut self, renderer: &mut Renderer, value: bool);
    fn set_controls(&mut self, renderer: &mut Renderer, value: bool);
    fn set_loop(&mut self, renderer: &mut Renderer, value: bool);
    fn set_muted(&mut self, renderer: &mut Renderer, value: bool);
    fn set_preload(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait HtmlBaseElement<Renderer: ?Sized>: HtmlElement<Renderer> + ElementWithHrefAttribute<Renderer> + ElementWithTargetAttribute<Renderer> {}
pub trait HtmlQuoteElement<Renderer: ?Sized>: HtmlElement<Renderer> + ElementWithCiteAttribute<Renderer> {}
pub trait HtmlBodyElement<Renderer: ?Sized>: HtmlElement<Renderer> {}
pub trait HtmlBrElement<Renderer: ?Sized>: HtmlElement<Renderer> {
    fn set_clear(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait HtmlButtonElement<Renderer: ?Sized>:
    HtmlElement<Renderer> + ElementWithTypeAttribute<Renderer> + ElementWithFormAttributes<Renderer> + ElementWithDisabledAttribute<Renderer> + ElementWithNameAttribute<Renderer> + ElementWithValueStrAttribute<Renderer>
{
}
pub trait HtmlCanvasElement<Renderer: ?Sized>: HtmlElement<Renderer> + ElementWithHeightWidthU32Attributes<Renderer> {}
pub trait HtmlTableCaptionElement<Renderer: ?Sized>: HtmlElement<Renderer> + ElementWithAlignAttribute<Renderer> {}
pub trait HtmlDataElement<Renderer: ?Sized>: HtmlElement<Renderer> + ElementWithValueStrAttribute<Renderer> {}
pub trait HtmlModElement<Renderer: ?Sized>: HtmlElement<Renderer> + ElementWithCiteAttribute<Renderer> + ElementWithDateTimeAttribute<Renderer> {}
pub trait HtmlDetailsElement<Renderer: ?Sized>: HtmlElement<Renderer> + ElementWithOpenAttribute<Renderer> {}
pub trait HtmlDialogElement<Renderer: ?Sized>: HtmlElement<Renderer> + ElementWithOpenAttribute<Renderer> {}
pub trait HtmlEmbedElement<Renderer: ?Sized>: HtmlElement<Renderer> + ElementWithTypeAttribute<Renderer> + ElementWithSrcAttribute<Renderer> + ElementWithHeightWidthStrAttributes<Renderer> {}
pub trait HtmlFieldSetElement<Renderer: ?Sized>: HtmlElement<Renderer> + ElementWithFormAttribute<Renderer> + ElementWithDisabledAttribute<Renderer> + ElementWithNameAttribute<Renderer> {}
pub trait HtmlFormElement<Renderer: ?Sized>:
    HtmlElement<Renderer>
    + ElementWithTargetAttribute<Renderer>
    + ElementWithAutoCompleteAttribute<Renderer>
    + ElementWithAcceptAttribute<Renderer>
    + ElementWithRelAttribute<Renderer>
    + ElementWithNameAttribute<Renderer>
    + OnEvent<Renderer, event_types::on_form_data>
    + OnEvent<Renderer, event_types::on_reset>
    + OnEvent<Renderer, event_types::on_submit>
{
    fn set_accept_charset(&mut self, renderer: &mut Renderer, value: &str);
    fn set_action(&mut self, renderer: &mut Renderer, value: &str);
    fn set_enctype(&mut self, renderer: &mut Renderer, value: &str);
    fn set_method(&mut self, renderer: &mut Renderer, value: &str);
    fn set_no_validate(&mut self, renderer: &mut Renderer, value: bool);
}
pub trait HtmlHtmlElement<Renderer: ?Sized>: HtmlElement<Renderer> {}
pub trait HtmlIFrameElement<Renderer: ?Sized>:
    HtmlElement<Renderer>
    + ElementWithSrcAttribute<Renderer>
    + ElementWithFetchPriorityAttribute<Renderer>
    + ElementWithLoadingAttribute<Renderer>
    + ElementWithReferrerPolicyAttribute<Renderer>
    + ElementWithNameAttribute<Renderer>
    + ElementWithHeightWidthStrAttributes<Renderer>
{
    fn set_allow_fullscreen(&mut self, renderer: &mut Renderer, value: bool);
    fn set_allow_payment_request(&mut self, renderer: &mut Renderer, value: bool);
    fn set_srcdoc(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait HtmlImageElement<Renderer: ?Sized>:
    HtmlElement<Renderer>
    + ElementWithSrcsetAttribute<Renderer>
    + ElementWithUseMapAttribute<Renderer>
    + ElementWithSizesAttribute<Renderer>
    + ElementWithLoadingAttribute<Renderer>
    + ElementWithAltAttribute<Renderer>
    + ElementWithReferrerPolicyAttribute<Renderer>
    + ElementWithCrossOriginAttribute<Renderer>
    + ElementWithHeightWidthU32Attributes<Renderer>
{
    fn set_decoding(&mut self, renderer: &mut Renderer, value: &str);
    fn set_is_map(&mut self, renderer: &mut Renderer, value: bool);
}
pub trait HtmlInputElement<Renderer: ?Sized>:
    HtmlElement<Renderer>
    + ElementWithReadOnlyAttribute<Renderer>
    + ElementWithPlaceHolderAttribute<Renderer>
    + ElementWithMaxMinLengthAttributes<Renderer>
    + ElementWithSrcAttribute<Renderer>
    + ElementWithSizeU32Attribute<Renderer>
    + ElementWithRequiredAttribute<Renderer>
    + ElementWithMultipleAttribute<Renderer>
    + ElementWithFormAttributes<Renderer>
    + ElementWithAutoCompleteAttribute<Renderer>
    + ElementWithAutoCorrectAttribute<Renderer>
    + ElementWithAcceptAttribute<Renderer>
    + ElementWithAltAttribute<Renderer>
    + ElementWithDisabledAttribute<Renderer>
    + ElementWithNameAttribute<Renderer>
    + ElementWithHeightWidthU32Attributes<Renderer>
    + frender_form_control::input::InputElement<Renderer>
    + behaviors::ElementWithTypeAttribute<Renderer>
{
    fn set_max(&mut self, renderer: &mut Renderer, value: &str);
    fn set_min(&mut self, renderer: &mut Renderer, value: &str);
    fn set_pattern(&mut self, renderer: &mut Renderer, value: &str);
    fn set_step(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait HtmlLabelElement<Renderer: ?Sized>: HtmlElement<Renderer> + ElementWithForAttribute<Renderer> {}
pub trait HtmlLiElement<Renderer: ?Sized>: HtmlElement<Renderer> {
    fn set_value(&mut self, renderer: &mut Renderer, value: i32);
}
pub trait HtmlLinkElement<Renderer: ?Sized>:
    HtmlElement<Renderer>
    + ElementWithHrefAttribute<Renderer>
    + ElementWithTypeAttribute<Renderer>
    + ElementWithMediaAttribute<Renderer>
    + ElementWithBlockingAttribute<Renderer>
    + ElementWithIntegrityAttribute<Renderer>
    + ElementWithSizesAttribute<Renderer>
    + ElementWithHrefLangAttribute<Renderer>
    + ElementWithFetchPriorityAttribute<Renderer>
    + ElementWithReferrerPolicyAttribute<Renderer>
    + ElementWithRelAttribute<Renderer>
    + ElementWithCrossOriginAttribute<Renderer>
{
    fn set_as(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait HtmlMapElement<Renderer: ?Sized>: HtmlElement<Renderer> + ElementWithNameAttribute<Renderer> {}
pub trait HtmlMetaElement<Renderer: ?Sized>: HtmlElement<Renderer> + ElementWithNameAttribute<Renderer> {
    fn set_content(&mut self, renderer: &mut Renderer, value: &str);
    fn set_http_equiv(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait HtmlMeterElement<Renderer: ?Sized>: HtmlElement<Renderer> + ElementWithMaxF64Attribute<Renderer> + ElementWithValueF64Attribute<Renderer> {
    fn set_min(&mut self, renderer: &mut Renderer, value: f64);
    fn set_low(&mut self, renderer: &mut Renderer, value: f64);
    fn set_high(&mut self, renderer: &mut Renderer, value: f64);
    fn set_optimum(&mut self, renderer: &mut Renderer, value: f64);
}
pub trait HtmlObjectElement<Renderer: ?Sized>:
    HtmlElement<Renderer> + ElementWithTypeAttribute<Renderer> + ElementWithUseMapAttribute<Renderer> + ElementWithFormAttribute<Renderer> + ElementWithNameAttribute<Renderer> + ElementWithHeightWidthStrAttributes<Renderer>
{
    fn set_data(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait HtmlOListElement<Renderer: ?Sized>: HtmlElement<Renderer> + ElementWithTypeAttribute<Renderer> {
    fn set_reversed(&mut self, renderer: &mut Renderer, value: bool);
    fn set_start(&mut self, renderer: &mut Renderer, value: i32);
}
pub trait HtmlOptGroupElement<Renderer: ?Sized>: HtmlElement<Renderer> + ElementWithLabelAttribute<Renderer> + ElementWithDisabledAttribute<Renderer> {}
pub trait HtmlOptionElement<Renderer: ?Sized>: HtmlElement<Renderer> + ElementWithLabelAttribute<Renderer> + ElementWithDisabledAttribute<Renderer> + ElementWithValueStrAttribute<Renderer> {
    fn set_selected(&mut self, renderer: &mut Renderer, value: bool);
}
pub trait HtmlOutputElement<Renderer: ?Sized>: HtmlElement<Renderer> + ElementWithForAttribute<Renderer> + ElementWithFormAttribute<Renderer> + ElementWithNameAttribute<Renderer> {}
pub trait HtmlProgressElement<Renderer: ?Sized>: HtmlElement<Renderer> + ElementWithMaxF64Attribute<Renderer> + ElementWithValueF64Attribute<Renderer> {}
pub trait HtmlScriptElement<Renderer: ?Sized>:
    HtmlElement<Renderer>
    + ElementWithTypeAttribute<Renderer>
    + ElementWithSrcAttribute<Renderer>
    + ElementWithBlockingAttribute<Renderer>
    + ElementWithIntegrityAttribute<Renderer>
    + ElementWithFetchPriorityAttribute<Renderer>
    + ElementWithReferrerPolicyAttribute<Renderer>
    + ElementWithCrossOriginAttribute<Renderer>
{
    fn set_async(&mut self, renderer: &mut Renderer, value: bool);
    fn set_defer(&mut self, renderer: &mut Renderer, value: bool);
    fn set_no_module(&mut self, renderer: &mut Renderer, value: bool);
}
pub trait HtmlSelectElement<Renderer: ?Sized>:
    HtmlElement<Renderer>
    + ElementWithSizeU32Attribute<Renderer>
    + ElementWithRequiredAttribute<Renderer>
    + ElementWithMultipleAttribute<Renderer>
    + ElementWithFormAttribute<Renderer>
    + ElementWithAutoCompleteAttribute<Renderer>
    + ElementWithDisabledAttribute<Renderer>
    + ElementWithNameAttribute<Renderer>
{
}
pub trait HtmlSlotElement<Renderer: ?Sized>: HtmlElement<Renderer> + ElementWithNameAttribute<Renderer> {}
pub trait HtmlSourceElement<Renderer: ?Sized>:
    HtmlElement<Renderer>
    + ElementWithTypeAttribute<Renderer>
    + ElementWithMediaAttribute<Renderer>
    + ElementWithSrcsetAttribute<Renderer>
    + ElementWithSizesAttribute<Renderer>
    + ElementWithHeightWidthU32Attributes<Renderer>
{
}
pub trait HtmlStyleElement<Renderer: ?Sized>: HtmlElement<Renderer> + ElementWithTypeAttribute<Renderer> + ElementWithMediaAttribute<Renderer> + ElementWithBlockingAttribute<Renderer> {}
pub trait HtmlTableElement<Renderer: ?Sized>: HtmlElement<Renderer> + ElementWithAlignAttribute<Renderer> + ElementWithBgColorAttribute<Renderer> {
    fn set_border(&mut self, renderer: &mut Renderer, value: &str);
    fn set_cell_padding(&mut self, renderer: &mut Renderer, value: &str);
    fn set_cell_spacing(&mut self, renderer: &mut Renderer, value: &str);
    fn set_frame(&mut self, renderer: &mut Renderer, value: &str);
    fn set_rules(&mut self, renderer: &mut Renderer, value: &str);
    fn set_summary(&mut self, renderer: &mut Renderer, value: &str);
    fn set_width(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait HtmlTableChildElement<Renderer: ?Sized>: HtmlElement<Renderer> + ElementWithAlignAttribute<Renderer> + ElementWithBgColorAttribute<Renderer> {
    fn set_ch(&mut self, renderer: &mut Renderer, value: &str);
    fn set_ch_off(&mut self, renderer: &mut Renderer, value: &str);
    fn set_v_align(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait HtmlTableSectionElement<Renderer: ?Sized>: HtmlElement<Renderer> + HtmlTableChildElement<Renderer> {}
pub trait HtmlTableRowElement<Renderer: ?Sized>: HtmlElement<Renderer> + HtmlTableChildElement<Renderer> {}
pub trait HtmlTableColElement<Renderer: ?Sized>: HtmlElement<Renderer> + HtmlTableChildElement<Renderer> {
    fn set_span(&mut self, renderer: &mut Renderer, value: u32);
    fn set_width(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait HtmlTableCellElement<Renderer: ?Sized>: HtmlElement<Renderer> + ElementWithHeightWidthStrAttributes<Renderer> + HtmlTableChildElement<Renderer> {
    fn set_col_span(&mut self, renderer: &mut Renderer, value: u32);
    fn set_headers(&mut self, renderer: &mut Renderer, value: &str);
    fn set_row_span(&mut self, renderer: &mut Renderer, value: u32);
    fn set_axis(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait HtmlTextAreaElement<Renderer: ?Sized>:
    HtmlElement<Renderer>
    + ElementWithReadOnlyAttribute<Renderer>
    + ElementWithPlaceHolderAttribute<Renderer>
    + ElementWithMaxMinLengthAttributes<Renderer>
    + ElementWithRequiredAttribute<Renderer>
    + ElementWithFormAttribute<Renderer>
    + ElementWithAutoCompleteAttribute<Renderer>
    + ElementWithAutoCorrectAttribute<Renderer>
    + ElementWithDisabledAttribute<Renderer>
    + ElementWithNameAttribute<Renderer>
    + frender_form_control::element::FormControlElement<str, Renderer>
{
    fn set_cols(&mut self, renderer: &mut Renderer, value: u32);
    fn set_rows(&mut self, renderer: &mut Renderer, value: u32);
    fn set_wrap(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait HtmlTimeElement<Renderer: ?Sized>: HtmlElement<Renderer> + ElementWithDateTimeAttribute<Renderer> {}
pub trait HtmlTrackElement<Renderer: ?Sized>: HtmlElement<Renderer> + ElementWithSrcAttribute<Renderer> + ElementWithLabelAttribute<Renderer> {
    fn set_default(&mut self, renderer: &mut Renderer, value: bool);
    fn set_kind(&mut self, renderer: &mut Renderer, value: &str);
    fn set_src_lang(&mut self, renderer: &mut Renderer, value: &str);
}
pub trait HtmlUListElement<Renderer: ?Sized>: HtmlElement<Renderer> + ElementWithTypeAttribute<Renderer> {
    fn set_compact(&mut self, renderer: &mut Renderer, value: bool);
}
pub trait HtmlAudioElement<Renderer: ?Sized>: HtmlMediaElement<Renderer> {}
pub trait HtmlVideoElement<Renderer: ?Sized>: HtmlMediaElement<Renderer> + ElementWithHeightWidthU32Attributes<Renderer> {
    fn set_poster(&mut self, renderer: &mut Renderer, value: &str);
}
