pub use render_state_trait::{RenderState, StateUnmount};

pub mod render;
pub mod render_state;

mod render_state_trait;

pub mod event_listener;

pin_project_lite::pin_project! {
    struct T<'a> {
        #[pin]
        a: &'a ()
    }
}
