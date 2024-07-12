#[repr(transparent)]
pub struct Event<E: ?Sized>(pub(crate) E);

impl<E> Event<E> {
    pub fn new_from_ref(inner: &E) -> &Self {
        // SAFETY: Self is just a wrapper around the inner type,
        // therefore converting &Inner to &Self is safe.
        unsafe { &*(inner as *const E as *const Self) }
    }
}

/// By default, wrongly typed events are ignored.
///
/// KeyboardEvent and InputEvent should override this behavior,
/// because auto completing inputs on Chrome will emit `keydown` `keyup` `input` events with a raw [`Event`](web_sys::Event).
pub trait JsEventHandledBy {
    fn js_event_handled_by(event: &web_sys::Event, f: impl FnOnce(&Self));
}

mod imp_js_events {
    use wasm_bindgen::JsCast;

    use super::JsEventHandledBy;

    /// No casting is needed
    impl JsEventHandledBy for web_sys::Event {
        fn js_event_handled_by(event: &web_sys::Event, f: impl FnOnce(&Self)) {
            f(event)
        }
    }

    /// [`JsCast`], if failed, create a new KeyboardEvent.
    impl JsEventHandledBy for web_sys::KeyboardEvent {
        fn js_event_handled_by(event: &web_sys::Event, f: impl FnOnce(&Self)) {
            let event: &Self = if let Some(event) = event.dyn_ref() {
                event
            } else {
                &shims::new_keyboard_event_from(event)
            };

            f(event)
        }
    }

    /// [`JsCast`], if failed, create a new InputEvent.
    impl JsEventHandledBy for web_sys::InputEvent {
        fn js_event_handled_by(event: &web_sys::Event, f: impl FnOnce(&Self)) {
            let event: &Self = if let Some(event) = event.dyn_ref() {
                event
            } else {
                &shims::new_input_event_from(event)
            };

            f(event)
        }
    }

    /// [`JsCast`], if failed, ignore the event and warn on debug.
    trait JsCastEventHandledByJsCastOrIgnored: JsCast {}

    impl<E: JsCastEventHandledByJsCastOrIgnored> JsEventHandledBy for E {
        fn js_event_handled_by(event: &web_sys::Event, f: impl FnOnce(&Self)) {
            let event: &Self = if let Some(event) = event.dyn_ref() {
                event
            } else {
                #[cfg(debug_assertions)]
                web_sys::console::warn_4(
                    &"Event is ignored. The event is expected to be a ".into(),
                    &std::any::type_name::<Self>().into(),
                    &", but it is:".into(),
                    event,
                );

                return;
            };

            f(event)
        }
    }

    frender_macro_rules::impl_many!(
        impl<__> JsCastEventHandledByJsCastOrIgnored
            for each_of![
                web_sys::SecurityPolicyViolationEvent,
                web_sys::UiEvent,
                web_sys::MouseEvent,
                web_sys::WheelEvent,
                web_sys::PointerEvent,
                web_sys::CompositionEvent,
                web_sys::FocusEvent,
                web_sys::TouchEvent,
                web_sys::TransitionEvent,
                web_sys::AnimationEvent,
            ]
        {
        }
    );

    mod shims {
        use wasm_bindgen::prelude::*;
        use web_sys::js_sys::JsString;

        #[wasm_bindgen]
        extern "C" {
            type Event;
            #[wasm_bindgen(structural, method, getter, js_class = "Event", js_name = "type")]
            fn type_(this: &Event) -> JsString;

            type KeyboardEvent;
            #[wasm_bindgen(constructor, js_class = "KeyboardEvent")]
            fn new(type_arg: JsString, options: &JsValue) -> KeyboardEvent;

            type InputEvent;
            #[wasm_bindgen(constructor, js_class = "InputEvent")]
            fn new(type_arg: JsString, options: &JsValue) -> InputEvent;
        }

        fn event_type(e: &web_sys::Event) -> JsString {
            e.unchecked_ref::<Event>().type_()
        }

        pub(super) fn new_keyboard_event_from(event: &web_sys::Event) -> web_sys::KeyboardEvent {
            KeyboardEvent::new(event_type(event), event).unchecked_into()
        }

        pub(super) fn new_input_event_from(event: &web_sys::Event) -> web_sys::InputEvent {
            InputEvent::new(event_type(event), event).unchecked_into()
        }
    }
}

pub trait JsCastEventType: crate::HasEventTypeName + crate::event_types::EventType {
    type JsEventTarget;
    type JsCastEvent: JsEventHandledBy;

    // let event = Event::new_from_ref(event);
    fn js_event_as_event(event: &Self::JsCastEvent) -> &Self::Event;

    fn handle_js_cast_event(event: &web_sys::Event, f: impl FnOnce(&Self::Event)) {
        Self::JsCastEvent::js_event_handled_by(event, |event| f(Self::js_event_as_event(event)))
    }
}
