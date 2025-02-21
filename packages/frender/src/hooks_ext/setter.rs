use std::borrow::Cow;

use frender_common::{HandleEvent, MaybeHandleEvent};
use hooks::ShareValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventTargetFormControlValue;

#[derive(Debug)]
pub struct SetEventTargetFormControlValue<S: ShareValue>(pub S);

impl<S: ShareValue> SetEventTargetFormControlValue<S>
where
    // S::Value: for<'a> From<Cow<'a, str>>,
    for<'a> Cow<'a, str>: Into<S::Value>,
{
    pub fn handle_event_by_ref<E: ?Sized + frender_events::event::Event>(&self, event: &E) {
        if let Some(value) = event.target_form_control_value().map(Into::into) {
            // only update when the event target is form control
            self.0.set(value)
        }
    }

    pub fn into_fn<E: ?Sized + frender_events::event::Event>(self) -> impl Fn(&E) {
        move |e| self.handle_event_by_ref(e)
    }
}

impl<E: ?Sized + frender_events::event::Event, S: ShareValue> HandleEvent<E>
    for SetEventTargetFormControlValue<S>
where
    // S::Value: for<'a> From<Cow<'a, str>>,
    for<'a> Cow<'a, str>: Into<S::Value>,
{
    fn handle_event(&mut self, event: &E) {
        self.handle_event_by_ref(event)
    }
}

impl<E: ?Sized + frender_events::event::Event, S: ShareValue> MaybeHandleEvent<E>
    for SetEventTargetFormControlValue<S>
where
    // S::Value: for<'a> From<Cow<'a, str>>,
    for<'a> Cow<'a, str>: Into<S::Value>,
{
    type HandleEvent = Self;
}
