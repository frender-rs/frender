use frender_events::web::JsCastEventType;

use std::marker::PhantomPinned;

use frender_csr::event_listener::{HandleEvent, RegisterOrUpdate};

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

    use frender_csr::event_listener::{HandleEvent, RegisterUpdate};
    use frender_events::web::JsCastEventType;

    use super::handle_js_cast_event::HandleJsCastEvent;

    /// An updatable EventListener.
    #[derive(Debug)]
    pub struct EventListener<F: ?Sized> {
        _event_listener: gloo_events::EventListener,
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
    pub struct EventListenerOfType<F, ET: ?Sized + JsCastEventType> {
        pub(super) inner: EventListener<super::handle_js_cast_event::HandleJsCastEvent<ET, F>>,
    }

    impl<
            N: AsRef<web_sys::EventTarget>,
            R: ?Sized,
            F: HandleEvent<ET::Event> + 'static,
            ET: ?Sized + JsCastEventType + 'static,
        > RegisterUpdate<super::super::Node<N>, R, F> for EventListenerOfType<F, ET>
    {
        fn register(node: &mut super::super::Node<N>, _: &mut R, f: F) -> Self {
            let target: &web_sys::EventTarget = node.0.as_ref();
            let f = HandleJsCastEvent::new(f);
            Self {
                inner: EventListener::new(target, ET::EVENT_TYPE_NAME, f),
            }
        }

        fn update(&mut self, _: &mut super::super::Node<N>, _: &mut R, f: F) {
            let f = HandleJsCastEvent::new(f);
            // TODO: what if the event target is not the same
            self.inner.update(f)
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
        inner: Option<unpinned::EventListener<F>>,
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
    > RegisterOrUpdate<super::Node<N>, R, F> for MaybeEventListenerOfType<F, ET>
{
    fn register_or_update(
        self: std::pin::Pin<&mut Self>,
        element: &mut super::Node<N>,
        _: &mut R,
        f: F,
    ) {
        {
            let this = self.project().inner.project().inner;
            let f = handle_js_cast_event::HandleJsCastEvent::new(f);

            if let Some(this) = this {
                this.update(f)
            } else {
                let target: &web_sys::EventTarget = element.0.as_ref();

                *this = Some(unpinned::EventListener::new(target, ET::EVENT_TYPE_NAME, f))
            }
        }
    }
}
