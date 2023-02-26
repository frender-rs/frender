use ::frender_dom::props::StaticEventType;

macro_rules! def_event_types {
    (
        $(
            $(#$attr:tt)*
            $event_enum:ident ($event_type_str:literal : $event_type:ty)
        ),* $(,)?
    ) => {
        $(
            $(#$attr)*
            pub enum $event_enum {}

            impl StaticEventType for $event_enum {
                const EVENT_TYPE: &'static str = $event_type_str;

                type Event = $event_type;
            }
        )*
    };
}

// Element events
def_event_types! {
    /// Event [`cancel`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/cancel_event)
    ///
    /// Fires on a [`<dialog>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dialog) when the user instructs the browser that they wish to dismiss the currently open modal dialog. The browser fires this event when the user presses the <kbd>Esc</kbd> key to close the modal dialog.
    Cancel("cancel" : web_sys::Event),
    /// Event [`error`](https://developer.mozilla.org/en-US/docs/Web/API/Element/error_event)
    ///
    /// Fired when a resource failed to load, or can't be used. For example, if a script has an execution error or an image can't be found or is invalid.
    Error("error" : web_sys::Event),
    /// Event [`scroll`](https://developer.mozilla.org/en-US/docs/Web/API/Element/scroll_event)
    ///
    /// Fired when the document view or an element has been scrolled.
    Scroll("scroll" : web_sys::Event),
    /// Event [`securitypolicyviolation`](https://developer.mozilla.org/en-US/docs/Web/API/Element/securitypolicyviolation_event)
    ///
    /// Fired when a [Content Security Policy](https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP) is violated.
    SecurityPolicyViolation("securitypolicyviolation" : web_sys::SecurityPolicyViolationEvent),
    /// Event [`select`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/select_event)
    ///
    /// Fired when some text has been selected.
    Select("select" : web_sys::Event),
    /// Event [`wheel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/wheel_event)
    ///
    /// Fired when the user rotates a wheel button on a pointing device (typically a mouse).
    Wheel("wheel" : web_sys::WheelEvent),

    // Clipboard events
    /// Event [`copy`](https://developer.mozilla.org/en-US/docs/Web/API/Element/copy_event)
    ///
    /// Fired when the user initiates a copy action through the browser's user interface.
    Copy("copy" : web_sys::Event),
    /// Event [`cut`](https://developer.mozilla.org/en-US/docs/Web/API/Element/cut_event)
    ///
    /// Fired when the user initiates a cut action through the browser's user interface.
    Cut("cut" : web_sys::Event),
    /// Event [`paste`](https://developer.mozilla.org/en-US/docs/Web/API/Element/paste_event)
    ///
    /// Fired when the user initiates a paste action through the browser's user interface.
    Paste("paste" : web_sys::Event),

    // Composition events
    /// Event [`compositionend`](https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionend_event)
    ///
    /// Fired when a text composition system such as an [input method editor](https://developer.mozilla.org/en-US/docs/Glossary/Input_method_editor) completes or cancels the current composition session.
    CompositionEnd("compositionend" : web_sys::CompositionEvent),
    /// Event [`compositionstart`](https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionstart_event)
    ///
    /// Fired when a text composition system such as an [input method editor](https://developer.mozilla.org/en-US/docs/Glossary/Input_method_editor) starts a new composition session.
    CompositionStart("compositionstart" : web_sys::CompositionEvent),
    /// Event [`compositionupdate`](https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionupdate_event)
    ///
    /// Fired when a new character is received in the context of a text composition session controlled by a text composition system such as an [input method editor](https://developer.mozilla.org/en-US/docs/Glossary/Input_method_editor).
    CompositionUpdate("compositionupdate" : web_sys::CompositionEvent),

    // Focus events
    /// Event [`blur`](https://developer.mozilla.org/en-US/docs/Web/API/Element/blur_event)
    ///
    /// Fired when an element has lost focus.
    Blur("blur" : web_sys::FocusEvent),
    /// Event [`focus`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focus_event)
    ///
    /// Fired when an element has gained focus.
    Focus("focus" : web_sys::FocusEvent),
    /// Event [`focusin`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focusin_event)
    ///
    /// Fired when an element has gained focus, after [`focus`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focus_event).
    FocusIn("focusin" : web_sys::FocusEvent),
    /// Event [`focusout`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focusout_event)
    ///
    /// Fired when an element has lost focus, after [`blur`](https://developer.mozilla.org/en-US/docs/Web/API/Element/blur_event).
    FocusOut("focusout" : web_sys::FocusEvent),

    // Fullscreen events
    /// Event [`fullscreenchange`](https://developer.mozilla.org/en-US/docs/Web/API/Element/fullscreenchange_event)
    ///
    /// Sent to an [`Element`](https://developer.mozilla.org/en-US/docs/Web/API/Element) when it transitions into or out of [fullscreen](https://developer.mozilla.org/en-US/docs/Web/API/Fullscreen_API/Guide) mode.
    FullscreenChange("fullscreenchange" : web_sys::Event),
    /// Event [`fullscreenerror`](https://developer.mozilla.org/en-US/docs/Web/API/Element/fullscreenerror_event)
    ///
    /// Sent to an `Element` if an error occurs while attempting to switch it into or out of [fullscreen](https://developer.mozilla.org/en-US/docs/Web/API/Fullscreen_API/Guide) mode.
    FullscreenError("fullscreenerror" : web_sys::Event),

    // Keyboard events
    /// Event [`keydown`](https://developer.mozilla.org/en-US/docs/Web/API/Element/keydown_event)
    ///
    /// Fired when a key is pressed.
    KeyDown("keydown" : web_sys::KeyboardEvent),
    /// Event [`keyup`](https://developer.mozilla.org/en-US/docs/Web/API/Element/keyup_event)
    ///
    /// Fired when a key is released.
    KeyUp("keyup" : web_sys::KeyboardEvent),

    // Mouse events
    /// Event [`auxclick`](https://developer.mozilla.org/en-US/docs/Web/API/Element/auxclick_event)
    ///
    /// Fired when a non-primary pointing device button (e.g., any mouse button other than the left button) has been pressed and released on an element.
    AuxClick("auxclick" : web_sys::MouseEvent),
    /// Event [`click`](https://developer.mozilla.org/en-US/docs/Web/API/Element/click_event)
    ///
    /// Fired when a pointing device button (e.g., a mouse's primary button) is pressed and released on a single element.
    Click("click" : web_sys::MouseEvent),
    /// Event [`contextmenu`](https://developer.mozilla.org/en-US/docs/Web/API/Element/contextmenu_event)
    ///
    /// Fired when the user attempts to open a context menu.
    ContextMenu("contextmenu" : web_sys::MouseEvent),
    /// Event [`dblclick`](https://developer.mozilla.org/en-US/docs/Web/API/Element/dblclick_event)
    ///
    /// Fired when a pointing device button (e.g., a mouse's primary button) is clicked twice on a single element.
    DoubleClick("dblclick" : web_sys::MouseEvent),
    /// Event [`mousedown`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mousedown_event)
    ///
    /// Fired when a pointing device button is pressed on an element.
    MouseDown("mousedown" : web_sys::MouseEvent),
    /// Event [`mouseenter`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseenter_event)
    ///
    /// Fired when a pointing device (usually a mouse) is moved over the element that has the listener attached.
    MouseEnter("mouseenter" : web_sys::MouseEvent),
    /// Event [`mouseleave`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseleave_event)
    ///
    /// Fired when the pointer of a pointing device (usually a mouse) is moved out of an element that has the listener attached to it.
    MouseLeave("mouseleave" : web_sys::MouseEvent),
    /// Event [`mousemove`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mousemove_event)
    ///
    /// Fired when a pointing device (usually a mouse) is moved while over an element.
    MouseMove("mousemove" : web_sys::MouseEvent),
    /// Event [`mouseout`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseout_event)
    ///
    /// Fired when a pointing device (usually a mouse) is moved off the element to which the listener is attached or off one of its children.
    MouseOut("mouseout" : web_sys::MouseEvent),
    /// Event [`mouseover`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseover_event)
    ///
    /// Fired when a pointing device is moved onto the element to which the listener is attached or onto one of its children.
    MouseOver("mouseover" : web_sys::MouseEvent),
    /// Event [`mouseup`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseup_event)
    ///
    /// Fired when a pointing device button is released on an element.
    MouseUp("mouseup" : web_sys::MouseEvent),

    // Touch events
    /// Event [`touchcancel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchcancel_event)
    ///
    /// Fired when one or more touch points have been disrupted in an implementation-specific manner (for example, too many touch points are created).
    TouchCancel("touchcancel" : web_sys::TouchEvent),
    /// Event [`touchend`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchend_event)
    ///
    /// Fired when one or more touch points are removed from the touch surface.
    TouchEnd("touchend" : web_sys::TouchEvent),
    /// Event [`touchmove`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchmove_event)
    ///
    /// Fired when one or more touch points are moved along the touch surface.
    TouchMove("touchmove" : web_sys::TouchEvent),
    /// Event [`touchstart`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchstart_event)
    ///
    /// Fired when one or more touch points are placed on the touch surface.
    TouchStart("touchstart" : web_sys::TouchEvent),
}

// HTMLElement events
def_event_types! {
    /// Event [`invalid`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/invalid_event)
    ///
    /// Fired when an element does not satisfy its constraints during constraint validation.
    Invalid("invalid" : web_sys::Event),

    // Animation events
    /// Event [`animationcancel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animationcancel_event)
    ///
    /// Fired when an animation unexpectedly aborts.
    AnimationCancel("animationcancel" : web_sys::AnimationEvent),
    /// Event [`animationend`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animationend_event)
    ///
    /// Fired when an animation has completed normally.
    AnimationEnd("animationend" : web_sys::AnimationEvent),
    /// Event [`animationiteration`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animationiteration_event)
    ///
    /// Fired when an animation iteration has completed.
    AnimationIteration("animationiteration" : web_sys::AnimationEvent),
    /// Event [`animationstart`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animationstart_event)
    ///
    /// Fired when an animation starts.
    AnimationStart("animationstart" : web_sys::AnimationEvent),

    // Input events
    /// Event [`beforeinput`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/beforeinput_event)
    ///
    /// Fired when the value of an [`<input>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input), [`<select>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select), or [`<textarea>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea) element is about to be modified.
    BeforeInput("beforeinput" : web_sys::InputEvent),
    /// Event [`input`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/input_event)
    ///
    /// Fired when the `value` of an [`<input>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input), [`<select>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select), or [`<textarea>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea) element has been changed.
    Input("input" : web_sys::Event),
    /// Event [`change`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/change_event)
    ///
    /// Fired when the `value` of an [`<input>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input), [`<select>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select), or [`<textarea>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea) element has been changed and committed by the user. Unlike the [`input`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/input_event) event, the `change` event is not necessarily fired for each alteration to an element's `value`.
    Change("change" : web_sys::Event),

    // Pointer events
    /// Event [`gotpointercapture`](https://developer.mozilla.org/en-US/docs/Web/API/Element/gotpointercapture_event)
    ///
    /// Fired when an element captures a pointer using [`setPointerCapture()`](https://developer.mozilla.org/en-US/docs/Web/API/Element/setPointerCapture).
    GotPointerCapture("gotpointercapture" : web_sys::PointerEvent),
    /// Event [`lostpointercapture`](https://developer.mozilla.org/en-US/docs/Web/API/Element/lostpointercapture_event)
    ///
    /// Fired when a [captured pointer](https://developer.mozilla.org/en-US/docs/Web/API/Pointer_events#pointer_capture) is released.
    LostPointerCapture("lostpointercapture" : web_sys::PointerEvent),
    /// Event [`pointercancel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointercancel_event)
    ///
    /// Fired when a pointer event is canceled.
    PointerCancel("pointercancel" : web_sys::PointerEvent),
    /// Event [`pointerdown`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerdown_event)
    ///
    /// Fired when a pointer becomes active.
    PointerDown("pointerdown" : web_sys::PointerEvent),
    /// Event [`pointerenter`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerenter_event)
    ///
    /// Fired when a pointer is moved into the hit test boundaries of an element or one of its descendants.
    PointerEnter("pointerenter" : web_sys::PointerEvent),
    /// Event [`pointerleave`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerleave_event)
    ///
    /// Fired when a pointer is moved out of the hit test boundaries of an element.
    PointerLeave("pointerleave" : web_sys::PointerEvent),
    /// Event [`pointermove`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointermove_event)
    ///
    /// Fired when a pointer changes coordinates.
    PointerMove("pointermove" : web_sys::PointerEvent),
    /// Event [`pointerout`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerout_event)
    ///
    /// Fired when a pointer is moved out of the *hit test* boundaries of an element (among other reasons).
    PointerOut("pointerout" : web_sys::PointerEvent),
    /// Event [`pointerover`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerover_event)
    ///
    /// Fired when a pointer is moved into an element's hit test boundaries.
    PointerOver("pointerover" : web_sys::PointerEvent),
    /// Event [`pointerup`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerup_event)
    ///
    /// Fired when a pointer is no longer active.
    PointerUp("pointerup" : web_sys::PointerEvent),

    // Transition events
    /// Event [`transitioncancel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/transitioncancel_event)
    ///
    /// Fired when a [CSS transition](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Transitions/Using_CSS_transitions) is canceled.
    TransitionCancel("transitioncancel" : web_sys::TransitionEvent),
    /// Event [`transitionend`](https://developer.mozilla.org/en-US/docs/Web/API/Element/transitionend_event)
    ///
    /// Fired when a [CSS transition](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Transitions/Using_CSS_transitions) has completed.
    TransitionEnd("transitionend" : web_sys::TransitionEvent),
    /// Event [`transitionrun`](https://developer.mozilla.org/en-US/docs/Web/API/Element/transitionrun_event)
    ///
    /// Fired when a [CSS transition](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Transitions/Using_CSS_transitions) is first created.
    TransitionRun("transitionrun" : web_sys::TransitionEvent),
    /// Event [`transitionstart`](https://developer.mozilla.org/en-US/docs/Web/API/Element/transitionstart_event)
    ///
    /// Fired when a [CSS transition](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Transitions/Using_CSS_transitions) has actually started.
    TransitionStart("transitionstart" : web_sys::TransitionEvent),
}

def_event_types! {
    /// Event [`drag`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/drag_event)
    ///
    /// This event is fired when an element or text selection is being dragged.
    Drag("drag" : web_sys::Event),
    /// Event [`dragend`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragend_event)
    ///
    /// This event is fired when a drag operation is being ended (by releasing a mouse button or hitting the escape key).
    DragEnd("dragend" : web_sys::Event),
    /// Event [`dragenter`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragenter_event)
    ///
    /// This event is fired when a dragged element or text selection enters a valid drop target.
    DragEnter("dragenter" : web_sys::Event),
    /// Event [`dragleave`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragleave_event)
    ///
    /// This event is fired when a dragged element or text selection leaves a valid drop target.
    DragLeave("dragleave" : web_sys::Event),
    /// Event [`dragover`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragover_event)
    ///
    /// This event is fired continuously when an element or text selection is being dragged and the mouse pointer is over a valid drop target (every 50 ms WHEN mouse is not moving ELSE much faster between 5 ms (slow movement) and 1ms (fast movement) approximately. This firing pattern is different than [`mouseover`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseover_event) ).
    DragOver("dragover" : web_sys::Event),
    /// Event [`dragstart`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragstart_event)
    ///
    /// This event is fired when the user starts dragging an element or text selection.
    DragStart("dragstart" : web_sys::Event),
    /// Event [`drop`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/drop_event)
    ///
    /// This event is fired when an element or text selection is dropped on a valid drop target.
    Drop("drop" : web_sys::Event),
}

// https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement#events
def_event_types! {
    /// Event [`formdata`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/formdata_event)
    ///
    /// The `formdata` event fires after the entry list representing the form's data is constructed.
    FormData("formdata" : web_sys::Event),
    /// Event [`reset`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/reset_event)
    ///
    /// The `reset` event fires when a form is reset.
    Reset("reset" : web_sys::Event),
    /// Event [`submit`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/submit_event)
    ///
    /// The `submit` event fires when a form is submitted.
    Submit("submit" : web_sys::Event),
}

// https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement#events
def_event_types! {
    /// Event [`abort`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/abort_event)
    ///
    /// Fired when the resource was not fully loaded, but not as the result of an error.
    Abort("abort" : web_sys::Event),
    /// Event [`canplay`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/canplay_event)
    ///
    /// Fired when the user agent can play the media, but estimates that **not** enough data has been loaded to play the media up to its end without having to stop for further buffering of content.
    CanPlay("canplay" : web_sys::Event),
    /// Event [`canplaythrough`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/canplaythrough_event)
    ///
    /// Fired when the user agent can play the media, and estimates that enough data has been loaded to play the media up to its end without having to stop for further buffering of content.
    CanPlayThrough("canplaythrough" : web_sys::Event),
    /// Event [`durationchange`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/durationchange_event)
    ///
    /// Fired when the duration property has been updated.
    DurationChange("durationchange" : web_sys::Event),
    /// Event [`emptied`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/emptied_event)
    ///
    /// Fired when the media has become empty; for example, when the media has already been loaded (or partially loaded), and the [`HTMLMediaElement.load()`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/load) method is called to reload it.
    Emptied("emptied" : web_sys::Event),
    /// Event [`ended`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/ended_event)
    ///
    /// Fired when playback stops when end of the media (<audio> or <video>) is reached or because no further data is available.
    Ended("ended" : web_sys::Event),
    /// Event [`loadeddata`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loadeddata_event)
    ///
    /// Fired when the first frame of the media has finished loading.
    LoadedData("loadeddata" : web_sys::Event),
    /// Event [`loadedmetadata`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loadedmetadata_event)
    ///
    /// Fired when the metadata has been loaded.
    LoadedMetadata("loadedmetadata" : web_sys::Event),
    /// Event [`loadstart`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loadstart_event)
    ///
    /// Fired when the browser has started to load a resource.
    LoadStart("loadstart" : web_sys::Event),
    /// Event [`pause`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/pause_event)
    ///
    /// Fired when a request to pause play is handled and the activity has entered its paused state, most commonly occurring when the media's [`HTMLMediaElement.pause()`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/pause) method is called.
    Pause("pause" : web_sys::Event),
    /// Event [`play`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/play_event)
    ///
    /// Fired when the `paused` property is changed from `true` to `false`, as a result of the [`HTMLMediaElement.play()`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/play) method, or the `autoplay` attribute.
    Play("play" : web_sys::Event),
    /// Event [`playing`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/playing_event)
    ///
    /// Fired when playback is ready to start after having been paused or delayed due to lack of data.
    Playing("playing" : web_sys::Event),
    /// Event [`progress`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/progress_event)
    ///
    /// Fired periodically as the browser loads a resource.
    Progress("progress" : web_sys::Event),
    /// Event [`ratechange`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/ratechange_event)
    ///
    /// Fired when the playback rate has changed.
    RateChange("ratechange" : web_sys::Event),
    /// Event [`resize`]()
    ///
    /// Fired when one or both of the `videoWidth` and `videoHeight` properties have just been updated.
    Resize("resize" : web_sys::Event),
    /// Event [`seeked`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/seeked_event)
    ///
    /// Fired when a seek operation completes.
    Seeked("seeked" : web_sys::Event),
    /// Event [`seeking`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/seeking_event)
    ///
    /// Fired when a seek operation begins.
    Seeking("seeking" : web_sys::Event),
    /// Event [`stalled`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/stalled_event)
    ///
    /// Fired when the user agent is trying to fetch media data, but data is unexpectedly not forthcoming.
    Stalled("stalled" : web_sys::Event),
    /// Event [`suspend`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/suspend_event)
    ///
    /// Fired when the media data loading has been suspended.
    Suspend("suspend" : web_sys::Event),
    /// Event [`timeupdate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/timeupdate_event)
    ///
    /// Fired when the time indicated by the [`currentTime`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/currentTime) property has been updated.
    TimeUpdate("timeupdate" : web_sys::Event),
    /// Event [`volumechange`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/volumechange_event)
    ///
    /// Fired when the volume has changed.
    VolumeChange("volumechange" : web_sys::Event),
    /// Event [`waiting`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/waiting_event)
    ///
    /// Fired when playback has stopped because of a temporary lack of data.
    Waiting("waiting" : web_sys::Event),
}
