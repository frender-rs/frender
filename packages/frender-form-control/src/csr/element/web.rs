use frender_events::{
    event::Event, event_types::EventType, web::JsCastEventType, HasEventTypeName,
};

use frender_dom::csr::{self, web::event_listener::unpinned};

use crate::value::{KindOfChecked, KindOfValue, KindOfValueAsNumber};

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

impl<Renderer: ?Sized + csr::web::Renderer> FormControlElement<KindOfValue, Renderer>
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

    fn remove_default_value(&mut self, renderer: &mut Renderer) {
        self.set_default_value(renderer, "")
    }

    fn remove_value(&mut self, renderer: &mut Renderer) {
        self.set_default_value(renderer, "");
        self.set_value(renderer, "");
    }

    type OnValueChangeEventListenerUnpinned<F: HandleFormControlValue<KindOfValue> + 'static> =
        unpinned::EventListenerOfType<HandleFormControlValueChange<KindOfValue, F>, Input>;

    type OnValueChangeElementUnpinned = Self;

    fn on_value_change_element_unpinned(&mut self) -> &mut Self::OnValueChangeElementUnpinned {
        self
    }

    type OnValueChangeFUnpinned<F: HandleFormControlValue<KindOfValue> + 'static> =
        HandleFormControlValueChange<KindOfValue, F>;
}

impl<Renderer: ?Sized + csr::web::Renderer> FormControlElement<KindOfValue, Renderer>
    for csr::web::Node<
        //
        web_sys::HtmlInputElement,
    >
{
    fn set_default_value(&mut self, _: &mut Renderer, value: &str) {
        self.0.set_default_value(value)
    }

    fn remove_default_value(&mut self, renderer: &mut Renderer) {
        <Self as FormControlElement<KindOfValue, Renderer>>::set_default_value(self, renderer, "");
    }

    fn set_value(&mut self, _: &mut Renderer, value: &str) {
        self.0.set_value(value)
    }

    fn remove_value(&mut self, renderer: &mut Renderer) {
        <Self as FormControlElement<KindOfValue, Renderer>>::set_default_value(self, renderer, "");
        <Self as FormControlElement<KindOfValue, Renderer>>::set_value(self, renderer, "");
    }

    type OnValueChangeEventListenerUnpinned<F: HandleFormControlValue<KindOfValue> + 'static> =
        unpinned::EventListenerOfType<HandleFormControlValueChange<KindOfValue, F>, Input>;

    type OnValueChangeElementUnpinned = Self;

    fn on_value_change_element_unpinned(&mut self) -> &mut Self::OnValueChangeElementUnpinned {
        self
    }

    type OnValueChangeFUnpinned<F: HandleFormControlValue<KindOfValue> + 'static> =
        HandleFormControlValueChange<KindOfValue, F>;
}

impl<Renderer: ?Sized + csr::web::Renderer> FormControlElement<KindOfChecked, Renderer>
    for csr::web::Node<
        //
        web_sys::HtmlInputElement,
    >
{
    fn set_default_value(&mut self, _: &mut Renderer, value: bool) {
        self.0.set_default_checked(value)
    }

    fn remove_default_value(&mut self, renderer: &mut Renderer) {
        <Self as FormControlElement<KindOfChecked, Renderer>>::set_default_value(
            self, renderer, false,
        )
    }

    fn set_value(&mut self, _: &mut Renderer, value: bool) {
        self.0.set_checked(value)
    }

    fn remove_value(&mut self, renderer: &mut Renderer) {
        <Self as FormControlElement<KindOfChecked, Renderer>>::set_default_value(
            self, renderer, false,
        );
        <Self as FormControlElement<KindOfChecked, Renderer>>::set_value(self, renderer, false);
    }

    type OnValueChangeEventListenerUnpinned<F: HandleFormControlValue<KindOfChecked> + 'static> =
        unpinned::EventListenerOfType<HandleFormControlValueChange<KindOfChecked, F>, Change>;

    type OnValueChangeElementUnpinned = Self;

    fn on_value_change_element_unpinned(&mut self) -> &mut Self::OnValueChangeElementUnpinned {
        self
    }

    type OnValueChangeFUnpinned<F: HandleFormControlValue<KindOfChecked> + 'static> =
        HandleFormControlValueChange<KindOfChecked, F>;
}

impl<Renderer: ?Sized + csr::web::Renderer> FormControlElement<KindOfValueAsNumber, Renderer>
    for csr::web::Node<
        //
        web_sys::HtmlInputElement,
    >
{
    fn set_default_value(&mut self, _: &mut Renderer, value: f64) {
        crate::input::web::set_default_value(&self.0, value);
    }

    fn remove_default_value(&mut self, _: &mut Renderer) {
        self.0.set_default_value("")
    }

    fn set_value(&mut self, _: &mut Renderer, value: f64) {
        self.0.set_value_as_number(value) // TODO: this might throw exception if input.type does not support number values
    }

    fn remove_value(&mut self, _: &mut Renderer) {
        self.0.set_default_value("");
        self.0.set_value_as_number(f64::NAN);
    }

    type OnValueChangeEventListenerUnpinned<
        F: HandleFormControlValue<KindOfValueAsNumber> + 'static,
    > = unpinned::EventListenerOfType<HandleFormControlValueChange<KindOfValueAsNumber, F>, Input>;

    type OnValueChangeElementUnpinned = Self;

    fn on_value_change_element_unpinned(&mut self) -> &mut Self::OnValueChangeElementUnpinned {
        self
    }

    type OnValueChangeFUnpinned<F: HandleFormControlValue<KindOfValueAsNumber> + 'static> =
        HandleFormControlValueChange<KindOfValueAsNumber, F>;
}
