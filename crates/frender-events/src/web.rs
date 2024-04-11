pub use handle_js_cast_event::HandleJsCastEvent;

#[repr(transparent)]
pub struct Event<E: ?Sized>(pub(crate) E);

impl<E> Event<E> {
    pub fn new_from_ref(inner: &E) -> &Self {
        // SAFETY: Self is just a wrapper around the inner type,
        // therefore converting &Inner to &Self is safe.
        unsafe { &*(inner as *const E as *const Self) }
    }
}

mod handle_js_cast_event {
    use frender_common::HandleEvent;

    use super::Event;

    pub struct HandleJsCastEvent<E: ?Sized, F: ?Sized> {
        _e: std::marker::PhantomData<E>,
        f: F,
    }

    impl<E: ?Sized, F> From<F> for HandleJsCastEvent<E, F> {
        fn from(f: F) -> Self {
            Self::new(f)
        }
    }

    impl<E: ?Sized, F> HandleJsCastEvent<E, F> {
        pub fn new(f: F) -> Self {
            Self {
                _e: std::marker::PhantomData,
                f,
            }
        }
    }

    impl<E: ?Sized + wasm_bindgen::JsCast, F: ?Sized + HandleEvent<Event<E>>>
        HandleEvent<web_sys::Event> for HandleJsCastEvent<E, F>
    {
        fn handle_event(&mut self, event: &web_sys::Event) {
            use wasm_bindgen::JsCast;
            // TODO: check event type
            let event: &E = event.unchecked_ref();
            let event = Event::new_from_ref(event);
            self.f.handle_event(event)
        }
    }
}
