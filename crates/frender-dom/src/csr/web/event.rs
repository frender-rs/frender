use web_sys::js_sys::Function;
use web_sys::EventTarget;

pub struct EventListenerPreventDefault<E: AsRef<str>> {
    target: EventTarget,
    event_type: E,
}

thread_local!(
    static PREVENT_DEFAULT: Function = Function::new_with_args("e", "e.preventDefault()");
);

impl<E: AsRef<str>> EventListenerPreventDefault<E> {
    pub(crate) fn new(target: EventTarget, event_type: E) -> Self {
        PREVENT_DEFAULT.with(|callback| {
            if let Err(error) =
                target.add_event_listener_with_callback(event_type.as_ref(), callback)
            {
                web_sys::console::warn_6(
                    &"addEventListener failed".into(),
                    &error,
                    &"target =".into(),
                    &target,
                    &"event =".into(),
                    &event_type.as_ref().into(),
                );
            }
            Self { target, event_type }
        })
    }
}

impl<E: AsRef<str>> Drop for EventListenerPreventDefault<E> {
    fn drop(&mut self) {
        PREVENT_DEFAULT.with(|callback| {
            if let Err(error) = self
                .target
                .remove_event_listener_with_callback(self.event_type.as_ref(), callback)
            {
                web_sys::console::warn_6(
                    &"removeEventListener failed".into(),
                    &error,
                    &"target =".into(),
                    &self.target,
                    &"event =".into(),
                    &self.event_type.as_ref().into(),
                );
            }
        })
    }
}
