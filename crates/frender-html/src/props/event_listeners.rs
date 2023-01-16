macro_rules! __impl_mod_overwrite {
    (+ $fields:tt) => {
        __impl_mod_overwrite! {
            @ $fields $fields
        }
    };
    (- $field:ident [$($fields:ident)*]) => {
        dyn super::Types<
            $(
                $fields = $field![$fields TypeDefs Value],
            )*
        >
    };
    (@ [$($field:ident)*] $fields:tt) => {
        $(
            macro_rules! $field {
                ($field $base:ident $new_ty:ident) => {
                    $new_ty
                };
                ($other_field:ident $base:ident $new_ty:ident) => {
                    <$base as super::Types>::$other_field
                };
            }

            pub type $field<TypeDefs, Value> = __impl_mod_overwrite![
                - $field $fields
            ];
        )*
    };
}

macro_rules! def_events_struct {
    (
        $vis:vis
        $kw_struct:ident
        $name:ident
        {
            $(
                $(#$attr:tt)*
                $field:ident : $event_type:ident
            ),* $(,)?
        }
    ) => {
        #[allow(non_snake_case)]
        $vis mod $name {
            pub mod overwrite {
                #![allow(non_camel_case_types)]

                __impl_mod_overwrite! { +[$($field)*] }
            }

            #[allow(non_camel_case_types)]
            pub trait Types {$(
                type $field;
            )*}

            #[non_exhaustive]
            pub $kw_struct $name<TypeDefs: Types> where
                TypeDefs: ?::core::marker::Sized {
                $(
                    pub $field: <TypeDefs as Types>::$field,
                )*
            }

            pub use $name as Data;

            pub type TypesInitial = dyn Types<$(
                $field = (),
            )*>;

            pub type DataInitial = $name<TypesInitial>;
        }

        #[allow(non_snake_case)]
        $vis fn $name() -> $name::DataInitial {
            $name::$name {
                $(
                    $field: (),
                )*
            }
        }

        impl<TypeDefs: $name::Types + ?Sized> $crate::props::UpdateHtmlElementEventListeners for $name::Data<TypeDefs>
        where $(
            <TypeDefs as $name::Types>::$field: $crate::props::UpdateDomEventListener<$crate::props::events::$event_type>,
        )* {
            type State = $name::$name<
                dyn $name::Types<$(
                    $field = <<TypeDefs as $name::Types>::$field as $crate::props::UpdateDomEventListener<$crate::props::events::$event_type>>::State,
                )*>
            >;

            fn update_dom_event_listeners(
                self,
                element: &::web_sys::HtmlElement,
                state: &mut Self::State,
            ) {
                $(
                    $crate::props::UpdateDomEventListener::<$crate::props::events::$event_type>::update_dom_event_listener(
                        self.$field,
                        element,
                        &mut state.$field,
                    );
                )*
            }
        }

        // impl<TypeDefs: $crate::props::HtmlCommonSharedProps::Types + ?Sized> $crate::props::HtmlCommonSharedProps::Building<TypeDefs>
        //     where TypeDefs::__html_element_event_listeners: $name::Types
        // {$(
        //     $(#$attr)*
        //     $vis fn $field<E: $crate::props::UpdateDomEventListener<$crate::props::events::$event_type>>(
        //         self,
        //         $field: E,
        //     ) -> $crate::props::HtmlCommonSharedProps::Building<
        //         $crate::props::HtmlCommonSharedProps::overwrite::__html_element_event_listeners<
        //             TypeDefs,
        //             $name::Data<
        //                 $name::overwrite::$field<TypeDefs::__html_element_event_listeners, E>
        //             >,
        //         >
        //     > {
        //         let _ = $field;
        //         todo!()
        //         // use $crate::props::HtmlCommonSharedProps::prelude::*;

        //         // let __tmp_value = $field;

        //         // let $name::Data {} = ;

        //         // self.__html_element_event_listeners($name);
        //     }
        // )*}
    };
}

// HtmlElement events
def_events_struct! {
    pub struct HtmlElementEventListeners {
        /// Event [`cancel`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/cancel_event)
        ///
        /// Fires on a [`<dialog>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dialog) when the user instructs the browser that they wish to dismiss the currently open modal dialog. The browser fires this event when the user presses the <kbd>Esc</kbd> key to close the modal dialog.
        on_cancel: Cancel,
        /// Event [`error`](https://developer.mozilla.org/en-US/docs/Web/API/Element/error_event)
        ///
        /// Fired when a resource failed to load, or can't be used. For example, if a script has an execution error or an image can't be found or is invalid.
        on_error: Error,
        /// Event [`scroll`](https://developer.mozilla.org/en-US/docs/Web/API/Element/scroll_event)
        ///
        /// Fired when the document view or an element has been scrolled.
        on_scroll: Scroll,
        /// Event [`securitypolicyviolation`](https://developer.mozilla.org/en-US/docs/Web/API/Element/securitypolicyviolation_event)
        ///
        /// Fired when a [Content Security Policy](https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP) is violated.
        on_security_policy_violation: SecurityPolicyViolation,
        /// Event [`select`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/select_event)
        ///
        /// Fired when some text has been selected.
        on_select: Select,
        /// Event [`wheel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/wheel_event)
        ///
        /// Fired when the user rotates a wheel button on a pointing device (typically a mouse).
        on_wheel: Wheel,

        // Clipboard events
        /// Event [`copy`](https://developer.mozilla.org/en-US/docs/Web/API/Element/copy_event)
        ///
        /// Fired when the user initiates a copy action through the browser's user interface.
        on_copy: Copy,
        /// Event [`cut`](https://developer.mozilla.org/en-US/docs/Web/API/Element/cut_event)
        ///
        /// Fired when the user initiates a cut action through the browser's user interface.
        on_cut: Cut,
        /// Event [`paste`](https://developer.mozilla.org/en-US/docs/Web/API/Element/paste_event)
        ///
        /// Fired when the user initiates a paste action through the browser's user interface.
        on_paste: Paste,

        // Composition events
        /// Event [`compositionend`](https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionend_event)
        ///
        /// Fired when a text composition system such as an [input method editor](https://developer.mozilla.org/en-US/docs/Glossary/Input_method_editor) completes or cancels the current composition session.
        on_composition_end: CompositionEnd,
        /// Event [`compositionstart`](https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionstart_event)
        ///
        /// Fired when a text composition system such as an [input method editor](https://developer.mozilla.org/en-US/docs/Glossary/Input_method_editor) starts a new composition session.
        on_composition_start: CompositionStart,
        /// Event [`compositionupdate`](https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionupdate_event)
        ///
        /// Fired when a new character is received in the context of a text composition session controlled by a text composition system such as an [input method editor](https://developer.mozilla.org/en-US/docs/Glossary/Input_method_editor).
        on_composition_update: CompositionUpdate,

        // Focus events
        /// Event [`blur`](https://developer.mozilla.org/en-US/docs/Web/API/Element/blur_event)
        ///
        /// Fired when an element has lost focus.
        on_blur: Blur,
        /// Event [`focus`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focus_event)
        ///
        /// Fired when an element has gained focus.
        on_focus: Focus,
        /// Event [`focusin`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focusin_event)
        ///
        /// Fired when an element has gained focus, after [`focus`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focus_event).
        on_focus_in: FocusIn,
        /// Event [`focusout`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focusout_event)
        ///
        /// Fired when an element has lost focus, after [`blur`](https://developer.mozilla.org/en-US/docs/Web/API/Element/blur_event).
        on_focus_out: FocusOut,

        // Fullscreen events
        /// Event [`fullscreenchange`](https://developer.mozilla.org/en-US/docs/Web/API/Element/fullscreenchange_event)
        ///
        /// Sent to an [`Element`](https://developer.mozilla.org/en-US/docs/Web/API/Element) when it transitions into or out of [fullscreen](https://developer.mozilla.org/en-US/docs/Web/API/Fullscreen_API/Guide) mode.
        on_fullscreen_change: FullscreenChange,
        /// Event [`fullscreenerror`](https://developer.mozilla.org/en-US/docs/Web/API/Element/fullscreenerror_event)
        ///
        /// Sent to an `Element` if an error occurs while attempting to switch it into or out of [fullscreen](https://developer.mozilla.org/en-US/docs/Web/API/Fullscreen_API/Guide) mode.
        on_fullscreen_error: FullscreenError,

        // Keyboard events
        /// Event [`keydown`](https://developer.mozilla.org/en-US/docs/Web/API/Element/keydown_event)
        ///
        /// Fired when a key is pressed.
        on_key_down: KeyDown,
        /// Event [`keyup`](https://developer.mozilla.org/en-US/docs/Web/API/Element/keyup_event)
        ///
        /// Fired when a key is released.
        on_key_up: KeyUp,

        // Mouse events
        /// Event [`auxclick`](https://developer.mozilla.org/en-US/docs/Web/API/Element/auxclick_event)
        ///
        /// Fired when a non-primary pointing device button (e.g., any mouse button other than the left button) has been pressed and released on an element.
        on_aux_click: AuxClick,
        /// Event [`click`](https://developer.mozilla.org/en-US/docs/Web/API/Element/click_event)
        ///
        /// Fired when a pointing device button (e.g., a mouse's primary button) is pressed and released on a single element.
        on_click: Click,
        /// Event [`contextmenu`](https://developer.mozilla.org/en-US/docs/Web/API/Element/contextmenu_event)
        ///
        /// Fired when the user attempts to open a context menu.
        on_context_menu: ContextMenu,
        /// Event [`dblclick`](https://developer.mozilla.org/en-US/docs/Web/API/Element/dblclick_event)
        ///
        /// Fired when a pointing device button (e.g., a mouse's primary button) is clicked twice on a single element.
        on_double_click: DoubleClick,
        /// Event [`mousedown`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mousedown_event)
        ///
        /// Fired when a pointing device button is pressed on an element.
        on_mouse_down: MouseDown,
        /// Event [`mouseenter`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseenter_event)
        ///
        /// Fired when a pointing device (usually a mouse) is moved over the element that has the listener attached.
        on_mouse_enter: MouseEnter,
        /// Event [`mouseleave`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseleave_event)
        ///
        /// Fired when the pointer of a pointing device (usually a mouse) is moved out of an element that has the listener attached to it.
        on_mouse_leave: MouseLeave,
        /// Event [`mousemove`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mousemove_event)
        ///
        /// Fired when a pointing device (usually a mouse) is moved while over an element.
        on_mouse_move: MouseMove,
        /// Event [`mouseout`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseout_event)
        ///
        /// Fired when a pointing device (usually a mouse) is moved off the element to which the listener is attached or off one of its children.
        on_mouse_out: MouseOut,
        /// Event [`mouseover`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseover_event)
        ///
        /// Fired when a pointing device is moved onto the element to which the listener is attached or onto one of its children.
        on_mouse_over: MouseOver,
        /// Event [`mouseup`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseup_event)
        ///
        /// Fired when a pointing device button is released on an element.
        on_mouse_up: MouseUp,

        // Touch events
        /// Event [`touchcancel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchcancel_event)
        ///
        /// Fired when one or more touch points have been disrupted in an implementation-specific manner (for example, too many touch points are created).
        on_touch_cancel: TouchCancel,
        /// Event [`touchend`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchend_event)
        ///
        /// Fired when one or more touch points are removed from the touch surface.
        on_touch_end: TouchEnd,
        /// Event [`touchmove`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchmove_event)
        ///
        /// Fired when one or more touch points are moved along the touch surface.
        on_touch_move: TouchMove,
        /// Event [`touchstart`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchstart_event)
        ///
        /// Fired when one or more touch points are placed on the touch surface.
        on_touch_start: TouchStart,

        // HTMLElement
        /// Event [`invalid`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/invalid_event)
        ///
        /// Fired when an element does not satisfy its constraints during constraint validation.
        on_invalid: Invalid,

        // Animation events
        /// Event [`animationcancel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animationcancel_event)
        ///
        /// Fired when an animation unexpectedly aborts.
        on_animation_cancel: AnimationCancel,
        /// Event [`animationend`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animationend_event)
        ///
        /// Fired when an animation has completed normally.
        on_animation_end: AnimationEnd,
        /// Event [`animationiteration`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animationiteration_event)
        ///
        /// Fired when an animation iteration has completed.
        on_animation_iteration: AnimationIteration,
        /// Event [`animationstart`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animationstart_event)
        ///
        /// Fired when an animation starts.
        on_animation_start: AnimationStart,

        // Input events
        /// Event [`beforeinput`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/beforeinput_event)
        ///
        /// Fired when the value of an [`<input>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input), [`<select>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select), or [`<textarea>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea) element is about to be modified.
        on_before_input: BeforeInput,
        /// Event [`input`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/input_event)
        ///
        /// Fired when the `value` of an [`<input>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input), [`<select>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select), or [`<textarea>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea) element has been changed.
        on_input: Input,
        /// Event [`change`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/change_event)
        ///
        /// Fired when the `value` of an [`<input>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input), [`<select>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select), or [`<textarea>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea) element has been changed and committed by the user. Unlike the [`input`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/input_event) event, the `change` event is not necessarily fired for each alteration to an element's `value`.
        on_change: Change,

        // Pointer events
        /// Event [`gotpointercapture`](https://developer.mozilla.org/en-US/docs/Web/API/Element/gotpointercapture_event)
        ///
        /// Fired when an element captures a pointer using [`setPointerCapture()`](https://developer.mozilla.org/en-US/docs/Web/API/Element/setPointerCapture).
        on_got_pointer_capture: GotPointerCapture,
        /// Event [`lostpointercapture`](https://developer.mozilla.org/en-US/docs/Web/API/Element/lostpointercapture_event)
        ///
        /// Fired when a [captured pointer](https://developer.mozilla.org/en-US/docs/Web/API/Pointer_events#pointer_capture) is released.
        on_lost_pointer_capture: LostPointerCapture,
        /// Event [`pointercancel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointercancel_event)
        ///
        /// Fired when a pointer event is canceled.
        on_pointer_cancel: PointerCancel,
        /// Event [`pointerdown`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerdown_event)
        ///
        /// Fired when a pointer becomes active.
        on_pointer_down: PointerDown,
        /// Event [`pointerenter`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerenter_event)
        ///
        /// Fired when a pointer is moved into the hit test boundaries of an element or one of its descendants.
        on_pointer_enter: PointerEnter,
        /// Event [`pointerleave`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerleave_event)
        ///
        /// Fired when a pointer is moved out of the hit test boundaries of an element.
        on_pointer_leave: PointerLeave,
        /// Event [`pointermove`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointermove_event)
        ///
        /// Fired when a pointer changes coordinates.
        on_pointer_move: PointerMove,
        /// Event [`pointerout`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerout_event)
        ///
        /// Fired when a pointer is moved out of the *hit test* boundaries of an element (among other reasons).
        on_pointer_out: PointerOut,
        /// Event [`pointerover`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerover_event)
        ///
        /// Fired when a pointer is moved into an element's hit test boundaries.
        on_pointer_over: PointerOver,
        /// Event [`pointerup`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerup_event)
        ///
        /// Fired when a pointer is no longer active.
        on_pointer_up: PointerUp,

        // Transition events
        /// Event [`transitioncancel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/transitioncancel_event)
        ///
        /// Fired when a [CSS transition](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Transitions/Using_CSS_transitions) is canceled.
        on_transition_cancel: TransitionCancel,
        /// Event [`transitionend`](https://developer.mozilla.org/en-US/docs/Web/API/Element/transitionend_event)
        ///
        /// Fired when a [CSS transition](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Transitions/Using_CSS_transitions) has completed.
        on_transition_end: TransitionEnd,
        /// Event [`transitionrun`](https://developer.mozilla.org/en-US/docs/Web/API/Element/transitionrun_event)
        ///
        /// Fired when a [CSS transition](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Transitions/Using_CSS_transitions) is first created.
        on_transition_run: TransitionRun,
        /// Event [`transitionstart`](https://developer.mozilla.org/en-US/docs/Web/API/Element/transitionstart_event)
        ///
        /// Fired when a [CSS transition](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Transitions/Using_CSS_transitions) has actually started.
        on_transition_start: TransitionStart,

        // Drag and drop
        /// Event [`drag`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/drag_event)
        ///
        /// This event is fired when an element or text selection is being dragged.
        on_drag: Drag,
        /// Event [`dragend`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragend_event)
        ///
        /// This event is fired when a drag operation is being ended (by releasing a mouse button or hitting the escape key).
        on_drag_end: DragEnd,
        /// Event [`dragenter`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragenter_event)
        ///
        /// This event is fired when a dragged element or text selection enters a valid drop target.
        on_drag_enter: DragEnter,
        /// Event [`dragleave`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragleave_event)
        ///
        /// This event is fired when a dragged element or text selection leaves a valid drop target.
        on_drag_leave: DragLeave,
        /// Event [`dragover`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragover_event)
        ///
        /// This event is fired continuously when an element or text selection is being dragged and the mouse pointer is over a valid drop target (every 50 ms WHEN mouse is not moving ELSE much faster between 5 ms (slow movement) and 1ms (fast movement) approximately. This firing pattern is different than [`mouseover`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseover_event) ).
        on_drag_over: DragOver,
        /// Event [`dragstart`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragstart_event)
        ///
        /// This event is fired when the user starts dragging an element or text selection.
        on_drag_start: DragStart,
        /// Event [`drop`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/drop_event)
        ///
        /// This event is fired when an element or text selection is dropped on a valid drop target.
        on_drop: Drop,
    }
}

#[cfg(aaa)]
// https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement#events
def_event_types! {
    /// Event [`formdata`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/formdata_event)
    ///
    /// The `formdata` event fires after the entry list representing the form's data is constructed.
    on_form_data: FormData,
    /// Event [`reset`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/reset_event)
    ///
    /// The `reset` event fires when a form is reset.
    on_reset: Reset,
    /// Event [`submit`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/submit_event)
    ///
    /// The `submit` event fires when a form is submitted.
    on_submit: Submit,
}

#[cfg(aaa)]
// https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement#events
def_event_types! {
    /// Event [`abort`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/abort_event)
    ///
    /// Fired when the resource was not fully loaded, but not as the result of an error.
    on_abort: Abort,
    /// Event [`canplay`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/canplay_event)
    ///
    /// Fired when the user agent can play the media, but estimates that **not** enough data has been loaded to play the media up to its end without having to stop for further buffering of content.
    on_can_play: CanPlay,
    /// Event [`canplaythrough`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/canplaythrough_event)
    ///
    /// Fired when the user agent can play the media, and estimates that enough data has been loaded to play the media up to its end without having to stop for further buffering of content.
    on_can_play_through: CanPlayThrough,
    /// Event [`durationchange`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/durationchange_event)
    ///
    /// Fired when the duration property has been updated.
    on_duration_change: DurationChange,
    /// Event [`emptied`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/emptied_event)
    ///
    /// Fired when the media has become empty; for example, when the media has already been loaded (or partially loaded), and the [`HTMLMediaElement.load()`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/load) method is called to reload it.
    on_emptied: Emptied,
    /// Event [`ended`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/ended_event)
    ///
    /// Fired when playback stops when end of the media (<audio> or <video>) is reached or because no further data is available.
    on_ended: Ended,
    /// Event [`loadeddata`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loadeddata_event)
    ///
    /// Fired when the first frame of the media has finished loading.
    on_loaded_data: LoadedData,
    /// Event [`loadedmetadata`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loadedmetadata_event)
    ///
    /// Fired when the metadata has been loaded.
    on_loaded_metadata: LoadedMetadata,
    /// Event [`loadstart`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loadstart_event)
    ///
    /// Fired when the browser has started to load a resource.
    on_load_start: LoadStart,
    /// Event [`pause`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/pause_event)
    ///
    /// Fired when a request to pause play is handled and the activity has entered its paused state, most commonly occurring when the media's [`HTMLMediaElement.pause()`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/pause) method is called.
    on_pause: Pause,
    /// Event [`play`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/play_event)
    ///
    /// Fired when the `paused` property is changed from `true` to `false`, as a result of the [`HTMLMediaElement.play()`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/play) method, or the `autoplay` attribute.
    on_play: Play,
    /// Event [`playing`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/playing_event)
    ///
    /// Fired when playback is ready to start after having been paused or delayed due to lack of data.
    on_playing: Playing,
    /// Event [`progress`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/progress_event)
    ///
    /// Fired periodically as the browser loads a resource.
    on_progress: Progress,
    /// Event [`ratechange`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/ratechange_event)
    ///
    /// Fired when the playback rate has changed.
    on_rate_change: RateChange,
    /// Event [`resize`]()
    ///
    /// Fired when one or both of the `videoWidth` and `videoHeight` properties have just been updated.
    on_resize: Resize,
    /// Event [`seeked`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/seeked_event)
    ///
    /// Fired when a seek operation completes.
    on_seeked: Seeked,
    /// Event [`seeking`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/seeking_event)
    ///
    /// Fired when a seek operation begins.
    on_seeking: Seeking,
    /// Event [`stalled`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/stalled_event)
    ///
    /// Fired when the user agent is trying to fetch media data, but data is unexpectedly not forthcoming.
    on_stalled: Stalled,
    /// Event [`suspend`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/suspend_event)
    ///
    /// Fired when the media data loading has been suspended.
    on_suspend: Suspend,
    /// Event [`timeupdate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/timeupdate_event)
    ///
    /// Fired when the time indicated by the [`currentTime`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/currentTime) property has been updated.
    on_time_update: TimeUpdate,
    /// Event [`volumechange`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/volumechange_event)
    ///
    /// Fired when the volume has changed.
    on_volume_change: VolumeChange,
    /// Event [`waiting`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/waiting_event)
    ///
    /// Fired when playback has stopped because of a temporary lack of data.
    on_waiting: Waiting,
}
