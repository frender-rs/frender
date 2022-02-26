use js_sys::Object;
use wasm_bindgen::prelude::*;

use super::BaseSyntheticEvent;

#[wasm_bindgen]
extern "C" {
    /// React synthetic ClipboardEvent
    #[wasm_bindgen(extends = Object, extends = BaseSyntheticEvent)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type ClipboardEvent;
    #[wasm_bindgen(method, getter = clipboardData)]
    pub fn clipboard_data(this: &ClipboardEvent) -> web_sys::DataTransfer;

    /// React synthetic CompositionEvent
    #[wasm_bindgen(extends = Object, extends = BaseSyntheticEvent)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type CompositionEvent;
    #[wasm_bindgen(method, getter)]
    pub fn data(this: &CompositionEvent) -> String;

    /// React synthetic FocusEvent
    #[wasm_bindgen(extends = Object, extends = BaseSyntheticEvent)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type FocusEvent;
    #[wasm_bindgen(method, getter = relatedTarget)]
    pub fn related_target(this: &FocusEvent) -> Option<web_sys::EventTarget>;

    /// React synthetic FormEvent
    #[wasm_bindgen(extends = Object, extends = BaseSyntheticEvent)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type FormEvent;

    /// React synthetic KeyboardEvent
    #[wasm_bindgen(extends = Object, extends = BaseSyntheticEvent)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type KeyboardEvent;
    #[wasm_bindgen(method, getter = altKey)]
    pub fn alt_key(this: &KeyboardEvent) -> bool;
    #[wasm_bindgen(method, getter = charCode)]
    #[deprecated]
    pub fn char_code(this: &KeyboardEvent) -> u32;
    #[wasm_bindgen(method, getter = ctrlKey)]
    pub fn ctrl_key(this: &KeyboardEvent) -> bool;
    #[wasm_bindgen(method, getter)]
    pub fn code(this: &KeyboardEvent) -> String;
    /// See [DOM Level 3 Events spec](https://www.w3.org/TR/uievents-key/#keys-modifier). for a list of valid (case-sensitive) arguments to this method.
    #[wasm_bindgen(method, js_name = getModifierState)]
    pub fn get_modifier_state(this: &KeyboardEvent, key: &str) -> bool;
    /// See the [DOM Level 3 Events spec](https://www.w3.org/TR/uievents-key/#named-key-attribute-values). for possible values
    #[wasm_bindgen(method, getter)]
    pub fn key(this: &KeyboardEvent) -> String;
    #[wasm_bindgen(method, getter = keyCode)]
    #[deprecated]
    pub fn key_code(this: &KeyboardEvent) -> u32;
    #[wasm_bindgen(method, getter)]
    pub fn locale(this: &KeyboardEvent) -> String;
    #[wasm_bindgen(method, getter)]
    pub fn location(this: &KeyboardEvent) -> u32;
    #[wasm_bindgen(method, getter = metaKey)]
    pub fn meta_key(this: &KeyboardEvent) -> bool;
    #[wasm_bindgen(method, getter)]
    pub fn repeat(this: &KeyboardEvent) -> bool;
    #[wasm_bindgen(method, getter = shiftKey)]
    pub fn shift_key(this: &KeyboardEvent) -> bool;
    #[wasm_bindgen(method, getter)]
    #[deprecated]
    pub fn which(this: &KeyboardEvent) -> u32;

    /// React synthetic TouchEvent
    #[wasm_bindgen(extends = Object, extends = BaseSyntheticEvent, extends = super::UiEvent)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type TouchEvent;
    #[wasm_bindgen(method, getter = altKey)]
    pub fn alt_key(this: &TouchEvent) -> bool;
    #[wasm_bindgen(method, getter = changedTouches)]
    pub fn changed_touches(this: &TouchEvent) -> web_sys::TouchList;
    #[wasm_bindgen(method, getter = ctrlKey)]
    pub fn ctrl_key(this: &TouchEvent) -> bool;
    /// See [DOM Level 3 Events spec](https://www.w3.org/TR/uievents-key/#keys-modifier). for a list of valid (case-sensitive) arguments to this method.
    #[wasm_bindgen(method, js_name = getModifierState)]
    pub fn get_modifier_state(this: &TouchEvent, key: &str) -> bool;
    #[wasm_bindgen(method, getter = metaKey)]
    pub fn meta_key(this: &TouchEvent) -> bool;
    #[wasm_bindgen(method, getter = shiftKey)]
    pub fn shift_key(this: &TouchEvent) -> bool;
    #[wasm_bindgen(method, getter = targetTouches)]
    pub fn target_touches(this: &TouchEvent) -> web_sys::TouchList;
    #[wasm_bindgen(method, getter)]
    pub fn touches(this: &TouchEvent) -> web_sys::TouchList;

    /// React synthetic DragEvent
    #[wasm_bindgen(extends = Object, extends = BaseSyntheticEvent, extends = super::UiEvent, extends = super::MouseEvent)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type DragEvent;
    #[wasm_bindgen(method, getter = dataTransfer)]
    pub fn data_transfer(this: &DragEvent) -> web_sys::DataTransfer;

    /// React synthetic DragEvent
    #[wasm_bindgen(extends = Object, extends = BaseSyntheticEvent, extends = super::UiEvent, extends = super::MouseEvent)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type PointerEvent;
    #[wasm_bindgen(method, getter = pointerId)]
    pub fn pointer_id(this: &PointerEvent) -> i32;
    #[wasm_bindgen(method, getter)]
    pub fn pressure(this: &PointerEvent) -> f32;
    #[wasm_bindgen(method, getter = tangentialPressure)]
    pub fn tangential_pressure(this: &PointerEvent) -> f32;
    #[wasm_bindgen(method, getter = tiltX)]
    pub fn tilt_x(this: &PointerEvent) -> i32;
    #[wasm_bindgen(method, getter = tiltY)]
    pub fn tilt_y(this: &PointerEvent) -> i32;
    #[wasm_bindgen(method, getter)]
    pub fn twist(this: &PointerEvent) -> i32;
    #[wasm_bindgen(method, getter)]
    pub fn width(this: &PointerEvent) -> i32;
    #[wasm_bindgen(method, getter)]
    pub fn height(this: &PointerEvent) -> i32;
    #[wasm_bindgen(method, getter = pointerType)]
    pub fn pointer_type(this: &PointerEvent) -> String;
    #[wasm_bindgen(method, getter = isPrimary)]
    pub fn is_primary(this: &PointerEvent) -> bool;

    /// React synthetic WheelEvent
    #[wasm_bindgen(extends = Object, extends = BaseSyntheticEvent, extends = super::UiEvent, extends = super::MouseEvent)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type WheelEvent;
    #[wasm_bindgen(method, getter = deltaMode)]
    pub fn delta_mode(this: &WheelEvent) -> u32;
    #[wasm_bindgen(method, getter = deltaX)]
    pub fn delta_x(this: &WheelEvent) -> f64;
    #[wasm_bindgen(method, getter = deltaY)]
    pub fn delta_y(this: &WheelEvent) -> f64;
    #[wasm_bindgen(method, getter = deltaZ)]
    pub fn delta_z(this: &WheelEvent) -> f64;

    /// React synthetic AnimationEvent
    #[wasm_bindgen(extends = Object, extends = BaseSyntheticEvent)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type AnimationEvent;
    #[wasm_bindgen(method, getter = animationName)]
    pub fn animation_name(this: &AnimationEvent) -> String;
    #[wasm_bindgen(method, getter = elapsedTime)]
    pub fn elapsed_time(this: &AnimationEvent) -> f32;
    #[wasm_bindgen(method, getter = pseudoElement)]
    pub fn pseudo_element(this: &AnimationEvent) -> String;

    /// React synthetic TransitionEvent
    #[wasm_bindgen(extends = Object, extends = BaseSyntheticEvent)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type TransitionEvent;
    #[wasm_bindgen(method, getter = elapsedTime)]
    pub fn elapsed_time(this: &TransitionEvent) -> f32;
    #[wasm_bindgen(method, getter = propertyName)]
    pub fn property_name(this: &TransitionEvent) -> String;
    #[wasm_bindgen(method, getter = pseudoElement)]
    pub fn pseudo_element(this: &TransitionEvent) -> String;

    /// React synthetic ChangeEvent
    #[wasm_bindgen(extends = Object, extends = BaseSyntheticEvent)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type ChangeEvent;
}
