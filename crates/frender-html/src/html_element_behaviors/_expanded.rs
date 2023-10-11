pub trait HtmlElement<R: ?Sized>: Element<R> + HtmlElementWithEvents<R> {
    fn set_access_key(&mut self, renderer: &mut R, value: &str);
    fn set_content_editable(&mut self, renderer: &mut R, value: &str);
    fn set_dir(&mut self, renderer: &mut R, value: &str);
    fn set_draggable(&mut self, renderer: &mut R, value: bool);
    fn set_hidden(&mut self, renderer: &mut R, value: bool);
    fn set_lang(&mut self, renderer: &mut R, value: &str);
    fn set_spellcheck(&mut self, renderer: &mut R, value: bool);
    fn set_tab_index(&mut self, renderer: &mut R, value: i32);
    fn set_title(&mut self, renderer: &mut R, value: &str);
}

pub trait HtmlElementWithHref<R: ?Sized>: HtmlElement<R> {
    fn set_download(&mut self, renderer: &mut R, value: &str);
    fn set_href(&mut self, renderer: &mut R, value: &str);
    fn set_ping(&mut self, renderer: &mut R, value: &str);
    fn set_referrer_policy(&mut self, renderer: &mut R, value: &str);

    type RelList<'a>: DomTokenList
    where
        Self: 'a,
        R: 'a;
    fn rel_list<'a>(&'a mut self, renderer: &'a mut R) -> Self::RelList<'a>;

    fn set_target(&mut self, renderer: &mut R, value: &str);
}

pub trait HtmlAnchorElement<R: ?Sized>: HtmlElementWithHref<R> {
    fn set_href_lang(&mut self, renderer: &mut R, value: &str);
    fn set_type(&mut self, renderer: &mut R, value: &str);
}

pub trait HtmlAreaElement<R: ?Sized>: HtmlElementWithHref<R> {
    fn set_alt(&mut self, renderer: &mut R, value: &str);
    fn set_coords(&mut self, renderer: &mut R, value: &str);
    fn set_shape(&mut self, renderer: &mut R, value: &str);
}

pub trait HtmlMediaElement<R: ?Sized>: HtmlElement<R> + HtmlMediaElementWithEvents<R> {
    fn set_auto_play(&mut self, renderer: &mut R, value: bool);
    fn set_controls(&mut self, renderer: &mut R, value: bool);
    fn set_cross_origin(&mut self, renderer: &mut R, value: Option<&str>);
    fn set_loop(&mut self, renderer: &mut R, value: bool);
    fn set_muted(&mut self, renderer: &mut R, value: bool);
    fn set_preload(&mut self, renderer: &mut R, value: &str);
    fn set_src(&mut self, renderer: &mut R, value: &str);
}

pub trait HtmlAudioElement<R: ?Sized>: HtmlMediaElement<R> {}
pub trait HtmlVideoElement<R: ?Sized>: HtmlMediaElement<R> {
    fn set_height(&mut self, renderer: &mut R, value: u32);
    fn set_poster(&mut self, renderer: &mut R, value: &str);
    fn set_width(&mut self, renderer: &mut R, value: u32);
}

pub trait HtmlBaseElement<R: ?Sized>: HtmlElement<R> {
    fn set_href(&mut self, renderer: &mut R, value: &str);
    fn set_target(&mut self, renderer: &mut R, value: &str);
}
pub trait HtmlQuoteElement<R: ?Sized>: HtmlElement<R> {
    fn set_cite(&mut self, renderer: &mut R, value: &str);
}
pub trait HtmlBodyElement<R: ?Sized>: HtmlElement<R> {}
pub trait HtmlBrElement<R: ?Sized>: HtmlElement<R> {
    #[deprecated]
    fn set_clear(&mut self, renderer: &mut R, value: &str);
}
pub trait HtmlButtonElement<R: ?Sized>: HtmlElement<R> {
    fn set_disabled(&mut self, renderer: &mut R, value: bool);
    fn set_form_action(&mut self, renderer: &mut R, value: &str);
    fn set_form_enctype(&mut self, renderer: &mut R, value: &str);
    fn set_form_method(&mut self, renderer: &mut R, value: &str);
    fn set_form_no_validate(&mut self, renderer: &mut R, value: bool);
    fn set_form_target(&mut self, renderer: &mut R, value: &str);
    fn set_name(&mut self, renderer: &mut R, value: &str);
    fn set_type(&mut self, renderer: &mut R, value: &str);
    fn set_value(&mut self, renderer: &mut R, value: &str);
}

pub trait HtmlCanvasElement<R: ?Sized>: HtmlElement<R> {
    fn set_height(&mut self, renderer: &mut R, value: u32);
    fn set_width(&mut self, renderer: &mut R, value: u32);
}
pub trait HtmlTableCaptionElement<R: ?Sized>: HtmlElement<R> {
    #[deprecated]
    fn set_align(&mut self, renderer: &mut R, value: &str);
}
pub trait HtmlDataElement<R: ?Sized>: HtmlElement<R> {
    fn set_value(&mut self, renderer: &mut R, value: &str);
}
pub trait HtmlModElement<R: ?Sized>: HtmlElement<R> {
    fn set_cite(&mut self, renderer: &mut R, value: &str);
    fn set_date_time(&mut self, renderer: &mut R, value: &str);
}

pub trait HtmlElementWithOpen<R: ?Sized>: HtmlElement<R> {
    fn set_open(&mut self, renderer: &mut R, value: bool);
}

pub trait HtmlDetailsElement<R: ?Sized>: HtmlElement<R> + HtmlElementWithOpen<R> {}
pub trait HtmlDialogElement<R: ?Sized>: HtmlElement<R> + HtmlElementWithOpen<R> {}
pub trait HtmlEmbedElement<R: ?Sized>: HtmlElement<R> {
    fn set_height(&mut self, renderer: &mut R, value: &str);
    fn set_src(&mut self, renderer: &mut R, value: &str);
    fn set_type(&mut self, renderer: &mut R, value: &str);
    fn set_width(&mut self, renderer: &mut R, value: &str);
}
pub trait HtmlFieldSetElement<R: ?Sized>: HtmlElement<R> {
    fn set_disabled(&mut self, renderer: &mut R, value: bool);
    fn set_name(&mut self, renderer: &mut R, value: &str);
}
pub trait HtmlFormElement<R: ?Sized>: HtmlElement<R> + HtmlFormElementWithEvents<R> {
    fn set_accept_charset(&mut self, renderer: &mut R, value: &str);
    fn set_auto_complete(&mut self, renderer: &mut R, value: &str);
    fn set_name(&mut self, renderer: &mut R, value: &str);
    fn set_action(&mut self, renderer: &mut R, value: &str);
    fn set_enctype(&mut self, renderer: &mut R, value: &str);
    fn set_method(&mut self, renderer: &mut R, value: &str);
    fn set_no_validate(&mut self, renderer: &mut R, value: bool);
    fn set_target(&mut self, renderer: &mut R, value: &str);
}
pub trait HtmlHtmlElement<R: ?Sized>: HtmlElement<R> {}
pub trait HtmlIFrameElement<R: ?Sized>: HtmlElement<R> {
    fn set_allow_fullscreen(&mut self, renderer: &mut R, value: bool);
    fn set_allow_payment_request(&mut self, renderer: &mut R, value: bool);
    fn set_height(&mut self, renderer: &mut R, value: &str);
    fn set_name(&mut self, renderer: &mut R, value: &str);
    fn set_referrer_policy(&mut self, renderer: &mut R, value: &str);
    fn set_src(&mut self, renderer: &mut R, value: &str);
    fn set_srcdoc(&mut self, renderer: &mut R, value: &str);
    fn set_width(&mut self, renderer: &mut R, value: &str);
}
pub trait HtmlImageElement<R: ?Sized>: HtmlElement<R> {
    fn set_alt(&mut self, renderer: &mut R, value: &str);
    fn set_cross_origin(&mut self, renderer: &mut R, value: Option<&str>);
    fn set_decoding(&mut self, renderer: &mut R, value: &str);
    fn set_height(&mut self, renderer: &mut R, value: u32);
    fn set_is_map(&mut self, renderer: &mut R, value: bool);
    fn set_referrer_policy(&mut self, renderer: &mut R, value: &str);
    fn set_sizes(&mut self, renderer: &mut R, value: &str);
    fn set_src(&mut self, renderer: &mut R, value: &str);
    fn set_srcset(&mut self, renderer: &mut R, value: &str);
    fn set_width(&mut self, renderer: &mut R, value: u32);
    fn set_use_map(&mut self, renderer: &mut R, value: &str);
}
pub trait HtmlInputElement<R: ?Sized>: HtmlElement<R> {
    fn set_accept(&mut self, renderer: &mut R, value: &str);
    fn set_alt(&mut self, renderer: &mut R, value: &str);
    fn set_auto_complete(&mut self, renderer: &mut R, value: &str);
    fn set_checked(&mut self, renderer: &mut R, value: bool);
    fn set_disabled(&mut self, renderer: &mut R, value: bool);
    fn set_form_action(&mut self, renderer: &mut R, value: &str);
    fn set_form_enctype(&mut self, renderer: &mut R, value: &str);
    fn set_form_method(&mut self, renderer: &mut R, value: &str);
    fn set_form_no_validate(&mut self, renderer: &mut R, value: bool);
    fn set_form_target(&mut self, renderer: &mut R, value: &str);
    fn set_height(&mut self, renderer: &mut R, value: u32);
    fn set_max(&mut self, renderer: &mut R, value: &str);
    fn set_max_length(&mut self, renderer: &mut R, value: i32);
    fn set_min(&mut self, renderer: &mut R, value: &str);
    fn set_min_length(&mut self, renderer: &mut R, value: i32);
    fn set_multiple(&mut self, renderer: &mut R, value: bool);
    fn set_name(&mut self, renderer: &mut R, value: &str);
    fn set_pattern(&mut self, renderer: &mut R, value: &str);
    fn set_placeholder(&mut self, renderer: &mut R, value: &str);
    fn set_read_only(&mut self, renderer: &mut R, value: bool);
    fn set_required(&mut self, renderer: &mut R, value: bool);
    fn set_size(&mut self, renderer: &mut R, value: u32);
    fn set_src(&mut self, renderer: &mut R, value: &str);
    fn set_step(&mut self, renderer: &mut R, value: &str);
    fn set_type(&mut self, renderer: &mut R, value: &str);
    fn set_value(&mut self, renderer: &mut R, value: &str);
    fn set_width(&mut self, renderer: &mut R, value: u32);
}
pub trait HtmlLabelElement<R: ?Sized>: HtmlElement<R> {
    fn set_html_for(&mut self, renderer: &mut R, value: &str);
}
pub trait HtmlLiElement<R: ?Sized>: HtmlElement<R> {
    fn set_html_for(&mut self, renderer: &mut R, value: i32);
}
pub trait HtmlLinkElement<R: ?Sized>: HtmlElement<R> {
    fn set_as(&mut self, renderer: &mut R, value: &str);
    fn set_href(&mut self, renderer: &mut R, value: &str);
    fn set_hreflang(&mut self, renderer: &mut R, value: &str);
    fn set_integrity(&mut self, renderer: &mut R, value: &str);
    fn set_media(&mut self, renderer: &mut R, value: &str);
    fn set_referrer_policy(&mut self, renderer: &mut R, value: &str);
    fn set_type(&mut self, renderer: &mut R, value: &str);
}
pub trait HtmlMapElement<R: ?Sized>: HtmlElement<R> {
    fn set_name(&mut self, renderer: &mut R, value: &str);
}
pub trait HtmlMetaElement<R: ?Sized>: HtmlElement<R> {
    fn set_content(&mut self, renderer: &mut R, value: &str);
    fn set_http_equiv(&mut self, renderer: &mut R, value: &str);
    fn set_name(&mut self, renderer: &mut R, value: &str);
}
pub trait HtmlMeterElement<R: ?Sized>: HtmlElement<R> {
    fn set_value(&mut self, renderer: &mut R, value: f64);
    fn set_min(&mut self, renderer: &mut R, value: f64);
    fn set_max(&mut self, renderer: &mut R, value: f64);
    fn set_low(&mut self, renderer: &mut R, value: f64);
    fn set_high(&mut self, renderer: &mut R, value: f64);
    fn set_optimum(&mut self, renderer: &mut R, value: f64);
}
pub trait HtmlObjectElement<R: ?Sized>: HtmlElement<R> {
    fn set_data(&mut self, renderer: &mut R, value: &str);
    fn set_height(&mut self, renderer: &mut R, value: &str);
    fn set_name(&mut self, renderer: &mut R, value: &str);
    fn set_type(&mut self, renderer: &mut R, value: &str);
    fn set_use_map(&mut self, renderer: &mut R, value: &str);
    fn set_width(&mut self, renderer: &mut R, value: &str);
}
pub trait HtmlOListElement<R: ?Sized>: HtmlElement<R> {
    fn set_reversed(&mut self, renderer: &mut R, value: bool);
    fn set_start(&mut self, renderer: &mut R, value: i32);
    fn set_type(&mut self, renderer: &mut R, value: &str);
}
pub trait HtmlOptGroupElement<R: ?Sized>: HtmlElement<R> {
    fn set_disabled(&mut self, renderer: &mut R, value: bool);
    fn set_label(&mut self, renderer: &mut R, value: &str);
}
pub trait HtmlOptionElement<R: ?Sized>: HtmlElement<R> {
    fn set_disabled(&mut self, renderer: &mut R, value: bool);
    fn set_label(&mut self, renderer: &mut R, value: &str);
    fn set_selected(&mut self, renderer: &mut R, value: bool);
    fn set_value(&mut self, renderer: &mut R, value: &str);
}
pub trait HtmlOutputElement<R: ?Sized>: HtmlElement<R> {
    fn set_name(&mut self, renderer: &mut R, value: &str);
}
pub trait HtmlProgressElement<R: ?Sized>: HtmlElement<R> {
    fn set_max(&mut self, renderer: &mut R, value: f64);
    fn set_value(&mut self, renderer: &mut R, value: f64);
}
pub trait HtmlScriptElement<R: ?Sized>: HtmlElement<R> {
    fn set_async(&mut self, renderer: &mut R, value: bool);
    fn set_defer(&mut self, renderer: &mut R, value: bool);
    fn set_integrity(&mut self, renderer: &mut R, value: &str);
    fn set_no_module(&mut self, renderer: &mut R, value: bool);
    fn set_src(&mut self, renderer: &mut R, value: &str);
    fn set_type(&mut self, renderer: &mut R, value: &str);
}
pub trait HtmlSelectElement<R: ?Sized>: HtmlElement<R> {
    fn set_auto_complete(&mut self, renderer: &mut R, value: &str);
    fn set_disabled(&mut self, renderer: &mut R, value: bool);
    fn set_multiple(&mut self, renderer: &mut R, value: bool);
    fn set_name(&mut self, renderer: &mut R, value: &str);
    fn set_required(&mut self, renderer: &mut R, value: bool);
    fn set_size(&mut self, renderer: &mut R, value: u32);
}
pub trait HtmlSlotElement<R: ?Sized>: HtmlElement<R> {
    fn set_name(&mut self, renderer: &mut R, value: &str);
}
pub trait HtmlSourceElement<R: ?Sized>: HtmlElement<R> {
    fn set_type(&mut self, renderer: &mut R, value: &str);
    fn set_src(&mut self, renderer: &mut R, value: &str);
    fn set_srcset(&mut self, renderer: &mut R, value: &str);
    fn set_sizes(&mut self, renderer: &mut R, value: &str);
    fn set_media(&mut self, renderer: &mut R, value: &str);
}
pub trait HtmlStyleElement<R: ?Sized>: HtmlElement<R> {
    fn set_media(&mut self, renderer: &mut R, value: &str);
    fn set_type(&mut self, renderer: &mut R, value: &str);
}
pub trait HtmlTableElement<R: ?Sized>: HtmlElement<R> {
    #[deprecated]
    fn set_align(&mut self, renderer: &mut R, value: &str);
    #[deprecated]
    fn set_bg_color(&mut self, renderer: &mut R, value: &str);
    #[deprecated]
    fn set_border(&mut self, renderer: &mut R, value: &str);
    #[deprecated]
    fn set_cell_padding(&mut self, renderer: &mut R, value: &str);
    #[deprecated]
    fn set_cell_spacing(&mut self, renderer: &mut R, value: &str);
    #[deprecated]
    fn set_frame(&mut self, renderer: &mut R, value: &str);
    #[deprecated]
    fn set_rules(&mut self, renderer: &mut R, value: &str);
    #[deprecated]
    fn set_summary(&mut self, renderer: &mut R, value: &str);
    #[deprecated]
    fn set_width(&mut self, renderer: &mut R, value: &str);
}
pub trait HtmlTableChildElement<R: ?Sized>: HtmlElement<R> {
    #[deprecated]
    fn set_align(&mut self, renderer: &mut R, value: &str);
    #[deprecated]
    fn set_ch(&mut self, renderer: &mut R, value: &str);
    #[deprecated]
    fn set_ch_off(&mut self, renderer: &mut R, value: &str);
    #[deprecated]
    fn set_v_align(&mut self, renderer: &mut R, value: &str);
}
pub trait HtmlTableSectionElement<R: ?Sized>: HtmlElement<R> + HtmlTableChildElement<R> {}
pub trait HtmlTableRowElement<R: ?Sized>: HtmlElement<R> + HtmlTableChildElement<R> {}
pub trait HtmlTableColElement<R: ?Sized>: HtmlElement<R> + HtmlTableChildElement<R> {
    fn set_span(&mut self, renderer: &mut R, value: u32);
    #[deprecated]
    fn set_width(&mut self, renderer: &mut R, value: &str);
}
pub trait HtmlTableCellElement<R: ?Sized>: HtmlElement<R> {
    fn set_col_span(&mut self, renderer: &mut R, value: u32);
    fn set_headers(&mut self, renderer: &mut R, value: &str);
    fn set_row_span(&mut self, renderer: &mut R, value: u32);
    #[deprecated]
    fn set_axis(&mut self, renderer: &mut R, value: &str);
    #[deprecated]
    fn set_height(&mut self, renderer: &mut R, value: &str);
    #[deprecated]
    fn set_width(&mut self, renderer: &mut R, value: &str);
}
pub trait HtmlTextAreaElement<R: ?Sized>: HtmlElement<R> {
    fn set_auto_complete(&mut self, renderer: &mut R, value: &str);
    fn set_cols(&mut self, renderer: &mut R, value: u32);
    fn set_disabled(&mut self, renderer: &mut R, value: bool);
    fn set_max_length(&mut self, renderer: &mut R, value: i32);
    fn set_min_length(&mut self, renderer: &mut R, value: i32);
    fn set_name(&mut self, renderer: &mut R, value: &str);
    fn set_placeholder(&mut self, renderer: &mut R, value: &str);
    fn set_read_only(&mut self, renderer: &mut R, value: bool);
    fn set_required(&mut self, renderer: &mut R, value: bool);
    fn set_rows(&mut self, renderer: &mut R, value: u32);
    fn set_wrap(&mut self, renderer: &mut R, value: &str);
}
pub trait HtmlTimeElement<R: ?Sized>: HtmlElement<R> {
    fn set_date_time(&mut self, renderer: &mut R, value: &str);
}
pub trait HtmlTrackElement<R: ?Sized>: HtmlElement<R> {
    fn set_default(&mut self, renderer: &mut R, value: bool);
    fn set_kind(&mut self, renderer: &mut R, value: &str);
    fn set_label(&mut self, renderer: &mut R, value: &str);
    fn set_src(&mut self, renderer: &mut R, value: &str);
    fn set_src_lang(&mut self, renderer: &mut R, value: &str);
}
pub trait HtmlUListElement<R: ?Sized>: HtmlElement<R> {
    fn set_compact(&mut self, renderer: &mut R, value: bool);
    fn set_type(&mut self, renderer: &mut R, value: &str);
}
