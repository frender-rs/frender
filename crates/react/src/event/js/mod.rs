mod base;
mod mouse;

pub use base::*;
pub use mouse::*;

pub mod native {
    pub use web_sys::Element;
    pub use web_sys::Event;
    pub use web_sys::EventTarget;
    pub use web_sys::MouseEvent;
    pub use web_sys::UiEvent;
}
