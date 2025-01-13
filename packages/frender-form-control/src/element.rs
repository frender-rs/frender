use std::marker::PhantomData;

use frender_events::event::Event;

use frender_dom::{behaviors, HandleEvent, RegisterUpdate};

use super::value::{FormControlValueKind, HandleFormControlValue};

pub trait FormControlElement<V: ?Sized + FormControlValueKind, Renderer: ?Sized>:
    behaviors::HtmlElement<Renderer>
{
    fn set_default_value(&mut self, renderer: &mut Renderer, value: &V);
    fn set_value(&mut self, renderer: &mut Renderer, value: &V);

    fn remove_value(&mut self, renderer: &mut Renderer);

    type OnValueChangeEventListenerUnpinned<F: HandleFormControlValue<V> + 'static>: RegisterUpdate<
        Self::OnValueChangeElementUnpinned,
        Renderer,
        Self::OnValueChangeFUnpinned<F>,
    >;

    type OnValueChangeElementUnpinned;

    fn on_value_change_element_unpinned(&mut self) -> &mut Self::OnValueChangeElementUnpinned;

    type OnValueChangeFUnpinned<F: HandleFormControlValue<V> + 'static>: From<F>;
}

#[derive(Debug)]
pub struct HandleFormControlValueChange<
    VK: ?Sized + FormControlValueKind,
    F: HandleFormControlValue<VK>,
> {
    f: F,
    _value_kind: PhantomData<VK>,
}

impl<VK: ?Sized + FormControlValueKind, F: HandleFormControlValue<VK>> From<F>
    for HandleFormControlValueChange<VK, F>
{
    fn from(value: F) -> Self {
        Self::new(value)
    }
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
mod web;
