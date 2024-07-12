use frender_events::web::JsCastEventType;

use std::{borrow::Cow, marker::PhantomPinned, pin::Pin};

use frender_csr::event_listener::{EventListenerState, HandleEvent, RegisterOrUpdate};

mod handle_js_cast_event {
    use frender_common::HandleEvent;
    use frender_events::web::JsCastEventType;

    use crate::event_types::EventType;

    #[derive(Debug)]
    pub(super) struct HandleJsCastEvent<ET: ?Sized, F: ?Sized> {
        _e: std::marker::PhantomData<ET>,
        f: F,
    }

    impl<ET: ?Sized, F> HandleJsCastEvent<ET, F> {
        pub(super) fn new(f: F) -> Self {
            Self {
                _e: std::marker::PhantomData,
                f,
            }
        }
    }

    impl<E: ?Sized + JsCastEventType + EventType, F: ?Sized + HandleEvent<E::Event>>
        HandleEvent<web_sys::Event> for HandleJsCastEvent<E, F>
    {
        fn handle_event(&mut self, event: &web_sys::Event) {
            E::handle_js_cast_event(event, |event| self.f.handle_event(event))
        }

        fn event_listener_options(&self) -> frender_common::EventListenerOptions {
            self.f.event_listener_options()
        }
    }
}

pub mod unpinned {
    use std::{borrow::Cow, cell::RefCell, rc::Rc};

    use frender_csr::event_listener::HandleEvent;
    use frender_events::web::JsCastEventType;

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
            let options = f.event_listener_options();
            let f = Rc::new(RefCell::new(f));
            let callback = {
                let f = Rc::clone(&f);
                move |event: &_| {
                    let mut f = f.borrow_mut();
                    f.handle_event(event)
                }
            };
            Self {
                _event_listener: if options.is_default() {
                    gloo_events::EventListener::new(target, event_type, callback)
                } else {
                    gloo_events::EventListener::new_with_options(
                        target,
                        event_type,
                        gloo_events::EventListenerOptions {
                            phase: if options.capture {
                                gloo_events::EventListenerPhase::Capture
                            } else {
                                gloo_events::EventListenerPhase::Bubble
                            },
                            passive: options.passive,
                        },
                        callback,
                    )
                },
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
                // TODO: what if the event target is not the same
                this.update(f)
            } else {
                self.0 = Some(EventListener::new(target, event_type, f))
            }
        }
    }

    #[derive(Debug)]
    pub struct MaybeEventListenerOfType<F, ET: ?Sized + JsCastEventType> {
        pub(super) inner: MaybeEventListener<super::handle_js_cast_event::HandleJsCastEvent<ET, F>>,
    }

    impl<F, ET: ?Sized + JsCastEventType> Unpin for MaybeEventListenerOfType<F, ET> {}

    impl<F, ET: ?Sized + JsCastEventType> Default for MaybeEventListenerOfType<F, ET> {
        fn default() -> Self {
            Self {
                inner: Default::default(),
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

pin_project_lite::pin_project!(
    #[derive(Debug)]
    pub struct MaybeEventListenerOfType<F, ET: ?Sized> {
        #[pin]
        inner: MaybeEventListener<handle_js_cast_event::HandleJsCastEvent<ET, F>>,
    }
);

impl<F, ET: ?Sized> Default for MaybeEventListenerOfType<F, ET> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<
        N: AsRef<web_sys::EventTarget>,
        R: ?Sized,
        F: HandleEvent<ET::Event> + 'static,
        ET: ?Sized + JsCastEventType + 'static,
    > EventListenerState<super::Node<N>, R, F> for MaybeEventListenerOfType<F, ET>
{
    type EventListenerStateUnpinned = unpinned::MaybeEventListenerOfType<F, ET>;
}

impl<
        N: AsRef<web_sys::EventTarget>,
        R: ?Sized,
        F: HandleEvent<ET::Event> + 'static,
        ET: ?Sized + JsCastEventType + 'static,
    > RegisterOrUpdate<super::Node<N>, R, F> for unpinned::MaybeEventListenerOfType<F, ET>
{
    fn register_or_update(
        self: std::pin::Pin<&mut Self>,
        element: &mut super::Node<N>,
        _: &mut R,
        f: F,
    ) {
        let target: &web_sys::EventTarget = element.0.as_ref();
        self.get_mut().inner.register_or_update(
            target,
            ET::EVENT_TYPE_NAME,
            handle_js_cast_event::HandleJsCastEvent::new(f),
        )
    }
}

impl<
        N: AsRef<web_sys::EventTarget>,
        R: ?Sized,
        F: HandleEvent<ET::Event> + 'static,
        ET: ?Sized + JsCastEventType + 'static,
    > RegisterOrUpdate<super::Node<N>, R, F> for MaybeEventListenerOfType<F, ET>
{
    fn register_or_update(
        self: std::pin::Pin<&mut Self>,
        element: &mut super::Node<N>,
        _: &mut R,
        f: F,
    ) {
        let target: &web_sys::EventTarget = element.0.as_ref();
        MaybeEventListener::register_or_update(
            self.project().inner,
            target,
            ET::EVENT_TYPE_NAME,
            handle_js_cast_event::HandleJsCastEvent::new(f),
        )
    }
}
