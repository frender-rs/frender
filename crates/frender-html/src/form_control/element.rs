use frender_events::event::Event;

use super::value::{HandleFormControlValue, FormControlValueKind};

pub trait FormControlElement<V: ?Sized + FormControlValueKind, Renderer: ?Sized>: crate::html::behaviors::HtmlElement<Renderer> {
    fn set_default_value(&mut self, renderer: &mut Renderer, value: &V);
    fn set_value(&mut self, renderer: &mut Renderer, value: &V);

    fn remove_value(&mut self, renderer: &mut Renderer);

    type OnValueChangeEventListener<F: HandleFormControlValue<V> + 'static>: Default;

    fn on_value_change<F: HandleFormControlValue<V> + 'static>(&mut self, renderer: &mut Renderer, state: &mut Self::OnValueChangeEventListener<F>, f: F);
}

#[derive(Debug)]
pub struct HandleEventTargetFormControlValue<F: HandleFormControlValue<str>>(pub F);

impl<F: HandleFormControlValue<str>, E: ?Sized + Event> frender_dom::HandleEvent<E> for HandleEventTargetFormControlValue<F> {
    fn handle_event(&mut self, e: &E) {
        if let Some(v) = e.target_form_control_value() {
            self.0.handle_form_control_value(v)
        } else {
            // TODO: warn about unexpected event target
        }
    }
}

#[derive(Debug)]
pub struct HandleEventTargetInputChecked<F: HandleFormControlValue<bool>>(pub F);

impl<F: HandleFormControlValue<bool>, E: ?Sized + Event> frender_dom::HandleEvent<E> for HandleEventTargetInputChecked<F> {
    fn handle_event(&mut self, e: &E) {
        if let Some(v) = e.target_input_checked() {
            self.0.handle_form_control_value(v)
        } else {
            // TODO: warn about unexpected event target
        }
    }
}

#[derive(Debug)]
pub struct HandleEventTargetInputValueAsNumber<F: HandleFormControlValue<f64>>(pub F);

impl<F: HandleFormControlValue<f64>, E: ?Sized + Event> frender_dom::HandleEvent<E> for HandleEventTargetInputValueAsNumber<F> {
    fn handle_event(&mut self, e: &E) {
        if let Some(v) = e.target_input_value_as_number() {
            self.0.handle_form_control_value(v)
        } else {
            // TODO: warn about unexpected event target
        }
    }
}

#[cfg(feature = "web")]
impl<Renderer: ?Sized + frender_dom::csr::web::Renderer> FormControlElement<str, Renderer>
    for frender_dom::csr::web::Node<
        //
        web_sys::HtmlTextAreaElement,
    >
{
    fn set_value(&mut self, _: &mut Renderer, value: &str) {
        AsRef::<web_sys::HtmlTextAreaElement>::as_ref(&self.0).set_value(value)
    }

    fn set_default_value(&mut self, _: &mut Renderer, value: &str) {
        use wasm_bindgen::UnwrapThrowExt;

        AsRef::<web_sys::HtmlTextAreaElement>::as_ref(&self.0).set_default_value(value).unwrap_throw()
    }

    fn remove_value(&mut self, renderer: &mut Renderer) {
        self.set_default_value(renderer, "");
        self.set_value(renderer, "");
    }

    type OnValueChangeEventListener<F: HandleFormControlValue<str> + 'static> = crate::html::event_type_helpers::on_input::UnpinnedEventListenerOf<Self, Renderer, HandleEventTargetFormControlValue<F>>;

    fn on_value_change<F: HandleFormControlValue<str> + 'static>(&mut self, renderer: &mut Renderer, state: &mut Self::OnValueChangeEventListener<F>, f: F) {
        frender_dom::RegisterOrUpdate::register_or_update(std::pin::Pin::new(state), self, renderer, HandleEventTargetFormControlValue(f))
    }
}

#[cfg(feature = "web")]
mod web {
    use super::*;

    impl<Renderer: ?Sized + frender_dom::csr::web::Renderer> FormControlElement<str, Renderer>
        for frender_dom::csr::web::Node<
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

        type OnValueChangeEventListener<F: HandleFormControlValue<str> + 'static> = crate::html::event_type_helpers::on_input::UnpinnedEventListenerOf<Self, Renderer, HandleEventTargetFormControlValue<F>>;

        fn on_value_change<F: HandleFormControlValue<str> + 'static>(&mut self, renderer: &mut Renderer, state: &mut Self::OnValueChangeEventListener<F>, f: F) {
            frender_dom::RegisterOrUpdate::register_or_update(std::pin::Pin::new(state), self, renderer, HandleEventTargetFormControlValue(f))
        }
    }

    impl<Renderer: ?Sized + frender_dom::csr::web::Renderer> FormControlElement<bool, Renderer>
        for frender_dom::csr::web::Node<
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

        type OnValueChangeEventListener<F: HandleFormControlValue<bool> + 'static> = crate::html::event_type_helpers::on_change::UnpinnedEventListenerOf<Self, Renderer, HandleEventTargetInputChecked<F>>;

        fn on_value_change<F: HandleFormControlValue<bool> + 'static>(&mut self, renderer: &mut Renderer, state: &mut Self::OnValueChangeEventListener<F>, f: F) {
            frender_dom::RegisterOrUpdate::register_or_update(std::pin::Pin::new(state), self, renderer, HandleEventTargetInputChecked(f))
        }
    }

    impl<Renderer: ?Sized + frender_dom::csr::web::Renderer> FormControlElement<f64, Renderer>
        for frender_dom::csr::web::Node<
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

        type OnValueChangeEventListener<F: HandleFormControlValue<f64> + 'static> = crate::html::event_type_helpers::on_change::UnpinnedEventListenerOf<Self, Renderer, HandleEventTargetInputValueAsNumber<F>>;

        fn on_value_change<F: HandleFormControlValue<f64> + 'static>(&mut self, renderer: &mut Renderer, state: &mut Self::OnValueChangeEventListener<F>, f: F) {
            frender_dom::RegisterOrUpdate::register_or_update(std::pin::Pin::new(state), self, renderer, HandleEventTargetInputValueAsNumber(f))
        }
    }
}
