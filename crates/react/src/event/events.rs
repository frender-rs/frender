use super::{js, SyntheticEvent};

pub type UiEvent<
    TCurrent = js::native::Element,
    TNativeEvent = js::native::UiEvent,
    TBaseSyntheticEvent = js::UiEvent,
> = SyntheticEvent<TCurrent, TNativeEvent, TBaseSyntheticEvent>;

pub type ClipboardEvent<TCurrent = js::native::Element> =
    SyntheticEvent<TCurrent, js::native::ClipboardEvent, js::ClipboardEvent>;

pub type CompositionEvent<TCurrent = js::native::Element> =
    SyntheticEvent<TCurrent, js::native::CompositionEvent, js::CompositionEvent>;

pub type FocusEvent<TCurrent = js::native::Element> =
    SyntheticEvent<TCurrent, js::native::FocusEvent, js::FocusEvent>;

pub type FormEvent<TCurrent = js::native::Element> =
    SyntheticEvent<TCurrent, js::native::Event, js::FormEvent>;

pub type KeyboardEvent<TCurrent = js::native::Element> =
    SyntheticEvent<TCurrent, js::native::KeyboardEvent, js::KeyboardEvent>;

pub type MouseEvent<
    TCurrent = js::native::Element,
    TNativeEvent = js::native::MouseEvent,
    TBaseSyntheticEvent = js::MouseEvent,
> = UiEvent<TCurrent, TNativeEvent, TBaseSyntheticEvent>;

pub type DragEvent<TCurrent = js::native::Element> =
    MouseEvent<TCurrent, js::native::DragEvent, js::DragEvent>;

pub type PointerEvent<TCurrent = js::native::Element> =
    MouseEvent<TCurrent, js::native::PointerEvent, js::PointerEvent>;

pub type WheelEvent<TCurrent = js::native::Element> =
    MouseEvent<TCurrent, js::native::WheelEvent, js::WheelEvent>;

pub type TouchEvent<TCurrent = js::native::Element> =
    UiEvent<TCurrent, js::native::TouchEvent, js::TouchEvent>;

pub type AnimationEvent<TCurrent = js::native::Element> =
    SyntheticEvent<TCurrent, js::native::AnimationEvent, js::AnimationEvent>;

pub type TransitionEvent<TCurrent = js::native::Element> =
    SyntheticEvent<TCurrent, js::native::TransitionEvent, js::TransitionEvent>;

pub type ChangeEvent<TCurrent = js::native::Element> =
    SyntheticEvent<TCurrent, js::native::Event, js::ChangeEvent>;
