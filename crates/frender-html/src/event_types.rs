macro_rules! for_each_trait {
    ({$($vis:vis trait $WithEventsTrait:ident $body:tt)*} $commands:tt) => {
        $(crate::expand! {
            {$vis trait $WithEventsTrait $body}
            do $commands
        })*
    };
}

macro_rules! define_element_trait {
    (
        $(#[super_traits($($super_traits:ident),+ $(,)?)])?
        $vis:vis trait $WithEventsTrait:ident {
            $(
                const $on_event:ident : $event_trait_name:ident = (
                    $event_name:literal,
                    $type_event:ident,
                    $type_event_listener:ident $(,)?
                );
            )*

            $(fn sub_traits() {
                $($sub_traits:tt)*
            })?
        }
    ) => {
        $vis trait $WithEventsTrait<R: ?Sized>
        $(: $($super_traits)++ )?
        {
            $(
                type $type_event: crate::event::$event_trait_name + 'static;
                type $type_event_listener;

                fn $on_event(
                    &mut self,
                    renderer: &mut R,
                    listener: impl FnMut(&Self::$type_event) + 'static,
                ) -> Self::$type_event_listener;
            )*
        }

        #[cfg(feature = "web")]
        impl<N: AsRef<web_sys::Element>, R: ?Sized> $WithEventsTrait<R> for crate::csr::web::Node<N> {
            $(
                type $type_event = crate::csr::web::Event<web_sys::$event_trait_name>;
                type $type_event_listener = gloo_events::EventListener;

                fn $on_event(
                    &mut self,
                    _: &mut R,
                    mut listener: impl FnMut(&Self::$type_event) + 'static,
                ) -> Self::$type_event_listener {
                    let element: &web_sys::Element = self.0.as_ref();

                    gloo_events::EventListener::new(
                        element,
                        <crate::event_types::event_types::$on_event as crate::event::HasEventTypeName>::EVENT_TYPE_NAME,
                        move |event| {
                            use wasm_bindgen::JsCast;
                            let event = event.unchecked_ref();
                            listener(crate::csr::web::Event::new_from_ref(event))
                        },
                    )
                }
            )*
        }

        for_each_trait! {{$($($sub_traits)*)?}{
            prepend($(#[super_traits($($super_traits,)+ $WithEventsTrait)])?)
            wrap{}
            prepend(define_element_trait!)
        }}
    };
}

macro_rules! define_event_types {
    (

        $(#[super_traits($($super_traits:ident),+ $(,)?)])?
        $vis:vis trait $WithEventsTrait:ident {
            $(
                const $on_event:ident : $event_trait_name:ident = (
                    $event_name:literal,
                    $type_event:ident,
                    $type_event_listener:ident $(,)?
                );
            )*

            $(fn sub_traits() {
                $($sub_traits:tt)*
            })?
        }
    ) => {
        $(
            pub enum $on_event {}

            impl crate::event::HasEventTypeName for $on_event {
                const EVENT_TYPE_NAME: &'static str = $event_name;
            }

            crate::event::type_traits_impl::$event_trait_name! {
                $on_event,
                $WithEventsTrait,
                $type_event,
                $type_event_listener
            }
        )*

        for_each_trait! {{$($($sub_traits)*)?}{
            // prepend($(#[super_traits($($super_traits,)+ $WithEventsTrait)])?)
            wrap{}
            prepend(define_event_types!)
        }}
    };
}

macro_rules! define_type_of {
    (

        $(#[super_traits($($super_traits:ident),+ $(,)?)])?
        $vis:vis trait $WithEventsTrait:ident {
            $(
                const $on_event:ident : $event_trait_name:ident = (
                    $event_name:literal,
                    $type_event:ident,
                    $type_event_listener:ident $(,)?
                );
            )*

            $(fn sub_traits() {
                $($sub_traits:tt)*
            })?
        }
    ) => {
        $(
            pub mod $on_event {
                pub use crate::event::$event_trait_name as Event;

                pub type EventOf<E, R> = <E as crate::event_types::$WithEventsTrait<R>>::$type_event;
                pub type EventListenerOf<E, R> = <E as crate::event_types::$WithEventsTrait<R>>::$type_event_listener;

                pub fn on<
                    'event,
                    E: ?Sized + crate::event_types::$WithEventsTrait<R>,
                    R: ?Sized,
                    C: 'static
                        + Clone
                        + for<'e> frender_events::callable::Callable<(&'e (dyn Event + 'event),), Output = ()>,
                >(
                    element: &mut E,
                    renderer: &mut R,
                    callable: &C,
                ) -> E::$type_event_listener {
                    E::$on_event(element, renderer, {
                        let callable = (*callable).clone();
                        move |ev: &_| frender_events::callable::Callable::call_fn(&callable, (ev,))
                    })
                }
            }
        )*

        for_each_trait! {{$($($sub_traits)*)?}{
            // prepend($(#[super_traits($($super_traits,)+ $WithEventsTrait)])?)
            wrap{}
            prepend(define_type_of!)
        }}
    };
}

macro_rules! define {
    ($($t:tt)*) => {
        define_element_trait! { $($t)* }

        #[allow(non_camel_case_types)]
        pub mod event_types {
            define_event_types! { $($t)* }
        }

        pub mod type_of_event {
            define_type_of! { $($t)* }
        }
    };
}

define!(
    pub trait ElementWithEvents {
        const on_cancel: Event = ("cancel", OnCancelEvent, OnCancelEventListener);
        const on_error: Event = ("error", OnErrorEvent, OnErrorEventListener);
        const on_scroll: Event = ("scroll", OnScrollEvent, OnScrollEventListener);
        const on_security_policy_violation: SecurityPolicyViolationEvent = ("securitypolicyviolation", OnSecurityPolicyViolationEvent, OnSecurityPolicyViolationEventListener);
        const on_select: Event = ("select", OnSelectEvent, OnSelectEventListener);
        const on_wheel: WheelEvent = ("wheel", OnWheelEvent, OnWheelEventListener);
        const on_copy: Event = ("copy", OnCopyEvent, OnCopyEventListener);
        const on_cut: Event = ("cut", OnCutEvent, OnCutEventListener);
        const on_paste: Event = ("paste", OnPasteEvent, OnPasteEventListener);
        const on_composition_end: CompositionEvent = ("compositionend", OnCompositionEndEvent, OnCompositionEndEventListener);
        const on_composition_start: CompositionEvent = ("compositionstart", OnCompositionStartEvent, OnCompositionStartEventListener);
        const on_composition_update: CompositionEvent = ("compositionupdate", OnCompositionUpdateEvent, OnCompositionUpdateEventListener);
        const on_blur: FocusEvent = ("blur", OnBlurEvent, OnBlurEventListener);
        const on_focus: FocusEvent = ("focus", OnFocusEvent, OnFocusEventListener);
        const on_focus_in: FocusEvent = ("focusin", OnFocusInEvent, OnFocusInEventListener);
        const on_focus_out: FocusEvent = ("focusout", OnFocusOutEvent, OnFocusOutEventListener);
        const on_fullscreen_change: Event = ("fullscreenchange", OnFullscreenChangeEvent, OnFullscreenChangeEventListener);
        const on_fullscreen_error: Event = ("fullscreenerror", OnFullscreenErrorEvent, OnFullscreenErrorEventListener);
        const on_key_down: KeyboardEvent = ("keydown", OnKeyDownEvent, OnKeyDownEventListener);
        const on_key_up: KeyboardEvent = ("keyup", OnKeyUpEvent, OnKeyUpEventListener);
        const on_aux_click: MouseEvent = ("auxclick", OnAuxClickEvent, OnAuxClickEventListener);
        const on_click: MouseEvent = ("click", OnClickEvent, OnClickEventListener);
        const on_context_menu: MouseEvent = ("contextmenu", OnContextMenuEvent, OnContextMenuEventListener);
        const on_double_click: MouseEvent = ("dblclick", OnDoubleClickEvent, OnDoubleClickEventListener);
        const on_mouse_down: MouseEvent = ("mousedown", OnMouseDownEvent, OnMouseDownEventListener);
        const on_mouse_enter: MouseEvent = ("mouseenter", OnMouseEnterEvent, OnMouseEnterEventListener);
        const on_mouse_leave: MouseEvent = ("mouseleave", OnMouseLeaveEvent, OnMouseLeaveEventListener);
        const on_mouse_move: MouseEvent = ("mousemove", OnMouseMoveEvent, OnMouseMoveEventListener);
        const on_mouse_out: MouseEvent = ("mouseout", OnMouseOutEvent, OnMouseOutEventListener);
        const on_mouse_over: MouseEvent = ("mouseover", OnMouseOverEvent, OnMouseOverEventListener);
        const on_mouse_up: MouseEvent = ("mouseup", OnMouseUpEvent, OnMouseUpEventListener);
        const on_touch_cancel: TouchEvent = ("touchcancel", OnTouchCancelEvent, OnTouchCancelEventListener);
        const on_touch_end: TouchEvent = ("touchend", OnTouchEndEvent, OnTouchEndEventListener);
        const on_touch_move: TouchEvent = ("touchmove", OnTouchMoveEvent, OnTouchMoveEventListener);
        const on_touch_start: TouchEvent = ("touchstart", OnTouchStartEvent, OnTouchStartEventListener);

        fn sub_traits() {
            pub trait HtmlElementWithEvents {
                const on_invalid: Event = ("invalid", OnInvalidEvent, OnInvalidEventListener);
                const on_animation_cancel: AnimationEvent = ("animationcancel", OnAnimationCancelEvent, OnAnimationCancelEventListener);
                const on_animation_end: AnimationEvent = ("animationend", OnAnimationEndEvent, OnAnimationEndEventListener);
                const on_animation_iteration: AnimationEvent = ("animationiteration", OnAnimationIterationEvent, OnAnimationIterationEventListener);
                const on_animation_start: AnimationEvent = ("animationstart", OnAnimationStartEvent, OnAnimationStartEventListener);
                const on_before_input: InputEvent = ("beforeinput", OnBeforeInputEvent, OnBeforeInputEventListener);
                const on_input: Event = ("input", OnInputEvent, OnInputEventListener);
                const on_change: Event = ("change", OnChangeEvent, OnChangeEventListener);
                const on_got_pointer_capture: PointerEvent = ("gotpointercapture", OnGotPointerCaptureEvent, OnGotPointerCaptureEventListener);
                const on_lost_pointer_capture: PointerEvent = ("lostpointercapture", OnLostPointerCaptureEvent, OnLostPointerCaptureEventListener);
                const on_pointer_cancel: PointerEvent = ("pointercancel", OnPointerCancelEvent, OnPointerCancelEventListener);
                const on_pointer_down: PointerEvent = ("pointerdown", OnPointerDownEvent, OnPointerDownEventListener);
                const on_pointer_enter: PointerEvent = ("pointerenter", OnPointerEnterEvent, OnPointerEnterEventListener);
                const on_pointer_leave: PointerEvent = ("pointerleave", OnPointerLeaveEvent, OnPointerLeaveEventListener);
                const on_pointer_move: PointerEvent = ("pointermove", OnPointerMoveEvent, OnPointerMoveEventListener);
                const on_pointer_out: PointerEvent = ("pointerout", OnPointerOutEvent, OnPointerOutEventListener);
                const on_pointer_over: PointerEvent = ("pointerover", OnPointerOverEvent, OnPointerOverEventListener);
                const on_pointer_up: PointerEvent = ("pointerup", OnPointerUpEvent, OnPointerUpEventListener);
                const on_transition_cancel: TransitionEvent = ("transitioncancel", OnTransitionCancelEvent, OnTransitionCancelEventListener);
                const on_transition_end: TransitionEvent = ("transitionend", OnTransitionEndEvent, OnTransitionEndEventListener);
                const on_transition_run: TransitionEvent = ("transitionrun", OnTransitionRunEvent, OnTransitionRunEventListener);
                const on_transition_start: TransitionEvent = ("transitionstart", OnTransitionStartEvent, OnTransitionStartEventListener);
                const on_drag: Event = ("drag", OnDragEvent, OnDragEventListener);
                const on_drag_end: Event = ("dragend", OnDragEndEvent, OnDragEndEventListener);
                const on_drag_enter: Event = ("dragenter", OnDragEnterEvent, OnDragEnterEventListener);
                const on_drag_leave: Event = ("dragleave", OnDragLeaveEvent, OnDragLeaveEventListener);
                const on_drag_over: Event = ("dragover", OnDragOverEvent, OnDragOverEventListener);
                const on_drag_start: Event = ("dragstart", OnDragStartEvent, OnDragStartEventListener);
                const on_drop: Event = ("drop", OnDropEvent, OnDropEventListener);

                fn sub_traits() {
                    pub trait HtmlMediaElementWithEvents {
                        const on_abort: Event = ("abort", OnAbortEvent, OnAbortEventListener);
                        const on_can_play: Event = ("canplay", OnCanPlayEvent, OnCanPlayEventListener);
                        const on_can_play_through: Event = ("canplaythrough", OnCanPlayThroughEvent, OnCanPlayThroughEventListener);
                        const on_duration_change: Event = ("durationchange", OnDurationChangeEvent, OnDurationChangeEventListener);
                        const on_emptied: Event = ("emptied", OnEmptiedEvent, OnEmptiedEventListener);
                        const on_ended: Event = ("ended", OnEndedEvent, OnEndedEventListener);
                        const on_loaded_data: Event = ("loadeddata", OnLoadedDataEvent, OnLoadedDataEventListener);
                        const on_loaded_metadata: Event = ("loadedmetadata", OnLoadedMetadataEvent, OnLoadedMetadataEventListener);
                        const on_load_start: Event = ("loadstart", OnLoadStartEvent, OnLoadStartEventListener);
                        const on_pause: Event = ("pause", OnPauseEvent, OnPauseEventListener);
                        const on_play: Event = ("play", OnPlayEvent, OnPlayEventListener);
                        const on_playing: Event = ("playing", OnPlayingEvent, OnPlayingEventListener);
                        const on_progress: Event = ("progress", OnProgressEvent, OnProgressEventListener);
                        const on_rate_change: Event = ("ratechange", OnRateChangeEvent, OnRateChangeEventListener);
                        const on_resize: Event = ("resize", OnResizeEvent, OnResizeEventListener);
                        const on_seeked: Event = ("seeked", OnSeekedEvent, OnSeekedEventListener);
                        const on_seeking: Event = ("seeking", OnSeekingEvent, OnSeekingEventListener);
                        const on_stalled: Event = ("stalled", OnStalledEvent, OnStalledEventListener);
                        const on_suspend: Event = ("suspend", OnSuspendEvent, OnSuspendEventListener);
                        const on_time_update: Event = ("timeupdate", OnTimeUpdateEvent, OnTimeUpdateEventListener);
                        const on_volume_change: Event = ("volumechange", OnVolumeChangeEvent, OnVolumeChangeEventListener);
                        const on_waiting: Event = ("waiting", OnWaitingEvent, OnWaitingEventListener);
                    }

                    pub trait HtmlFormElementWithEvents {
                        const on_form_data: Event = ("formdata", OnFormDataEvent, OnFormDataEventListener);
                        const on_reset: Event = ("reset", OnResetEvent, OnResetEventListener);
                        const on_submit: Event = ("submit", OnSubmitEvent, OnSubmitEventListener);
                    }
                }
            }
        }
    }
);
