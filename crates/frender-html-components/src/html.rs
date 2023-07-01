#![allow(non_camel_case_types)]

use frender_events::events;

#[cfg(not(frender_macro_def_intrinsic_component_props))] // this attribute is for identifying when expanding this macro
frender_macros::def_intrinsic_component_props! {
    @[crate::imports]

    fully_typed (pub mod fully_typed  "fully-typed" )
    simply_typed(pub mod simply_typed "simply-typed")

    pub struct ElementProps (web_sys::Element) {
        children: () = () => {
            dom {
                bounds[::frender_core::UpdateRenderState<::frender_csr::Dom>]
                state pin
                    < <TypeDefs::children as frender_core::UpdateRenderState<frender_csr::Dom>>::State >
                    :[::frender_core::RenderState]
                    =(::frender_core::UpdateRenderState::initialize_render_state(this.children, children_ctx))
                impl {
                    ::frender_core::UpdateRenderState::update_render_state(this.children, children_ctx, state.children);
                }
            }
        },
        class: bounds![{
            trait Bounds = DomTokens;
            // const _: () = ();
            impl csr {
                fn initialize(this: Self, element: _, children_ctx: _) -> Self::State {}
                fn update(this: Self, element: _, children_ctx: _, state: _) {}
            }
            impl ssr {

            }
        }],
        id ? &str {set_id},
        part ? &str,
        /// Event [`cancel`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/cancel_event)
        ///
        /// Fires on a [`<dialog>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dialog) when the user instructs the browser that they wish to dismiss the currently open modal dialog. The browser fires this event when the user presses the <kbd>Esc</kbd> key to close the modal dialog.
        on_cancel @ "cancel" events::Event,
        /// Event [`error`](https://developer.mozilla.org/en-US/docs/Web/API/Element/error_event)
        ///
        /// Fired when a resource failed to load, or can't be used. For example, if a script has an execution error or an image can't be found or is invalid.
        on_error @ "error" events::Event,
        /// Event [`scroll`](https://developer.mozilla.org/en-US/docs/Web/API/Element/scroll_event)
        ///
        /// Fired when the document view or an element has been scrolled.
        on_scroll @ "scroll" events::Event,
        /// Event [`securitypolicyviolation`](https://developer.mozilla.org/en-US/docs/Web/API/Element/securitypolicyviolation_event)
        ///
        /// Fired when a [Content Security Policy](https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP) is violated.
        on_security_policy_violation @ "securitypolicyviolation" events::SecurityPolicyViolationEvent,
        /// Event [`select`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/select_event)
        ///
        /// Fired when some text has been selected.
        on_select @ "select" events::Event,
        /// Event [`wheel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/wheel_event)
        ///
        /// Fired when the user rotates a wheel button on a pointing device (typically a mouse).
        on_wheel @ "wheel" events::WheelEvent,

        // Clipboard events
        /// Event [`copy`](https://developer.mozilla.org/en-US/docs/Web/API/Element/copy_event)
        ///
        /// Fired when the user initiates a copy action through the browser's user interface.
        on_copy @ "copy" events::Event,
        /// Event [`cut`](https://developer.mozilla.org/en-US/docs/Web/API/Element/cut_event)
        ///
        /// Fired when the user initiates a cut action through the browser's user interface.
        on_cut @ "cut" events::Event,
        /// Event [`paste`](https://developer.mozilla.org/en-US/docs/Web/API/Element/paste_event)
        ///
        /// Fired when the user initiates a paste action through the browser's user interface.
        on_paste @ "paste" events::Event,

        // Composition events
        /// Event [`compositionend`](https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionend_event)
        ///
        /// Fired when a text composition system such as an [input method editor](https://developer.mozilla.org/en-US/docs/Glossary/Input_method_editor) completes or cancels the current composition session.
        on_composition_end @ "compositionend" events::CompositionEvent,
        /// Event [`compositionstart`](https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionstart_event)
        ///
        /// Fired when a text composition system such as an [input method editor](https://developer.mozilla.org/en-US/docs/Glossary/Input_method_editor) starts a new composition session.
        on_composition_start @ "compositionstart" events::CompositionEvent,
        /// Event [`compositionupdate`](https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionupdate_event)
        ///
        /// Fired when a new character is received in the context of a text composition session controlled by a text composition system such as an [input method editor](https://developer.mozilla.org/en-US/docs/Glossary/Input_method_editor).
        on_composition_update @ "compositionupdate" events::CompositionEvent,

        // Focus events
        /// Event [`blur`](https://developer.mozilla.org/en-US/docs/Web/API/Element/blur_event)
        ///
        /// Fired when an element has lost focus.
        on_blur @ "blur" events::FocusEvent,
        /// Event [`focus`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focus_event)
        ///
        /// Fired when an element has gained focus.
        on_focus @ "focus" events::FocusEvent,
        /// Event [`focusin`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focusin_event)
        ///
        /// Fired when an element has gained focus, after [`focus`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focus_event).
        on_focus_in @ "focusin" events::FocusEvent,
        /// Event [`focusout`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focusout_event)
        ///
        /// Fired when an element has lost focus, after [`blur`](https://developer.mozilla.org/en-US/docs/Web/API/Element/blur_event).
        on_focus_out @ "focusout" events::FocusEvent,

        // Fullscreen events
        /// Event [`fullscreenchange`](https://developer.mozilla.org/en-US/docs/Web/API/Element/fullscreenchange_event)
        ///
        /// Sent to an [`Element`](https://developer.mozilla.org/en-US/docs/Web/API/Element) when it transitions into or out of [fullscreen](https://developer.mozilla.org/en-US/docs/Web/API/Fullscreen_API/Guide) mode.
        on_fullscreen_change @ "fullscreenchange" events::Event,
        /// Event [`fullscreenerror`](https://developer.mozilla.org/en-US/docs/Web/API/Element/fullscreenerror_event)
        ///
        /// Sent to an `Element` if an error occurs while attempting to switch it into or out of [fullscreen](https://developer.mozilla.org/en-US/docs/Web/API/Fullscreen_API/Guide) mode.
        on_fullscreen_error @ "fullscreenerror" events::Event,

        // Keyboard events
        /// Event [`keydown`](https://developer.mozilla.org/en-US/docs/Web/API/Element/keydown_event)
        ///
        /// Fired when a key is pressed.
        on_key_down @ "keydown" events::KeyboardEvent,
        /// Event [`keyup`](https://developer.mozilla.org/en-US/docs/Web/API/Element/keyup_event)
        ///
        /// Fired when a key is released.
        on_key_up @ "keyup" events::KeyboardEvent,

        // Mouse events
        /// Event [`auxclick`](https://developer.mozilla.org/en-US/docs/Web/API/Element/auxclick_event)
        ///
        /// Fired when a non-primary pointing device button (e.g., any mouse button other than the left button) has been pressed and released on an element.
        on_aux_click @ "auxclick" events::MouseEvent,
        /// Event [`click`](https://developer.mozilla.org/en-US/docs/Web/API/Element/click_event)
        ///
        /// Fired when a pointing device button (e.g., a mouse's primary button) is pressed and released on a single element.
        on_click @ "click" events::MouseEvent,
        /// Event [`contextmenu`](https://developer.mozilla.org/en-US/docs/Web/API/Element/contextmenu_event)
        ///
        /// Fired when the user attempts to open a context menu.
        on_context_menu @ "contextmenu" events::MouseEvent,
        /// Event [`dblclick`](https://developer.mozilla.org/en-US/docs/Web/API/Element/dblclick_event)
        ///
        /// Fired when a pointing device button (e.g., a mouse's primary button) is clicked twice on a single element.
        on_double_click @ "dblclick" events::MouseEvent,
        /// Event [`mousedown`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mousedown_event)
        ///
        /// Fired when a pointing device button is pressed on an element.
        on_mouse_down @ "mousedown" events::MouseEvent,
        /// Event [`mouseenter`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseenter_event)
        ///
        /// Fired when a pointing device (usually a mouse) is moved over the element that has the listener attached.
        on_mouse_enter @ "mouseenter" events::MouseEvent,
        /// Event [`mouseleave`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseleave_event)
        ///
        /// Fired when the pointer of a pointing device (usually a mouse) is moved out of an element that has the listener attached to it.
        on_mouse_leave @ "mouseleave" events::MouseEvent,
        /// Event [`mousemove`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mousemove_event)
        ///
        /// Fired when a pointing device (usually a mouse) is moved while over an element.
        on_mouse_move @ "mousemove" events::MouseEvent,
        /// Event [`mouseout`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseout_event)
        ///
        /// Fired when a pointing device (usually a mouse) is moved off the element to which the listener is attached or off one of its children.
        on_mouse_out @ "mouseout" events::MouseEvent,
        /// Event [`mouseover`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseover_event)
        ///
        /// Fired when a pointing device is moved onto the element to which the listener is attached or onto one of its children.
        on_mouse_over @ "mouseover" events::MouseEvent,
        /// Event [`mouseup`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseup_event)
        ///
        /// Fired when a pointing device button is released on an element.
        on_mouse_up @ "mouseup" events::MouseEvent,

        // Touch events
        /// Event [`touchcancel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchcancel_event)
        ///
        /// Fired when one or more touch points have been disrupted in an implementation-specific manner (for example, too many touch points are created).
        on_touch_cancel @ "touchcancel" events::TouchEvent,
        /// Event [`touchend`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchend_event)
        ///
        /// Fired when one or more touch points are removed from the touch surface.
        on_touch_end @ "touchend" events::TouchEvent,
        /// Event [`touchmove`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchmove_event)
        ///
        /// Fired when one or more touch points are moved along the touch surface.
        on_touch_move @ "touchmove" events::TouchEvent,
        /// Event [`touchstart`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchstart_event)
        ///
        /// Fired when one or more touch points are placed on the touch surface.
        on_touch_start @ "touchstart" events::TouchEvent,
    } [
        pub struct HtmlElementProps (
            web_sys::HtmlElement :
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
                dl, // HTMLDListElement
                dt,
                em,
                figcaption,
                figure,
                footer,
                h1, h2, h3, h4, h5, h6, // HTMLHeadingElement
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
                p, // HTMLParagraphElement
                picture, // HTMLPictureElement
                pre, // HTMLPreElement with non-standard attributes
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
                title, // HTMLTitleElement
                u,
                var,
                wbr,
        ) {
            access_key ? &str {"accesskey" set_access_key},
            auto_capitalize ? &str {"autocapitalize"},
            auto_focus ? bool {"autofocus"},
            // content_editable [frender_html::props::MaybeInherit<bool>] : () = () => {
            //     dom {
            //         impl {
            //             // TODO:
            //         }
            //     }
            // },
            #[deprecated = "See https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/contextMenu"]
            context_menu ? &str {"contextmenu"},
            dir ? &str {set_dir},
            draggable ? bool { set_draggable },
            enter_key_hint ? &str {"enterkeyhint"},
            hidden ? bool { set_hidden }, // TODO: "until-found"
            inert ? bool,
            input_mode ? &str {"inputmode"},
            is ? &str,

            item_id    ? &str {"itemid"},
            item_prop  ? &str {"itemprop"},
            item_ref   ? &str {"itemref"},
            item_scope ? &str {"itemscope"},
            item_type  ? &str {"itemtype"},

            lang ? &str {set_lang},
            nonce ? &str,
            role ? &str,
            slot ? &str,
            spellcheck ? bool { set_spellcheck },
            style ? &str,
            tab_index ? i32 { "tabindex" set_tab_index },
            title ? &str {set_title},
            translate ? &str,
            virtual_keyboard_policy ? &str {"virtualkeyboardpolicy"},

            // TODO: aria-*
            // TODO: data-*

            /// Event [`invalid`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/invalid_event)
            ///
            /// Fired when an element does not satisfy its constraints during constraint validation.
            on_invalid @ "invalid" events::Event,

            // Animation events
            /// Event [`animationcancel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animationcancel_event)
            ///
            /// Fired when an animation unexpectedly aborts.
            on_animation_cancel @ "animationcancel" events::AnimationEvent,
            /// Event [`animationend`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animationend_event)
            ///
            /// Fired when an animation has completed normally.
            on_animation_end @ "animationend" events::AnimationEvent,
            /// Event [`animationiteration`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animationiteration_event)
            ///
            /// Fired when an animation iteration has completed.
            on_animation_iteration @ "animationiteration" events::AnimationEvent,
            /// Event [`animationstart`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animationstart_event)
            ///
            /// Fired when an animation starts.
            on_animation_start @ "animationstart" events::AnimationEvent,

            // Input events
            /// Event [`beforeinput`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/beforeinput_event)
            ///
            /// Fired when the value of an [`<input>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input), [`<select>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select), or [`<textarea>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea) element is about to be modified.
            on_before_input @ "beforeinput" events::InputEvent,
            /// Event [`input`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/input_event)
            ///
            /// Fired when the `value` of an [`<input>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input), [`<select>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select), or [`<textarea>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea) element has been changed.
            on_input @ "input" events::Event,
            /// Event [`change`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/change_event)
            ///
            /// Fired when the `value` of an [`<input>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input), [`<select>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select), or [`<textarea>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea) element has been changed and committed by the user. Unlike the [`input`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/input_event) event, the `change` event is not necessarily fired for each alteration to an element's `value`.
            on_change @ "change" events::Event,

            // Pointer events
            /// Event [`gotpointercapture`](https://developer.mozilla.org/en-US/docs/Web/API/Element/gotpointercapture_event)
            ///
            /// Fired when an element captures a pointer using [`setPointerCapture()`](https://developer.mozilla.org/en-US/docs/Web/API/Element/setPointerCapture).
            on_got_pointer_capture @ "gotpointercapture" events::PointerEvent,
            /// Event [`lostpointercapture`](https://developer.mozilla.org/en-US/docs/Web/API/Element/lostpointercapture_event)
            ///
            /// Fired when a [captured pointer](https://developer.mozilla.org/en-US/docs/Web/API/Pointer_events#pointer_capture) is released.
            on_lost_pointer_capture @ "lostpointercapture" events::PointerEvent,
            /// Event [`pointercancel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointercancel_event)
            ///
            /// Fired when a pointer event is canceled.
            on_pointer_cancel @ "pointercancel" events::PointerEvent,
            /// Event [`pointerdown`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerdown_event)
            ///
            /// Fired when a pointer becomes active.
            on_pointer_down @ "pointerdown" events::PointerEvent,
            /// Event [`pointerenter`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerenter_event)
            ///
            /// Fired when a pointer is moved into the hit test boundaries of an element or one of its descendants.
            on_pointer_enter @ "pointerenter" events::PointerEvent,
            /// Event [`pointerleave`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerleave_event)
            ///
            /// Fired when a pointer is moved out of the hit test boundaries of an element.
            on_pointer_leave @ "pointerleave" events::PointerEvent,
            /// Event [`pointermove`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointermove_event)
            ///
            /// Fired when a pointer changes coordinates.
            on_pointer_move @ "pointermove" events::PointerEvent,
            /// Event [`pointerout`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerout_event)
            ///
            /// Fired when a pointer is moved out of the *hit test* boundaries of an element (among other reasons).
            on_pointer_out @ "pointerout" events::PointerEvent,
            /// Event [`pointerover`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerover_event)
            ///
            /// Fired when a pointer is moved into an element's hit test boundaries.
            on_pointer_over @ "pointerover" events::PointerEvent,
            /// Event [`pointerup`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerup_event)
            ///
            /// Fired when a pointer is no longer active.
            on_pointer_up @ "pointerup" events::PointerEvent,

            // Transition events
            /// Event [`transitioncancel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/transitioncancel_event)
            ///
            /// Fired when a [CSS transition](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Transitions/Using_CSS_transitions) is canceled.
            on_transition_cancel @ "transitioncancel" events::TransitionEvent,
            /// Event [`transitionend`](https://developer.mozilla.org/en-US/docs/Web/API/Element/transitionend_event)
            ///
            /// Fired when a [CSS transition](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Transitions/Using_CSS_transitions) has completed.
            on_transition_end @ "transitionend" events::TransitionEvent,
            /// Event [`transitionrun`](https://developer.mozilla.org/en-US/docs/Web/API/Element/transitionrun_event)
            ///
            /// Fired when a [CSS transition](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Transitions/Using_CSS_transitions) is first created.
            on_transition_run @ "transitionrun" events::TransitionEvent,
            /// Event [`transitionstart`](https://developer.mozilla.org/en-US/docs/Web/API/Element/transitionstart_event)
            ///
            /// Fired when a [CSS transition](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Transitions/Using_CSS_transitions) has actually started.
            on_transition_start @ "transitionstart" events::TransitionEvent,

            /// Event [`drag`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/drag_event)
            ///
            /// This event is fired when an element or text selection is being dragged.
            on_drag @ "drag" events::Event,
            /// Event [`dragend`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragend_event)
            ///
            /// This event is fired when a drag operation is being ended (by releasing a mouse button or hitting the escape key).
            on_drag_end @ "dragend" events::Event,
            /// Event [`dragenter`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragenter_event)
            ///
            /// This event is fired when a dragged element or text selection enters a valid drop target.
            on_drag_enter @ "dragenter" events::Event,
            /// Event [`dragleave`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragleave_event)
            ///
            /// This event is fired when a dragged element or text selection leaves a valid drop target.
            on_drag_leave @ "dragleave" events::Event,
            /// Event [`dragover`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragover_event)
            ///
            /// This event is fired continuously when an element or text selection is being dragged and the mouse pointer is over a valid drop target (every 50 ms WHEN mouse is not moving ELSE much faster between 5 ms (slow movement) and 1ms (fast movement) approximately. This firing pattern is different than [`mouseover`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseover_event) ).
            on_drag_over @ "dragover" events::Event,
            /// Event [`dragstart`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragstart_event)
            ///
            /// This event is fired when the user starts dragging an element or text selection.
            on_drag_start @ "dragstart" events::Event,
            /// Event [`drop`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/drop_event)
            ///
            /// This event is fired when an element or text selection is dropped on a valid drop target.
            on_drop @ "drop" events::Event,
        } [
            virtual {
                download ? &str { set_download },
                href ? &str { set_href },
                ping ? &str { set_ping },
                referrer_policy ? &str {"referrerpolicy" set_referrer_policy},
                rel ? &str { set_rel },
                target ? &str { set_target },
            } [
                pub struct HtmlAnchorElementProps(web_sys::HtmlAnchorElement : a) {
                    href_lang ? &str { "hreflang" set_hreflang },
                    #[intrinsic_component(alias(type_))]
                    r#type ? &str {"type" set_type},
                }
            ][
                pub struct HtmlAreaElementProps(web_sys::HtmlAreaElement : area) {
                    alt ? &str {set_alt},
                    coords ? &str {set_coords},
                    shape ? &str {set_shape},
                }
            ]
        ][
            pub struct HtmlMediaElementProps(web_sys::HtmlMediaElement) {
                auto_play ? bool {"autoplay" set_autoplay},
                controls ? bool {set_controls},
                cross_origin ? &str {"crossorigin" [update el |v:&_| el.set_cross_origin(Some(v)) ] [remove el || el.set_cross_origin(None)]},
                #[intrinsic_component(alias(loop_))]
                r#loop ? bool {"loop" set_loop},
                muted ? bool {set_muted},
                preload ? &str {set_preload},
                src ? &str {set_src},

                // https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement#events

                /// Event [`abort`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/abort_event)
                ///
                /// Fired when the resource was not fully loaded, but not as the result of an error.
                on_abort @ "abort" events::Event,
                /// Event [`canplay`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/canplay_event)
                ///
                /// Fired when the user agent can play the media, but estimates that **not** enough data has been loaded to play the media up to its end without having to stop for further buffering of content.
                on_can_play @ "canplay" events::Event,
                /// Event [`canplaythrough`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/canplaythrough_event)
                ///
                /// Fired when the user agent can play the media, and estimates that enough data has been loaded to play the media up to its end without having to stop for further buffering of content.
                on_can_play_through @ "canplaythrough" events::Event,
                /// Event [`durationchange`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/durationchange_event)
                ///
                /// Fired when the duration property has been updated.
                on_duration_change @ "durationchange" events::Event,
                /// Event [`emptied`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/emptied_event)
                ///
                /// Fired when the media has become empty; for example, when the media has already been loaded (or partially loaded), and the [`HTMLMediaElement.load()`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/load) method is called to reload it.
                on_emptied @ "emptied" events::Event,
                /// Event [`ended`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/ended_event)
                ///
                /// Fired when playback stops when end of the media (<audio> or <video>) is reached or because no further data is available.
                on_ended @ "ended" events::Event,
                /// Event [`loadeddata`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loadeddata_event)
                ///
                /// Fired when the first frame of the media has finished loading.
                on_loaded_data @ "loadeddata" events::Event,
                /// Event [`loadedmetadata`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loadedmetadata_event)
                ///
                /// Fired when the metadata has been loaded.
                on_loaded_metadata @ "loadedmetadata" events::Event,
                /// Event [`loadstart`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loadstart_event)
                ///
                /// Fired when the browser has started to load a resource.
                on_load_start @ "loadstart" events::Event,
                /// Event [`pause`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/pause_event)
                ///
                /// Fired when a request to pause play is handled and the activity has entered its paused state, most commonly occurring when the media's [`HTMLMediaElement.pause()`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/pause) method is called.
                on_pause @ "pause" events::Event,
                /// Event [`play`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/play_event)
                ///
                /// Fired when the `paused` property is changed from `true` to `false`, as a result of the [`HTMLMediaElement.play()`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/play) method, or the `autoplay` attribute.
                on_play @ "play" events::Event,
                /// Event [`playing`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/playing_event)
                ///
                /// Fired when playback is ready to start after having been paused or delayed due to lack of data.
                on_playing @ "playing" events::Event,
                /// Event [`progress`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/progress_event)
                ///
                /// Fired periodically as the browser loads a resource.
                on_progress @ "progress" events::Event,
                /// Event [`ratechange`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/ratechange_event)
                ///
                /// Fired when the playback rate has changed.
                on_rate_change @ "ratechange" events::Event,
                /// Event [`resize`]()
                ///
                /// Fired when one or both of the `videoWidth` and `videoHeight` properties have just been updated.
                on_resize @ "resize" events::Event,
                /// Event [`seeked`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/seeked_event)
                ///
                /// Fired when a seek operation completes.
                on_seeked @ "seeked" events::Event,
                /// Event [`seeking`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/seeking_event)
                ///
                /// Fired when a seek operation begins.
                on_seeking @ "seeking" events::Event,
                /// Event [`stalled`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/stalled_event)
                ///
                /// Fired when the user agent is trying to fetch media data, but data is unexpectedly not forthcoming.
                on_stalled @ "stalled" events::Event,
                /// Event [`suspend`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/suspend_event)
                ///
                /// Fired when the media data loading has been suspended.
                on_suspend @ "suspend" events::Event,
                /// Event [`timeupdate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/timeupdate_event)
                ///
                /// Fired when the time indicated by the [`currentTime`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/currentTime) property has been updated.
                on_time_update @ "timeupdate" events::Event,
                /// Event [`volumechange`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/volumechange_event)
                ///
                /// Fired when the volume has changed.
                on_volume_change @ "volumechange" events::Event,
                /// Event [`waiting`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/waiting_event)
                ///
                /// Fired when playback has stopped because of a temporary lack of data.
                on_waiting @ "waiting" events::Event,
            } [
                pub struct HtmlAudioElementProps(web_sys::HtmlAudioElement : audio) {
                }
            ][
                pub struct HtmlVideoElementProps(web_sys::HtmlVideoElement : video) {
                    height ? u32 {set_height},
                    plays_inline ? bool {"playsinline"},
                    poster ? &str {set_poster},
                    width ? u32 {set_width},
                }
            ]
        ][
            pub struct HtmlBaseElementProps(web_sys::HtmlBaseElement : base) {
                href ? &str { set_href },
                target ? &str { set_target },
            }
        ][
            pub struct HtmlQuoteElementProps(web_sys::HtmlQuoteElement: blockquote, q) {
                cite ? &str { set_cite },
            }
        ][
            pub struct HtmlBodyElementProps(web_sys::HtmlBodyElement : body) {
                // TODO:
                // https://developer.mozilla.org/en-US/docs/Web/HTML/Element/body
                #[deprecated = "Use the CSS color property in conjunction with the :active pseudo-class instead."]
                alink ? &str,
            }
        ][
            pub struct HtmlBrElementProps(web_sys::HtmlBrElement : br) {
                #[deprecated]
                clear ? &str {set_clear},
            }
        ][
            pub struct HtmlButtonElementProps(web_sys::HtmlButtonElement : button) {
                disabled ? bool {set_disabled},
                form ? &str,
                form_action ? &str {"formaction" set_form_action},
                form_enc_type ? &str {"formenctype" set_form_enctype},
                form_method ? &str {"formmethod" set_form_method},
                form_no_validate ? bool {"formnovalidate" set_form_no_validate},
                form_target ? &str {"formtarget" set_form_target},
                name ? &str {set_name},
                #[intrinsic_component(alias(type_))]
                r#type ? &str {"type" set_type},
                value ? &str {set_value},
            }
        ][
            pub struct HtmlCanvasElementProps(web_sys::HtmlCanvasElement : canvas) {
                height ? u32 {set_height},
                width ? u32 {set_width},
            }
        ][
            pub struct HtmlTableCaptionElementProps(web_sys::HtmlTableCaptionElement : caption) {
                #[deprecated = "Do not use this attribute, as it has been deprecated. The <caption> element should be styled using the CSS properties caption-side and text-align."]
                align ? &str {set_align},
            }
        ][
            pub struct HtmlDataElementProps(web_sys::HtmlDataElement : data) {
                value ? &str {set_value},
            }
        ][
            pub struct HtmlModElementProps(web_sys::HtmlModElement: del, ins) {
                cite ? &str {set_cite},
                date_time ? &str {"datetime" set_date_time},
            }
        ][
            pub struct HtmlDetailsElementProps(web_sys::HtmlDetailsElement : details) {
                open ? bool {set_open},
            }
        ][
            pub struct HtmlDialogElementProps(web_sys::HtmlDialogElement : dialog) {
                open ? bool {set_open},
            }
        ][
            pub struct HtmlEmbedElementProps(web_sys::HtmlEmbedElement : embed) {
                height ? &str {set_height},
                src ? &str {set_src},
                #[intrinsic_component(alias(type_))]
                r#type ? &str {set_type},
                width ? &str {set_width},
            }
        ][
            pub struct HtmlFieldSetElementProps(web_sys::HtmlFieldSetElement : fieldset) {
                disabled ? bool {set_disabled},
                form ? &str,
                name ? &str {set_name},
            }
        ][
            pub struct HtmlFormElementProps(web_sys::HtmlFormElement : form) {
                #[deprecated = "This attribute has been deprecated and should not be used. Instead, use the accept attribute on <input type=file> elements."]
                accept ? &str,
                accept_charset ? &str {"accept-charset" set_accept_charset},
                auto_complete ? &str {"autocomplete" set_autocomplete},
                name ? &str {set_name},
                rel ? &str,
                action ? &str {set_action},
                enc_type ? &str {"enctype" set_enctype},
                method ? &str {set_method},
                no_validate ? bool {"novalidate" set_no_validate},
                target ? &str {set_target},

                // https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement#events

                /// Event [`formdata`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/formdata_event)
                ///
                /// The `formdata` event fires after the entry list representing the form's data is constructed.
                on_form_data @ "formdata" events::Event,
                /// Event [`reset`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/reset_event)
                ///
                /// The `reset` event fires when a form is reset.
                on_reset @ "reset" events::Event,
                /// Event [`submit`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/submit_event)
                ///
                /// The `submit` event fires when a form is submitted.
                on_submit @ "submit" events::Event,
            }
        ][
            pub struct HtmlHtmlElementProps(web_sys::HtmlHtmlElement : html) {
                xmlns ? &str,
            }
        ][
            pub struct HtmlIFrameElementProps(web_sys::HtmlIFrameElement : iframe) {
                allow ? &str,
                allow_fullscreen ? bool {"allowfullscreen" set_allow_fullscreen},
                allow_payment_request ? bool {"allowpaymentrequest" set_allow_payment_request},
                csp ? &str,
                fetch_priority ? &str {"fetchpriority"},
                height ? &str {set_height},
                loading ? &str,
                name ? &str {set_name},
                referrer_policy ? &str {"referrerpolicy" set_referrer_policy},
                sandbox ? &str,
                src ? &str {set_src},
                src_doc ? &str {"srcdoc" set_srcdoc},
                width ? &str {set_width},

                // https://developer.mozilla.org/en-US/docs/Web/HTML/Element/iframe#deprecated_attributes
            }
        ][
            pub struct HtmlImageElementProps(web_sys::HtmlImageElement : img) {
                alt ? &str {set_alt},
                cross_origin ? &str {"crossorigin" [update el |v:&_| el.set_cross_origin(Some(v)) ] [remove el || el.set_cross_origin(None)]},
                decoding ? &str {set_decoding},
                element_timing ? &str {"elementtiming"},
                height ? u32 {set_height},
                is_map ? bool {"ismap" set_is_map},
                loading ? &str,
                referrer_policy ? &str {"referrerpolicy" set_referrer_policy},
                sizes ? &str {set_sizes},
                src ? &str {set_src},
                srcset ? &str {set_srcset},
                width ? u32 {set_width},
                use_map ? &str {"usemap" set_use_map},
            }
        ][
            pub struct HtmlInputElementProps(web_sys::HtmlInputElement : input) {
                accept ? &str {set_accept},
                alt ? &str {set_alt},
                auto_complete ? &str {"autocomplete" set_autocomplete},
                capture ? &str,
                checked ? bool {set_checked},
                dirname ? &str,
                disabled ? bool {set_disabled},
                form ? &str,
                form_action ? &str {"formaction" set_form_action},
                form_enc_type ? &str {"formenctype" set_form_enctype},
                form_method ? &str {"formmethod" set_form_method},
                form_no_validate ? bool {"formnovalidate" set_form_no_validate},
                form_target ? &str {"formtarget" set_form_target},
                height ? u32 {set_height},
                list ? &str,
                max ? &str {set_max},
                max_length ? i32 {"maxlength" set_max_length},
                min ? &str {set_min},
                min_length ? i32 {"minlength" set_min_length},
                multiple ? bool {set_multiple},
                name ? &str {set_name},
                pattern ? &str {set_pattern},
                placeholder ? &str {set_placeholder},
                read_only ? bool {"readonly" set_read_only},
                required ? bool {set_required},
                size ? u32 {set_size},
                src ? &str {set_src},
                step ? &str {set_step},
                #[intrinsic_component(alias(type_))]
                r#type ? &str {"type" set_type},
                value ? &str {set_value},
                width ? u32 {set_width},
            }
        ][
            pub struct HtmlLabelElementProps(web_sys::HtmlLabelElement : label) {
                html_for ? &str {"for" set_html_for},
            }
        ][
            pub struct HtmlLiElementProps(web_sys::HtmlLiElement : li) {
                value ? i32 {set_value},
            }
        ][
            pub struct HtmlLinkElementProps(web_sys::HtmlLinkElement : link) {
                #[intrinsic_component(alias(as_))]
                r#as ? &str {"as" set_as},
                cross_origin ? &str {"crossorigin" [update el |v:&_| el.set_cross_origin(Some(v)) ] [remove el || el.set_cross_origin(None)]},
                fetch_priority ? &str {"fetchpriority"},
                href ? &str { set_href },
                href_lang ? &str { "hreflang" set_hreflang },
                image_sizes ? &str {"imagesizes"},
                image_src_set ? &str {"imagesrcset"},
                integrity ? &str {set_integrity},
                media ? &str {set_media},
                prefetch ? &str,
                referrer_policy ? &str {"referrerpolicy" set_referrer_policy},
                rel ? &str { set_rel },
                sizes ? &str,
                #[intrinsic_component(alias(type_))]
                r#type ? &str {"type" set_type},
                blocking ? &str,
            }
        ][
            pub struct HtmlMapElementProps(web_sys::HtmlMapElement : map) {
                name ? &str {set_name},
            }
        ][
            pub struct HtmlMetaElementProps(web_sys::HtmlMetaElement : meta) {
                charset ? &str,
                content ? &str {set_content},
                http_equiv ? &str {"http-equiv" set_http_equiv},
                name ? &str {set_name},
            }
        ][
            pub struct HtmlMeterElementProps(web_sys::HtmlMeterElement : meter) {
                value ? f64 {set_value},
                min ? f64 {set_min},
                max ? f64 {set_max},
                low ? f64 {set_low},
                high ? f64 {set_high},
                optimum ? f64 {set_optimum},
            }
        ][
            pub struct HtmlObjectElementProps(web_sys::HtmlObjectElement : object) {
                data ? &str {set_data},
                form ? &str,
                height ? &str {set_height},
                name ? &str {set_name},
                #[intrinsic_component(alias(type_))]
                r#type ? &str {"type" set_type},
                use_map ? &str {"usemap" set_use_map},
                width ? &str {set_width},
            }
        ][
            pub struct HtmlOListElementProps(web_sys::HtmlOListElement : ol) {
                reversed ? bool {set_reversed},
                start ? i32 {set_start},
                #[intrinsic_component(alias(type_))]
                r#type ? &str {"type" set_type},
            }
        ][
            pub struct HtmlOptGroupElementProps(web_sys::HtmlOptGroupElement : optgroup) {
                disabled ? bool {set_disabled},
                label ? &str {set_label},
            }
        ][
            pub struct HtmlOptionElementProps(web_sys::HtmlOptionElement : option) {
                disabled ? bool {set_disabled},
                label ? &str {set_label},
                selected ? bool {set_selected},
                value ? &str {set_value},
            }
        ][
            pub struct HtmlOutputElementProps(web_sys::HtmlOutputElement : output) {
                html_for ? &str {"for"},
                form ? &str,
                name ? &str {set_name},
            }
        ][
            pub struct HtmlProgressElementProps(web_sys::HtmlProgressElement : progress) {
                max ? f64 {set_max},
                value ? f64 {set_value},
            }
        ][
            pub struct HtmlScriptElementProps(
                web_sys::HtmlScriptElement :
                    script {
                        special_children: __,
                    }
            ) {
                r#async ? bool {set_async},
                cross_origin ? &str {"crossorigin" [update el |v:&_| el.set_cross_origin(Some(v)) ] [remove el || el.set_cross_origin(None)]},
                defer ? bool {set_defer},
                fetch_priority ? &str {"fetchpriority"},
                integrity ? &str {set_integrity},
                no_module ? bool {"nomodule" set_no_module},
                referrer_policy ? &str {"referrerpolicy"},
                src ? &str {set_src},
                #[intrinsic_component(alias(type_))]
                r#type ? &str {set_type},
                blocking ? &str,
            }
        ][
            pub struct HtmlSelectElementProps(web_sys::HtmlSelectElement : select) {
                auto_complete ? &str {"autocomplete" set_autocomplete},
                disabled ? bool {set_disabled},
                form ? &str,
                multiple ? bool {set_multiple},
                name ? &str {set_name},
                required ? bool {set_required},
                size ? u32 {set_size},
            }
        ][
            pub struct HtmlSlotElementProps(web_sys::HtmlSlotElement : slot) {
                name ? &str {set_name},
            }
        ][
            pub struct HtmlSourceElementProps(web_sys::HtmlSourceElement : source) {
                #[intrinsic_component(alias(type_))]
                r#type ? &str {set_type},
                src ? &str {set_src},
                srcset ? &str {set_srcset},
                sizes ? &str {set_sizes},
                media ? &str {set_media},
                height ? u32,
                width ? u32,
            }
        ][
            pub struct HtmlStyleElementProps(web_sys::HtmlStyleElement : style) {
                media ? &str {set_media},
                blocking ? &str,
                #[deprecated = "This attribute should not be provided: if it is, the only permitted values are the empty string or a case-insensitive match for \"text/css.\""]
                #[intrinsic_component(alias(type_))]
                r#type ? &str {set_type},
            }
        ][
            pub struct HtmlTableElementProps(web_sys::HtmlTableElement : table) {
                #[deprecated]
                align ? &str {set_align},
                #[deprecated]
                bg_color ? &str {"bgcolor" set_bg_color},
                #[deprecated]
                border ? &str {set_border},
                #[deprecated]
                cell_padding ? &str {"cellpadding" set_cell_padding},
                #[deprecated]
                cell_spacing ? &str {"cellspacing" set_cell_spacing},
                #[deprecated]
                frame ? &str {set_frame},
                #[deprecated]
                rules ? &str {set_rules},
                #[deprecated]
                summary ? &str {set_summary},
                #[deprecated]
                width ? &str {set_width},
            }
        ][
            virtual {
                #[deprecated]
                align ? &str {set_align},
                #[deprecated]
                bg_color ? &str {"bgcolor"},
                #[deprecated]
                char ? &str {set_ch},
                #[deprecated]
                char_off ? &str {"charoff" set_ch_off},
                #[deprecated]
                v_align ? &str {"valign" set_v_align},
            } [
                pub struct HtmlTableSectionElementProps(web_sys::HtmlTableSectionElement : tbody, tfoot, thead) {
                }
            ][
                pub struct HtmlTableRowElementProps(web_sys::HtmlTableRowElement : tr) {
                }
            ][
                pub struct HtmlTableColElementProps(web_sys::HtmlTableColElement : col, colgroup) {
                    span ? u32 {set_span},
                    #[deprecated]
                    width ? &str {set_width},
                }
            ][
                pub struct HtmlTableCellElementProps(web_sys::HtmlTableCellElement : td, th) {
                    col_span ? u32 {"colspan" set_col_span},
                    headers ? &str {set_headers},
                    row_span ? u32 {"rowspan" set_row_span},
                    #[deprecated = "Do not use this attribute as it is obsolete in the latest standard. Alternatively, you can put the abbreviated description inside the cell and place the long content in the title attribute."]
                    abbr ? &str,
                    #[deprecated]
                    axis ? &str {set_axis},
                    #[deprecated = "Use the CSS height property instead."]
                    height ? &str {set_height},
                    #[deprecated]
                    scope ? &str,
                    #[deprecated]
                    width ? &str {set_width},
                }
            ]
        ][
            pub struct HtmlTextAreaElementProps(web_sys::HtmlTextAreaElement : textarea) {
                auto_complete ? &str {"autocomplete" set_autocomplete},
                auto_correct ? &str,
                cols ? u32 {set_cols},
                disabled ? bool {set_disabled},
                form ? &str,
                max_length ? i32 {"maxlength" set_max_length},
                min_length ? i32 {"minlength" set_min_length},
                name ? &str {set_name},
                placeholder ? &str {set_placeholder},
                read_only ? bool {"readonly" set_read_only},
                required ? bool {set_required},
                rows ? u32 {set_rows},
                wrap ? &str {set_wrap},
            }
        ][
            pub struct HtmlTimeElementProps(web_sys::HtmlTimeElement : time) {
                date_time ? &str {"datetime" set_date_time},
            }
        ][
            pub struct HtmlTrackElementProps(web_sys::HtmlTrackElement : track) {
                default ? bool {set_default},
                kind ? &str {set_kind},
                label ? &str {set_label},
                src ? &str {set_src},
                src_lang ? &str {"srclang" set_srclang},
            }
        ][
            pub struct HtmlUListElementProps(web_sys::HtmlUListElement : ul) {
                #[deprecated = "Do not use this attribute, as it has been deprecated: use CSS instead. To give a similar effect as the compact attribute, the CSS property line-height can be used with a value of 80%."]
                compact ? bool {set_compact},
                #[deprecated = "Do not use this attribute, as it has been deprecated; use the CSS list-style-type property instead."]
                #[intrinsic_component(alias(type_))]
                r#type ? &str {"type" set_type},
            }
        ]
    ]
}
