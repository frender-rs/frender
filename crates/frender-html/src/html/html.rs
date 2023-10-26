super::macros::def_intrinsic_component_props!(
    #[root_path]
    use crate::imports;

    mod items {
        #[behaviors]
        pub mod behaviors {}

        #[attributes]
        pub mod attributes {}

        #[behavior_type_traits]
        pub mod behavior_type_traits {}

        #[tags]
        pub mod tags {}

        #[RenderHtml]
        pub trait RenderHtml {}
    }

    pub trait Node {
        verbatim_trait_items!(
            fn cursor_is_at_self(&self, renderer: &Renderer) -> bool;

            fn move_cursor_after_self(&mut self, renderer: &mut Renderer);

            /// should move cursor
            fn readd_self(&mut self, renderer: &mut Renderer, force_reposition: bool);

            fn remove_self(&mut self, renderer: &mut Renderer);
        );

        impl_for_web!(
            verbatim_trait_items!(
                fn cursor_is_at_self(&self, renderer: &Renderer) -> bool {
                    renderer.cursor_is_at_node(self.0.as_ref())
                }

                fn move_cursor_after_self(&mut self, renderer: &mut Renderer) {
                    renderer.move_cursor_after_node(self.0.as_ref())
                }

                fn readd_self(&mut self, renderer: &mut Renderer, force_reposition: bool) {
                    renderer.readd_node(self.0.as_ref(), force_reposition)
                }

                fn remove_self(&mut self, renderer: &mut Renderer) {
                    renderer.remove_node(self.0.as_ref())
                }
            );
        );

        sub_traits!(
            pub trait Element {
                define!(Props: ElementProps,);

                verbatim_trait_items!(
                    fn move_cursor_at_the_first_child_of_self(&mut self, renderer: &mut Renderer);

                    fn set_attribute(&mut self, renderer: &mut Renderer, name: &str, value: &str);
                    fn remove_attribute(&mut self, renderer: &mut Renderer, name: &str);

                    type ClassList<'a>: frender_html_common::dom_token::DomTokenList
                    where
                        Self: 'a,
                        Renderer: 'a;
                    fn class_list<'a>(&'a mut self, renderer: &'a mut Renderer) -> Self::ClassList<'a>;
                );

                impl_for_web!(
                    verbatim_trait_items!(
                        fn move_cursor_at_the_first_child_of_self(&mut self, renderer: &mut Renderer) {
                            renderer.move_cursor_at_the_first_child_of_element(self.0.as_ref())
                        }

                        fn set_attribute(&mut self, renderer: &mut Renderer, name: &str, value: &str) {
                            use frender_common::try_behavior::TryWithTryBehavior;

                            AsRef::<web_sys::Element>::as_ref(&self.0).set_attribute(name, value).unwrap_with_behavior(&mut renderer.try_behavior())
                        }

                        fn remove_attribute(&mut self, renderer: &mut Renderer, name: &str) {
                            use frender_common::try_behavior::TryWithTryBehavior;

                            AsRef::<web_sys::Element>::as_ref(&self.0).remove_attribute(name).unwrap_with_behavior(&mut renderer.try_behavior())
                        }

                        type ClassList<'a> = crate::csr::web::DomTokenList<Renderer::TryBehavior<'a>>
                        where
                            Self: 'a,
                            Renderer: 'a;

                        fn class_list<'a>(&'a mut self, renderer: &'a mut Renderer) -> Self::ClassList<'a> {
                            let element: &web_sys::Element = self.0.as_ref();
                            crate::csr::web::DomTokenList(element.class_list(), renderer.try_behavior())
                        }
                    );
                );

                fn children(value: children![]);
                fn css(value: bounds![Css]);
                fn class(
                    value: bounds![
                        DomTokens,
                        csr {
                            get_mut_dom_token_list: frender_html::renderer::node_behaviors::Element::class_list,
                        }
                    ],
                );

                fn id(value: maybe![&str]) {
                    update_with!(set_id);
                }
                fn part(value: maybe![&str]);
                /// Event [`cancel`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/cancel_event)
                ///
                /// Fires on a [`<dialog>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dialog) when the user instructs the browser that they wish to dismiss the currently open modal dialog. The browser fires this event when the user presses the <kbd>Esc</kbd> key to close the modal dialog.
                fn on_cancel(value: event![Event, "cancel", OnCancelEvent, OnCancelEventListener]);
                /// Event [`error`](https://developer.mozilla.org/en-US/docs/Web/API/Element/error_event)
                ///
                /// Fired when a resource failed to load, or can't be used. For example, if a script has an execution error or an image can't be found or is invalid.
                fn on_error(value: event![Event, "error", OnErrorEvent, OnErrorEventListener]);
                /// Event [`scroll`](https://developer.mozilla.org/en-US/docs/Web/API/Element/scroll_event)
                ///
                /// Fired when the document view or an element has been scrolled.
                fn on_scroll(value: event![Event, "scroll", OnScrollEvent, OnScrollEventListener]);
                /// Event [`securitypolicyviolation`](https://developer.mozilla.org/en-US/docs/Web/API/Element/securitypolicyviolation_event)
                ///
                /// Fired when a [Content Security Policy](https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP) is violated.
                fn on_security_policy_violation(value: event![SecurityPolicyViolationEvent, "securitypolicyviolation", OnSecurityPolicyViolationEvent, OnSecurityPolicyViolationEventListener]);
                /// Event [`select`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/select_event)
                ///
                /// Fired when some text has been selected.
                fn on_select(value: event![Event, "select", OnSelectEvent, OnSelectEventListener]);
                /// Event [`wheel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/wheel_event)
                ///
                /// Fired when the user rotates a wheel button on a pointing device (typically a mouse).
                fn on_wheel(value: event![WheelEvent, "wheel", OnWheelEvent, OnWheelEventListener]);

                // Clipboard events
                /// Event [`copy`](https://developer.mozilla.org/en-US/docs/Web/API/Element/copy_event)
                ///
                /// Fired when the user initiates a copy action through the browser's user interface.
                fn on_copy(value: event![Event, "copy", OnCopyEvent, OnCopyEventListener]);
                /// Event [`cut`](https://developer.mozilla.org/en-US/docs/Web/API/Element/cut_event)
                ///
                /// Fired when the user initiates a cut action through the browser's user interface.
                fn on_cut(value: event![Event, "cut", OnCutEvent, OnCutEventListener]);
                /// Event [`paste`](https://developer.mozilla.org/en-US/docs/Web/API/Element/paste_event)
                ///
                /// Fired when the user initiates a paste action through the browser's user interface.
                fn on_paste(value: event![Event, "paste", OnPasteEvent, OnPasteEventListener]);

                // Composition events
                /// Event [`compositionend`](https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionend_event)
                ///
                /// Fired when a text composition system such as an [input method editor](https://developer.mozilla.org/en-US/docs/Glossary/Input_method_editor) completes or cancels the current composition session.
                fn on_composition_end(value: event![CompositionEvent, "compositionend", OnCompositionEndEvent, OnCompositionEndEventListener]);
                /// Event [`compositionstart`](https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionstart_event)
                ///
                /// Fired when a text composition system such as an [input method editor](https://developer.mozilla.org/en-US/docs/Glossary/Input_method_editor) starts a new composition session.
                fn on_composition_start(value: event![CompositionEvent, "compositionstart", OnCompositionStartEvent, OnCompositionStartEventListener]);
                /// Event [`compositionupdate`](https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionupdate_event)
                ///
                /// Fired when a new character is received in the context of a text composition session controlled by a text composition system such as an [input method editor](https://developer.mozilla.org/en-US/docs/Glossary/Input_method_editor).
                fn on_composition_update(value: event![CompositionEvent, "compositionupdate", OnCompositionUpdateEvent, OnCompositionUpdateEventListener]);

                // Focus events
                /// Event [`blur`](https://developer.mozilla.org/en-US/docs/Web/API/Element/blur_event)
                ///
                /// Fired when an element has lost focus.
                fn on_blur(value: event![FocusEvent, "blur", OnBlurEvent, OnBlurEventListener]);
                /// Event [`focus`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focus_event)
                ///
                /// Fired when an element has gained focus.
                fn on_focus(value: event![FocusEvent, "focus", OnFocusEvent, OnFocusEventListener]);
                /// Event [`focusin`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focusin_event)
                ///
                /// Fired when an element has gained focus, after [`focus`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focus_event).
                fn on_focus_in(value: event![FocusEvent, "focusin", OnFocusInEvent, OnFocusInEventListener]);
                /// Event [`focusout`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focusout_event)
                ///
                /// Fired when an element has lost focus, after [`blur`](https://developer.mozilla.org/en-US/docs/Web/API/Element/blur_event).
                fn on_focus_out(value: event![FocusEvent, "focusout", OnFocusOutEvent, OnFocusOutEventListener]);

                // Fullscreen events
                /// Event [`fullscreenchange`](https://developer.mozilla.org/en-US/docs/Web/API/Element/fullscreenchange_event)
                ///
                /// Sent to an [`Element`](https://developer.mozilla.org/en-US/docs/Web/API/Element) when it transitions into or out of [fullscreen](https://developer.mozilla.org/en-US/docs/Web/API/Fullscreen_API/Guide) mode.
                fn on_fullscreen_change(value: event![Event, "fullscreenchange", OnFullscreenChangeEvent, OnFullscreenChangeEventListener]);
                /// Event [`fullscreenerror`](https://developer.mozilla.org/en-US/docs/Web/API/Element/fullscreenerror_event)
                ///
                /// Sent to an `Element` if an error occurs while attempting to switch it into or out of [fullscreen](https://developer.mozilla.org/en-US/docs/Web/API/Fullscreen_API/Guide) mode.
                fn on_fullscreen_error(value: event![Event, "fullscreenerror", OnFullscreenErrorEvent, OnFullscreenErrorEventListener]);

                // Keyboard events
                /// Event [`keydown`](https://developer.mozilla.org/en-US/docs/Web/API/Element/keydown_event)
                ///
                /// Fired when a key is pressed.
                fn on_key_down(value: event![KeyboardEvent, "keydown", OnKeyDownEvent, OnKeyDownEventListener]);
                /// Event [`keyup`](https://developer.mozilla.org/en-US/docs/Web/API/Element/keyup_event)
                ///
                /// Fired when a key is released.
                fn on_key_up(value: event![KeyboardEvent, "keyup", OnKeyUpEvent, OnKeyUpEventListener]);

                // Mouse events
                /// Event [`auxclick`](https://developer.mozilla.org/en-US/docs/Web/API/Element/auxclick_event)
                ///
                /// Fired when a non-primary pointing device button (e.g., any mouse button other than the left button) has been pressed and released on an element.
                fn on_aux_click(value: event![MouseEvent, "auxclick", OnAuxClickEvent, OnAuxClickEventListener]);
                /// Event [`click`](https://developer.mozilla.org/en-US/docs/Web/API/Element/click_event)
                ///
                /// Fired when a pointing device button (e.g., a mouse's primary button) is pressed and released on a single element.
                fn on_click(value: event![MouseEvent, "click", OnClickEvent, OnClickEventListener]);
                /// Event [`contextmenu`](https://developer.mozilla.org/en-US/docs/Web/API/Element/contextmenu_event)
                ///
                /// Fired when the user attempts to open a context menu.
                fn on_context_menu(value: event![MouseEvent, "contextmenu", OnContextMenuEvent, OnContextMenuEventListener]);
                /// Event [`dblclick`](https://developer.mozilla.org/en-US/docs/Web/API/Element/dblclick_event)
                ///
                /// Fired when a pointing device button (e.g., a mouse's primary button) is clicked twice on a single element.
                fn on_double_click(value: event![MouseEvent, "dblclick", OnDoubleClickEvent, OnDoubleClickEventListener]);
                /// Event [`mousedown`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mousedown_event)
                ///
                /// Fired when a pointing device button is pressed on an element.
                fn on_mouse_down(value: event![MouseEvent, "mousedown", OnMouseDownEvent, OnMouseDownEventListener]);
                /// Event [`mouseenter`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseenter_event)
                ///
                /// Fired when a pointing device (usually a mouse) is moved over the element that has the listener attached.
                fn on_mouse_enter(value: event![MouseEvent, "mouseenter", OnMouseEnterEvent, OnMouseEnterEventListener]);
                /// Event [`mouseleave`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseleave_event)
                ///
                /// Fired when the pointer of a pointing device (usually a mouse) is moved out of an element that has the listener attached to it.
                fn on_mouse_leave(value: event![MouseEvent, "mouseleave", OnMouseLeaveEvent, OnMouseLeaveEventListener]);
                /// Event [`mousemove`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mousemove_event)
                ///
                /// Fired when a pointing device (usually a mouse) is moved while over an element.
                fn on_mouse_move(value: event![MouseEvent, "mousemove", OnMouseMoveEvent, OnMouseMoveEventListener]);
                /// Event [`mouseout`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseout_event)
                ///
                /// Fired when a pointing device (usually a mouse) is moved off the element to which the listener is attached or off one of its children.
                fn on_mouse_out(value: event![MouseEvent, "mouseout", OnMouseOutEvent, OnMouseOutEventListener]);
                /// Event [`mouseover`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseover_event)
                ///
                /// Fired when a pointing device is moved onto the element to which the listener is attached or onto one of its children.
                fn on_mouse_over(value: event![MouseEvent, "mouseover", OnMouseOverEvent, OnMouseOverEventListener]);
                /// Event [`mouseup`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseup_event)
                ///
                /// Fired when a pointing device button is released on an element.
                fn on_mouse_up(value: event![MouseEvent, "mouseup", OnMouseUpEvent, OnMouseUpEventListener]);

                // Touch events
                /// Event [`touchcancel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchcancel_event)
                ///
                /// Fired when one or more touch points have been disrupted in an implementation-specific manner (for example, too many touch points are created).
                fn on_touch_cancel(value: event![TouchEvent, "touchcancel", OnTouchCancelEvent, OnTouchCancelEventListener]);
                /// Event [`touchend`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchend_event)
                ///
                /// Fired when one or more touch points are removed from the touch surface.
                fn on_touch_end(value: event![TouchEvent, "touchend", OnTouchEndEvent, OnTouchEndEventListener]);
                /// Event [`touchmove`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchmove_event)
                ///
                /// Fired when one or more touch points are moved along the touch surface.
                fn on_touch_move(value: event![TouchEvent, "touchmove", OnTouchMoveEvent, OnTouchMoveEventListener]);
                /// Event [`touchstart`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchstart_event)
                ///
                /// Fired when one or more touch points are placed on the touch surface.
                fn on_touch_start(value: event![TouchEvent, "touchstart", OnTouchStartEvent, OnTouchStartEventListener]);

                sub_traits!(
                    pub trait ElementWithHrefAttribute {
                        fn href(value: maybe![&str]) {
                            update_with!(set_href);
                        }
                    }

                    pub trait ElementWithTargetAttribute {
                        fn target(value: maybe![&str]) {
                            update_with!(set_target);
                        }
                    }

                    pub trait ElementWithTypeAttribute {
                        fn r#type(value: maybe![&str]) {
                            alias!(type_);
                            attr_name!("type");
                            update_with!(set_type);
                        }
                    }

                    pub trait ElementWithCiteAttribute {
                        fn cite(value: maybe![&str]) {
                            update_with!(set_cite);
                        }
                    }

                    pub trait ElementWithPlaceHolderAttribute {
                        fn placeholder(value: maybe![&str]) {
                            update_with!(set_placeholder);
                        }
                    }

                    pub trait ElementWithMaxMinLengthAttributes {
                        fn max_length(value: maybe![i32]) {
                            attr_name!("maxlength");
                            update_with!(set_max_length);
                        }
                        fn min_length(value: maybe![i32]) {
                            attr_name!("minlength");
                            update_with!(set_min_length);
                        }
                    }

                    pub trait ElementWithHeightWidthStrAttributes {
                        fn height(value: maybe![&str]) {
                            update_with!(set_height);
                        }
                        fn width(value: maybe![&str]) {
                            update_with!(set_width);
                        }
                    }
                    pub trait ElementWithHeightWidthU32Attributes {
                        fn height(value: maybe![u32]) {
                            update_with!(set_height);
                        }
                        fn width(value: maybe![u32]) {
                            update_with!(set_width);
                        }
                    }
                    pub trait ElementWithMaxF64Attribute {
                        fn max(value: maybe![f64]) {
                            update_with!(set_max);
                        }
                    }
                    pub trait ElementWithValueF64Attribute {
                        fn value(value: maybe![f64]) {
                            update_with!(set_value);
                        }
                    }
                    pub trait ElementWithValueStrAttribute {
                        fn value(value: maybe![&str]) {
                            update_with!(set_value);
                        }
                    }
                    pub trait ElementWithOpenAttribute {
                        fn open(value: maybe![bool]) {
                            update_with!(set_open);
                        }
                    }
                    pub trait ElementWithNameAttribute {
                        fn name(value: maybe![&str]) {
                            update_with!(set_name);
                        }
                    }
                    pub trait ElementWithDisabledAttribute {
                        fn disabled(value: maybe![bool]) {
                            update_with!(set_disabled);
                        }
                    }
                    pub trait ElementWithCrossOriginAttribute {
                        fn cross_origin(value: maybe![&str]) {
                            attr_name!("crossorigin");
                            update_with!(
                                set_cross_origin,
                                custom_type!(Option<&str>),
                                impl_with!(
                                    update = |element, renderer| element.set_cross_origin(renderer, Some(value)),
                                    remove = |element, renderer| element.set_cross_origin(renderer, None)
                                ),
                            );
                        }
                    }
                    pub trait ElementWithRelAttribute {
                        fn rel(
                            value: bounds![
                                DomTokens,
                                csr {
                                    get_mut_dom_token_list: frender_html::renderer::node_behaviors::HtmlElementWithRelList::rel_list,
                                }
                            ],
                        );
                    }
                    pub trait ElementWithReferrerPolicyAttribute {
                        fn referrer_policy(value: maybe![&str]) {
                            attr_name!("referrerpolicy");
                            update_with!(set_referrer_policy);
                        }
                    }
                    pub trait ElementWithAltAttribute {
                        fn alt(value: maybe![&str]) {
                            update_with!(set_alt);
                        }
                    }
                    pub trait ElementWithLoadingAttribute {
                        fn loading(value: maybe![&str]);
                    }
                    pub trait ElementWithAcceptAttribute {
                        fn accept(value: maybe![&str]) {
                            update_with!(set_accept);
                        }
                    }
                    pub trait ElementWithAutoCompleteAttribute {
                        fn auto_complete(value: maybe![&str]) {
                            attr_name!("autocomplete");
                            update_with!(set_auto_complete);
                        }
                    }
                    pub trait ElementWithFormAttribute {
                        fn form(value: maybe![&str]);
                    }
                    pub trait ElementWithFormAttributes {
                        special_super_traits!(ElementWithFormAttribute);

                        fn form_action(value: maybe![&str]) {
                            attr_name!("formaction");
                            update_with!(set_form_action);
                        }
                        fn form_enc_type(value: maybe![&str]) {
                            attr_name!("formenctype");
                            update_with!(set_form_enctype);
                        }
                        fn form_method(value: maybe![&str]) {
                            attr_name!("formmethod");
                            update_with!(set_form_method);
                        }
                        fn form_no_validate(value: maybe![bool]) {
                            attr_name!("formnovalidate");
                            update_with!(set_form_no_validate);
                        }
                        fn form_target(value: maybe![&str]) {
                            attr_name!("formtarget");
                            update_with!(set_form_target);
                        }
                    }
                    pub trait ElementWithFetchPriorityAttribute {
                        fn fetch_priority(value: maybe![&str]) {
                            attr_name!("fetchpriority");
                        }
                    }
                    pub trait ElementWithHrefLangAttribute {
                        special_super_traits!(ElementWithHrefAttribute);

                        fn href_lang(value: maybe![&str]) {
                            attr_name!("hreflang");
                            update_with!(set_href_lang, web_sys_name = set_hreflang);
                        }
                    }

                    pub trait ElementWithSizesAttribute {
                        fn sizes(value: maybe![&str]) {
                            update_with!(set_sizes);
                        }
                    }

                    pub trait ElementWithUseMapAttribute {
                        fn use_map(value: maybe![&str]) {
                            attr_name!("usemap");
                            update_with!(set_use_map);
                        }
                    }
                    pub trait ElementWithLabelAttribute {
                        fn label(value: maybe![&str]) {
                            update_with!(set_label);
                        }
                    }

                    pub trait ElementWithForAttribute {
                        fn r#for(value: maybe![&str]) {
                            alias!(html_for);
                            attr_name!("for");
                            update_with!(set_html_for);
                        }
                    }
                    pub trait ElementWithIntegrityAttribute {
                        fn integrity(value: maybe![&str]) {
                            update_with!(set_integrity);
                        }
                    }
                    pub trait ElementWithBlockingAttribute {
                        fn blocking(value: maybe![&str]);
                    }

                    pub trait ElementWithMultipleAttribute {
                        fn multiple(value: maybe![bool]) {
                            update_with!(set_multiple);
                        }
                    }
                    pub trait ElementWithRequiredAttribute {
                        fn required(value: maybe![bool]) {
                            update_with!(set_required);
                        }
                    }

                    pub trait ElementWithSizeU32Attribute {
                        fn size(value: maybe![u32]) {
                            update_with!(set_size);
                        }
                    }

                    pub trait ElementWithSrcAttribute {
                        fn src(value: maybe![&str]) {
                            update_with!(set_src);
                        }
                    }

                    pub trait ElementWithSrcsetAttribute {
                        special_super_traits!(ElementWithSrcAttribute);
                    }

                    pub trait ElementWithBgColorAttribute {
                        fn bg_color(value: maybe![&str]) {
                            attr_name!("bgcolor");
                            update_with!(set_bg_color);
                        }
                    }

                    pub trait ElementWithAlignAttribute {
                        fn align(value: maybe![&str]) {
                            update_with!(set_align);
                        }
                    }

                    pub trait ElementWithMediaAttribute {
                        fn media(value: maybe![&str]) {
                            update_with!(set_media);
                        }
                    }

                    pub trait ElementWithReadOnlyAttribute {
                        fn read_only(value: maybe![bool]) {
                            attr_name!("readonly");
                            update_with!(set_read_only);
                        }
                    }

                    pub trait ElementWithDateTimeAttribute {
                        fn date_time(value: maybe![&str]) {
                            attr_name!("datetime");
                            update_with!(set_date_time);
                        }
                    }

                    pub trait HtmlElement {
                        define!(
                            Props: HtmlElementProps,
                            tags: (
                                abbr,
                                address,
                                article,
                                aside,
                                b,
                                bdi,
                                bdo,
                                cite,
                                code,
                                datalist, // HTMLDataListElement
                                dd,
                                dfn,
                                div, // HTMLDivElement
                                dl,  // HTMLDListElement
                                dt,
                                em,
                                figcaption,
                                figure,
                                footer,
                                h1,
                                h2,
                                h3,
                                h4,
                                h5,
                                h6,   // HTMLHeadingElement
                                head, // HTMLHeadElement
                                header,
                                hgroup,
                                hr, // HTMLHRElement
                                i,
                                kbd,
                                legend, // HTMLLegendElement
                                main,
                                mark,
                                menu, // HTMLMenuElement
                                nav,
                                noscript,
                                p,       // HTMLParagraphElement
                                picture, // HTMLPictureElement
                                pre,     // HTMLPreElement with non-standard attributes
                                rp,
                                rt,
                                ruby,
                                s,
                                samp,
                                section,
                                small,
                                span, // HTMLSpanElement
                                strong,
                                sub,
                                summary,
                                sup,
                                template, // HTMLTemplateElement
                                title,    // HTMLTitleElement
                                u,
                                var,
                                wbr,
                            ),
                        );

                        impl_for_web!();

                        fn access_key(value: maybe![&str]) {
                            attr_name!("accesskey");
                            update_with!(set_access_key);
                        }
                        fn auto_capitalize(value: maybe![&str]) {
                            attr_name!("autocapitalize");
                        }
                        fn auto_focus(value: maybe![bool]) {
                            attr_name!("autofocus");
                        }
                        fn content_editable(
                            value: bounds![
                                MaybeContentEditable,
                                attr_name = "contenteditable",
                                csr {
                                    update: |el: &mut _, renderer: &mut _, _, v: &_| { frender_html::renderer::node_behaviors::HtmlElement::set_content_editable(el, renderer, v,) },
                                    remove: MaybeContentEditable::csr::default_remove,
                                },
                            ],
                        );
                        #[deprecated = "See https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/contextMenu"]
                        fn context_menu(value: maybe![&str]) {
                            attr_name!("contextmenu");
                        }
                        fn dir(value: maybe![&str]) {
                            update_with!(set_dir);
                        }
                        fn draggable(value: maybe![bool]) {
                            update_with!(set_draggable);
                        }
                        fn enter_key_hint(value: maybe![&str]) {
                            attr_name!("enterkeyhint");
                        }
                        fn hidden(value: maybe![bool]) {
                            // TODO: "until-found"
                            update_with!(set_hidden);
                        }
                        fn inert(value: maybe![bool]);
                        fn input_mode(value: maybe![&str]) {
                            attr_name!("inputmode");
                        }
                        fn is(value: maybe![&str]);

                        fn item_id(value: maybe![&str]) {
                            attr_name!("itemid");
                        }
                        fn item_prop(value: maybe![&str]) {
                            attr_name!("itemprop");
                        }
                        fn item_ref(value: maybe![&str]) {
                            attr_name!("itemref");
                        }
                        fn item_scope(value: maybe![&str]) {
                            attr_name!("itemscope");
                        }
                        fn item_type(value: maybe![&str]) {
                            attr_name!("itemtype");
                        }

                        fn lang(value: maybe![&str]) {
                            update_with!(set_lang);
                        }
                        fn nonce(value: maybe![&str]);
                        fn role(value: maybe![&str]);
                        fn slot(value: maybe![&str]);
                        fn spellcheck(value: maybe![bool]) {
                            update_with!(set_spellcheck);
                        }
                        fn style(value: maybe![&str]);
                        fn tab_index(value: maybe![i32]) {
                            attr_name!("tabindex");
                            update_with!(set_tab_index);
                        }
                        fn title(value: maybe![&str]) {
                            update_with!(set_title);
                        }
                        fn translate(value: maybe![&str]);
                        fn virtual_keyboard_policy(value: maybe![&str]) {
                            attr_name!("virtualkeyboardpolicy");
                        }

                        // TODO: aria-*
                        // TODO: data-*

                        /// Event [`invalid`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/invalid_event)
                        ///
                        /// Fired when an element does not satisfy its constraints during constraint validation.
                        fn on_invalid(value: event![Event, "invalid", OnInvalidEvent, OnInvalidEventListener]);

                        // Animation events
                        /// Event [`animationcancel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animationcancel_event)
                        ///
                        /// Fired when an animation unexpectedly aborts.
                        fn on_animation_cancel(value: event![AnimationEvent, "animationcancel", OnAnimationCancelEvent, OnAnimationCancelEventListener]);
                        /// Event [`animationend`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animationend_event)
                        ///
                        /// Fired when an animation has completed normally.
                        fn on_animation_end(value: event![AnimationEvent, "animationend", OnAnimationEndEvent, OnAnimationEndEventListener]);
                        /// Event [`animationiteration`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animationiteration_event)
                        ///
                        /// Fired when an animation iteration has completed.
                        fn on_animation_iteration(value: event![AnimationEvent, "animationiteration", OnAnimationIterationEvent, OnAnimationIterationEventListener]);
                        /// Event [`animationstart`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animationstart_event)
                        ///
                        /// Fired when an animation starts.
                        fn on_animation_start(value: event![AnimationEvent, "animationstart", OnAnimationStartEvent, OnAnimationStartEventListener]);

                        // Input events
                        /// Event [`beforeinput`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/beforeinput_event)
                        ///
                        /// Fired when the value of an [`<input>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input), [`<select>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select), or [`<textarea>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea) element is about to be modified.
                        fn on_before_input(value: event![InputEvent, "beforeinput", OnBeforeInputEvent, OnBeforeInputEventListener]);
                        /// Event [`input`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/input_event)
                        ///
                        /// Fired when the `value` of an [`<input>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input), [`<select>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select), or [`<textarea>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea) element has been changed.
                        fn on_input(value: event![Event, "input", OnInputEvent, OnInputEventListener]);
                        /// Event [`change`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/change_event)
                        ///
                        /// Fired when the `value` of an [`<input>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input), [`<select>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select), or [`<textarea>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea) element has been changed and committed by the user. Unlike the [`input`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/input_event) event, the `change` event is not necessarily fired for each alteration to an element's `value`.
                        fn on_change(value: event![Event, "change", OnChangeEvent, OnChangeEventListener]);

                        // Pointer events
                        /// Event [`gotpointercapture`](https://developer.mozilla.org/en-US/docs/Web/API/Element/gotpointercapture_event)
                        ///
                        /// Fired when an element captures a pointer using [`setPointerCapture()`](https://developer.mozilla.org/en-US/docs/Web/API/Element/setPointerCapture).
                        fn on_got_pointer_capture(value: event![PointerEvent, "gotpointercapture", OnGotPointerCaptureEvent, OnGotPointerCaptureEventListener]);
                        /// Event [`lostpointercapture`](https://developer.mozilla.org/en-US/docs/Web/API/Element/lostpointercapture_event)
                        ///
                        /// Fired when a [captured pointer](https://developer.mozilla.org/en-US/docs/Web/API/Pointer_events#pointer_capture) is released.
                        fn on_lost_pointer_capture(value: event![PointerEvent, "lostpointercapture", OnLostPointerCaptureEvent, OnLostPointerCaptureEventListener]);
                        /// Event [`pointercancel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointercancel_event)
                        ///
                        /// Fired when a pointer event is canceled.
                        fn on_pointer_cancel(value: event![PointerEvent, "pointercancel", OnPointerCancelEvent, OnPointerCancelEventListener]);
                        /// Event [`pointerdown`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerdown_event)
                        ///
                        /// Fired when a pointer becomes active.
                        fn on_pointer_down(value: event![PointerEvent, "pointerdown", OnPointerDownEvent, OnPointerDownEventListener]);
                        /// Event [`pointerenter`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerenter_event)
                        ///
                        /// Fired when a pointer is moved into the hit test boundaries of an element or one of its descendants.
                        fn on_pointer_enter(value: event![PointerEvent, "pointerenter", OnPointerEnterEvent, OnPointerEnterEventListener]);
                        /// Event [`pointerleave`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerleave_event)
                        ///
                        /// Fired when a pointer is moved out of the hit test boundaries of an element.
                        fn on_pointer_leave(value: event![PointerEvent, "pointerleave", OnPointerLeaveEvent, OnPointerLeaveEventListener]);
                        /// Event [`pointermove`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointermove_event)
                        ///
                        /// Fired when a pointer changes coordinates.
                        fn on_pointer_move(value: event![PointerEvent, "pointermove", OnPointerMoveEvent, OnPointerMoveEventListener]);
                        /// Event [`pointerout`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerout_event)
                        ///
                        /// Fired when a pointer is moved out of the *hit test* boundaries of an element (among other reasons).
                        fn on_pointer_out(value: event![PointerEvent, "pointerout", OnPointerOutEvent, OnPointerOutEventListener]);
                        /// Event [`pointerover`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerover_event)
                        ///
                        /// Fired when a pointer is moved into an element's hit test boundaries.
                        fn on_pointer_over(value: event![PointerEvent, "pointerover", OnPointerOverEvent, OnPointerOverEventListener]);
                        /// Event [`pointerup`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerup_event)
                        ///
                        /// Fired when a pointer is no longer active.
                        fn on_pointer_up(value: event![PointerEvent, "pointerup", OnPointerUpEvent, OnPointerUpEventListener]);

                        // Transition events
                        /// Event [`transitioncancel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/transitioncancel_event)
                        ///
                        /// Fired when a [CSS transition](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Transitions/Using_CSS_transitions) is canceled.
                        fn on_transition_cancel(value: event![TransitionEvent, "transitioncancel", OnTransitionCancelEvent, OnTransitionCancelEventListener]);
                        /// Event [`transitionend`](https://developer.mozilla.org/en-US/docs/Web/API/Element/transitionend_event)
                        ///
                        /// Fired when a [CSS transition](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Transitions/Using_CSS_transitions) has completed.
                        fn on_transition_end(value: event![TransitionEvent, "transitionend", OnTransitionEndEvent, OnTransitionEndEventListener]);
                        /// Event [`transitionrun`](https://developer.mozilla.org/en-US/docs/Web/API/Element/transitionrun_event)
                        ///
                        /// Fired when a [CSS transition](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Transitions/Using_CSS_transitions) is first created.
                        fn on_transition_run(value: event![TransitionEvent, "transitionrun", OnTransitionRunEvent, OnTransitionRunEventListener]);
                        /// Event [`transitionstart`](https://developer.mozilla.org/en-US/docs/Web/API/Element/transitionstart_event)
                        ///
                        /// Fired when a [CSS transition](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Transitions/Using_CSS_transitions) has actually started.
                        fn on_transition_start(value: event![TransitionEvent, "transitionstart", OnTransitionStartEvent, OnTransitionStartEventListener]);

                        /// Event [`drag`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/drag_event)
                        ///
                        /// This event is fired when an element or text selection is being dragged.
                        fn on_drag(value: event![Event, "drag", OnDragEvent, OnDragEventListener]);
                        /// Event [`dragend`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragend_event)
                        ///
                        /// This event is fired when a drag operation is being ended (by releasing a mouse button or hitting the escape key).
                        fn on_drag_end(value: event![Event, "dragend", OnDragEndEvent, OnDragEndEventListener]);
                        /// Event [`dragenter`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragenter_event)
                        ///
                        /// This event is fired when a dragged element or text selection enters a valid drop target.
                        fn on_drag_enter(value: event![Event, "dragenter", OnDragEnterEvent, OnDragEnterEventListener]);
                        /// Event [`dragleave`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragleave_event)
                        ///
                        /// This event is fired when a dragged element or text selection leaves a valid drop target.
                        fn on_drag_leave(value: event![Event, "dragleave", OnDragLeaveEvent, OnDragLeaveEventListener]);
                        /// Event [`dragover`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragover_event)
                        ///
                        /// This event is fired continuously when an element or text selection is being dragged and the mouse pointer is over a valid drop target (every 50 ms WHEN mouse is not moving ELSE much faster between 5 ms (slow movement) and 1ms (fast movement) approximately. This firing pattern is different than [`mouseover`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseover_event) ).
                        fn on_drag_over(value: event![Event, "dragover", OnDragOverEvent, OnDragOverEventListener]);
                        /// Event [`dragstart`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragstart_event)
                        ///
                        /// This event is fired when the user starts dragging an element or text selection.
                        fn on_drag_start(value: event![Event, "dragstart", OnDragStartEvent, OnDragStartEventListener]);
                        /// Event [`drop`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/drop_event)
                        ///
                        /// This event is fired when an element or text selection is dropped on a valid drop target.
                        fn on_drop(value: event![Event, "drop", OnDropEvent, OnDropEventListener]);

                        sub_traits!(
                            pub trait HtmlElementWithHref {
                                special_super_traits!(ElementWithHrefAttribute, ElementWithTargetAttribute, ElementWithReferrerPolicyAttribute, ElementWithRelAttribute,);

                                impl_for_web!(only_for_types!(web_sys::HtmlAnchorElement, web_sys::HtmlAreaElement););

                                fn download(value: maybe![&str]) {
                                    update_with!(set_download);
                                }
                                fn ping(value: maybe![&str]) {
                                    update_with!(set_ping);
                                }
                            }

                            pub trait HtmlAnchorElement {
                                special_super_traits!(HtmlElementWithHref, ElementWithTypeAttribute, ElementWithHrefLangAttribute);
                                special_inter_traits!(ElementWithHrefAttribute, ElementWithTargetAttribute, ElementWithReferrerPolicyAttribute, ElementWithRelAttribute);
                                define!(Props: HtmlAnchorElementProps, tags: (a,));
                                impl_for_web!();
                            }
                            pub trait HtmlAreaElement {
                                special_super_traits!(HtmlElementWithHref, ElementWithAltAttribute);
                                special_inter_traits!(ElementWithHrefAttribute, ElementWithTargetAttribute, ElementWithReferrerPolicyAttribute, ElementWithRelAttribute);
                                define!(Props: HtmlAreaElementProps, tags: (area,));
                                impl_for_web!();

                                fn coords(value: maybe![&str]) {
                                    update_with!(set_coords);
                                }
                                fn shape(value: maybe![&str]) {
                                    update_with!(set_shape);
                                }
                            }

                            pub trait HtmlMediaElement {
                                special_super_traits!(ElementWithSrcAttribute, ElementWithCrossOriginAttribute);
                                define!(Props: HtmlMediaElementProps);
                                impl_for_web!();

                                fn auto_play(value: maybe![bool]) {
                                    attr_name!("autoplay");
                                    update_with!(set_auto_play, web_sys_name = set_autoplay);
                                }
                                fn controls(value: maybe![bool]) {
                                    update_with!(set_controls);
                                }
                                fn r#loop(value: maybe![bool]) {
                                    alias!(loop_);
                                    attr_name!("loop");
                                    update_with!(set_loop);
                                }
                                fn muted(value: maybe![bool]) {
                                    update_with!(set_muted);
                                }
                                fn preload(value: maybe![&str]) {
                                    update_with!(set_preload);
                                }

                                // https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement#events

                                /// Event [`abort`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/abort_event)
                                ///
                                /// Fired when the resource was not fully loaded, but not as the result of an error.
                                fn on_abort(value: event![Event, "abort", OnAbortEvent, OnAbortEventListener]);
                                /// Event [`canplay`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/canplay_event)
                                ///
                                /// Fired when the user agent can play the media, but estimates that **not** enough data has been loaded to play the media up to its end without having to stop for further buffering of content.
                                fn on_can_play(value: event![Event, "canplay", OnCanPlayEvent, OnCanPlayEventListener]);
                                /// Event [`canplaythrough`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/canplaythrough_event)
                                ///
                                /// Fired when the user agent can play the media, and estimates that enough data has been loaded to play the media up to its end without having to stop for further buffering of content.
                                fn on_can_play_through(value: event![Event, "canplaythrough", OnCanPlayThroughEvent, OnCanPlayThroughEventListener]);
                                /// Event [`durationchange`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/durationchange_event)
                                ///
                                /// Fired when the duration property has been updated.
                                fn on_duration_change(value: event![Event, "durationchange", OnDurationChangeEvent, OnDurationChangeEventListener]);
                                /// Event [`emptied`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/emptied_event)
                                ///
                                /// Fired when the media has become empty; for example, when the media has already been loaded (or partially loaded), and the [`HTMLMediaElement.load()`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/load) method is called to reload it.
                                fn on_emptied(value: event![Event, "emptied", OnEmptiedEvent, OnEmptiedEventListener]);
                                /// Event [`ended`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/ended_event)
                                ///
                                /// Fired when playback stops when end of the media (<audio> or <video>) is reached or because no further data is available.
                                fn on_ended(value: event![Event, "ended", OnEndedEvent, OnEndedEventListener]);
                                /// Event [`loadeddata`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loadeddata_event)
                                ///
                                /// Fired when the first frame of the media has finished loading.
                                fn on_loaded_data(value: event![Event, "loadeddata", OnLoadedDataEvent, OnLoadedDataEventListener]);
                                /// Event [`loadedmetadata`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loadedmetadata_event)
                                ///
                                /// Fired when the metadata has been loaded.
                                fn on_loaded_metadata(value: event![Event, "loadedmetadata", OnLoadedMetadataEvent, OnLoadedMetadataEventListener]);
                                /// Event [`loadstart`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loadstart_event)
                                ///
                                /// Fired when the browser has started to load a resource.
                                fn on_load_start(value: event![Event, "loadstart", OnLoadStartEvent, OnLoadStartEventListener]);
                                /// Event [`pause`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/pause_event)
                                ///
                                /// Fired when a request to pause play is handled and the activity has entered its paused state, most commonly occurring when the media's [`HTMLMediaElement.pause()`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/pause) method is called.
                                fn on_pause(value: event![Event, "pause", OnPauseEvent, OnPauseEventListener]);
                                /// Event [`play`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/play_event)
                                ///
                                /// Fired when the `paused` property is changed from `true` to `false`, as a result of the [`HTMLMediaElement.play()`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/play) method, or the `autoplay` attribute.
                                fn on_play(value: event![Event, "play", OnPlayEvent, OnPlayEventListener]);
                                /// Event [`playing`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/playing_event)
                                ///
                                /// Fired when playback is ready to start after having been paused or delayed due to lack of data.
                                fn on_playing(value: event![Event, "playing", OnPlayingEvent, OnPlayingEventListener]);
                                /// Event [`progress`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/progress_event)
                                ///
                                /// Fired periodically as the browser loads a resource.
                                fn on_progress(value: event![Event, "progress", OnProgressEvent, OnProgressEventListener]);
                                /// Event [`ratechange`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/ratechange_event)
                                ///
                                /// Fired when the playback rate has changed.
                                fn on_rate_change(value: event![Event, "ratechange", OnRateChangeEvent, OnRateChangeEventListener]);
                                /// Event [`resize`]()
                                ///
                                /// Fired when one or both of the `videoWidth` and `videoHeight` properties have just been updated.
                                fn on_resize(value: event![Event, "resize", OnResizeEvent, OnResizeEventListener]);
                                /// Event [`seeked`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/seeked_event)
                                ///
                                /// Fired when a seek operation completes.
                                fn on_seeked(value: event![Event, "seeked", OnSeekedEvent, OnSeekedEventListener]);
                                /// Event [`seeking`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/seeking_event)
                                ///
                                /// Fired when a seek operation begins.
                                fn on_seeking(value: event![Event, "seeking", OnSeekingEvent, OnSeekingEventListener]);
                                /// Event [`stalled`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/stalled_event)
                                ///
                                /// Fired when the user agent is trying to fetch media data, but data is unexpectedly not forthcoming.
                                fn on_stalled(value: event![Event, "stalled", OnStalledEvent, OnStalledEventListener]);
                                /// Event [`suspend`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/suspend_event)
                                ///
                                /// Fired when the media data loading has been suspended.
                                fn on_suspend(value: event![Event, "suspend", OnSuspendEvent, OnSuspendEventListener]);
                                /// Event [`timeupdate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/timeupdate_event)
                                ///
                                /// Fired when the time indicated by the [`currentTime`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/currentTime) property has been updated.
                                fn on_time_update(value: event![Event, "timeupdate", OnTimeUpdateEvent, OnTimeUpdateEventListener]);
                                /// Event [`volumechange`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/volumechange_event)
                                ///
                                /// Fired when the volume has changed.
                                fn on_volume_change(value: event![Event, "volumechange", OnVolumeChangeEvent, OnVolumeChangeEventListener]);
                                /// Event [`waiting`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/waiting_event)
                                ///
                                /// Fired when playback has stopped because of a temporary lack of data.
                                fn on_waiting(value: event![Event, "waiting", OnWaitingEvent, OnWaitingEventListener]);

                                sub_traits!(
                                    pub trait HtmlAudioElement {
                                        define!(Props: HtmlAudioElementProps, tags: (audio,));
                                    }

                                    pub trait HtmlVideoElement {
                                        special_super_traits!(ElementWithHeightWidthU32Attributes);
                                        define!(Props: HtmlVideoElementProps, tags: (video,));

                                        fn plays_inline(value: maybe![bool]) {
                                            attr_name!("playsinline");
                                        }
                                        fn poster(value: maybe![&str]) {
                                            update_with!(set_poster);
                                        }
                                    }
                                );
                            }

                            pub trait HtmlBaseElementProps {
                                special_super_traits!(ElementWithHrefAttribute, ElementWithTargetAttribute);

                                define!(Props: HtmlBaseElement, tags: (base,));
                            }

                            pub trait HtmlQuoteElement {
                                special_super_traits!(ElementWithCiteAttribute);

                                define!(Props: HtmlQuoteElementProps, tags: (blockquote, q,));
                            }

                            pub trait HtmlBodyElement {
                                define!(Props: HtmlBodyElementProps, tags: (body,));
                                // TODO:
                                // https://developer.mozilla.org/en-US/docs/Web/HTML/Element/body
                                #[deprecated = "Use the CSS color property in conjunction with the :active pseudo-class instead."]
                                fn alink(value: maybe![&str]);
                            }

                            pub trait HtmlBrElement {
                                define!(Props: HtmlBrElementProps, tags: (br,));
                                #[deprecated]
                                fn clear(value: maybe![&str]) {
                                    update_with!(set_clear);
                                }
                            }

                            pub trait HtmlButtonElement {
                                special_super_traits!(
                                    ElementWithTypeAttribute,
                                    ElementWithFormAttributes,
                                    ElementWithDisabledAttribute,
                                    ElementWithNameAttribute,
                                    ElementWithValueStrAttribute,
                                );
                                special_inter_traits!(ElementWithFormAttribute);
                                define!(Props: HtmlButtonElementProps, tags: (button,));
                            }

                            pub trait HtmlCanvasElement {
                                special_super_traits!(ElementWithHeightWidthU32Attributes);
                                define!(Props: HtmlCanvasElementProps, tags: (canvas,));
                            }

                            pub trait HtmlTableCaptionElement {
                                special_super_traits!(ElementWithAlignAttribute);
                                define!(Props: HtmlTableCaptionElementProps, tags: (caption,));
                                // TODO: deprecate align
                                // #[deprecated = "Do not use this attribute, as it has been deprecated. The <caption> element should be styled using the CSS properties caption-side and text-align."]
                            }

                            pub trait HtmlDataElement {
                                special_super_traits!(ElementWithValueStrAttribute);
                                define!(Props: HtmlDataElementProps, tags: (data,));
                            }

                            pub trait HtmlModElement {
                                special_super_traits!(ElementWithCiteAttribute, ElementWithDateTimeAttribute);
                                define!(Props: HtmlModElementProps, tags: (del, ins,));
                            }

                            pub trait HtmlDetailsElement {
                                special_super_traits!(ElementWithOpenAttribute);
                                define!(Props: HtmlDetailsElementProps, tags: (details,));
                            }

                            pub trait HtmlDialogElement {
                                special_super_traits!(ElementWithOpenAttribute);
                                define!(Props: HtmlDialogElementProps, tags: (dialog,));
                            }

                            pub trait HtmlEmbedElement {
                                special_super_traits!(ElementWithTypeAttribute, ElementWithSrcAttribute, ElementWithHeightWidthStrAttributes);
                                define!(Props: HtmlEmbedElementProps, tags: (embed,));
                            }

                            pub trait HtmlFieldSetElement {
                                special_super_traits!(ElementWithFormAttribute, ElementWithDisabledAttribute, ElementWithNameAttribute);
                                define!(Props: HtmlFieldSetElementProps, tags: (fieldset,));
                            }

                            pub trait HtmlFormElement {
                                special_super_traits!(
                                    ElementWithTargetAttribute,
                                    ElementWithAutoCompleteAttribute,
                                    ElementWithAcceptAttribute,
                                    ElementWithRelAttribute,
                                    ElementWithNameAttribute,
                                );
                                define!(Props: HtmlFormElementProps, tags: (form,));
                                // TODO: mark HtmlFormElement.accept as deprecated
                                // #[deprecated = "This attribute has been deprecated and should not be used. Instead, use the accept attribute on <input type=file> elements."]
                                fn accept_charset(value: maybe![&str]) {
                                    attr_name!("accept-charset");
                                    update_with!(set_accept_charset);
                                }
                                fn action(value: maybe![&str]) {
                                    update_with!(set_action);
                                }
                                fn enc_type(value: maybe![&str]) {
                                    attr_name!("enctype");
                                    update_with!(set_enctype);
                                }
                                fn method(value: maybe![&str]) {
                                    update_with!(set_method);
                                }
                                fn no_validate(value: maybe![bool]) {
                                    attr_name!("novalidate");
                                    update_with!(set_no_validate);
                                }

                                // https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement#events

                                /// Event [`formdata`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/formdata_event)
                                ///
                                /// The `formdata` event fires after the entry list representing the form's data is constructed.
                                fn on_form_data(value: event![Event, "formdata", OnFormDataEvent, OnFormDataEventListener]);
                                /// Event [`reset`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/reset_event)
                                ///
                                /// The `reset` event fires when a form is reset.
                                fn on_reset(value: event![Event, "reset", OnResetEvent, OnResetEventListener]);
                                /// Event [`submit`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/submit_event)
                                ///
                                /// The `submit` event fires when a form is submitted.
                                fn on_submit(value: event![Event, "submit", OnSubmitEvent, OnSubmitEventListener]);
                            }

                            pub trait HtmlHtmlElement {
                                define!(Props: HtmlHtmlElementProps, tags: (html,));
                                fn xmlns(value: maybe![&str]);
                            }

                            pub trait HtmlIFrameElement {
                                special_super_traits!(
                                    ElementWithSrcAttribute,
                                    ElementWithFetchPriorityAttribute,
                                    ElementWithLoadingAttribute,
                                    ElementWithReferrerPolicyAttribute,
                                    ElementWithNameAttribute,
                                    ElementWithHeightWidthStrAttributes,
                                );
                                define!(Props: HtmlIFrameElementProps, tags: (iframe,));
                                fn allow(value: maybe![&str]);
                                fn allow_fullscreen(value: maybe![bool]) {
                                    attr_name!("allowfullscreen");
                                    update_with!(set_allow_fullscreen);
                                }
                                fn allow_payment_request(value: maybe![bool]) {
                                    attr_name!("allowpaymentrequest");
                                    update_with!(set_allow_payment_request);
                                }
                                fn csp(value: maybe![&str]);
                                fn sandbox(value: maybe![&str]);
                                fn src_doc(value: maybe![&str]) {
                                    attr_name!("srcdoc");
                                    update_with!(set_srcdoc);
                                }

                                // https://developer.mozilla.org/en-US/docs/Web/HTML/Element/iframe#deprecated_attributes
                            }

                            pub trait HtmlImageElement {
                                special_super_traits!(
                                    ElementWithSrcsetAttribute,
                                    ElementWithUseMapAttribute,
                                    ElementWithSizesAttribute,
                                    ElementWithLoadingAttribute,
                                    ElementWithAltAttribute,
                                    ElementWithReferrerPolicyAttribute,
                                    ElementWithCrossOriginAttribute,
                                    ElementWithHeightWidthU32Attributes,
                                );
                                special_inter_traits!(ElementWithSrcAttribute);

                                define!(Props: HtmlImageElementProps, tags: (img,));

                                fn decoding(value: maybe![&str]) {
                                    update_with!(set_decoding);
                                }
                                fn element_timing(value: maybe![&str]) {
                                    attr_name!("elementtiming");
                                }
                                fn is_map(value: maybe![bool]) {
                                    attr_name!("ismap");
                                    update_with!(set_is_map);
                                }
                            }

                            pub trait HtmlInputElement {
                                special_super_traits!(
                                    ElementWithTypeAttribute,
                                    ElementWithReadOnlyAttribute,
                                    ElementWithPlaceHolderAttribute,
                                    ElementWithMaxMinLengthAttributes,
                                    ElementWithSrcAttribute,
                                    ElementWithSizeU32Attribute,
                                    ElementWithRequiredAttribute,
                                    ElementWithMultipleAttribute,
                                    ElementWithFormAttributes,
                                    ElementWithAutoCompleteAttribute,
                                    ElementWithAcceptAttribute,
                                    ElementWithAltAttribute,
                                    ElementWithDisabledAttribute,
                                    ElementWithNameAttribute,
                                    ElementWithValueStrAttribute,
                                    ElementWithHeightWidthU32Attributes,
                                );
                                special_inter_traits!(ElementWithFormAttribute);
                                define!(Props: HtmlInputElementProps, tags: (input,));

                                fn capture(value: maybe![&str]);
                                fn checked(value: maybe![bool]) {
                                    update_with!(set_checked);
                                }
                                fn dirname(value: maybe![&str]);
                                fn list(value: maybe![&str]);
                                fn max(value: maybe![&str]) {
                                    update_with!(set_max);
                                }
                                fn min(value: maybe![&str]) {
                                    update_with!(set_min);
                                }
                                fn pattern(value: maybe![&str]) {
                                    update_with!(set_pattern);
                                }
                                fn step(value: maybe![&str]) {
                                    update_with!(set_step);
                                }
                            }

                            pub trait HtmlLabelElement {
                                special_super_traits!(ElementWithForAttribute);
                                define!(Props: HtmlLabelElementProps, tags: (label,));
                            }

                            pub trait HtmlLiElement {
                                define!(Props: HtmlLiElementProps, tags: (li,));
                                fn value(value: maybe![i32]) {
                                    update_with!(set_value);
                                }
                            }

                            pub trait HtmlLinkElementProps {
                                special_super_traits!(
                                    ElementWithHrefAttribute,
                                    ElementWithTypeAttribute,
                                    ElementWithMediaAttribute,
                                    ElementWithBlockingAttribute,
                                    ElementWithIntegrityAttribute,
                                    ElementWithSizesAttribute,
                                    ElementWithHrefLangAttribute,
                                    ElementWithFetchPriorityAttribute,
                                    ElementWithReferrerPolicyAttribute,
                                    ElementWithRelAttribute,
                                    ElementWithCrossOriginAttribute,
                                );
                                define!(Props: HtmlLinkElement, tags: (link,));
                                fn r#as(value: maybe![&str]) {
                                    alias!(as_);
                                    attr_name!("as");
                                    update_with!(set_as);
                                }
                                fn image_sizes(value: maybe![&str]) {
                                    attr_name!("imagesizes");
                                }
                                fn image_src_set(value: maybe![&str]) {
                                    attr_name!("imagesrcset");
                                }
                                fn prefetch(value: maybe![&str]);

                                // TODO: no set_sizes
                            }

                            pub trait HtmlMapElement {
                                special_super_traits!(ElementWithNameAttribute);
                                define!(Props: HtmlMapElementProps, tags: (map,));
                            }

                            pub trait HtmlMetaElement {
                                special_super_traits!(ElementWithNameAttribute);
                                define!(Props: HtmlMetaElementProps, tags: (meta,));
                                fn charset(value: maybe![&str]);
                                fn content(value: maybe![&str]) {
                                    update_with!(set_content);
                                }
                                fn http_equiv(value: maybe![&str]) {
                                    attr_name!("http-equiv");
                                    update_with!(set_http_equiv);
                                }
                            }

                            pub trait HtmlMeterElement {
                                special_super_traits!(ElementWithMaxF64Attribute, ElementWithValueF64Attribute);
                                define!(Props: HtmlMeterElementProps, tags: (meter,));

                                fn min(value: maybe![f64]) {
                                    update_with!(set_min);
                                }
                                fn low(value: maybe![f64]) {
                                    update_with!(set_low);
                                }
                                fn high(value: maybe![f64]) {
                                    update_with!(set_high);
                                }
                                fn optimum(value: maybe![f64]) {
                                    update_with!(set_optimum);
                                }
                            }

                            pub trait HtmlObjectElement {
                                special_super_traits!(
                                    ElementWithTypeAttribute,
                                    ElementWithUseMapAttribute,
                                    ElementWithFormAttribute,
                                    ElementWithNameAttribute,
                                    ElementWithHeightWidthStrAttributes,
                                );
                                define!(Props: HtmlObjectElementProps, tags: (object,));
                                fn data(value: maybe![&str]) {
                                    update_with!(set_data);
                                }
                            }

                            pub trait HtmlOListElement {
                                special_super_traits!(ElementWithTypeAttribute);
                                define!(Props: HtmlOListElementProps, tags: (ol,));
                                fn reversed(value: maybe![bool]) {
                                    update_with!(set_reversed);
                                }
                                fn start(value: maybe![i32]) {
                                    update_with!(set_start);
                                }
                            }

                            pub trait HtmlOptGroupElement {
                                special_super_traits!(ElementWithLabelAttribute, ElementWithDisabledAttribute);
                                define!(Props: HtmlOptGroupElementProps, tags: (optgroup,));
                            }

                            pub trait HtmlOptionElement {
                                special_super_traits!(ElementWithLabelAttribute, ElementWithDisabledAttribute, ElementWithValueStrAttribute);
                                define!(Props: HtmlOptionElementProps, tags: (option,));

                                fn selected(value: maybe![bool]) {
                                    update_with!(set_selected);
                                }
                            }

                            pub trait HtmlOutputElement {
                                special_super_traits!(ElementWithForAttribute, ElementWithFormAttribute, ElementWithNameAttribute);
                                define!(Props: HtmlOutputElementProps, tags: (output,));

                                // TODO: no set_html_for
                            }

                            pub trait HtmlProgressElement {
                                special_super_traits!(ElementWithMaxF64Attribute, ElementWithValueF64Attribute);
                                define!(Props: HtmlProgressElementProps, tags: (progress,));
                            }

                            pub trait HtmlScriptElement {
                                special_super_traits!(
                                    ElementWithTypeAttribute,
                                    ElementWithSrcAttribute,
                                    ElementWithBlockingAttribute,
                                    ElementWithIntegrityAttribute,
                                    ElementWithFetchPriorityAttribute,
                                    ElementWithReferrerPolicyAttribute,
                                    ElementWithCrossOriginAttribute,
                                );
                                define!(Props: HtmlScriptElementProps, tags: (script,)); // TODO: special children
                                fn r#async(value: maybe![bool]) {
                                    update_with!(set_async);
                                }
                                fn defer(value: maybe![bool]) {
                                    update_with!(set_defer);
                                }
                                fn no_module(value: maybe![bool]) {
                                    attr_name!("nomodule");
                                    update_with!(set_no_module);
                                }
                                // TODO: no set_referrer_policy
                            }

                            pub trait HtmlSelectElement {
                                special_super_traits!(
                                    ElementWithSizeU32Attribute,
                                    ElementWithRequiredAttribute,
                                    ElementWithMultipleAttribute,
                                    ElementWithFormAttribute,
                                    ElementWithAutoCompleteAttribute,
                                    ElementWithDisabledAttribute,
                                    ElementWithNameAttribute,
                                );
                                define!(Props: HtmlSelectElementProps, tags: (select,));
                            }

                            pub trait HtmlSlotElement {
                                special_super_traits!(ElementWithNameAttribute);
                                define!(Props: HtmlSlotElementProps, tags: (slot,));
                            }

                            pub trait HtmlSourceElement {
                                special_super_traits!(
                                    ElementWithTypeAttribute,
                                    ElementWithMediaAttribute,
                                    ElementWithSrcsetAttribute,
                                    ElementWithSizesAttribute,
                                    ElementWithHeightWidthU32Attributes,
                                );
                                special_inter_traits!(ElementWithSrcAttribute);
                                define!(Props: HtmlSourceElementProps, tags: (source,));

                                // TODO: no set_height set_width
                            }

                            pub trait HtmlStyleElement {
                                special_super_traits!(ElementWithTypeAttribute, ElementWithMediaAttribute, ElementWithBlockingAttribute);
                                define!(Props: HtmlStyleElementProps, tags: (style,));

                                // TODO: `HtmlStyleElement.type` should be marked as deprecated
                                // #[deprecated = "This attribute should not be provided: if it is, the only permitted values are the empty string or a case-insensitive match for \"text/css.\""]
                            }

                            pub trait HtmlTableElement {
                                special_super_traits!(ElementWithAlignAttribute, ElementWithBgColorAttribute);
                                define!(Props: HtmlTableElementProps, tags: (table,));

                                // TODO: #[deprecated] align
                                // TODO: #[deprecated] bg_color

                                #[deprecated]
                                fn border(value: maybe![&str]) {
                                    update_with!(set_border);
                                }
                                #[deprecated]
                                fn cell_padding(value: maybe![&str]) {
                                    attr_name!("cellpadding");
                                    update_with!(set_cell_padding);
                                }
                                #[deprecated]
                                fn cell_spacing(value: maybe![&str]) {
                                    attr_name!("cellspacing");
                                    update_with!(set_cell_spacing);
                                }
                                #[deprecated]
                                fn frame(value: maybe![&str]) {
                                    update_with!(set_frame);
                                }
                                #[deprecated]
                                fn rules(value: maybe![&str]) {
                                    update_with!(set_rules);
                                }
                                #[deprecated]
                                fn summary(value: maybe![&str]) {
                                    update_with!(set_summary);
                                }
                                #[deprecated]
                                fn width(value: maybe![&str]) {
                                    update_with!(set_width);
                                }
                            }

                            pub trait HtmlTableChildElement {
                                special_super_traits!(ElementWithAlignAttribute, ElementWithBgColorAttribute);
                                // TODO: #[deprecated] align
                                // TODO: #[deprecated] bg_color

                                #[deprecated]
                                fn char(value: maybe![&str]) {
                                    update_with!(set_ch);
                                }
                                #[deprecated]
                                fn char_off(value: maybe![&str]) {
                                    attr_name!("charoff");
                                    update_with!(set_ch_off);
                                }
                                #[deprecated]
                                fn v_align(value: maybe![&str]) {
                                    attr_name!("valign");
                                    update_with!(set_v_align);
                                }

                                sub_traits!(
                                    pub trait HtmlTableSectionElement {
                                        define!(Props: HtmlTableSectionElementProps, tags: (tbody, tfoot, thead,));
                                    }

                                    pub trait HtmlTableRowElement {
                                        define!(Props: HtmlTableRowElementProps, tags: (tr,));
                                    }

                                    pub trait HtmlTableColElement {
                                        define!(Props: HtmlTableColElementProps, tags: (col, colgroup,));
                                        fn span(value: maybe![u32]) {
                                            update_with!(set_span);
                                        }
                                        #[deprecated]
                                        fn width(value: maybe![&str]) {
                                            update_with!(set_width);
                                        }
                                    }

                                    pub trait HtmlTableCellElement {
                                        special_super_traits!(ElementWithHeightWidthStrAttributes);
                                        define!(Props: HtmlTableCellElementProps, tags: (td, th,));

                                        fn col_span(value: maybe![u32]) {
                                            attr_name!("colspan");
                                            update_with!(set_col_span);
                                        }
                                        fn headers(value: maybe![&str]) {
                                            update_with!(set_headers);
                                        }
                                        fn row_span(value: maybe![u32]) {
                                            attr_name!("rowspan");
                                            update_with!(set_row_span);
                                        }
                                        #[deprecated = "Do not use this attribute as it is obsolete in the latest standard. Alternatively, you can put the abbreviated description inside the cell and place the long content in the title attribute."]
                                        fn abbr(value: maybe![&str]);
                                        #[deprecated]
                                        fn axis(value: maybe![&str]) {
                                            update_with!(set_axis);
                                        }

                                        // TODO: mark HtmlTableCellElement.height as deprecated
                                        // #[deprecated = "Use the CSS height property instead."]
                                        // TODO: mark HtmlTableCellElement.width as deprecated

                                        #[deprecated]
                                        fn scope(value: maybe![&str]);
                                    }
                                );
                            }

                            pub trait HtmlTextAreaElement {
                                special_super_traits!(
                                    ElementWithReadOnlyAttribute,
                                    ElementWithPlaceHolderAttribute,
                                    ElementWithMaxMinLengthAttributes,
                                    ElementWithRequiredAttribute,
                                    ElementWithFormAttribute,
                                    ElementWithAutoCompleteAttribute,
                                    ElementWithDisabledAttribute,
                                    ElementWithNameAttribute,
                                );
                                define!(Props: HtmlTextAreaElementProps, tags: (textarea,));

                                fn auto_correct(value: maybe![&str]);
                                fn cols(value: maybe![u32]) {
                                    update_with!(set_cols);
                                }
                                fn rows(value: maybe![u32]) {
                                    update_with!(set_rows);
                                }
                                fn wrap(value: maybe![&str]) {
                                    update_with!(set_wrap);
                                }
                            }

                            pub trait HtmlTimeElement {
                                define!(Props: HtmlTimeElementProps, tags: (time,));
                            }

                            pub trait HtmlTrackElement {
                                special_super_traits!(ElementWithSrcAttribute, ElementWithLabelAttribute);

                                define!(Props: HtmlTrackElementProps, tags: (track,));

                                fn default(value: maybe![bool]) {
                                    update_with!(set_default);
                                }
                                fn kind(value: maybe![&str]) {
                                    update_with!(set_kind);
                                }
                                fn src_lang(value: maybe![&str]) {
                                    attr_name!("srclang");
                                    update_with!(set_src_lang);
                                }
                            }

                            pub trait HtmlUListElement {
                                define!(Props: HtmlUListElementProps, tags: (ul,));
                                #[deprecated = "Do not use this attribute, as it has been deprecated: use CSS instead. To give a similar effect as the compact attribute, the CSS property line-height can be used with a value of 80%."]
                                fn compact(value: maybe![bool]) {
                                    update_with!(set_compact);
                                }
                                // TODO: type attribute should be marked as deprecated
                                // #[deprecated = "Do not use this attribute, as it has been deprecated; use the CSS list-style-type property instead."]
                            }
                        );
                    }
                );
            }
        );
    }
);
