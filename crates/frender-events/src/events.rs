crate::event::wrap_events!(
    pub struct Event(web_sys::Event);
    pub struct SecurityPolicyViolationEvent(web_sys::SecurityPolicyViolationEvent);
    pub struct WheelEvent(web_sys::WheelEvent);
    pub struct CompositionEvent(web_sys::CompositionEvent);
    pub struct FocusEvent(web_sys::FocusEvent);
    pub struct KeyboardEvent(web_sys::KeyboardEvent);
    pub struct MouseEvent(web_sys::MouseEvent);
    pub struct TouchEvent(web_sys::TouchEvent);
    pub struct AnimationEvent(web_sys::AnimationEvent);
    pub struct InputEvent(web_sys::InputEvent);
    pub struct PointerEvent(web_sys::PointerEvent);
    pub struct TransitionEvent(web_sys::TransitionEvent);
);
