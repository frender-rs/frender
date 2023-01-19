pub trait IntrinsicComponent {
    const INTRINSIC_TAG: &'static str;
}

pub struct FieldData<'a, Data, State, Element> {
    pub data: Data,
    pub state: State,
    pub element: &'a Element,
    pub dom_element: &'a ::web_sys::Element,
    pub children_ctx: &'a mut ::frender_dom::Dom,
}

crate::def_intrinsic_component_props! {
    pub struct ElementProps (web_sys::Element) {
        children: () = () => {
            dom {
                bounds[::frender_core::UpdateRenderState<::frender_dom::Dom>]
                state pin
                    < <TypeDefs::children as frender_core::UpdateRenderState<frender_dom::Dom>>::State >
                    :[::frender_core::RenderState]=(::frender_core::RenderState::new_uninitialized())
                impl { data, state, children_ctx, .. } {
                    ::frender_core::UpdateRenderState::update_render_state(data, children_ctx, state)
                }
            }
        },
        class ? str,
        id ? str {set_id},
        part ? str,
    } [
        pub struct HtmlElementProps (web_sys::HtmlElement) {
            access_key ? str {"accesskey" set_access_key},
            auto_capitalize ? str {"autocapitalize"},
            auto_focus ? bool {"autofocus"},
            content_editable [crate::props::MaybeInherit<bool>] : () = () => {
                dom {
                    impl { data, .. } {
                        // TODO:
                    }
                }
            },
            #[deprecated = "See https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/contextMenu"]
            context_menu ? str {"contextmenu"},
            dir ? str {set_dir},
            draggable ? bool { set_draggable* },
            enter_key_hint ? str {"enterkeyhint"},
            hidden ? bool { set_hidden* }, // TODO: "until-found"
            inert ? bool,
            input_mode ? str {"inputmode"},
            is ? str,

            item_id    ? str {"itemid"},
            item_prop  ? str {"itemprop"},
            item_ref   ? str {"itemref"},
            item_scope ? str {"itemscope"},
            item_type  ? str {"itemtype"},

            lang ? str {set_lang},
            nonce ? str,
            role ? str,
            slot ? str,
            spellcheck ? bool { set_spellcheck* },
            // style ? str { set_style }, // TODO:
            tab_index ? i32 { "tabindex" set_tab_index* },
            title ? str {set_title},
            translate ? str,
            virtual_keyboard_policy ? str {"virtualkeyboardpolicy"},

            // TODO: aria-*
            // TODO: data-*
            // TODO: events
        } [
            pub struct HtmlHrefCommonProps(web_sys::HtmlAnchorElement) {
                download ? str { set_download },
                href ? str { set_href },
                ping ? str { set_ping },
                referrer_policy ? str {"referrerpolicy" set_referrer_policy},
                rel ? str { set_rel },
                target ? str { set_target },
            } [
                pub struct HtmlAnchorElementProps(web_sys::HtmlAnchorElement) {
                    href_lang ? str { "hreflang" set_hreflang },
                    type_ ? str {"type" set_type},
                }
            ][
                pub struct HtmlAreaElementProps(web_sys::HtmlAreaElement) {
                    alt ? str {set_alt},
                    coords ? str {set_coords},
                    shape ? str {set_shape},
                }
            ]
        ][
            pub struct HtmlMediaElementProps(web_sys::HtmlMediaElement) {
                auto_play ? bool {"autoplay" set_autoplay*},
                controls ? bool {set_controls*},
                cross_origin ? str {"crossorigin" [update el |v:&_| el.set_cross_origin(Some(v)) ] [remove el || el.set_cross_origin(None)]},
                loop_ ? bool {"loop" set_loop*},
                muted ? bool {set_muted*},
                preload ? str {set_preload},
                src ? str {set_src},
            } [
                pub struct HtmlAudioElementProps(web_sys::HtmlAudioElement) {
                }
            ][
                pub struct HtmlVideoElementProps(web_sys::HtmlVideoElement) {
                    height ? u32 {set_height*},
                    plays_inline ? bool {"playsinline"},
                    poster ? str {set_poster},
                    width ? u32 {set_width*},
                }
            ]
        ][
            pub struct HtmlBaseElementProps(web_sys::HtmlBaseElement) {
                href ? str { set_href },
                target ? str { set_target },
            }
        ][
            // blockquote q
            pub struct HtmlQuoteElementProps(web_sys::HtmlQuoteElement) {
                cite ? str { set_cite },
            }
        ][
            pub struct HtmlBodyElementProps(web_sys::HtmlBodyElement) {
                // TODO:
                // https://developer.mozilla.org/en-US/docs/Web/HTML/Element/body
            }
        ][
            pub struct HtmlBrElementProps(web_sys::HtmlBrElement) {
                clear ? str {set_clear},
            }
        ][
            pub struct HtmlButtonElementProps(web_sys::HtmlButtonElement) {
                disabled ? bool {set_disabled*},
                form ? str,
                form_action ? str {"formaction" set_form_action},
                form_enc_type ? str {"formenctype" set_form_enctype},
                form_method ? str {"formmethod" set_form_method},
                form_no_validate ? bool {"formnovalidate" set_form_no_validate*},
                form_target ? str {"formtarget" set_form_target},
                name ? str {set_name},
                type_ ? str {"type" set_type},
                value ? str {set_value},
            }
        ][
            pub struct HtmlCanvasElementProps(web_sys::HtmlCanvasElement) {
                height ? u32 {set_height*},
                width ? u32 {set_width*},
            }
        ][
            pub struct HtmlTableCaptionElementProps(web_sys::HtmlTableCaptionElement) {
                #[deprecated = "Do not use this attribute, as it has been deprecated. The <caption> element should be styled using the CSS properties caption-side and text-align."]
                align ? str {set_align},
            }
        ][
            pub struct HtmlDataElementProps(web_sys::HtmlDataElement) {
                value ? str {set_value},
            }
        ][
            // del ins
            pub struct HtmlModElementProps(web_sys::HtmlModElement) {
                cite ? str {set_cite},
                date_time ? str {"datetime" set_date_time},
            }
        ][
            pub struct HtmlDetailsElementProps(web_sys::HtmlDetailsElement) {
                open ? bool {set_open*},
            }
        ][
            pub struct HtmlDialogElementProps(web_sys::HtmlDialogElement) {
                open ? bool {set_open*},
            }
        ][
            pub struct HtmlEmbedElementProps(web_sys::HtmlEmbedElement) {
                height ? str {set_height},
                src ? str {set_src},
                type_ ? str {set_type},
                width ? str {set_width},
            }
        ][
            pub struct HtmlFieldSetElementProps(web_sys::HtmlFieldSetElement) {
                disabled ? bool {set_disabled*},
                form ? str,
                name ? str {set_name},
            }
        ][
            pub struct HtmlFormElementProps(web_sys::HtmlFormElement) {
                #[deprecated = "This attribute has been deprecated and should not be used. Instead, use the accept attribute on <input type=file> elements."]
                accept ? str,
                accept_charset ? str {"accept-charset" set_accept_charset},
                auto_complete ? str {"autocomplete" set_autocomplete},
                name ? str {set_name},
                rel ? str,
                action ? str {set_action},
                enc_type ? str {"enctype" set_enctype},
                method ? str {set_method},
                no_validate ? bool {"novalidate" set_no_validate*},
                target ? str {set_target},
            }
        ][
            pub struct HtmlHtmlElementProps(web_sys::HtmlHtmlElement) {
                xmlns ? str,
            }
        ][
            pub struct HtmlIFrameElementProps(web_sys::HtmlIFrameElement) {
                allow ? str,
                allow_fullscreen ? bool {"allowfullscreen" set_allow_fullscreen*},
                allow_payment_request ? bool {"allowpaymentrequest" set_allow_payment_request*},
                csp ? str,
                fetch_priority ? str {"fetchpriority"},
                height ? str {set_height},
                loading ? str,
                name ? str {set_name},
                referrer_policy ? str {"referrerpolicy" set_referrer_policy},
                sandbox ? str,
                src ? str {set_src},
                src_doc ? str {"srcdoc" set_srcdoc},
                width ? str {set_width},

                // https://developer.mozilla.org/en-US/docs/Web/HTML/Element/iframe#deprecated_attributes
            }
        ][
            pub struct HtmlImageElementProps(web_sys::HtmlImageElement) {
                alt ? str {set_alt},
                cross_origin ? str {"crossorigin" [update el |v:&_| el.set_cross_origin(Some(v)) ] [remove el || el.set_cross_origin(None)]},
                decoding ? str {set_decoding},
                element_timing ? str {"elementtiming"},
                height ? u32 {set_height*},
                is_map ? bool {"ismap" set_is_map*},
                loading ? str,
                referrer_policy ? str {"referrerpolicy" set_referrer_policy},
                sizes ? str {set_sizes},
                src ? str {set_src},
                srcset ? str {set_srcset},
                width ? u32 {set_width*},
                use_map ? str {"usemap" set_use_map},
            }
        ][
            pub struct HtmlInputElementProps(web_sys::HtmlInputElement) {
                accept ? str {set_accept},
                alt ? str {set_alt},
                auto_complete ? str {"autocomplete" set_autocomplete},
                capture ? str,
                checked ? bool {set_checked*},
                dirname ? str,
                disabled ? bool {set_disabled*},
                form ? str,
                form_action ? str {"formaction" set_form_action},
                form_enc_type ? str {"formenctype" set_form_enctype},
                form_method ? str {"formmethod" set_form_method},
                form_no_validate ? bool {"formnovalidate" set_form_no_validate*},
                form_target ? str {"formtarget" set_form_target},
                height ? u32 {set_height*},
                list ? str,
                max ? str {set_max},
                max_length ? i32 {"maxlength" set_max_length*},
                min ? str {set_min},
                min_length ? i32 {"minlength" set_min_length*},
                multiple ? bool {set_multiple*},
                name ? str {set_name},
                pattern ? str {set_pattern},
                placeholder ? str {set_placeholder},
                read_only ? bool {"readonly" set_read_only*},
                required ? bool {set_required*},
                size ? u32 {set_size*},
                src ? str {set_src},
                step ? str {set_step},
                type_ ? str {"type" set_type},
                value ? str {set_value},
                width ? u32 {set_width*},
            }
        ][
            pub struct HtmlLabelElementProps(web_sys::HtmlLabelElement) {
                html_for ? str {"for" set_html_for},
            }
        ][
            pub struct HtmlLiElementProps(web_sys::HtmlLiElement) {
                value ? i32 {set_value*},
            }
        ][
            pub struct HtmlLinkElementProps(web_sys::HtmlLinkElement) {
                as_ ? str {"as" set_as},
                cross_origin ? str {"crossorigin" [update el |v:&_| el.set_cross_origin(Some(v)) ] [remove el || el.set_cross_origin(None)]},
                fetch_priority ? str {"fetchpriority"},
                href ? str { set_href },
                href_lang ? str { "hreflang" set_hreflang },
                image_sizes ? str {"imagesizes"},
                image_src_set ? str {"imagesrcset"},
                integrity ? str {set_integrity},
                media ? str {set_media},
                prefetch ? str,
                referrer_policy ? str {"referrerpolicy" set_referrer_policy},
                rel ? str { set_rel },
                sizes ? str,
                type_ ? str {"type" set_type},
                blocking ? str,
            }
        ][
            pub struct HtmlMapElementProps(web_sys::HtmlMapElement) {
                name ? str {set_name},
            }
        ][
            pub struct HtmlMetaElementProps(web_sys::HtmlMetaElement) {
                charset ? str,
                content ? str {set_content},
                http_equiv ? str {"http-equiv" set_http_equiv},
                name ? str {set_name},
            }
        ][
            pub struct HtmlMeterElementProps(web_sys::HtmlMeterElement) {
                value ? f64 {set_value*},
                min ? f64 {set_min*},
                max ? f64 {set_max*},
                low ? f64 {set_low*},
                high ? f64 {set_high*},
                optimum ? f64 {set_optimum*},
            }
        ][
            pub struct HtmlObjectElementProps(web_sys::HtmlObjectElement) {
                data ? str {set_data},
                form ? str,
                height ? str {set_height},
                name ? str {set_name},
                type_ ? str {"type" set_type},
                use_map ? str {"usemap" set_use_map},
                width ? str {set_width},
            }
        ][
            pub struct HtmlOListElementProps(web_sys::HtmlOListElement) {
                reversed ? bool {set_reversed*},
                start ? i32 {set_start*},
                type_ ? str {"type" set_type},
            }
        ][
            pub struct HtmlOptGroupElementProps(web_sys::HtmlOptGroupElement) {
                disabled ? bool {set_disabled*},
                label ? str {set_label},
            }
        ][
            pub struct HtmlOptionElementProps(web_sys::HtmlOptionElement) {
                disabled ? bool {set_disabled*},
                label ? str {set_label},
                selected ? bool {set_selected*},
                value ? str {set_value},
            }
        ][
            pub struct HtmlOutputElementProps(web_sys::HtmlOutputElement) {
                html_for ? str {"for"},
                form ? str,
                name ? str {set_name},
            }
        ][
            pub struct HtmlProgressElementProps(web_sys::HtmlProgressElement) {
                max ? f64 {set_max*},
                value ? f64 {set_value*},
            }
        ][
            pub struct HtmlScriptElementProps(web_sys::HtmlScriptElement) {
                r#async ? bool {set_async*},
                cross_origin ? str {"crossorigin" [update el |v:&_| el.set_cross_origin(Some(v)) ] [remove el || el.set_cross_origin(None)]},
                defer ? bool {set_defer*},
                fetch_priority ? str {"fetchpriority"},
                integrity ? str {set_integrity},
                no_module ? bool {"nomodule" set_no_module*},
                referrer_policy ? str {"referrerpolicy"},
                src ? str {set_src},
                type_ ? str {set_type},
                blocking ? str,
            }
        ][
            pub struct HtmlSelectElementProps(web_sys::HtmlSelectElement) {
                auto_complete ? str {"autocomplete" set_autocomplete},
                disabled ? bool {set_disabled*},
                form ? str,
                multiple ? bool {set_multiple*},
                name ? str {set_name},
                required ? bool {set_required*},
                size ? u32 {set_size*},
            }
        ][
            pub struct HtmlSlotElementProps(web_sys::HtmlSlotElement) {
                name ? str {set_name},
            }
        ][
            pub struct HtmlSourceElementProps(web_sys::HtmlSourceElement) {
                type_ ? str {set_type},
                src ? str {set_src},
                srcset ? str {set_srcset},
                sizes ? str {set_sizes},
                media ? str {set_media},
                height ? u32,
                width ? u32,
            }
        ][
            pub struct HtmlStyleElementProps(web_sys::HtmlStyleElement) {
                media ? str {set_media},
                blocking ? str,
                #[deprecated = "This attribute should not be provided: if it is, the only permitted values are the empty string or a case-insensitive match for \"text/css.\""]
                type_ ? str {set_type},
            }
        ][
            pub struct HtmlTableElementProps(web_sys::HtmlTableElement) {
                #[deprecated]
                align ? str {set_align},
                #[deprecated]
                bg_color ? str {"bgcolor" set_bg_color},
                #[deprecated]
                border ? str {set_border},
                #[deprecated]
                cell_padding ? str {"cellpadding" set_cell_padding},
                #[deprecated]
                cell_spacing ? str {"cellspacing" set_cell_spacing},
                #[deprecated]
                frame ? str {set_frame},
                #[deprecated]
                rules ? str {set_rules},
                #[deprecated]
                summary ? str {set_summary},
                #[deprecated]
                width ? str {set_width},
            }
        ][
            // tbody tfoot
            pub struct HtmlTableSectionElementProps(web_sys::HtmlTableSectionElement) {
                #[deprecated]
                align ? str {set_align},
                #[deprecated]
                bg_color ? str {"bgcolor"},
                #[deprecated]
                char ? str {set_ch},
                #[deprecated]
                char_off ? str {"charoff" set_ch_off},
                #[deprecated]
                v_align ? str {"valign" set_v_align},
            } [
                pub struct HtmlTableRowElementProps(web_sys::HtmlTableRowElement) {
                }
            ][
                // col colgroup
                pub struct HtmlTableColElementProps(web_sys::HtmlTableColElement) {
                    span ? u32 {set_span*},
                    #[deprecated]
                    width ? str {set_width},
                }
            ][
                // td th thead
                pub struct HtmlTableCellElementProps(web_sys::HtmlTableCellElement) {
                    col_span ? u32 {"colspan" set_col_span*},
                    headers ? str {set_headers},
                    row_span ? u32 {"rowspan" set_row_span*},
                    #[deprecated = "Do not use this attribute as it is obsolete in the latest standard. Alternatively, you can put the abbreviated description inside the cell and place the long content in the title attribute."]
                    abbr ? str,
                    #[deprecated]
                    axis ? str {set_axis},
                    #[deprecated = "Use the CSS height property instead."]
                    height ? str {set_height},
                    #[deprecated]
                    scope ? str,
                    #[deprecated]
                    width ? str {set_width},
                }
            ]
        ][
            pub struct HtmlTextAreaElementProps(web_sys::HtmlTextAreaElement) {
                auto_complete ? str {"autocomplete" set_autocomplete},
                auto_correct ? str,
                cols ? u32 {set_cols*},
                disabled ? bool {set_disabled*},
                form ? str,
                max_length ? i32 {"maxlength" set_max_length*},
                min_length ? i32 {"minlength" set_min_length*},
                name ? str {set_name},
                placeholder ? str {set_placeholder},
                read_only ? bool {"readonly" set_read_only*},
                required ? bool {set_required*},
                rows ? u32 {set_rows*},
                wrap ? str {set_wrap},
            }
        ][
            pub struct HtmlTimeElementProps(web_sys::HtmlTimeElement) {
                date_time ? str {"datetime" set_date_time},
            }
        ][
            pub struct HtmlTrackElementProps(web_sys::HtmlTrackElement) {
                default ? bool {set_default*},
                kind ? str {set_kind},
                label ? str {set_label},
                src ? str {set_src},
                src_lang ? str {"srclang" set_srclang},
            }
        ][
            pub struct HtmlUListElementProps(web_sys::HtmlUListElement) {
            }
        ]
    ]
}

fn tests() {
    ElementProps().class("");
    HtmlElementProps()
        .id("id")
        .access_key("")
        .class("adfas afs")
        .context_menu("a");
}

pub trait UpdateHtmlElementEventListeners {
    type State;
    fn update_dom_event_listeners(self, element: &web_sys::HtmlElement, state: &mut Self::State);
}
