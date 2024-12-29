use std::marker::PhantomData;

use frender_events::event::Event;

use frender_dom::{behaviors, HandleEvent};

use super::value::{FormControlValueKind, HandleFormControlValue};

pub trait FormControlElement<V: ?Sized + FormControlValueKind, Renderer: ?Sized>:
    behaviors::HtmlElement<Renderer>
{
    fn set_default_value(&mut self, renderer: &mut Renderer, value: &V);
    fn set_value(&mut self, renderer: &mut Renderer, value: &V);

    fn remove_value(&mut self, renderer: &mut Renderer);

    type OnValueChangeEventListener<F: HandleFormControlValue<V> + 'static>: Default;

    fn on_value_change<F: HandleFormControlValue<V> + 'static>(
        &mut self,
        renderer: &mut Renderer,
        state: &mut Self::OnValueChangeEventListener<F>,
        f: F,
    );
}

#[derive(Debug)]
pub struct HandleFormControlValueChange<
    VK: ?Sized + FormControlValueKind,
    F: HandleFormControlValue<VK>,
> {
    f: F,
    _value_kind: PhantomData<VK>,
}

impl<VK: ?Sized + FormControlValueKind, F: HandleFormControlValue<VK>>
    HandleFormControlValueChange<VK, F>
{
    pub fn new(f: F) -> Self {
        Self {
            f,
            _value_kind: PhantomData,
        }
    }
}

pub trait HandleFormControlValueKind: FormControlValueKind {
    fn event_form_control_value<E: ?Sized + Event>(e: &E) -> Option<Self::FormControlValue<'_>>;
}

impl HandleFormControlValueKind for str {
    fn event_form_control_value<E: ?Sized + Event>(e: &E) -> Option<Self::FormControlValue<'_>> {
        e.target_form_control_value()
    }
}

impl HandleFormControlValueKind for bool {
    fn event_form_control_value<E: ?Sized + Event>(e: &E) -> Option<Self::FormControlValue<'_>> {
        e.target_input_checked()
    }
}

impl HandleFormControlValueKind for f64 {
    fn event_form_control_value<E: ?Sized + Event>(e: &E) -> Option<Self::FormControlValue<'_>> {
        e.target_input_value_as_number()
    }
}

impl<VK: ?Sized + HandleFormControlValueKind, F: HandleFormControlValue<VK>, E: ?Sized + Event>
    HandleEvent<E> for HandleFormControlValueChange<VK, F>
{
    fn handle_event(&mut self, e: &E) {
        if let Some(v) = VK::event_form_control_value(e) {
            self.f.handle_form_control_value(v)
        } else {
            // TODO: warn about unexpected event target
        }
    }
}

#[cfg(feature = "web")]
mod web {
    use frender_events::{
        event::Event, event_types::EventType, web::JsCastEventType, HasEventTypeName,
    };

    use crate::csr::web::event_listener::unpinned;

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
            crate::csr::web::Event::new_from_ref(event)
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
            crate::csr::web::Event::new_from_ref(event)
        }
    }

    impl<Renderer: ?Sized + crate::csr::web::Renderer> FormControlElement<str, Renderer>
        for crate::csr::web::Node<
            //
            web_sys::HtmlTextAreaElement,
        >
    {
        fn set_value(&mut self, _: &mut Renderer, value: &str) {
            AsRef::<web_sys::HtmlTextAreaElement>::as_ref(&self.0).set_value(value)
        }

        fn set_default_value(&mut self, _: &mut Renderer, value: &str) {
            use wasm_bindgen::UnwrapThrowExt;

            AsRef::<web_sys::HtmlTextAreaElement>::as_ref(&self.0)
                .set_default_value(value)
                .unwrap_throw()
        }

        fn remove_value(&mut self, renderer: &mut Renderer) {
            self.set_default_value(renderer, "");
            self.set_value(renderer, "");
        }

        type OnValueChangeEventListener<F: HandleFormControlValue<str> + 'static> =
            unpinned::MaybeEventListenerOfType<HandleFormControlValueChange<str, F>, Input>;

        fn on_value_change<F: HandleFormControlValue<str> + 'static>(
            &mut self,
            renderer: &mut Renderer,
            state: &mut Self::OnValueChangeEventListener<F>,
            f: F,
        ) {
            crate::RegisterOrUpdate::register_or_update(
                std::pin::Pin::new(state),
                self,
                renderer,
                HandleFormControlValueChange::new(f),
            )
        }
    }

    impl<Renderer: ?Sized + crate::csr::web::Renderer> FormControlElement<str, Renderer>
        for crate::csr::web::Node<
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

        type OnValueChangeEventListener<F: HandleFormControlValue<str> + 'static> =
            unpinned::MaybeEventListenerOfType<HandleFormControlValueChange<str, F>, Input>;

        fn on_value_change<F: HandleFormControlValue<str> + 'static>(
            &mut self,
            renderer: &mut Renderer,
            state: &mut Self::OnValueChangeEventListener<F>,
            f: F,
        ) {
            crate::RegisterOrUpdate::register_or_update(
                std::pin::Pin::new(state),
                self,
                renderer,
                HandleFormControlValueChange::new(f),
            )
        }
    }

    impl<Renderer: ?Sized + crate::csr::web::Renderer> FormControlElement<bool, Renderer>
        for crate::csr::web::Node<
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

        type OnValueChangeEventListener<F: HandleFormControlValue<bool> + 'static> =
            unpinned::MaybeEventListenerOfType<HandleFormControlValueChange<bool, F>, Change>;

        fn on_value_change<F: HandleFormControlValue<bool> + 'static>(
            &mut self,
            renderer: &mut Renderer,
            state: &mut Self::OnValueChangeEventListener<F>,
            f: F,
        ) {
            crate::RegisterOrUpdate::register_or_update(
                std::pin::Pin::new(state),
                self,
                renderer,
                HandleFormControlValueChange::new(f),
            )
        }
    }

    impl<Renderer: ?Sized + crate::csr::web::Renderer> FormControlElement<f64, Renderer>
        for crate::csr::web::Node<
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

        type OnValueChangeEventListener<F: HandleFormControlValue<f64> + 'static> =
            unpinned::MaybeEventListenerOfType<HandleFormControlValueChange<f64, F>, Input>;

        fn on_value_change<F: HandleFormControlValue<f64> + 'static>(
            &mut self,
            renderer: &mut Renderer,
            state: &mut Self::OnValueChangeEventListener<F>,
            f: F,
        ) {
            crate::RegisterOrUpdate::register_or_update(
                std::pin::Pin::new(state),
                self,
                renderer,
                HandleFormControlValueChange::new(f),
            )
        }
    }
}
