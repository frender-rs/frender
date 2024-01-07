use frender_dom::render::RenderTextFrom;

use crate::impl_bounds::{Css, DomTokens, MaybeContentEditable};

#[cfg(not(feature = "props_builders_not_expanded"))]
mod props_builders;

crate::def_intrinsic_component_props!(
    #[expand_html_traits]
    // #[macro_export]
    pub(crate) use __impl_expand_html_traits;

    mod items {
        #[behaviors]
        pub mod behaviors {
            #[cfg(feature = "web")]
            use crate::shims::prelude::*;
        }

        #[behaviors_prelude]
        pub mod behaviors_prelude {}

        #[attributes]
        pub mod attributes {}

        #[behavior_type_traits]
        pub mod behavior_type_traits {}

        #[tags]
        pub mod tags {}

        #[event_types]
        pub mod event_types {}

        #[event_type_helpers]
        pub mod event_type_helpers {}

        #[props_without_builders]
        pub mod props;

        #[components]
        pub mod components;

        #[props_builders]
        #[cfg(feature = "props_builders_not_expanded")]
        mod props_builders;

        #[RenderHtml]
        pub trait RenderHtml {
            additional_bounds!(
                dyn RenderTextFrom<Self::Text, str>
                    + RenderTextFrom<Self::Text, i8>
                    + RenderTextFrom<Self::Text, u8>
                    + RenderTextFrom<Self::Text, i16>
                    + RenderTextFrom<Self::Text, u16>
                    + RenderTextFrom<Self::Text, i32>
                    + RenderTextFrom<Self::Text, u32>
                    + RenderTextFrom<Self::Text, i64>
                    + RenderTextFrom<Self::Text, u64>
                    + RenderTextFrom<Self::Text, i128>
                    + RenderTextFrom<Self::Text, u128>
                    + RenderTextFrom<Self::Text, isize>
                    + RenderTextFrom<Self::Text, usize>
                    + RenderTextFrom<Self::Text, f32>
                    + RenderTextFrom<Self::Text, f64>
                    + RenderTextFrom<Self::Text, char>
            );

            type Text: behaviors::Node<Self>;
        }
    }

    pub trait Node {
        trait_bounds!(frender_dom::behaviors::Node<Renderer>);
        impl_for_web!();

        sub_traits!(
            pub trait Element {
                trait_bounds!(frender_dom::behaviors::Element<Renderer>);

                verbatim_trait_items!(
                    type ClassList<'a>: frender_html_common::dom_token::DomTokenList
                    where
                        Self: 'a,
                        Renderer: 'a;
                    fn class_list<'a>(&'a mut self, renderer: &'a mut Renderer) -> Self::ClassList<'a>;
                );

                impl_for_web!(
                    verbatim_trait_items!(
                        type ClassList<'a> = ::frender_dom::csr::web::DomTokenList<Renderer::TryBehavior<'a>>
                        where
                            Self: 'a,
                            Renderer: 'a;

                        fn class_list<'a>(&'a mut self, renderer: &'a mut Renderer) -> Self::ClassList<'a> {
                            let element: &web_sys::Element = self.0.as_ref();
                            ::frender_dom::csr::web::DomTokenList(element.class_list(), renderer.try_behavior())
                        }
                    );
                );

                fn children(value: children![impl Sized]); // TODO: limit bounds
                fn css(value: bounds![Css]);
                fn class(value: bounds![DomTokens]) {
                    impl_with!(csr {
                        get_mut_dom_token_list: behaviors::Element::class_list,
                    });
                }

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
                        impl_for_web!(only_for_types!(web_sys::HtmlAnchorElement, web_sys::HtmlAreaElement, web_sys::HtmlLinkElement, web_sys::HtmlBaseElement,););
                        fn href(value: maybe![&str]) {
                            update_with!(set_href);
                        }
                    }

                    pub trait ElementWithTargetAttribute {
                        impl_for_web!(only_for_types!(web_sys::HtmlAnchorElement, web_sys::HtmlAreaElement, web_sys::HtmlBaseElement, web_sys::HtmlFormElement,););

                        fn target(value: maybe![&str]) {
                            update_with!(set_target);
                        }
                    }

                    pub trait ElementWithTypeAttribute {
                        impl_for_web!(only_for_types!(
                            web_sys::HtmlAnchorElement,
                            web_sys::HtmlButtonElement,
                            web_sys::HtmlEmbedElement,
                            web_sys::HtmlInputElement,
                            web_sys::HtmlLinkElement,
                            web_sys::HtmlObjectElement,
                            web_sys::HtmlOListElement,
                            web_sys::HtmlScriptElement,
                            web_sys::HtmlSourceElement,
                            web_sys::HtmlStyleElement,
                            web_sys::HtmlUListElement,
                        ););

                        fn r#type(value: maybe![&str]) {
                            alias!(type_);
                            attr_name!("type");
                            update_with!(set_type);
                        }
                    }

                    pub trait ElementWithCiteAttribute {
                        impl_for_web!(only_for_types!(web_sys::HtmlQuoteElement, web_sys::HtmlModElement,););

                        fn cite(value: maybe![&str]) {
                            update_with!(set_cite);
                        }
                    }

                    pub trait ElementWithPlaceHolderAttribute {
                        impl_for_web!(only_for_types!(web_sys::HtmlInputElement, web_sys::HtmlTextAreaElement,););
                        fn placeholder(value: maybe![&str]) {
                            update_with!(set_placeholder);
                        }
                    }

                    pub trait ElementWithMaxMinLengthAttributes {
                        impl_for_web!(only_for_types!(web_sys::HtmlInputElement, web_sys::HtmlTextAreaElement,););
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
                        impl_for_web!(only_for_types!(web_sys::HtmlEmbedElement, web_sys::HtmlIFrameElement, web_sys::HtmlObjectElement, web_sys::HtmlTableCellElement,););
                        fn height(value: maybe![&str]) {
                            update_with!(set_height);
                        }
                        fn width(value: maybe![&str]) {
                            update_with!(set_width);
                        }
                    }
                    pub trait ElementWithHeightWidthU32Attributes {
                        impl_for_web!(only_for_types!(
                            web_sys::HtmlVideoElement,
                            web_sys::HtmlCanvasElement,
                            web_sys::HtmlImageElement,
                            web_sys::HtmlInputElement,
                            web_sys::HtmlSourceElement,
                        ););

                        fn height(value: maybe![u32]) {
                            update_with!(set_height);
                        }
                        fn width(value: maybe![u32]) {
                            update_with!(set_width);
                        }
                    }
                    pub trait ElementWithMaxF64Attribute {
                        impl_for_web!(only_for_types!(web_sys::HtmlMeterElement, web_sys::HtmlProgressElement,););
                        fn max(value: maybe![f64]) {
                            update_with!(set_max);
                        }
                    }
                    pub trait ElementWithValueF64Attribute {
                        impl_for_web!(only_for_types!(web_sys::HtmlMeterElement, web_sys::HtmlProgressElement,););
                        fn value(value: maybe![f64]) {
                            update_with!(set_value);
                        }
                    }
                    pub trait ElementWithValueStrAttribute {
                        impl_for_web!(only_for_types!(web_sys::HtmlButtonElement, web_sys::HtmlDataElement, web_sys::HtmlInputElement, web_sys::HtmlOptionElement,););

                        fn value(value: maybe![&str]) {
                            update_with!(set_value);
                        }
                    }
                    pub trait ElementWithOpenAttribute {
                        impl_for_web!(only_for_types!(web_sys::HtmlDetailsElement, web_sys::HtmlDialogElement,););
                        fn open(value: maybe![bool]) {
                            update_with!(set_open);
                        }
                    }
                    pub trait ElementWithNameAttribute {
                        impl_for_web!(only_for_types!(
                            web_sys::HtmlButtonElement,
                            web_sys::HtmlFieldSetElement,
                            web_sys::HtmlFormElement,
                            web_sys::HtmlIFrameElement,
                            web_sys::HtmlInputElement,
                            web_sys::HtmlMapElement,
                            web_sys::HtmlMetaElement,
                            web_sys::HtmlObjectElement,
                            web_sys::HtmlOutputElement,
                            web_sys::HtmlSelectElement,
                            web_sys::HtmlSlotElement,
                            web_sys::HtmlTextAreaElement,
                        ););

                        fn name(value: maybe![&str]) {
                            update_with!(set_name);
                        }
                    }
                    pub trait ElementWithDisabledAttribute {
                        impl_for_web!(only_for_types!(
                            web_sys::HtmlButtonElement,
                            web_sys::HtmlFieldSetElement,
                            web_sys::HtmlInputElement,
                            web_sys::HtmlOptGroupElement,
                            web_sys::HtmlOptionElement,
                            web_sys::HtmlSelectElement,
                            web_sys::HtmlTextAreaElement,
                        ););

                        fn disabled(value: maybe![bool]) {
                            update_with!(set_disabled);
                        }
                    }
                    pub trait ElementWithCrossOriginAttribute {
                        impl_for_web!(only_for_types!(
                            web_sys::HtmlMediaElement,
                            web_sys::HtmlAudioElement,
                            web_sys::HtmlVideoElement,
                            web_sys::HtmlImageElement,
                            web_sys::HtmlLinkElement,
                            web_sys::HtmlScriptElement,
                        ););

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
                        verbatim_trait_items!(
                            type RelList<'a>: ::frender_html_common::dom_token::DomTokenList
                            where
                                Self: 'a,
                                Renderer: 'a;
                            fn rel_list<'a>(&'a mut self, renderer: &'a mut Renderer) -> Self::RelList<'a>;
                        );
                        impl_for_web!(
                            only_for_types!(web_sys::HtmlAnchorElement, web_sys::HtmlAreaElement, web_sys::HtmlFormElement, web_sys::HtmlLinkElement,);
                            verbatim_trait_items!(
                                type RelList<'a> = ::frender_dom::csr::web::DomTokenList<Renderer::TryBehavior<'a>>
                                where
                                    Self: 'a,
                                    Renderer: 'a;
                                fn rel_list<'a>(&'a mut self, renderer: &'a mut Renderer) -> Self::RelList<'a> {
                                    ::frender_dom::csr::web::DomTokenList(self.0.rel_list(), renderer.try_behavior())
                                }
                            );
                        );

                        fn rel(value: bounds![DomTokens]) {
                            impl_with!(csr {
                                get_mut_dom_token_list: behaviors::ElementWithRelAttribute::rel_list,
                            });
                        }
                    }
                    pub trait ElementWithReferrerPolicyAttribute {
                        impl_for_web!(only_for_types!(
                            web_sys::HtmlAnchorElement,
                            web_sys::HtmlAreaElement,
                            web_sys::HtmlIFrameElement,
                            web_sys::HtmlImageElement,
                            web_sys::HtmlLinkElement,
                            web_sys::HtmlScriptElement,
                        ););

                        fn referrer_policy(value: maybe![&str]) {
                            attr_name!("referrerpolicy");
                            update_with!(set_referrer_policy);
                        }
                    }
                    pub trait ElementWithAltAttribute {
                        impl_for_web!(only_for_types!(web_sys::HtmlAreaElement, web_sys::HtmlImageElement, web_sys::HtmlInputElement,););

                        fn alt(value: maybe![&str]) {
                            update_with!(set_alt);
                        }
                    }
                    pub trait ElementWithLoadingAttribute {
                        impl_for_web!(only_for_types!(web_sys::HtmlIFrameElement, web_sys::HtmlImageElement,););
                        fn loading(value: maybe![&str]);
                    }
                    pub trait ElementWithAcceptAttribute {
                        impl_for_web!(only_for_types!(web_sys::HtmlFormElement, web_sys::HtmlInputElement,););
                        fn accept(value: maybe![&str]) {
                            update_with!(set_accept);
                        }
                    }
                    pub trait ElementWithAutoCompleteAttribute {
                        impl_for_web!(only_for_types!(web_sys::HtmlFormElement, web_sys::HtmlInputElement, web_sys::HtmlSelectElement, web_sys::HtmlTextAreaElement,););
                        fn auto_complete(value: maybe![&str]) {
                            attr_name!("autocomplete");
                            update_with!(set_auto_complete, web_sys_name = set_autocomplete);
                        }
                    }
                    pub trait ElementWithFormAttribute {
                        impl_for_web!(only_for_types!(
                            web_sys::HtmlButtonElement,
                            web_sys::HtmlFieldSetElement,
                            web_sys::HtmlInputElement,
                            web_sys::HtmlObjectElement,
                            web_sys::HtmlOutputElement,
                            web_sys::HtmlSelectElement,
                            web_sys::HtmlTextAreaElement,
                        ););

                        fn form(value: maybe![&str]);
                    }
                    pub trait ElementWithFormAttributes {
                        special_super_traits!(ElementWithFormAttribute);
                        impl_for_web!(only_for_types!(web_sys::HtmlButtonElement, web_sys::HtmlInputElement,););

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
                        impl_for_web!(only_for_types!(web_sys::HtmlIFrameElement, web_sys::HtmlLinkElement, web_sys::HtmlScriptElement,););
                        fn fetch_priority(value: maybe![&str]) {
                            attr_name!("fetchpriority");
                        }
                    }
                    pub trait ElementWithHrefLangAttribute {
                        special_super_traits!(ElementWithHrefAttribute);
                        impl_for_web!(only_for_types!(web_sys::HtmlAnchorElement, web_sys::HtmlLinkElement););

                        fn href_lang(value: maybe![&str]) {
                            attr_name!("hreflang");
                            update_with!(set_href_lang, web_sys_name = set_hreflang);
                        }
                    }

                    pub trait ElementWithSizesAttribute {
                        impl_for_web!(only_for_types!(web_sys::HtmlImageElement, web_sys::HtmlLinkElement, web_sys::HtmlSourceElement,););
                        fn sizes(value: maybe![&str]) {
                            update_with!(set_sizes);
                        }
                    }

                    pub trait ElementWithUseMapAttribute {
                        impl_for_web!(only_for_types!(web_sys::HtmlImageElement, web_sys::HtmlObjectElement,););
                        fn use_map(value: maybe![&str]) {
                            attr_name!("usemap");
                            update_with!(set_use_map);
                        }
                    }
                    pub trait ElementWithLabelAttribute {
                        impl_for_web!(only_for_types!(web_sys::HtmlOptGroupElement, web_sys::HtmlOptionElement, web_sys::HtmlTrackElement,););
                        fn label(value: maybe![&str]) {
                            update_with!(set_label);
                        }
                    }

                    pub trait ElementWithForAttribute {
                        impl_for_web!(only_for_types!(web_sys::HtmlLabelElement, web_sys::HtmlOutputElement,););
                        fn r#for(value: maybe![&str]) {
                            alias!(html_for);
                            attr_name!("for");
                            update_with!(set_html_for, web_sys_name = set_html_for);
                        }
                    }
                    pub trait ElementWithIntegrityAttribute {
                        impl_for_web!(only_for_types!(web_sys::HtmlLinkElement, web_sys::HtmlScriptElement,););
                        fn integrity(value: maybe![&str]) {
                            update_with!(set_integrity);
                        }
                    }
                    pub trait ElementWithBlockingAttribute {
                        impl_for_web!(only_for_types!(web_sys::HtmlLinkElement, web_sys::HtmlScriptElement, web_sys::HtmlStyleElement,););
                        fn blocking(value: maybe![&str]);
                    }

                    pub trait ElementWithMultipleAttribute {
                        impl_for_web!(only_for_types!(web_sys::HtmlInputElement, web_sys::HtmlSelectElement,););
                        fn multiple(value: maybe![bool]) {
                            update_with!(set_multiple);
                        }
                    }
                    pub trait ElementWithRequiredAttribute {
                        impl_for_web!(only_for_types!(web_sys::HtmlInputElement, web_sys::HtmlSelectElement, web_sys::HtmlTextAreaElement,););
                        fn required(value: maybe![bool]) {
                            update_with!(set_required);
                        }
                    }

                    pub trait ElementWithSizeU32Attribute {
                        impl_for_web!(only_for_types!(web_sys::HtmlInputElement, web_sys::HtmlSelectElement,););
                        fn size(value: maybe![u32]) {
                            update_with!(set_size);
                        }
                    }

                    pub trait ElementWithSrcAttribute {
                        impl_for_web!(only_for_types!(
                            web_sys::HtmlMediaElement,
                            web_sys::HtmlAudioElement,
                            web_sys::HtmlVideoElement,
                            web_sys::HtmlEmbedElement,
                            web_sys::HtmlIFrameElement,
                            web_sys::HtmlImageElement,
                            web_sys::HtmlInputElement,
                            web_sys::HtmlScriptElement,
                            web_sys::HtmlSourceElement,
                            web_sys::HtmlTrackElement,
                        ););

                        fn src(value: maybe![&str]) {
                            update_with!(set_src);
                        }
                    }

                    pub trait ElementWithSrcsetAttribute {
                        special_super_traits!(ElementWithSrcAttribute);
                        impl_for_web!(only_for_types!(web_sys::HtmlImageElement, web_sys::HtmlSourceElement,););

                        fn srcset(value: maybe![&str]) {
                            update_with!(set_srcset);
                        }
                    }

                    pub trait ElementWithBgColorAttribute {
                        impl_for_web!(only_for_types!(
                            web_sys::HtmlTableElement,
                            web_sys::HtmlTableSectionElement,
                            web_sys::HtmlTableRowElement,
                            web_sys::HtmlTableColElement,
                            web_sys::HtmlTableCellElement,
                        ););
                        fn bg_color(value: maybe![&str]) {
                            attr_name!("bgcolor");
                            update_with!(set_bg_color);
                        }
                    }

                    pub trait ElementWithAlignAttribute {
                        impl_for_web!(only_for_types!(
                            web_sys::HtmlTableCaptionElement,
                            web_sys::HtmlTableElement,
                            web_sys::HtmlTableSectionElement,
                            web_sys::HtmlTableRowElement,
                            web_sys::HtmlTableColElement,
                            web_sys::HtmlTableCellElement,
                        ););
                        fn align(value: maybe![&str]) {
                            update_with!(set_align);
                        }
                    }

                    pub trait ElementWithMediaAttribute {
                        impl_for_web!(only_for_types!(web_sys::HtmlLinkElement, web_sys::HtmlSourceElement, web_sys::HtmlStyleElement,););
                        fn media(value: maybe![&str]) {
                            update_with!(set_media);
                        }
                    }

                    pub trait ElementWithReadOnlyAttribute {
                        impl_for_web!(only_for_types!(web_sys::HtmlInputElement, web_sys::HtmlTextAreaElement,););
                        fn read_only(value: maybe![bool]) {
                            attr_name!("readonly");
                            update_with!(set_read_only);
                        }
                    }

                    pub trait ElementWithDateTimeAttribute {
                        impl_for_web!(only_for_types!(web_sys::HtmlModElement, web_sys::HtmlTimeElement,););
                        fn date_time(value: maybe![&str]) {
                            attr_name!("datetime");
                            update_with!(set_date_time);
                        }
                    }

                    pub trait HtmlElement {
                        trait_bounds!(frender_dom::behaviors::HtmlElement<Renderer>);

                        define!(
                            tags = (
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
                                hr { custom_content_model }, // HTMLHRElement
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
                                wbr { custom_content_model },
                            ),
                        );

                        verbatim_trait_items!(
                            fn set_content_editable(&mut self, renderer: &mut Renderer, value: &str);
                        );
                        impl_for_web!(verbatim_trait_items!(
                            fn set_content_editable(&mut self, renderer: &mut Renderer, value: &str) {
                                AsRef::<::web_sys::HtmlElement>::as_ref(&self.0).set_content_editable(value)
                            }
                        ););

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
                        fn content_editable(value: bounds![MaybeContentEditable]) {
                            attr_name!("contenteditable");
                            impl_with!(csr {
                                update: |el: &mut _, renderer: &mut _, _, v: &_| { behaviors::HtmlElement::set_content_editable(el, renderer, v,) },
                                remove: frender_dom::behaviors::Element::remove_attribute,
                            });
                        }
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
                        fn style(value: maybe![&str]); // TODO: UpdateStyle
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
                        fn on_input(value: event![InputEvent, "input", OnInputEvent, OnInputEventListener]);
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
                                define!(tags = (a,));
                                impl_for_web!();
                            }
                            pub trait HtmlAreaElement {
                                special_super_traits!(HtmlElementWithHref, ElementWithAltAttribute);
                                special_inter_traits!(ElementWithHrefAttribute, ElementWithTargetAttribute, ElementWithReferrerPolicyAttribute, ElementWithRelAttribute);
                                define!(tags = (area { custom_content_model },));
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
                                        define!(tags = (audio,));
                                        impl_for_web!();
                                    }

                                    pub trait HtmlVideoElement {
                                        special_super_traits!(ElementWithHeightWidthU32Attributes);
                                        define!(tags = (video,));
                                        impl_for_web!();

                                        fn plays_inline(value: maybe![bool]) {
                                            attr_name!("playsinline");
                                        }
                                        fn poster(value: maybe![&str]) {
                                            update_with!(set_poster);
                                        }
                                    }
                                );
                            }

                            pub trait HtmlBaseElement {
                                special_super_traits!(ElementWithHrefAttribute, ElementWithTargetAttribute);

                                define!(tags = (base { custom_content_model },));

                                impl_for_web!();
                            }

                            pub trait HtmlQuoteElement {
                                special_super_traits!(ElementWithCiteAttribute);

                                define!(tags = (blockquote, q,));

                                impl_for_web!();
                            }

                            pub trait HtmlBodyElement {
                                define!(tags = (body,));
                                impl_for_web!();
                                // TODO:
                                // https://developer.mozilla.org/en-US/docs/Web/HTML/Element/body
                                #[deprecated = "Use the CSS color property in conjunction with the :active pseudo-class instead."]
                                fn alink(value: maybe![&str]);
                            }

                            pub trait HtmlBrElement {
                                define!(tags = (br { custom_content_model },));
                                impl_for_web!();
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
                                define!(tags = (button,));
                                impl_for_web!();
                            }

                            pub trait HtmlCanvasElement {
                                special_super_traits!(ElementWithHeightWidthU32Attributes);
                                define!(tags = (canvas,));
                                impl_for_web!();
                            }

                            pub trait HtmlTableCaptionElement {
                                special_super_traits!(ElementWithAlignAttribute);
                                define!(tags = (caption,));
                                impl_for_web!();
                                // TODO: deprecate align
                                // #[deprecated = "Do not use this attribute, as it has been deprecated. The <caption> element should be styled using the CSS properties caption-side and text-align."]
                            }

                            pub trait HtmlDataElement {
                                special_super_traits!(ElementWithValueStrAttribute);
                                define!(tags = (data,));
                                impl_for_web!();
                            }

                            pub trait HtmlModElement {
                                special_super_traits!(ElementWithCiteAttribute, ElementWithDateTimeAttribute);
                                define!(tags = (del, ins,));
                                impl_for_web!();
                            }

                            pub trait HtmlDetailsElement {
                                special_super_traits!(ElementWithOpenAttribute);
                                define!(tags = (details,));
                                impl_for_web!();
                            }

                            pub trait HtmlDialogElement {
                                special_super_traits!(ElementWithOpenAttribute);
                                define!(tags = (dialog,));
                                impl_for_web!();
                            }

                            pub trait HtmlEmbedElement {
                                special_super_traits!(ElementWithTypeAttribute, ElementWithSrcAttribute, ElementWithHeightWidthStrAttributes);
                                define!(tags = (embed { custom_content_model },));
                                impl_for_web!();
                            }

                            pub trait HtmlFieldSetElement {
                                special_super_traits!(ElementWithFormAttribute, ElementWithDisabledAttribute, ElementWithNameAttribute);
                                define!(tags = (fieldset,));
                                impl_for_web!();
                            }

                            pub trait HtmlFormElement {
                                special_super_traits!(
                                    ElementWithTargetAttribute,
                                    ElementWithAutoCompleteAttribute,
                                    ElementWithAcceptAttribute,
                                    ElementWithRelAttribute,
                                    ElementWithNameAttribute,
                                );
                                define!(tags = (form,));
                                impl_for_web!();
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
                                define!(tags = (html,));
                                impl_for_web!();
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
                                define!(tags = (iframe,));
                                impl_for_web!();
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

                                define!(tags = (img { custom_content_model },));
                                impl_for_web!();

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
                                define!(tags = (input { custom_content_model },));
                                impl_for_web!();

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
                                define!(tags = (label,));
                                impl_for_web!();
                            }

                            pub trait HtmlLiElement {
                                define!(tags = (li,));
                                impl_for_web!();
                                fn value(value: maybe![i32]) {
                                    update_with!(set_value);
                                }
                            }

                            pub trait HtmlLinkElement {
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
                                define!(tags = (link { custom_content_model },));
                                impl_for_web!();
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
                                define!(tags = (map,));
                                impl_for_web!();
                            }

                            pub trait HtmlMetaElement {
                                special_super_traits!(ElementWithNameAttribute);
                                define!(tags = (meta { custom_content_model },));
                                impl_for_web!();
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
                                define!(tags = (meter,));
                                impl_for_web!();

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
                                define!(tags = (object,));
                                impl_for_web!();
                                fn data(value: maybe![&str]) {
                                    update_with!(set_data);
                                }
                            }

                            pub trait HtmlOListElement {
                                special_super_traits!(ElementWithTypeAttribute);
                                define!(tags = (ol,));
                                impl_for_web!();
                                fn reversed(value: maybe![bool]) {
                                    update_with!(set_reversed);
                                }
                                fn start(value: maybe![i32]) {
                                    update_with!(set_start);
                                }
                            }

                            pub trait HtmlOptGroupElement {
                                special_super_traits!(ElementWithLabelAttribute, ElementWithDisabledAttribute);
                                define!(tags = (optgroup,));
                                impl_for_web!();
                            }

                            pub trait HtmlOptionElement {
                                special_super_traits!(ElementWithLabelAttribute, ElementWithDisabledAttribute, ElementWithValueStrAttribute);
                                define!(tags = (option,));
                                impl_for_web!();

                                fn selected(value: maybe![bool]) {
                                    update_with!(set_selected);
                                }
                            }

                            pub trait HtmlOutputElement {
                                special_super_traits!(ElementWithForAttribute, ElementWithFormAttribute, ElementWithNameAttribute);
                                define!(tags = (output,));
                                impl_for_web!();

                                // TODO: no set_html_for
                            }

                            pub trait HtmlProgressElement {
                                special_super_traits!(ElementWithMaxF64Attribute, ElementWithValueF64Attribute);
                                define!(tags = (progress,));
                                impl_for_web!();
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
                                define!(tags = (script { custom_content_model },)); // TODO: special children
                                impl_for_web!();
                                fn r#async(value: maybe![bool]) {
                                    attr_name!("async");
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
                                define!(tags = (select,));
                                impl_for_web!();
                            }

                            pub trait HtmlSlotElement {
                                special_super_traits!(ElementWithNameAttribute);
                                define!(tags = (slot,));
                                impl_for_web!();
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
                                define!(tags = (source { custom_content_model },));
                                impl_for_web!();

                                // TODO: no set_height set_width
                            }

                            pub trait HtmlStyleElement {
                                special_super_traits!(ElementWithTypeAttribute, ElementWithMediaAttribute, ElementWithBlockingAttribute);
                                define!(tags = (style { custom_content_model },));
                                impl_for_web!();

                                // TODO: `HtmlStyleElement.type` should be marked as deprecated
                                // #[deprecated = "This attribute should not be provided: if it is, the only permitted values are the empty string or a case-insensitive match for \"text/css.\""]
                            }

                            pub trait HtmlTableElement {
                                special_super_traits!(ElementWithAlignAttribute, ElementWithBgColorAttribute);
                                define!(tags = (table,));
                                impl_for_web!();

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
                                impl_for_web!(only_for_types!(
                                    web_sys::HtmlTableSectionElement,
                                    web_sys::HtmlTableRowElement,
                                    web_sys::HtmlTableColElement,
                                    web_sys::HtmlTableCellElement,
                                ););
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
                            }

                            pub trait HtmlTableSectionElement {
                                special_super_traits!(HtmlTableChildElement);
                                special_inter_traits!(ElementWithAlignAttribute, ElementWithBgColorAttribute);
                                define!(tags = (tbody, tfoot, thead,));
                                impl_for_web!();
                            }

                            pub trait HtmlTableRowElement {
                                special_super_traits!(HtmlTableChildElement);
                                special_inter_traits!(ElementWithAlignAttribute, ElementWithBgColorAttribute);
                                define!(tags = (tr,));
                                impl_for_web!();
                            }

                            pub trait HtmlTableColElement {
                                special_super_traits!(HtmlTableChildElement);
                                special_inter_traits!(ElementWithAlignAttribute, ElementWithBgColorAttribute);
                                define!(tags = (col { custom_content_model }, colgroup,));
                                impl_for_web!();
                                fn span(value: maybe![u32]) {
                                    update_with!(set_span);
                                }
                                #[deprecated]
                                fn width(value: maybe![&str]) {
                                    update_with!(set_width);
                                }
                            }

                            pub trait HtmlTableCellElement {
                                special_super_traits!(ElementWithHeightWidthStrAttributes, HtmlTableChildElement);
                                special_inter_traits!(ElementWithAlignAttribute, ElementWithBgColorAttribute);
                                define!(tags = (td, th,));
                                impl_for_web!();

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
                                trait_bounds!(crate::form_control::element::FormControlElement<str, Renderer>);

                                define!(tags = (textarea { custom_content_model },));
                                impl_for_web!();

                                fn auto_correct(value: maybe![&str]) {
                                    attr_name!("autocorrect");
                                }
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
                                special_super_traits!(ElementWithDateTimeAttribute);
                                define!(tags = (time,));
                                impl_for_web!();
                            }

                            pub trait HtmlTrackElement {
                                special_super_traits!(ElementWithSrcAttribute, ElementWithLabelAttribute);

                                define!(tags = (track { custom_content_model },));
                                impl_for_web!();

                                fn default(value: maybe![bool]) {
                                    update_with!(set_default);
                                }
                                fn kind(value: maybe![&str]) {
                                    update_with!(set_kind);
                                }
                                fn src_lang(value: maybe![&str]) {
                                    attr_name!("srclang");
                                    update_with!(set_src_lang, web_sys_name = set_srclang);
                                }
                            }

                            pub trait HtmlUListElement {
                                special_super_traits!(ElementWithTypeAttribute);
                                define!(tags = (ul,));
                                impl_for_web!();
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
