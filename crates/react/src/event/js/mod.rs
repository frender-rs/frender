mod base;
mod events;
mod mouse;
mod ui;

pub use base::*;
pub use events::*;
pub use mouse::*;
pub use ui::*;

pub mod native {
    pub use web_sys::AnimationEvent;

    #[cfg(web_sys_unstable_apis)]
    pub use web_sys::ClipboardEvent;
    #[cfg(not(web_sys_unstable_apis))]
    pub use web_sys::Event as ClipboardEvent;

    pub use web_sys::CompositionEvent;
    pub use web_sys::DragEvent;
    pub use web_sys::Element;
    pub use web_sys::Event;
    pub use web_sys::EventTarget;
    pub use web_sys::FocusEvent;
    pub use web_sys::KeyboardEvent;
    pub use web_sys::MouseEvent;
    pub use web_sys::PointerEvent;
    pub use web_sys::TouchEvent;
    pub use web_sys::TransitionEvent;
    pub use web_sys::UiEvent;
    pub use web_sys::WheelEvent;
}
