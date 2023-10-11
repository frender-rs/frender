use crate::{
    event_types::{HtmlElementWithEvents, HtmlFormElementWithEvents, HtmlMediaElementWithEvents},
    renderer::node_behaviors::{Element, Node},
};

pub use special::*;

mod define;
mod special;

define::define!(
    pub mod prelude {}

    pub trait HtmlElement {
        super_traits!(Element, Node);
        extends!(HtmlElementWithEvents);

        fn set_access_key(value: &str);
        fn set_content_editable(value: &str);
        fn set_dir(value: &str);
        fn set_draggable(value: bool);
        fn set_hidden(value: bool);
        fn set_lang(value: &str);
        fn set_spellcheck(value: bool);
        fn set_tab_index(value: i32);
        fn set_title(value: &str);

        sub_traits!(
            pub trait HtmlAnchorElement {
                special_extends!(HtmlElementWithHref);

                fn set_href_lang(value: &str) {
                    #![web_sys_name(set_hreflang)]
                }
                fn set_type(value: &str);
            }

            pub trait HtmlAreaElement {
                special_extends!(HtmlElementWithHref);

                fn set_alt(value: &str);
                fn set_coords(value: &str);
                fn set_shape(value: &str);
            }

            pub trait HtmlMediaElement {
                extends!(HtmlMediaElementWithEvents);
                fn set_auto_play(value: bool) {
                    #![web_sys_name(set_autoplay)]
                }
                fn set_controls(value: bool);
                fn set_cross_origin(value: Option<&str>);
                fn set_loop(value: bool);
                fn set_muted(value: bool);
                fn set_preload(value: &str);
                fn set_src(value: &str);

                sub_traits!(
                    pub trait HtmlAudioElement {}
                    pub trait HtmlVideoElement {
                        fn set_height(value: u32);
                        fn set_poster(value: &str);
                        fn set_width(value: u32);
                    }
                );
            }

            pub trait HtmlBaseElement {
                fn set_href(value: &str);
                fn set_target(value: &str);
            }
            pub trait HtmlQuoteElement {
                fn set_cite(value: &str);
            }
            pub trait HtmlBodyElement {}
            pub trait HtmlBrElement {
                #[deprecated]
                fn set_clear(value: &str);
            }
            pub trait HtmlButtonElement {
                fn set_disabled(value: bool);
                fn set_form_action(value: &str);
                fn set_form_enctype(value: &str);
                fn set_form_method(value: &str);
                fn set_form_no_validate(value: bool);
                fn set_form_target(value: &str);
                fn set_name(value: &str);
                fn set_type(value: &str);
                fn set_value(value: &str);
            }

            pub trait HtmlCanvasElement {
                fn set_height(value: u32);
                fn set_width(value: u32);
            }
            pub trait HtmlTableCaptionElement {
                #[deprecated]
                fn set_align(value: &str);
            }
            pub trait HtmlDataElement {
                fn set_value(value: &str);
            }
            pub trait HtmlModElement {
                fn set_cite(value: &str);
                fn set_date_time(value: &str);
            }

            pub trait HtmlDetailsElement {
                special_extends!(HtmlElementWithOpen);
            }
            pub trait HtmlDialogElement {
                special_extends!(HtmlElementWithOpen);
            }
            pub trait HtmlEmbedElement {
                fn set_height(value: &str);
                fn set_src(value: &str);
                fn set_type(value: &str);
                fn set_width(value: &str);
            }
            pub trait HtmlFieldSetElement {
                fn set_disabled(value: bool);
                fn set_name(value: &str);
            }
            pub trait HtmlFormElement {
                extends!(HtmlFormElementWithEvents);

                fn set_accept_charset(value: &str);
                fn set_auto_complete(value: &str) {
                    #![web_sys_name(set_autocomplete)]
                }
                fn set_name(value: &str);
                fn set_action(value: &str);
                fn set_enctype(value: &str);
                fn set_method(value: &str);
                fn set_no_validate(value: bool);
                fn set_target(value: &str);
            }
            pub trait HtmlHtmlElement {}
            pub trait HtmlIFrameElement {
                fn set_allow_fullscreen(value: bool);
                fn set_allow_payment_request(value: bool);
                fn set_height(value: &str);
                fn set_name(value: &str);
                fn set_referrer_policy(value: &str);
                fn set_src(value: &str);
                fn set_srcdoc(value: &str);
                fn set_width(value: &str);
            }
            pub trait HtmlImageElement {
                fn set_alt(value: &str);
                fn set_cross_origin(value: Option<&str>);
                fn set_decoding(value: &str);
                fn set_height(value: u32);
                fn set_is_map(value: bool);
                fn set_referrer_policy(value: &str);
                fn set_sizes(value: &str);
                fn set_src(value: &str);
                fn set_srcset(value: &str);
                fn set_width(value: u32);
                fn set_use_map(value: &str);
            }
            pub trait HtmlInputElement {
                fn set_accept(value: &str);
                fn set_alt(value: &str);
                fn set_auto_complete(value: &str) {
                    #![web_sys_name(set_autocomplete)]
                }
                fn set_checked(value: bool);
                fn set_disabled(value: bool);
                fn set_form_action(value: &str);
                fn set_form_enctype(value: &str);
                fn set_form_method(value: &str);
                fn set_form_no_validate(value: bool);
                fn set_form_target(value: &str);
                fn set_height(value: u32);
                fn set_max(value: &str);
                fn set_max_length(value: i32);
                fn set_min(value: &str);
                fn set_min_length(value: i32);
                fn set_multiple(value: bool);
                fn set_name(value: &str);
                fn set_pattern(value: &str);
                fn set_placeholder(value: &str);
                fn set_read_only(value: bool);
                fn set_required(value: bool);
                fn set_size(value: u32);
                fn set_src(value: &str);
                fn set_step(value: &str);
                fn set_type(value: &str);
                fn set_value(value: &str);
                fn set_width(value: u32);
            }
            pub trait HtmlLabelElement {
                fn set_html_for(value: &str);
            }
            pub trait HtmlLiElement {
                fn set_value(value: i32);
            }
            pub trait HtmlLinkElement {
                special_extends!(HtmlElementWithRelList);

                fn set_as(value: &str);
                fn set_cross_origin(value: Option<&str>);
                fn set_href(value: &str);
                fn set_hreflang(value: &str);
                fn set_integrity(value: &str);
                fn set_media(value: &str);
                fn set_referrer_policy(value: &str);
                fn set_type(value: &str);
            }
            pub trait HtmlMapElement {
                fn set_name(value: &str);
            }
            pub trait HtmlMetaElement {
                fn set_content(value: &str);
                fn set_http_equiv(value: &str);
                fn set_name(value: &str);
            }
            pub trait HtmlMeterElement {
                fn set_value(value: f64);
                fn set_min(value: f64);
                fn set_max(value: f64);
                fn set_low(value: f64);
                fn set_high(value: f64);
                fn set_optimum(value: f64);
            }
            pub trait HtmlObjectElement {
                fn set_data(value: &str);
                fn set_height(value: &str);
                fn set_name(value: &str);
                fn set_type(value: &str);
                fn set_use_map(value: &str);
                fn set_width(value: &str);
            }
            pub trait HtmlOListElement {
                fn set_reversed(value: bool);
                fn set_start(value: i32);
                fn set_type(value: &str);
            }
            pub trait HtmlOptGroupElement {
                fn set_disabled(value: bool);
                fn set_label(value: &str);
            }
            pub trait HtmlOptionElement {
                fn set_disabled(value: bool);
                fn set_label(value: &str);
                fn set_selected(value: bool);
                fn set_value(value: &str);
            }
            pub trait HtmlOutputElement {
                fn set_name(value: &str);
            }
            pub trait HtmlProgressElement {
                fn set_max(value: f64);
                fn set_value(value: f64);
            }
            pub trait HtmlScriptElement {
                fn set_async(value: bool);
                fn set_cross_origin(value: Option<&str>);
                fn set_defer(value: bool);
                fn set_integrity(value: &str);
                fn set_no_module(value: bool);
                fn set_src(value: &str);
                fn set_type(value: &str);
            }
            pub trait HtmlSelectElement {
                fn set_auto_complete(value: &str) {
                    #![web_sys_name(set_autocomplete)]
                }
                fn set_disabled(value: bool);
                fn set_multiple(value: bool);
                fn set_name(value: &str);
                fn set_required(value: bool);
                fn set_size(value: u32);
            }
            pub trait HtmlSlotElement {
                fn set_name(value: &str);
            }
            pub trait HtmlSourceElement {
                fn set_type(value: &str);
                fn set_src(value: &str);
                fn set_srcset(value: &str);
                fn set_sizes(value: &str);
                fn set_media(value: &str);
            }
            pub trait HtmlStyleElement {
                fn set_media(value: &str);
                fn set_type(value: &str);
            }
            pub trait HtmlTableElement {
                #[deprecated]
                fn set_align(value: &str);
                #[deprecated]
                fn set_bg_color(value: &str);
                #[deprecated]
                fn set_border(value: &str);
                #[deprecated]
                fn set_cell_padding(value: &str);
                #[deprecated]
                fn set_cell_spacing(value: &str);
                #[deprecated]
                fn set_frame(value: &str);
                #[deprecated]
                fn set_rules(value: &str);
                #[deprecated]
                fn set_summary(value: &str);
                #[deprecated]
                fn set_width(value: &str);
            }
            pub trait HtmlTableSectionElement {
                special_extends!(HtmlTableChildElement);
            }
            pub trait HtmlTableRowElement {
                special_extends!(HtmlTableChildElement);
            }
            pub trait HtmlTableColElement {
                special_extends!(HtmlTableChildElement);

                fn set_span(value: u32);
                #[deprecated]
                fn set_width(value: &str);
            }
            pub trait HtmlTableCellElement {
                special_extends!(HtmlTableChildElement);

                fn set_col_span(value: u32);
                fn set_headers(value: &str);
                fn set_row_span(value: u32);
                #[deprecated]
                fn set_axis(value: &str);
                #[deprecated]
                fn set_height(value: &str);
                #[deprecated]
                fn set_width(value: &str);
            }
            pub trait HtmlTextAreaElement {
                fn set_auto_complete(value: &str) {
                    #![web_sys_name(set_autocomplete)]
                }
                fn set_cols(value: u32);
                fn set_disabled(value: bool);
                fn set_max_length(value: i32);
                fn set_min_length(value: i32);
                fn set_name(value: &str);
                fn set_placeholder(value: &str);
                fn set_read_only(value: bool);
                fn set_required(value: bool);
                fn set_rows(value: u32);
                fn set_wrap(value: &str);
            }
            pub trait HtmlTimeElement {
                fn set_date_time(value: &str);
            }
            pub trait HtmlTrackElement {
                fn set_default(value: bool);
                fn set_kind(value: &str);
                fn set_label(value: &str);
                fn set_src(value: &str);
                fn set_src_lang(value: &str) {
                    #![web_sys_name(set_srclang)]
                }
            }
            pub trait HtmlUListElement {
                fn set_compact(value: bool);
                fn set_type(value: &str);
            }
        );
    }
);
