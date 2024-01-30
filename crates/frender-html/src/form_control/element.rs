use frender_events::event::Event;

use super::value::{TempAsRef, Value};

pub trait FormControlElement<V: ?Sized + Value, Renderer: ?Sized>: crate::html::behaviors::HtmlElement<Renderer> {
    fn set_default_value(&mut self, renderer: &mut Renderer, value: &V);
    fn set_value(&mut self, renderer: &mut Renderer, value: &V);

    fn remove_value(&mut self, renderer: &mut Renderer);

    /// When dropped, the form control's value should not be forced.
    type ForceValue;

    fn force_value<Val: TempAsRef<V> + 'static>(&mut self, renderer: &mut Renderer, value: Val) -> Self::ForceValue;

    type OnValueChangeEventListener;

    fn on_value_change(&mut self, renderer: &mut Renderer, f: impl for<'v> FnMut(V::Passed<'v>) + 'static) -> Self::OnValueChangeEventListener;
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

    fn set_default_value(&mut self, renderer: &mut Renderer, value: &str) {
        use frender_common::try_behavior::TryWithTryBehavior;

        AsRef::<web_sys::HtmlTextAreaElement>::as_ref(&self.0).set_default_value(value).unwrap_with_behavior(&mut renderer.try_behavior())
    }

    fn remove_value(&mut self, renderer: &mut Renderer) {
        self.set_default_value(renderer, "");
        self.set_value(renderer, "");
    }

    type ForceValue = <Self as frender_dom::behaviors::HtmlElement<Renderer>>::OnBeforeInputPreventDefault;

    fn force_value<Val: TempAsRef<str> + 'static>(&mut self, renderer: &mut Renderer, _: Val) -> Self::ForceValue {
        frender_dom::behaviors::HtmlElement::on_before_input_prevent_default(self, renderer)
    }

    type OnValueChangeEventListener = <Self as crate::html::behaviors::HtmlElement<Renderer>>::OnInputEventListener;

    fn on_value_change(&mut self, renderer: &mut Renderer, mut f: impl FnMut(std::borrow::Cow<'_, str>) + 'static) -> Self::OnValueChangeEventListener {
        crate::html::behaviors::HtmlElement::on_input(self, renderer, move |e| {
            if let Some(v) = e.target_form_control_value() {
                f(v)
            } else {
                // TODO: warn about unexpected event target
            }
        })
    }
}
