use std::{borrow::Cow, marker::PhantomPinned, pin::Pin};

use frender_csr::event_listener::{EventListenerState, HandleEvent, RegisterOrUpdate};

pub mod unpinned {
    use std::{borrow::Cow, cell::RefCell, rc::Rc};

    use frender_csr::event_listener::HandleEvent;

    /// An updatable EventListener.
    #[derive(Debug)]
    pub struct EventListener<F: ?Sized> {
        _event_listener: gloo_events::EventListener,
        // TODO: maybe this can be implemented with self-referential structs without Rc. See https://doc.rust-lang.org/nightly/std/pin/index.html#a-self-referential-struct
        f: Rc<RefCell<F>>,
    }

    impl<F: ?Sized> Unpin for EventListener<F> {}

    impl<F: HandleEvent<web_sys::Event> + 'static> EventListener<F> {
        pub(super) fn new<S: Into<Cow<'static, str>>>(
            target: &web_sys::EventTarget,
            event_type: S,
            f: F,
        ) -> Self {
            let f = Rc::new(RefCell::new(f));
            Self {
                _event_listener: gloo_events::EventListener::new(target, event_type, {
                    let f = Rc::clone(&f);
                    move |event| {
                        let mut f = f.borrow_mut();
                        f.handle_event(event)
                    }
                }),
                f,
            }
        }
    }

    impl<F> EventListener<F> {
        pub(super) fn update(&self, f: F) {
            *self.f.borrow_mut() = f;
        }
    }

    #[derive(Debug)]
    pub struct MaybeEventListener<F: ?Sized>(pub Option<EventListener<F>>);

    impl<F: ?Sized> Default for MaybeEventListener<F> {
        fn default() -> Self {
            Self(None)
        }
    }

    impl<F: HandleEvent<web_sys::Event> + 'static> MaybeEventListener<F> {
        pub(super) fn register_or_update<S: Into<Cow<'static, str>>>(
            &mut self,
            target: &web_sys::EventTarget,
            event_type: S,
            f: F,
        ) {
            if let Some(this) = &self.0 {
                this.update(f)
            } else {
                self.0 = Some(EventListener::new(target, event_type, f))
            }
        }
    }
}

pin_project_lite::pin_project!(
    /// An updatable EventListener.
    #[derive(Debug)]
    pub struct MaybeEventListener<F: ?Sized> {
        // marked as !Unpin for future optimization.
        #[pin]
        _pin: PhantomPinned,
        // TODO: maybe this can be implemented with self-referential structs without Rc. See https://doc.rust-lang.org/nightly/std/pin/index.html#a-self-referential-struct
        inner: unpinned::MaybeEventListener<F>,
    }
);

impl<F: ?Sized> Default for MaybeEventListener<F> {
    fn default() -> Self {
        Self {
            _pin: PhantomPinned,
            inner: Default::default(),
        }
    }
}

impl<F: HandleEvent<web_sys::Event> + 'static> MaybeEventListener<F> {
    fn register_or_update<S: Into<Cow<'static, str>>>(
        self: Pin<&mut Self>,
        target: &web_sys::EventTarget,
        event_type: S,
        f: F,
    ) {
        self.project()
            .inner
            .register_or_update(target, event_type, f)
    }
}

impl<
        N: AsRef<web_sys::EventTarget>,
        R: ?Sized,
        F: HandleEvent<web_sys::Event> + 'static + From<H>,
        H,
    > EventListenerState<super::Node<N>, R, H> for MaybeEventListener<F>
{
    type EventListenerStateUnpinned = unpinned::MaybeEventListener<F>;
}

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

impl<E: ?Sized + wasm_bindgen::JsCast, F: ?Sized + HandleEvent<crate::csr::web::Event<E>>>
    HandleEvent<web_sys::Event> for HandleJsCastEvent<E, F>
{
    fn handle_event(&mut self, event: &web_sys::Event) {
        use wasm_bindgen::JsCast;
        // TODO: check event type
        let event: &E = event.unchecked_ref();
        let event = crate::csr::web::Event::new_from_ref(event);
        self.f.handle_event(event)
    }
}

impl<
        N: AsRef<web_sys::EventTarget>,
        R: ?Sized,
        F: HandleEvent<web_sys::Event> + 'static + From<H>,
        H,
    > RegisterOrUpdate<super::Node<N>, R, H> for unpinned::MaybeEventListener<F>
{
    fn register_or_update(
        self: std::pin::Pin<&mut Self>,
        element: &mut super::Node<N>,
        _: &mut R,
        event_type: impl Into<std::borrow::Cow<'static, str>>,
        f: H,
    ) {
        let target: &web_sys::EventTarget = element.0.as_ref();
        self.get_mut()
            .register_or_update(target, event_type, f.into())
    }
}

impl<
        N: AsRef<web_sys::EventTarget>,
        R: ?Sized,
        F: HandleEvent<web_sys::Event> + 'static + From<H>,
        H,
    > RegisterOrUpdate<super::Node<N>, R, H> for MaybeEventListener<F>
{
    fn register_or_update(
        self: std::pin::Pin<&mut Self>,
        element: &mut super::Node<N>,
        _: &mut R,
        event_type: impl Into<std::borrow::Cow<'static, str>>,
        f: H,
    ) {
        let target: &web_sys::EventTarget = element.0.as_ref();
        MaybeEventListener::register_or_update(self, target, event_type, f.into())
    }
}
