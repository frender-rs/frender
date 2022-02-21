use std::marker::PhantomData;

use web_sys::EventTarget;

use super::js;

pub struct SyntheticEvent<
    TCurrent: AsRef<EventTarget> = js::native::Element,
    TEvent = js::native::Event,
> {
    base: js::BaseSyntheticEvent,
    _phantom: PhantomData<(TCurrent, TEvent)>,
}

pub struct UiEvent<TCurrent: AsRef<EventTarget> = js::native::Element, TEvent = js::native::UiEvent>(
    SyntheticEvent<TCurrent, TEvent>,
);
