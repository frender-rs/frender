#[repr(transparent)]
pub struct Event<E: ?Sized>(pub(crate) E);

impl<E> Event<E> {
    pub fn new_from_ref(inner: &E) -> &Self {
        // SAFETY: Self is just a wrapper around the inner type,
        // therefore converting &Inner to &Self is safe.
        unsafe { &*(inner as *const E as *const Self) }
    }
}

pub trait JsCastEventType: crate::HasEventTypeName + crate::event_types::EventType {
    type JsEventTarget;
    type JsCastEvent: wasm_bindgen::JsCast;

    // let event = Event::new_from_ref(event);
    fn js_event_as_event(event: &Self::JsCastEvent) -> &Self::Event;
}
