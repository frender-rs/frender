use frender_events::event::Event;

use super::value::{HandleValue, TempAsRef, Value};

pub trait FormControlElement<V: ?Sized + Value, Renderer: ?Sized>: crate::html::behaviors::HtmlElement<Renderer> {
    fn set_default_value(&mut self, renderer: &mut Renderer, value: &V);
    fn set_value(&mut self, renderer: &mut Renderer, value: &V);

    fn remove_value(&mut self, renderer: &mut Renderer);

    /// When dropped, the form control's value should not be forced.
    type ForceValue;

    fn force_value<Val: TempAsRef<V> + 'static>(&mut self, renderer: &mut Renderer, value: Val) -> Self::ForceValue;

    type OnValueChangeEventListener<F: HandleValue<V> + 'static>: Default;

    fn on_value_change<F: HandleValue<V> + 'static>(&mut self, renderer: &mut Renderer, state: &mut Self::OnValueChangeEventListener<F>, f: F);
}

#[derive(Debug)]
pub struct HandleEventTargetFormControlValue<F: HandleValue<str>>(pub F);

impl<F: HandleValue<str>, E: ?Sized + Event> frender_dom::HandleEvent<E> for HandleEventTargetFormControlValue<F> {
    fn handle_event(&mut self, e: &E) {
        if let Some(v) = e.target_form_control_value() {
            self.0.handle_value(v)
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

    type ForceValue = <Self as frender_dom::behaviors::HtmlElement<Renderer>>::OnBeforeInputPreventDefault;

    fn force_value<Val: TempAsRef<str> + 'static>(&mut self, renderer: &mut Renderer, _: Val) -> Self::ForceValue {
        frender_dom::behaviors::HtmlElement::on_before_input_prevent_default(self, renderer)
    }

    type OnValueChangeEventListener<F: HandleValue<str> + 'static> = crate::html::event_type_helpers::on_input::UnpinnedEventListenerOf<Self, Renderer, HandleEventTargetFormControlValue<F>>;

    fn on_value_change<F: HandleValue<str> + 'static>(&mut self, renderer: &mut Renderer, state: &mut Self::OnValueChangeEventListener<F>, f: F) {
        frender_dom::RegisterOrUpdate::register_or_update(std::pin::Pin::new(state), self, renderer, HandleEventTargetFormControlValue(f))
    }
}
