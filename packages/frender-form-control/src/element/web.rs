use frender_events::{
    event::Event, event_types::EventType, web::JsCastEventType, HasEventTypeName,
};

use frender_dom::csr::{self, web::event_listener::unpinned};

use super::{
    super::value::HandleFormControlValue, FormControlElement, HandleFormControlValueChange,
};

pub enum Input {}

impl EventType for Input {
    type Event = dyn Event;
}

impl HasEventTypeName for Input {
    const EVENT_TYPE_NAME: &'static str = "input";
}

impl JsCastEventType for Input {
    type JsEventTarget = web_sys::EventTarget;
    type JsCastEvent = web_sys::Event;

    fn js_event_as_event(event: &Self::JsCastEvent) -> &Self::Event {
        csr::web::Event::new_from_ref(event)
    }
}

pub enum Change {}

impl EventType for Change {
    type Event = dyn Event;
}

impl HasEventTypeName for Change {
    const EVENT_TYPE_NAME: &'static str = "change";
}

impl JsCastEventType for Change {
    type JsEventTarget = web_sys::EventTarget;
    type JsCastEvent = web_sys::Event;

    fn js_event_as_event(event: &Self::JsCastEvent) -> &Self::Event {
        csr::web::Event::new_from_ref(event)
    }
}

impl<Renderer: ?Sized + csr::web::Renderer> FormControlElement<str, Renderer>
    for csr::web::Node<
        //
        web_sys::HtmlTextAreaElement,
    >
{
    fn set_value(&mut self, _: &mut Renderer, value: &str) {
        AsRef::<web_sys::HtmlTextAreaElement>::as_ref(&self.0).set_value(value)
    }

    fn set_default_value(&mut self, _: &mut Renderer, value: &str) {
        use web_sys::wasm_bindgen::UnwrapThrowExt;

        AsRef::<web_sys::HtmlTextAreaElement>::as_ref(&self.0)
            .set_default_value(value)
            .unwrap_throw()
    }

    fn remove_value(&mut self, renderer: &mut Renderer) {
        self.set_default_value(renderer, "");
        self.set_value(renderer, "");
    }

    type OnValueChangeEventListenerUnpinned<F: HandleFormControlValue<str> + 'static> =
        unpinned::EventListenerOfType<HandleFormControlValueChange<str, F>, Input>;

    type OnValueChangeElementUnpinned = Self;

    fn on_value_change_element_unpinned(&mut self) -> &mut Self::OnValueChangeElementUnpinned {
        self
    }

    type OnValueChangeFUnpinned<F: HandleFormControlValue<str> + 'static> =
        HandleFormControlValueChange<str, F>;
}

impl<Renderer: ?Sized + csr::web::Renderer> FormControlElement<str, Renderer>
    for csr::web::Node<
        //
        web_sys::HtmlInputElement,
    >
{
    fn set_value(&mut self, _: &mut Renderer, value: &str) {
        self.0.set_value(value)
    }

    fn set_default_value(&mut self, _: &mut Renderer, value: &str) {
        self.0.set_default_value(value)
    }

    fn remove_value(&mut self, renderer: &mut Renderer) {
        self.set_default_value(renderer, "");
        self.set_value(renderer, "");
    }

    type OnValueChangeEventListenerUnpinned<F: HandleFormControlValue<str> + 'static> =
        unpinned::EventListenerOfType<HandleFormControlValueChange<str, F>, Input>;

    type OnValueChangeElementUnpinned = Self;

    fn on_value_change_element_unpinned(&mut self) -> &mut Self::OnValueChangeElementUnpinned {
        self
    }

    type OnValueChangeFUnpinned<F: HandleFormControlValue<str> + 'static> =
        HandleFormControlValueChange<str, F>;
}

impl<Renderer: ?Sized + csr::web::Renderer> FormControlElement<bool, Renderer>
    for csr::web::Node<
        //
        web_sys::HtmlInputElement,
    >
{
    fn set_value(&mut self, _: &mut Renderer, &value: &bool) {
        self.0.set_checked(value)
    }

    fn set_default_value(&mut self, _: &mut Renderer, &value: &bool) {
        self.0.set_default_checked(value)
    }

    fn remove_value(&mut self, renderer: &mut Renderer) {
        self.set_default_value(renderer, &false);
        self.set_value(renderer, &false);
    }

    type OnValueChangeEventListenerUnpinned<F: HandleFormControlValue<bool> + 'static> =
        unpinned::EventListenerOfType<HandleFormControlValueChange<bool, F>, Change>;

    type OnValueChangeElementUnpinned = Self;

    fn on_value_change_element_unpinned(&mut self) -> &mut Self::OnValueChangeElementUnpinned {
        self
    }

    type OnValueChangeFUnpinned<F: HandleFormControlValue<bool> + 'static> =
        HandleFormControlValueChange<bool, F>;
}

impl<Renderer: ?Sized + csr::web::Renderer> FormControlElement<f64, Renderer>
    for csr::web::Node<
        //
        web_sys::HtmlInputElement,
    >
{
    fn set_value(&mut self, _: &mut Renderer, &value: &f64) {
        self.0.set_value_as_number(value) // TODO: this might throw exception if input.type does not support number values
    }

    fn set_default_value(&mut self, _: &mut Renderer, &value: &f64) {
        super::super::input::web::set_default_value(&self.0, value);
    }

    fn remove_value(&mut self, renderer: &mut Renderer) {
        self.0.set_default_value("");
        self.set_value(renderer, &f64::NAN);
    }

    type OnValueChangeEventListenerUnpinned<F: HandleFormControlValue<f64> + 'static> =
        unpinned::EventListenerOfType<HandleFormControlValueChange<f64, F>, Input>;

    type OnValueChangeElementUnpinned = Self;

    fn on_value_change_element_unpinned(&mut self) -> &mut Self::OnValueChangeElementUnpinned {
        self
    }

    type OnValueChangeFUnpinned<F: HandleFormControlValue<f64> + 'static> =
        HandleFormControlValueChange<f64, F>;
}
